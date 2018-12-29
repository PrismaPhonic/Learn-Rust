# Chapter 16

# Table of Contents
1. [Fearless Concurrency](#fearless-concurrency)
2. [Spawning Threads](#spawning-threads)
3. [Using move Closures with Threads](#using-move-closures-with-threads)
4. [Message Passing Between Threads](#message-passing-between-threads)
5. [Sending Multiple Messages to a
   Receiver](#sending-multiple-messages-to-a-receiver)
6. [Creating Multiple Producers](#creating-multiple-producers)

# Fearless Concurrency

Rust is all about helping programmers to efficiently manage concurrency.  Let's
get something out of the way: concurrent programming is not the same as parallel
programming!  Concurrent programming is where different parts of a program execute
**independently** while parallel programming is where different parts of a program
execute at the **same** time.

In this book when concurrency is referred to however it means both concurrency
and/or parallel.  This is because the Rust developers found that the type
checking system and ownership model did an extraordinary job of solving issues
caused by either concurrancy or parallelism at compile time! So both are treated
with the same tools.

## Spawning Threads

We can easily create new threads by calling `std::thread::spawn` and passing it
a closure of what that thread will run for us:

```Rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

Running this will print the following:

```
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```

Note that even though the thread we've spawned stops at 5 even though it should
go up to 10 (exclusive).  Why?  Because the main thread finishes it's loop and
we exit the main function.  How can we prevent this?  Well, thread::spawn
returns a type called `JoinHandle` which itself has a method called `join`.
This method will pause the execution of our main thread and wait until all
spawned threads finish their execution.  Let's try that now:

```Rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

When we run our crate we now get this:

```
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

This is because we are halting the execution of the main thread after it
finishes it's for loop until the child threads finish their execution.  

Let's now look at how to handle the case of when we need our threads to have
access to their outer lexical environment:

## Using move Closures with Threads

Often times we want to use the `move` closure alongside `thread::spawn` because
it allows us to take ownership of variables in the closures outer environment.
Let's first see what happens if we try to use variables in the outer environment
without using move:

```Rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

This will fail to run!  We'll get an error that the `closure may outlive the
current function, but it borrows v`.  What's going on here?  Well, rust can't
garauntee the life of the child thread (which is kind of weird right?  Wouldn't
the handle.join here garauntee the thread will live until that statement which
is one line before the scope ends?). To solve this we need to move the vector
into the thread and have it take ownership.  When the thread finishes (At the
handle.join line) it will give up ownership and we'll be solid:

```Rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

Now that wee have a basic idea of how to spawn threads, let's actually do
something _cool_ with them!

## Message Passing Between Threads

One way we can handle concurrency is to pass message between threads.  In Rust
we can have many transmitters (senders) but only one receiver.  we can create a
channel and destructure it into it's tx and rx parts like such:

```Rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}
```

mpsc stands for _multiple producer, single consumer_.  It's that idea of a
single receiver but multiple transmitters.  Let's create a simple example where
we send a string from a spawned thread to the main thread and have it print out
what it received:

```Rust
use std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

The `send` method on `tx` will return a `Result<T, E>` - a success if the
receiving end got it, or an error if it fialed for some reason. The same pattern
happens on the receiving end which is why we use `unwrap` on both the `send` and
`recv` functions.  The `recv` function will **block** the main threads execution
until there is nothing left to receive. In contrast if we used the `try_recv`
method it would be **non-blocking**.  

When we run this crate we get what we expect:

```
Got: hi
```

Sweet!

Let's look now at sending multiple messages to a receiver.

### Sending Multiple Messages to a Receiver

We can send multiple messages to a receiver pretty simply and unpack them using
a `for..in` loop:

```Rust
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

Notice the pattern here on the receiving end.  We loop through all the values in
`rx` - using `for..in` like this will turn our `rx` into an iterator and iterate
through all received values.

Let's look now at how to create multiple producers.

### Creating Multiple Producers

We can create multiple producers by cloning the transmitter.  For each new
producer we want we will need to clone:

```Rust

let (tx, rx) = mpsc::channel();

let tx1 = mpsc::Sender::clone(&tx);
thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];

    for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

thread::spawn(move || {
    let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ];

    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

for received in rx {
    println!("Got: {}", received);
}
```

Running this code gets us the following:

```
Got: hi
Got: more
Got: from
Got: messages
Got: for
Got: the
Got: thread
Got: you
```

The threads alternate sending in this example (partly because of the thread
sleep function) and so as we iterate through our received values we bounce
between the two child threads!

Now that we've seen how channels work, let's look at an opposite approach:
sharing memory.


