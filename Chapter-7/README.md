# Chapter 7

# Table of Contents
1. [Packages](#packages)
2. [Modules](#modules)
    1. [Super](#super)
    2. [Pub with Structs vs Enums](#pubs-with-structs-vs-enums)
3. [The Use Keyword](#the-use-keyword)
    1. [Idiomatic Use](#idiomatic-use)
    2. [Nested Paths](#nested-paths)
4. [Separate Modules Into Files](#separate-modules-into-files)


## Cargo
### Packages

A package has a Cargo.toml file which tells cargo how to build the crate. A
crate is a bin or library. If a package contains a src/main.rs and a src/lib.rs
then it has both a binary and a libary.  A package can have multiple bin crates
by placing files in the src/bin directory.

### Modules

We can organize our code in modules, which is very similar to organizing files
into folders on a computer.  let's take a look at an example:

```Rust
mod sound {
    mod instrument {
        mod woodwind {
            fn clarinet() {
                // Function body code goes here
            }
        }
    }

    mod voice {

    }
}

fn main() {

}
```

In this example we are  nesting mods and we can put appropriate code in the mod
it relates too. src/main.rs actually sits at the crate root because it creates a
module called crate which sits at the root of modules.

```Rust
crate
 └── sound
     └── instrument
        └── woodwind
     └── voice
```

How do we call a module?  Think of it like a folder structure, but instead of
separating folders by a foward slash, we separate modules with a :: like such

```Rust
mod sound {
    mod instrument {
        fn clarinet() {
            // Function body code goes here
        }
    }
}

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
}
```

The problem is, this code won't compile!  Why?  Because these modules by default
are declared as **private**.  So how should they be declared so we can use them?

```Rust
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
}
```

Why do we need pub?  Because by default all items (functions, methods, structs,
enums, modules, annd constants) are private!  The rules around this are that you
can't use modules that are private and children of the current module, but you
are allowed to use code in the current module and any ancestor modules. Because
of these rules, if we had left out `pub` in front of `fn clarinet()` we would
have hit a compile time error due to the fact that we are trying to access the
**child** of a module we have access to, but the child (the function) is private
(by default).

Something else to keep in mind is that we have kept the sound module private so
why can we access it?  We would not outside of the main function but he because the main function is defined in the same module
that sound is defined, we’re allowed to refer to sound from main.

#### Super

We can call functions from inside modules using a relative path.  We do this
with Super, which is akin to `..` in the terminal.  We are looking from our
current module up to the parent module.  For example:

```Rust
mod instrument {
    fn clarinet() {
        super::breathe_in();
    }
}

fn breathe_in() {
    // Function body code goes here
}
```

In this example we are going up to the crate module (root) in which the
breathe_in function resides.

#### Pub with Structs vs Enums

We can use pub with structs and enums that are put inside modules, but they
behave differently for each.  Let's look at structs first:

```Rust
mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

fn main() {
    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    // The next line won't compile if we uncomment it:
    // println!("The ID is {}", v.id);
}
```

In the above example we can see that id doesn't have `pub` in front.  With
enums, we have to specify which _fields_ are public!  This is useful because we
limit the instantiation of the Vegetable type to the associated function `new`
that is itself public
(remember, these are usually called static functions in other languages).   

With enums setting the enum to `pub` makes all of it's variants public.  

### The use keyword

You can use the 'use' keyword to create a shorter alias to a module, almost like
making a symbolic link in the terminal:

```Rust
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // Function body code goes here
        }
    }
}

use crate::sound::instrument;

fn main() {
    instrument::clarinet();
    instrument::clarinet();
    instrument::clarinet();
}
```

We can now call modules that are children of instrument directly.  

### Idiomatic use

You might be wondering why we didn't do `use crate::sound::instrument::clarinet` so that we could simply call it with `clarinet()` in `fn main()`.  It's considered idiomatic to `use` the direct parent of a function you want to use so it's obvious that when you are calling the function that it wasn't created locally.

What about if we are importing two functions of the same name?  That would
create a problem that could be resolved in two different ways.  

1.

```Rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
}
fn function2() -> io::Result<()> {
}
```

Or we could solve it this way using `as` to customize the import

```Rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
}
fn function2() -> IoResult<()> {
}
```

### Nested Paths

If we are importing multiple things from the same library, there are nice ways
that we can declare nested paths on one line.  Let's look at one:

```Rust
use std::{cmp::Ordering, io};
```

This is the same as typing:

```Rust
use std::cmp::Ordering;
use std::io;
```

Much cleaner to do it on one line, right?  Essentially we declare the common
path first, and then branches from that common path in {} separated by commas
for each branch.  

We can also do this if the path of one import is completely shared by the path
of another by utilizing `self`:

```Rust
use std::io;
use std::io::Write;
```

can become:

```Rust
use std::io::{self, Write};
```

### Separate Modules into Files

We can separate modules into different files instead of declaring them all in
main.rs.  We simply need to put the file that holds the definitions associated
with our module into it's own file in `src` directory that `main.rs` is in, and
then link it into main like so:

```Rust
mod sound;

fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
}
```

When we link it in this way with a semicolon after `mod sound` it tells rust to
load the contents from a file in the src directory by **the same name**.  Now we
move the body of sound into src/sound.rs like such:

```
pub mod instrument {
    pub fn clarinet() {
        // Function body code goes here
    }
}
```

If we wanted to take this further and put instrument into it's own file then we
would need to create a directory in the src/ directory by the name of the parent
module.  In this case that would be `src/sound` and in that directory put the
current body of instrument, and change instrument to be linked as just `pub mod
instrument;`
