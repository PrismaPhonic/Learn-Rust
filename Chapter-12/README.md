# Chapter 11

# Table of Contents
1. [Reading Console Arguments](#reading-console-arguments)

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


