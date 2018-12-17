# Chapter 9

# Table of Contents
1. [Unrecoverable Errors](#unrecoverable-errors)
2. [Recoverable Errors](#recoverable-errors)
    1. [Shortcuts for Match Result](#shortcuts-for-match-result)
    2. [Propogating Errors](#propogating-errors)
        1. [Shortcut for Propogation](#shortcut-for-propogation)
3. [When to Panic?](#when-to-panic)
4. [Custom Types for Input Validation](#custom-types-for-input-validation)

## Unrecoverable Errors

Rust has the panic! macro which when executed will print some kind of failure
message and quit - this is reserved for **unrecoverable errors** - usually the
result of some bug where it's not clear how the unknown error should be handled.

It's as simple as specifying where to panic and supplying a message:

```Rust
fn main() {
    panic!("crash and burn");
}
```

Often times when our code relies on other libraries, a panic! might be called
from a library when we improperly use it. Often when that happens we will get
an error message that relates **to that library** and doesn't point to where we
went wrong in **our code**. If we want to see that we need to run a backtrace.
Lets look at a simple example where we try to access an index in a vector that
doesn't exist. Running this:

```Rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

Will result in this:

```terminal
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is
99', /checkout/src/liballoc/vec.rs:1555:10
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

Not very helpful right? Because it simply points to where the panic was called
in the library that implements Vec<T>. Lets' try with the backtrace by
prefixing with `RUST_BACKTRACE=1` in front of `cargo run`:

```terminal
$ RUST_BACKTRACE=1 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is
99', /checkout/src/liballoc/vec.rs:1555:10
stack backtrace:
   0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
             at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at /checkout/src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at /checkout/src/libstd/sys_common/backtrace.rs:60
             at /checkout/src/libstd/panicking.rs:381
   3: std::panicking::default_hook
             at /checkout/src/libstd/panicking.rs:397
   4: std::panicking::rust_panic_with_hook
             at /checkout/src/libstd/panicking.rs:611
   5: std::panicking::begin_panic
             at /checkout/src/libstd/panicking.rs:572
   6: std::panicking::begin_panic_fmt
             at /checkout/src/libstd/panicking.rs:522
   7: rust_begin_unwind
             at /checkout/src/libstd/panicking.rs:498
   8: core::panicking::panic_fmt
             at /checkout/src/libcore/panicking.rs:71
   9: core::panicking::panic_bounds_check
             at /checkout/src/libcore/panicking.rs:58
  10: <alloc::vec::Vec<T> as core::ops::index::Index<usize>>::index
             at /checkout/src/liballoc/vec.rs:1555
  11: panic::main
             at src/main.rs:4
  12: __rust_maybe_catch_panic
             at /checkout/src/libpanic_unwind/lib.rs:99
  13: std::rt::lang_start
             at /checkout/src/libstd/panicking.rs:459
             at /checkout/src/libstd/panic.rs:361
             at /checkout/src/libstd/rt.rs:61
  14: main
  15: __libc_start_main
  16: <unknown>
```

We want to look for the line that corresponds to **our code** which is item 11.
It shows us that line 4 is the culprit in our code.

We'll come back to panic! macro in more detail later, but for now let's look at
dealing with **recoverable errors** using `Result`

## Recoverable Errors

For recoverable errors we can take care of those typically by dealing with the
Result<T, E> they return. This is pretty similar to Option<T> except that it
returns a success type of Ok<T> or a generic Error type that we can deal with
through further branching depending on the error. Let's look at a simple
program that either opens a file by the name of "hello.txt" - which itself would
return a Result<T, E>. We can then handle writing this file if it doesn't exist
yet (if the error type is ErrorKind::NotFound), and if there's a problem when
creating the file handle another Result<T, E> by either succeeding gracefully,
or running a panic! with a message that we couldn't write the file. Finally,
for all other Error types (other than NotFound) we will also panic and pass the value inside the `Err` to the user.

```Rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };
}
```

A couple things to note here:  What the hell is error.kind()?  Well, the type of value that File::open returns inside the Err variant of the Result enum is `io::Error` which is a struct with methods. There's a method of `kind` which will return what kind of error we got. We then check that error matches ErrorKing::NotFound and if so we write to the file. 

### Shortcuts for Match Result

As nice as it is to have this much control, sometimes it's unecessary and we
only want to either expose the value inside the `Ok`, or panic! in the case of
an `Err` type. There's a method on the Result type called `unwrap` that does
just this for us:

```Rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

By running this we will get a panic and a custom error message that unwrap
**wrote for us** which can sometimes be unfriendly.

What if we want to still write the panic message ourselves?  That's where
`expect` comes in (instead of `unwrap`):

```Rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

This works just like unwrap - it returns the file handle or calls the panic!
macro - but we get to design the error message.

### Propogating Errors

If we intend for our program to be used by others in their programs, then it can
be nice to **propoage** the error - that is, to pass it back to the function
that called our function. We do this by having our function return a `Result<T,
E>` like such:

```Rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

Other users of our function will need to be aware that they will need to either
`match`, `unwrap`, or `expect` to get the value out of the `Result`, or handle
the error themselves if the `Result` matches an `io::Error` type. Take notice
of our first match how we explicitely `return Err(e)` - this will break out of
the function and return this error to the function that called it. If we stay
within the function (a successful open) then we also run f.read_to_string which
itself returns a result so we use another match, and either return the `Ok` or
the `Err` depending on what matches. In either case, the first or second match,
we are passing the error back to the function that called
read_username_from_file.

#### Shortcut for Propogation

Propogation is so common in Rust, that Rust provides a handy shortcut for
propogating errors by using the `?` operator. Let's re-write the last function
but use the `?` operator instead of match:

```Rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

In the above example the `?` operator will get the `Ok` value if that's the
match, and otherwise propogates the error back to the function that called our
function. The only difference is that it will also convert whatever error it
catches to the type specified in our return. In this example it will convert
any error it finds to `io::Error` type because that's what we specified in the
function return.

We can also chain methods after the `?` suffix like such:

```Rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

One last caviat is one that might be obvious already: We can only use the `?`
suffix operator on functions that return a `Result` type. 

### When to Panic

It might seem like we should never use `unwrap` or `except` because it will
crash our program completely! So when should you use these? (or manually call a
panic!) - well, it's great for **prototyping**. When you are just building an
app, you **want** failures to be obvious and crash your code. an `unwrap` or
`expect` can be a nice marker of where to make your code more robust with better
error handling once you've finished writing the core functionality. 

When else might you want to use `unwrap`?  When it is obvious that there is no
possibility whatsoever for an Err type, and you simply want to get the result
from `Ok` because the method you are using always returns a `Result`.

When else?  Whenever it's possible for your code to end up in a _bad state_.
That is that your code would be left in an unexpected state that keeps your code
from moving on in a correct manner. You should also call panic if you find out
that you are being supplied with `invalid data`. Think about the analogy (if
you have experience with databases) of a sql injection attack. Would you pass
that long, or validate your input?  Should your program panic at this point or
proceed to deal with **bad data**. Definitely panic! 

#### Custom Types for Input Validation

Instead of panic! we can write custom types to handle our input validation.
Here's an example of a struct we could have made for the guessing game that
would have validated that a guess was between 1 and 100.

```Rust
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
```

The new method acts as a _setter_ which validates that the number is between 1
and 100, and if not - throws a panic!.  If it's valid, then it returns an
instance of Guess.  We then have an instance method of value that acts as a
_getter_ to return the u32 directly.  
