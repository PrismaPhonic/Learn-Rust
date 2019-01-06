# Chapter 20

# Table of Contents
1. [Single Threaded Web Server](#single-threaded-web-server)
   1. [Writing a Response](#writing-a-response)
   2. [Selective Routes](#selective-routes)
2. [Going Multithreaded](#going-multithreaded)
   1. [Simulating Slowness](#simulating-slowness)

# Web Server Project

For the last chapter of the book we build a multiple-threaded web server, but we
have to walk before we can crawl!  We'll start out by building a simple single
threaded web server from the ground up. Like the other project heavy sections of
this book I'll mostly be posting large blocks of code and explaining what went
on - the book already does a great job of stepping you through every single
piece of the build out.

## Single Threaded Web Server

Let's build a very simple single threaded web server.  For now we will just
implement the ability to read an http request over TCP, but not respond to it:

```rust
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
```

Let's look at what's going on here. First wee use `TcpListener` to setup a
listener on port `7878` of localhost. The `bind` function returns a new
`TcpListener` instance (like new would) but it binds itself to the network port.
The bind function actually returns a `Result<T, E>` because the binding might
fail (if we don't have access to that port for instance) and so we have to
`unwrap` it to get the successful result which would be a new bound TcpListener
instance. 

The `incoming` method on `TcpListener` instance we got back returns an iterator
over a sequence of `TcpStream` types. A single _stream_ represents and open
connection between the client and server. A _connection_ is the full request and
response cycle. This for loop then will allow us to iterate through these
connections to see what requests were sent over. A stream might have errors so
we have to `unwrap` that too! We then pass on the successful stream (because we
haven't panicked) to our `handle_connection` function.

`handle_connection` takes a mutable stream even though we are only reading from
it - that's because the stream is active - it doesn't always come all at once so
it might be changed over time as more data is actively streamed.

We setup a mutable buffer of 512 bytes and we read from our stream and write to
this buffer (that's enough to handle each request).  We then print out the
request using `String::from_utf8_lossy` by giving it a full slice of the buffer.

`String::from_utf8_lossy` is a function that takes `&[u8]` and produces a
`String` from it.  The  lossy part just meant if it finds an invalid UTF-8
sequence it will replace it with this odd question mark character thing. 

If we run our server and type `127.0.0.1:7878` into our web browser we will see the http request in
our terminal that the browser is making:

```
Request: GET / HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:65.0) Gecko/20100101 Firefox/65.0
Accept:
text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8
Accept-Language: en-US,en;q=0.5
Accept-Encoding: gzip, deflate
Connection: keep-alive
Upgrade-Insecure-Requests: 1
```

What does all this mean?  Well, this is the http request which has this general
format:

```
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

In our case the first line contains the method `GET` and then the Request-URI
which is the root address of `/`. We then get the HTTP-Version of `1.1` and
`CRLF` basically stands for carriage return (a new-line character). The rest of
this request is just the headers because we didn't send a message body. We can
send a message body with curl!

If we curl our single threaded web server with a command line we will get a much
shorter web request:

```
Request: GET / HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: curl/7.63.0
Accept: */*
```

We still have just a simple get request so the first line is the same, but our
headers are much shorter now.

We can instead query our server with a POST request and some json to actually
see what's in the message body:

```
Request: POST / HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: curl/7.63.0
Accept: */*
Content-Type: application/json
Content-Length: 17

{"key1":"value1"}
```

Now we see that our body includes this json - and we have all three sections of
an http request!

Next we will write a response.

## Writing a Response

A response has the following format:

```
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```

We can write a mini success response with no message-body or headers like such:

```
HTTP/1.1 200 OK\r\n\r\n
```

We've specified an HTTP version, a status code of 200, a reason of OK and then a
carriage return.  We can include this in our code so if we don't fail to read
the incoming stream we respond with a success:

```rust
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

We define the response as an `&str` of the response string. We call `as_bytes`
on our response so it converts it to a `&[u8]` that `stream` can take and write
with.  We then use `flush` on the stream to make our program wait on this line
until the stream finishes writing (otherwise our function may end before it
finishes).

If we pull up the network tab of our dev tools in our browser and go to the same
web address again of `127.0.0.1:7878` we will see a 200 status code response!
success!

Now let's return some real HTML.  

First we'll write a simple hello message:

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>
```

We now will modify our `handle_connection` function:

```rust
use std::fs;
// --snip--

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("hello.html").unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

We have brought in `std::fs` which stands for filesystem. From there we read the
contents of a file in our root directory (the html file we made) and parse it to
a string. Then we simply tack it onto the end of our html response acting as our
body.  Now if we go to the address in our browser we'll see a hello message
rendered from html!

## Selective Routes

Right now our web server always responds with the same thing regardless of the
route we go to.  Let's set it up so we only respond with the `hello.html` when
we get a `/` GET request and otherwise respond with a 404 status code and an
error page:

```rust
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
```

What we've done here is create a **byte string** (with b in front of our string
literal syntax) of the initial part of a GET to `/` request.  Then we check if
the buffer starts with that byte string (since the buffer is also in bytes
right now) and if so we create a tuple of the response code along with the file
location for a success - otherwise we do the same but the response code for not
found and the error page.  Our if **expression** returns a result so we
destructure the tuple into variables `status_line` and `filename`.

Then we do what we've always done and just read that file into a string and
format it together with the statusline - and then write the response back to the
string and wait for the stream to end (the request/response cycle to finish).

Lastly we just need to build out our error page - I've written it with a helpful
link back to the homepage:

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <meta http-equiv="X-UA-Compatible" content="ie=edge">
  <title>Hello World!</title>
</head>
<body>
  <h1>Oops!</h1>
  <p>Sorry, there's no resource at this web address. Go <a href="/">home?</a></p>
</body>
</html>
```

and we name this `404.html`.  Now if we go to `/` we get our hello message and
any other address we get our 404 page linking us back to our home address!

Now that we've gotten our very simple http server built out in a single threaded
application, let's look at how to turn it into a multi-threaded web server.

# Going Multithreaded

Right now our server is singlethreaded.  Let's simulate a slow request to see
why multithreading might be useful and then convert to a multithreaded server.


## Simulating Slowness

I thought this example from the book was a bit silly - but it tries to show that
when the server hangs from one long request (on a single threaded server) that
it can hold up all other requests. This isn't completely true because you could
have a single threaded Node.js server where nearly all operations are async
(non-blocking) and you would still get very fast response. Rust can do both
async **and** multithreading though so why not use both?  Here's the code to
simulate a lockup:

```rust
use std::thread;
use std::time::Duration;
// --snip--

fn handle_connection(mut stream: TcpStream) {
    // --snip--

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // --snip--
}
```

We can simulate this slowness by opening up two tabs in our browser and going to
`/sleep` and then in another tab going to `/`.  Because the `/sleep` request is
taking so long (and we aren't taking advantage of async) it locks up loading
time on the other tab. Let's resolve this by going multithreaded.


