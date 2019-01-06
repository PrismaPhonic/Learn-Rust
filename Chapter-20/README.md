# Chapter 20

# Table of Contents
1. [Single Threaded Web Server](#single-threaded-web-server)

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


