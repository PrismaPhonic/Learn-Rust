# Chapter 11

# Table of Contents
1. [Testing](#testing)
    1. [Anatomy of Test Function](#anatomy-of-test-function)
    2. [Assert Types](#assert-types)
    3. [Bringing Functions into Test Scope](#bringing-functions-into-test-scope)
    4. [Ensuring Panic on Errors](#ensuring-panic-on-errors)

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


