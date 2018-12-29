# Chapter 16

# Table of Contents
1. [Fearless Concurrency](#fearless-concurrency)
2. [Spawning Threads](#spawning-threads)
3. [Using move Closures with Threads](#using-move-closures-with-threads)
4. [Message Passing Between Threads](#message-passing-between-threads)
5. [Sending Multiple Messages to a
   Receiver](#sending-multiple-messages-to-a-receiver)
6. [Creating Multiple Producers](#creating-multiple-producers)
7. [Shared State Concurrency](#shared-state-concurrency)
    1. [Mutex to Manage Data Access](#mutex-to-manage-data-access)
    2. [Mutex API](#mutex-api)
    3. [Sharing a Mutex between Threads](#sharing-a-mutex-between-threads)
    4. [Multiple Ownership with Threads](#multiple-ownership-with-threads)
8. [Sync and Send Traits](#sync-and-send-traits)

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

# Shared State Concurrency

Instead of message passing we can handle concurrency by sharing memory between
threads.  The first way to do this that we'll look at is using a mutex.

## Mutex To Manage Data Access

Mutex stands for mutual exclusive.  A mutex allows only one thread to access a
piece of datta at a time.  To access that data a thread has to acquire a lock on
the mutex.  A lock is a data structure that's part of the mutex and keeps track
of who currently has access rights to the data.  Mutexes are very difficult to
use and you have to always remember these two very firm rules:

1. You must attempt to acquire the lock before using the data
2. When you're done with the data that the mutex guards, you must unlock the
   data so other threads can acquire the lock.

Let's look now at the basics of the mutex API:

## Mutex API

As we mentioned before we need to acquire a lock to get access to the data that
Mutex points at:

```Rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
```

This should start to look pretty familiar.  We make a new Mutex and have it
point at the data (an integer 5 in this example).  We then get a lock on it by
using the `lock` method and then we unwrap the result because lock returns a `
Result` type called a `LockResult` which if successful will itself contain a
`MutexGuard<T>`. While `Mutex<T>` is a smart pointer, calling `lock` will itself
return a smart pointner called `MutexGuard`.  When the Mutex guard goes out of
scope (after we mutate the num) then the Mutex is **unlocked** and can be access
again later.  Let's look now at sharing a mutex between multiple threads

## Sharing a Mutex between Threads

Let's try right now (and fail miserably) to share a mutex between two threads:

```Rust
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();

        *num += 1;
    });
    handles.push(handle);

    let handle2 = thread::spawn(move || {
        let mut num2 = counter.lock().unwrap();

        *num2 += 1;
    });
    handles.push(handle2);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

Why does this fail?  Well, we aren't allowed to move ownership of the mutex into
multiple threads.  This makes sense because threads run in parallel and data can
only have one owner.  How do we fix this?  

## Multiple Ownership with Threads

Remember when we used Rc<T> to have multiple owners of a piece of data?  Well,
we can't use it when multithreading because it's not _thread safe_.  Instead we
can use `Arc<T>` which has the exact same API as `Rc<T>` but it's a thread safe
verison.  Why not just use `Arc<T>` all the time then?  There's a performance
penalty to make multiple ownership thread safe, so if we are just writing single
threaded applications it's more performant to use `Rc<T>` over `Arc<T>`.  Let's
look at how we could use it now:

```Rust
use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

This program will work!  We now can move counter into each thread in a thread
safe way.  You might also notice that we are mutating the value inside the
`counter` even though we declared it as an immutable type.  This is because
`Mutex<T>` is a smart pointer that like `RefCell` provides _interior
mutability_.  

Now that we've discussed sharing memory between threads lets look at the `Sync`
and `Send` traits.

## Sync and Send Traits

Let's very briefly wrap up by talking about the `Sync` and `Send` traits.  The
`Send` marker trait indicates that ownership of the type can be transferred
between threads (think back to message passing we covered in this chapter).
Almost every Rust type is `Send`.  There are some exceptions (like `Rc<T>` that
we discovered wasn't thread safe for `Send`).  

What about `Sync`?  `Sync` marker trait indicates that it is safe for the type
implementing `Sync` to be referenced by multiple threads.  Any type implementing
`Send` for &T is automatically `Sync` for T.  Primitive types in Rust are all
`Sync` and custom types composed entirely of types that are Sync are also Sync.
Most things are Sync, with the exception of `Rc<T>` and `RefCell<T>` because the
way they work is not thread safe!

That's all for concurrency! 
