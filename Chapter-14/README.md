# Chapter 14

# Table of Contents
1. [Customizing Builds](#customizing-builds)
2. [Documentation Comments](#documentation-comments)
3. [DocTests](#doctests)
4. [General Crate Comments](#general-crate-comments)
5. [Exporting Public API](#exporting-public-api)
6. [Preparing a Crate for Publishing](#preparing-a-crate-for-publishing)
7. [Publishing to Crates.io](#publishing-to-crates.io)
8. [Removing Broken Versions](#removing-broken-versions)

# Cargo and Crates

We'll be diving in deep on cargo and crates in this chapter. Let's get started

## Customizing Builds

Out of the  box Rust comes with two build profiles that we've already used:
`dev` and `release`. By default `dev` uses optimization level 0 which has the
fastest compile time while `release` uses optimization level 3 (the highest)
which has the slowest compile time but applies all optimizations.  We can
override these settings if we want in Cargo.toml:

```Rust
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

It's pretty easy to change these to whatever our heart desires, but they are set
sensibly already.

## Documentation Comments

We can write comments that will automatically be turned into HTML documentation
for other developers to understand how our crate is _implemented_. Documentation
comments start with three slashes, `///`, and support **markdown**.  Example:

```Rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, my_crate::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

If we want to generate the html for our docs we can run `cargo doc`.  If we want
it to instantly open it we can run `cargo doc --open` where we'll see how the
documentation will look in html.  

Here are 4 common sections crate authors include (h1 heading in markdown):
 * **Examples**: Show some examples
 * **Panics**: The scenarios in which the function being documented could panic.
Callers of the function who don’t want their programs to panic should make sure
they don’t call the function in these situations.
 * **Errors**: If the function returns a Result, describing the kinds of errors
that might occur and what conditions might cause those errors to be returned can
be helpful to callers so they can write code to handle the different kinds of
errors in different ways.
 * **Safety**: If the function is unsafe to call (we discuss unsafety in Chapter
19), there should be a section explaining why the function is unsafe and
covering the invariants that the function expects callers to uphold.
)

### Doctests

We can write doctests in Rust similarly to doctests in python.  This happens
automatically for our Examples sections when running `cargo test`.

## General Crate Comments

If we want comments that apply to our entire crate rather than a specific
function we can use `//!` at the top of our `lib.rs` file.

## Exporting Public API

Often times when we are developing internally we might be using a heavily nested
module structure that would be confusing for other developers who simply want to
**use** our crate.  We can instead re-export at the top level of our library
using `pub use` which makes it so other developers can call our various modules
from the top level of our crate and not have to hunt through layers of nesting
to find the modules they want to use.  Here's a simple example where we
re-export modules at the top level of our library:

```Rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    // --snip--
}

pub mod utils {
    // --snip--
}
```

## Preparing a Crate for Publishing

Besides re-exporting at the top and setting up good documentation, there are a
few more things we'll want to do to get our crate ready for publishing.  One is
to give our crate a good name in our `Cargo.toml` file:

```Rust
[package]
name = "guessing_game"
```

Next we need to add a license identifier value.  You can find a list of [License
Identifiers here](https://spdx.org/licenses/).

In this example we'll pick the MIT license:

```Rust
[package]
name = "guessing_game"
license = "MIT"
```

Lastly we need to make sure we have an appropriate version, authors list,
edition (year), and description:

```Rust
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"
```

Now we are finally ready to publish to Crates.io

## Publishing to Crates.io

We can now publish to crates.io but be careful!  Crates.io acts as a _permanent_
archive!  to publish just run `cargo publish`.  When you are ready to publsih a
new version just change the version number in `Cargo.toml` file and republish
again.  Lets lastly talk about removing broken versions of a crate:

## Removing Broken Versions

We can't remove broken versions actually, but we can prevent future projects
from adding them as a dependency and using them.  We do that with `cargo yank`.
When we use it we need to specify which version we want to yank from active use
like such:

```terminal
cargo yank --vers 1.0.1
```


