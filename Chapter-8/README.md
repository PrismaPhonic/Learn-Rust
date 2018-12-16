# Chapter 8

# Table of Contents
1. [Vectors](#vectors)
  1. [Accessing Elements](#accessing-elements)
  2. [Iterating Vectors](#iterating-vectors)
  3. [Vectors With Varying Types](#vectors-with-variant-types)
2. [Strings](#strings)
  1. [Updating Strings](#updating-strings)
  2. [Concatenation](#concatenation)
  3. [Indexing](#indexing)
  4. [Iteration](#iteration)
3. [HashMaps](#hashmaps)
  1. [HashMaps and Ownership](#hashmaps-and-ownership)

## Collections	
### Vectors

Vectors are basically like arrays in javascript or python - they are like
growable mutable arrays. Because their size isn't known at the time of creation
they are placed on the heap. Like Rust arrays, they only take one **type**

We can create a new vector like so:

```Rust
let v: Vec<i32> = Vec::new();
```

This is unusual as we typically want to create a vector with some data in it.
In that case we can just use the `vec!` macro like such:

```Rust
let v = vec![1, 2, 3];
```

Because we've provided rust with values, it can do type inferrence for us. 

Even though vectors are placed in the heap, we still have to declare them at
mutable to mutate them (almost always want to do this). If mutable, we can add
elements to an array with push:

```Rust
let v = vec![1, 2, 3];

v.push(4);
v.push(5);
```

#### Accessing Elements

There are two ways we can access elements from a vector:

1. An easy way that will crash your program if someone tries to access a value
  outside of the bounds of the vector
2. A harder way that will return None Option<&T> type and not crash your
  program, where you can handle it gracefully.

Let's look at the easy (but crash prone) way:

```Rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
```

Easy right? Yep, I like it too. If we want our program to not crash when a user
tries to access an element beyond the vector bounds we need to handle it using
a match with vector.get(index) (which returns Option<&T>):

```Rust
let v = vec![1, 2, 3, 4, 5];
let v_index = 2;

match v.get(v_index) {
  Some(_) => { println!("Reachable element at index: {}", v_index); },
  None => { println!("Unreachable element at index: {}", v_index); }
}
```

Now if someone tries to access a `v_index` that does not exist the program will
keep running but they'll see a useful message in their terminal letting them
know what they are doing is totally wack.

#### Other Vector Methods

I dug through the docs to find other vector methods and will relate them to
Javascript array methods they are similar to. 

Here's something that operates a lot like Array(5).fill(0);

```Rust
let vec = vec![0; 5];
```

Just like how we pushed, we can also pop with `v.pop()`. We can use
`v.insert(index, element) to insert an element at an index position (just like
splice, but only one element). There actually is a splice method but I don't
quite understand how to use it from reading the docs (yet).

There's a vector sort method `v.sort` which is considered **safe** (will
maintain order of equal elements) but not as efficient as v.sort_unstable.

`v.contains` is a lot like `Array.includes`. 

#### Iterating Vectors

We can iterate through vectors with a simple for loop:

```Rust
let v = vec![100, 32, 57];
for i in &v {
  println!("{}", i);
}
```

If we want to mutate the values (like map array method in javascript) then we
can do it like such:

```Rust
let mut v = vec![100, 32, 57];
for i in &mut v {
  *i += 50;
}
```

Looks like we get into the dereference operator later, but for some reason
(poorly explained in the book) this is necessary for us to manipulate the
elements in the vector directly.

#### Vectors with Varying Types

What about if we want to have a vector with various types? We can hack this
together by making an enum (which itself will be it's own type) which is
composed of various types itself. Then the vector will allow it because each
varying type is a member of the enum. Like such:

```Rust
enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

let row = vec![
  SpreadsheetCell::Int(3),
  SpreadsheetCell::Text(String::from("blue")),
  SpreadsheetCell::Float(10.12),
];
```

## Strings

When we talk about Strings, we are talking about the String type which is really just syntactic sugar over a vector of scalar chars - in that sense it is a growable list of UTF-8 chars. Deep down strings are actually very complicated so you can't just iterate over a String by character without specifying **how** you want to iterate through. Let's come back to this, but first, two ways to make a String from a string literal (&str type)

```Rust
let s = String::from("initial contents")
```

or:

```Rust
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
```

We can also create an empty String type to fill later:

```Rust
String::new();
```

### Updating Strings

We can grow a String by appending a string slice (&str type) with `push_str`:

```Rust
let mut s = String::from("foo");
s.push_str("bar");
```

Or if we just want to append a single char we can use `push`:

```Rust

let mut s = String::from("lo");
s.push('l');
```

### Concatenation

We can concatenate using `+` operator or `format!` macro (hint: just use format!
so you don't have to worry about losing ownership):

```Rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // Note s1 has been moved here and can no longer be used
```

This is how add works (which under the hood is an implementation on the struct) for the String type. Add takes two parameters, self and then &str, so it takes ownership of the first String being added. Let's look at a way to concatenate without losing access to s1:

```Rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

### Indexing

Want to access an index in a string? In many higher level languages you can do
this without a thought! But Rust exposes what goes on under the hood and
strings are much more complicated than most developers think! Strings are vec(u8) under the hood.  The problem is that non-english letters can often takes up more bytes than english letters.  The old standard of one byte per character doesn't hold up, and UTF-8 can be anywhere from 1 to 8 bytes per character.  Do you want an index by bytes, which can sometimes land between characters?  Unlikely.  What about by chars?  That seems reasonable and is what most languages would do, but can also be problematic with langauges that have accent chars that overlap with other chars. What might seem like a string of 4 characters might actually by 8 characters long, and some of those characters (like accents) should be pakaged with their relevant character to form what we as humans think of as a **letter**.  For these reasons we have to be more explicit with how we traverse a string in rust.  We can think of it as three options:

1. Access by bytes (unlikely you'd ever want this)
2. Access by char (perhaps you want this if only standard english chars?) 
3. Access by graphene clusters (will always get you what we think of as a
   'letter') - this requires importing an external crate

Let's look at how we can iterate over strings in more detail

#### Iteration

Let's look at the two ways you would actually want to iterate through string.
First, by char (which with some languages won't always get you 'letters'):

```Rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

This will print out:

```Rust
न
म
स
्
त
े
```

Now let's look at how to do this for graphenes.  We have to install the external
crate `unicode-segmentation`, and then import it and use it like such:

```Rust
extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

fn main() {
   for c in "नमस्ते".graphemes(true).collect::<Vec<&str>>() {
       println!("{}", &c);
   }
}
```

That's it for Strings!  Let's talk now about Hash Maps!

## HashMaps

HashMaps are just like other languages - they are a mapping of keys to values
using a hash algorithm.  Let's look at how we can create a new HashMap and
insert some key/value pairs;

```Rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

We first import HashMap from the standard collections library and then generate
a new HashMap with `HashMap::new()`.  Then we insert with `instance.insert(key,
value);

### HashMaps and Ownership

HashMaps can be tricky when it comes to ownership. Let's look at an example:

```Rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
```

In this example map has taken ownership of `field_name` and `field_value` when
they are inserted into map.  We cannot use field_name and field_value anymore.
An alternative is to pass in a reference instead, but doing so means we also
have to validate that the data being referenced will exist for the **lifetime**
of the map's existence (using a feature called lifetimes).


