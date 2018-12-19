# Chapter 11

# Table of Contents
1. [Testing](#testing)
    1. [Anatomy of Test Function](#anatomy-of-test-function)
    2. [Assert Types](#assert-types)
    3. [Bringing Functions into Test Scope](#bringing-functions-into-test-scope)
    4. [Ensuring Panic on Errors](#ensuring-panic-on-errors)
    5. [Running Tests](#running-tests)
    6. [Organizing Tests](#organizing-tests)
    7. [Integration Tests](#integration-tests)
        1. [Submodules in Integration Tests](#submodules-in-integration-tests)

# Testing

Since I already have extensive experience doing TDD in node and python I will
primarily be highlighting (as this is mostly a repo for documenting my own
education) the key differences/takeaways when it comes to testing in rust.

## Anatomy of Test Function

We need to place a `#[test]` annotation above the function body in the test
file:

```Rust
#[test]
fn exploration() {
    assert_eq!(4, add_two(2));
}
```

We use `assert_eq!` and pass it what our function should equal as the first
parameter, and then our function as the second parameter.  Then we just run
tests with `cargo test`.

We also need to define all of this inside of a module named tests that has
`#[cfg(test)]` annotation above it:

```Rust
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
```

## Assert Types

We saw `assert_eq!` but we also have other test types like `assert!` which
checks that a test evaluates to true rather than is equal to a value we specify.
There's also `assert_ne!` which is the opposite of `assert_eq!`. 

## Bringing Functions into Test Scope
If we are referencing a function or method that
lives outside of our test module that we bring it into the scope of the test
module.  Like such:

```Rust
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }
}
```

To get access to `assert_eq!`, `assert_ne!`, and debug printing when errors
happen our custom enums and structs need to implement PartialEq and Debug traits
from the standard library.  This is as simple as adding `#[derive(PartialEq,
Debug)]` annotation to our own struct and enum definitions.

## Custom Test Error Messages

We can pass along a custom string to print to the console when a test fails b
simply passing the string as a second argument to `assert!`, followed by
variables we want to inject between `{}` in the string literal:

```Rust
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`", result
    );
}
```

## Ensuring Panic on Errors

We also want to check that our functions are failing when they should (maybe the
most important part of a good program).  We can do this by adding an
`#[should_panic]` annotation above our function we expect to issue a panic due
to failure like such:

```Rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

This isn't a very good test because it doesn't specify exactly why our test
panicked.  For that we can add an `expected` parameter to the `should_panic`
annotation:

```Rust
#[test]
#[should_panic(expected = "Guess value must be between 1 and 100, got {}.")]
fn greater_than_100() {
    Guess::new(200);
}
```

## Running Tests

We can run tests in parallel (default) or in series.  To limit testing to only
run in series we can run it with `cargo test -- --test-threads=1`.  Sometimes
functions will print output the screen which we will see when our tests run
along with the test results.  To not show this output we can pass the
`--nocapture` flag like such: `$ cargo test -- --nocapture`.  To run only a
specific test by it's name then we can pass that in implicitely.  If we had a
test function by the name of `one_hundred` we could run a test just on that with
`cargo test one_hundred`. If we would like to put a test into an 'ignore' group
that will be ignored by default, we can add an `#[ignore]` annotation above it
(but below the tests `#[test]` annotation).  Then any tests with this annotation
will not be run by default when we type `cargo test` in our terminal.  If we
want to run tests in our ignore group we can pass the `ignored` flag, such as:
`$ cargo test -- --ignored`.

## Organizing Tests

By convention we should create a **module** name `tests` in every file of our
project that contains our test functions, and to annotate the module with
`#[cfg(test)]`.  This annotation tells rust to skip compiling the test code when
we run `cargo build` and only run the test code when we type `cargo test`. 

## Integration Tests

When we want to run integration tests it's convention to put our integration
tests in a tests **folder**. Make a test directory at the base of your project
folder called `tests` (exactly that).  Then put your integration tests in a
file.  When we do this we don't have to include a `#[cfg(test)]` because rust
knows that any files inside a `tests` folder are integration tests.  We will
need to bring into scope the function that we plan to test, like such:

```Rust
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

We used `use adder` to bring the adder crate into scope, and then called add_two
by typing `adder::add_two` which is a pub function that lives in our adder crate
in `lib.rs`.  We can only import functions from `lib.rs` as this is intended for
our local library. If our function is in `main.rs` then they are part of our
binary and can't be brought in for testing.  This is why it is common practice
to write nearly all your functions in `lib.rs` and have a very _dry_ main
function that kicks off your program running.  

### Submodules in Integration Tests

What if we want a setup file to set up our testing environment?  One way we can
do this is to make a folder named `common` inside `tests` and inside that put a
file called `mod.rs`.  Rust will understand this to mean that we have a module
named common (almost as if we named it `common.rs`) and that module is not a
test itself.  We can do whatever we need to setup the test environment there,
and then we can bring it into scope in our tests like such:

```Rust
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```

We are calling a setup function before we use `assert_eq!` which will setup our
test environment for us.  Unfortunately as far as I can tell Rust does not `yet`
have something similar to `beforeeach` like the beauty that is jest.  
