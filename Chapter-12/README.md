# Chapter 11

# Table of Contents
1. [Reading Console Arguments](#reading-console-arguments)
2. [Reading a File](#reading-a-file)
3. [Separation of Concerns](#separation-of-concerns)
    1. [Making a Config Struct](#making-a-config-struct)
        2. [Making a Constructor](#making-a-constructor)
4. [Fixing Error Handling](#fixing-error-handling)

# Minigrep Project

This documents some of my learnings from the minigrep project that are not
already documented in the previous chapters.  In that sense it will be far more
sparse than the project in the Rust Programming Language book.

## Reading Console Arguments

To build a console based application we need to be able to read command line
arguments which means bringing in the `args` function from `std::env`.  This
will return an iterator of the arguments supplied by the console. Just like if
we were to do this in node.js, the first argument at index 0 is always the
filename itself followed in order by the args.  Here we bring `std::env` into
scope and then grab the args and collect them into a vector of `String`'s:

```Rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
```

As I mentioned the first index in the collection will be the filename so we
store the second index as the query (what the user will search for in our grep
clone) and then the next argument is stored as the filename to search.

## Reading a File

Next we need to adapt the program so it can read a file.  To do that we need to
import `std::fs` from the standard library which has a function on it called
`read_to_string`:

```Rust
use std::env;
use std::fs;

fn main() {
    // --snip--
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
```

We use `expect` because if you remember `expect` is just like `unwrap` except it
let's us set the panic message.  This is not a great way to do error handling
but it let's us build the app quick and dirty, and we can come back to clean up
the error handling later.

## Separation of Concerns

I've decided to copy paste a section on process for our program regarding
separation of concerns from the book directly because I frankly couldn't write
it better myself:

* Split your program into a main.rs and a lib.rs and move your program’s logic
to lib.rs.

* As long as your command line parsing logic is small, it can remain in
main.rs.

* When the command line parsing logic starts getting complicated, extract it
from main.rs and move it to lib.rs.

* The responsibilities that remain in the main function after this process
should be limited to the following:
  * Calling the command line parsing logic with the argument values
  * Setting up any other configuration
  * Calling a run function in lib.rs
  * Handling the error if run returns an error

We start out by doing a simple refactoring to create a mini function called
parse_config whose only job is to parse the query and filename out of the args
and return them to main.  Nothing new here - a little bit of rust destructuring
which is neat:

```Rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    // --snip--
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
```

### Making a Config Struct

We further refactor to use a config struct that helps to give further meaning to
the data our parse_config function parses:

```Rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    // --snip--
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
```

What we've done here is make a custom type `Config` that has two fields, `query`
and `filename`.  Because these fields are owned `String` type we clone the args
before putting them into variables in the `parse_config` function.  Then we
return an instance of Config which gets stored in the `config` variable.  This
now gives users of our program one place (the Config struct) to look at what
fields they have access to out of the Config returned by parse_config.

#### Making a Constructor

Right now we are creating instances of `Config` using `parse_config` when
convention for instantiating a new instance in Rust is to call it like such:
`Config::new`.  We'll rename our `parse_config` function to `new` and put it
inside of an `impl` block applying to the `Config` struct:

```Rust

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
```

## Fixing Error Handling

Our program currently issues a panic if we don't get enough arguments.  We will
use a technique to manually handle displaying an error message and exiting
that's friendlier for our users.  First we have to make our constructor return
a Result<T, E> type and wrap up our `Config` instance in an `Ok` and our panic
message as a string literal into our `Err` to be passed back to `main.rs`:

```Rust
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
```

Now that we are passing back a different type to `main.rs` we have to change
that to.  We can handle a `Result` return from our constructor with
`unwrap_or_else` which is just like `unwrap` but if we get an `Err` type
returned it will allow us to define a custom action type using a _closure_.  The
closure allows us to get at the string wrapped up in the `Err` and manually
exit the program.  We can issue a manual exit with an error code of 1 by calling
`std::process::exit(1)` but we'll bring `std::process` into scope so we don't
have to be quite so verbose:

```Rust
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--))
}
```

nice!


