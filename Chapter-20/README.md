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


## Building our Own Threadpool

The rest of this section is very dense. I'm going to post the final code and try
to walk through it. Before we do that though we need to restructure our
project. I called my project `hello_webserver` so my imports will match that.
First we create a library file at `src/lib.rs`.  Then we'll move our
`src/main.rs` into `src/bin/main.rs` (create the bin folder first).  Alright,
now let's lay out all the steps we want to accomplish before we dive into the
finished code:

1. Define a `Worker` struct that holds an `id` and a `JoinHandle<()>`
2. Change `ThreadPool` to hold a vector of `Worker` instances.
3. Define a `Worker::new` function that takes an `id` number and returns a
   `Worker` instance that holds the `id` and a thread spawned with an empty
closure.
4. In `ThreadPool::new` use the `for` loop counter to generate an `id`, create a
   new `Worker` with that `id`, and store the worker in a new vector.
5. Setup the `ThreadPool` so it creates a channel and holds onto the sending
   side of the channel
6. Each Receiver will need to be wrapped in an Arc (shared ownership that's
   thread safe) and a Mutex (for inner mutability) and then held onto by each
worker.  We will send the closures down these channels as jobs to be called
7. We'll create a new Job struct that will hold the closures we want to send
   down the channel
8. In it's thread, the Worker will loop over the receiving side of the channel
   trying to get a lock on the Mutex - once it has it, it grabs a job and
immediately releases the lock before executing it's job.

Alright, thats a lot and probably will make more sense once you see the code and
we walk through it:

```rust
use std::thread;
use std::sync::{Arc, Mutex, mpsc};

pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: mpsc::Sender<Job>,
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = rx.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                job.call_box();
            }
        });

        Worker {
            id,
            thread: thread,
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (tx, rx) = mpsc::channel();

        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }

        ThreadPool { workers, tx }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.tx.send(job).unwrap();
    }
}
```

Alright, let's walk through this code. We bring in `thread` (so we can spawn
them) into scope and then from `sync` we bring in `Arc`, `Mutex`, and `mpsc`.

We have ouor `ThreadPool` struct and that has a field of `workers` which is a
vector of `Worker` structs, and then a transmitter which is of the type
`mpsc::Sender<Job>`.  the type of all `mpsc` tx's are `mpsc::Sender<T>` and in
this case our sender will be sending a `Job`.

Let's skip pass the confusing `FnBox` stuff and honestly ignore that. It's a
hacky solution to what I would consider to be a bug in the compiler.

We get down then to `Job` which is just a type alias for a `Box<dyn FnBox + Send
+ 'static>`.  We essentially stole this signature from `thread::spawn` API
  (since we'll be managing our own threads manually in a pool). But basically
whatever the box points at needs to have a static lifetime since the thread
might outlive whatever it was called with, and send must be enabled so we can
send stuff to other threads.  `FnBox` is just a hacky way that we can implement
`call_box` which allows us to invoke the closure we are sending as a job inside
the thread itself once it's aquired a job.

We then have a `Worker` struct which has an id and a thread.  The thread type of
`JoinHandle` is the type all returned threads have.  Our `new` method on Worker
first creates a thread where we move the receiver into it - that's fine becausee
we cloned it with Arc::clone so we can take ownership of this clone.  We then
get a lock on the mutex and once we do we call `recv` which is a method on the
`mpsc` receiver to get whatever message was being sent down to the receiver. 

We print out that we got a job and our thread id (now we see where the id comes
into play) and lastly we invoke that `call_box` method onn the job which just
invokes the closure for us stored in the job.  

That was all just for our thread (and yeah, we loop infinitely so our thread is
looking for work non-stop).  We lastly return a new worker with the id passed to
new and the created thread ready to start working for us non-stop.

We finally get to our `impl` for `ThreadPool`.  We have a `new` method which
returns a `ThreadPool`.  We first assert that the size being requested for a new
`ThreadPool` is not zero since that would be pointless. Then we get our
transmitter and receiver.  We shadow our receiver with one wrapped in a `Mutex`
(for threadsafe interior mutability) and then an `Arc` (for shared ownership
that is threadsafe) since if we remember we can only have one rx.  Arc::clone in
this case does not actually clone the receiver but just increases the strong
count.

We then create workers as a vector with a known capacity (for efficiency reasons) of the usize supplied. We have a loop that creates our id's and pushes new workers into that workers vec which we finally put into our new ThreadPool along with our single transmitter and return it!

Boy that was a lot! and we still aren't done.

So we lastly have an execute method on the threadpool that takes the threadpool
itself (which has our transmitter stored in a field) and a function. We specify
that this function must implement FnOnce() (onetime closure with no input or
return), Send trait (so we can send data between threads) and static lifetime.
We create a new job by putting our closure into a box and then sending it down
our channel where it will be received by whatever available thread currently has
a lock on the mutex.  they will receive the job, store it, release the lock and
then run a method that will invoke the closure.

Phew! Ok, finally done with our library! Now we just need to modify our
`main.rs` to call our threadpool:

```rust
use hello_webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
```

We import our `ThreadPool` and then instantiate it with how many threads we want
to use.  As we loop over our streams we can do a `pool.execute` on each stream
and then create our closure which will be our `handle_connection` function with
the stream passed to it.  Now we won't exceed the threads on our system because
we are limiting ourselves to just 4 threads. If there's more work than threads
the remaining work will sit on the stack waiting for a free thread to receive
it.
