# Chapter 12

# Table of Contents
1. [Reading Console Arguments](#reading-console-arguments)
2. [Reading a File](#reading-a-file)
3. [Separation of Concerns](#separation-of-concerns)
    1. [Making a Config Struct](#making-a-config-struct)
        2. [Making a Constructor](#making-a-constructor)
4. [Fixing Error Handling](#fixing-error-handling)
5. [Extracting to Our Library](#extracting-to-our-library)
6. [Test Driven Development](#test-driven-development)
    1. [Write a Test](#write-a-test)
    2. [Fix our Code](#fix-our-code)
7. [Working with Environment Variables](#working-with-environment-variables)
8. [Sending Errors to stderr](#sending-errors-to-stderr)

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

## Extracting to our Library

Lastly we want to extract nearly all of our logic to functions that live in our
library `lib.rs` so we can test them.  We will make main very dry and have our
other functions pass errors back to main where they will be handled.

```Rust
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// --snip--)))
```

Now we'll setup our `run` function to return our errors to main:

```Rust
use std::error::Error;

// --snip--

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}
```

Now we change main to handle what we are returning from `run`:

```Rust
fn main() {
    // --snip--

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
```

Why did we use `if let`?  If you remember from the chapter on match, `if let` is
a shortcut to deal with a single match arm if we only care about one arm.  In
this case our run function returns `()` if it's successful or `Err` if we got an
error.  We can handle error explicitly rather than using `unwrap` because we
don't care about unwrapping the `Ok` response in the event of a success.

We now move all of our code except for `main` into our `src/lib.rs` file:

```Rust
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // --snip--
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --snip--
}
```

Now we have to go back to `src/main.rs` and bring our functions into scope in
`main`:

```Rust


use std::env;
use std::process;

use minigrep;
use minigrep::Config;

fn main() {
    // --snip--
    if let Err(e) = minigrep::run(config) {
        // --snip--
    }
}
```

## Test Driven Development

Let's add some tests.  The process for test driven development is:

1. Write a test for a new function/method you will write and make sure it fails
2. Write or modify just enough code until the test passes.
3. Refactor your code and make sure your code still passes
4. Rinse and repeat

So we'll start from step one:

### Write a Test

We'll write a test for a new function we haven't written yet called `search`
that will take a query and contents and return a vector of positive matches:

```Rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
```

This goes in our `lib.rs` file, and we bring all of our public functions into
scope by writing `use super::*` inside of our tests module.  

We now need to write a function declaration and return a placeholder so we have
enough of a function to run the test (and have it fail, rather than the compiler
yell at us):

```Rust
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
```

Because this is not a method and we have two inputs, the compiler doesn't know
which of the input lifetimes to apply to our return type - so we specify that it
will be the same as contents.  This is because we are taking slices of strings
from contents (the matches) and returning those in a vector - so the lifetimes
of contents and our return will match.

### Fixing our Code

When we run our tests now with `cargo test` they definitely fail! (that's a good
thing right now)  Let's fix that:

```Rust
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

We create a new mutable vector to hold our matches and then simply iterate
through each line of contents (this is the file contents).  Then we just do a
simple `contains` to see if the string contains that match pattern and if so
push that line to the results vector which we return at the end of the function.

Now our tests pass! Let's go ahead and actually use our new function by making
run use our search function:

```Rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}
```

Perfect!  Now our grep clone actually works and will return search matches
correctly!

## Working with Environment Variables

We'll now get some practice working with environment variables.  For this
exercise we wrote a simple function very similar to our `search` function that
does case insensitive search. The pattern difference is trivial and if you've
coded in any other language before you'll immediately recognize the pattern of
using `to_lowercase` on the input and search match:

```Rust
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

Then we need to modify our `Config` struct to have an additional field that will
store a boolean representing whether a user requests a case insensitive search
or not:

```Rust
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
```

Perfect!  Now we we just need to modify run to check if the instance of config
it receives and run case sensisitve or insensitive search per the environment
variable set by the user:

```Rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
```

The only thing a little tricky here if you are used to higher level languages is
to think of rusts `if` statements as not statements (they aren't!), but as
**expressions**, just like ternarys in other languages.

Lastly we need to change our method on `Config` to check for the **presence** of
our choosen environment variable.  All that we care about is if the environment
variable exists, we actually don't care at all about it's value:

```Rust
use std::env;

// --snip--

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
```

We first bring in `std::env` so we can use `env::var` which checks for an
environment variable for us.  This returns a `Result` which if we remember can
be either `Ok` or `Err`.  Since we only care about the environment variable
existing at all, we can simply use `is_err()` method on the `Result` struct that
is returned, which will store a boolean response in our new variable
`case_sensitive`.  Note that we've flipped the logic here - as evidence by how
our variable name is the opposite of our environment variable.  

And that's it!  We can test it like so and it works:

```terminal
$ CASE_INSENSITIVE=1 cargo run to poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

## Sending Errors to stderr

Right now we are printing all our errors to the console with `println!` macro,
which only ever sends errors to standard out.  That means if we are sending our
output to a file and there's an error, that error will populate our output file.
If we run this:

```terminal
$ cargo run > output.txt
```

and we look at our file we will see the error in it:

```terminal
$ cat output.txt
Problem parsing arguments: not enough arguments
```

That's not good!  What can we do?

We can use `eprintln!` macro instead to send our errors to stderr which will hit
the terminal but not send to stdout and hit an output stream.  It's a pretty
simple change since we pass all our errors back to `main.rs`:

```Rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
```

And we are done!  Now our terminal application will only send a successful
output to stdout and we won't see errors hitting our output streams.
