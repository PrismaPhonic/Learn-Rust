# Chapter 5

# Table of Contents
1. [Defining and Instantiating](#defining-and-instantiating)
2. [Field Init Shorthand](#field-init-shorthand)
3. [Struct Update Syntax](#struct-update-syntax)
4. [Tuple Structs](#tuple-structs)
5. [Unit Like Structs](#unit-like-structs)
6. [Rectangles](#rectangles)
    1. [Method Syntax](#method-syntax)

## Structs

Structs are akin to Classes in other languages.  

### Defining and Instantiating

When we define a struct, we list all the pieces of data together in what are
called _fields_.  In other languages, these are akin to Class attributes.

```Rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

To use a struct after we've defined it, we create an instance of the struct and
pass values for each of the fields. by defining a set of key:value pairs, in no
particular order:

```Rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

Values are retrieved from structs using dot notation.  If you would like to
change a field in an instance, the entire instance must have been instantiated
with a mutatable variable like such:

```Rust
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
}
```

Why bother to use String type and not &str which is more flexible? Because
struct ownership is tricky, and using a reference in struct requires the use of
_lifetimes_ which we will get into much later.  For now we need to make sure
that each instance is the **owner** of it's values.

### Field Init Shorthand

We can write a function that builds a user, and we can use _field init
shorthand_ when variables and fields have the same name (very similar to object
property shorthand in Javascript)

```Rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

### Struct Update Syntax

There's also a feature called struct update syntax that works very similar to
spread in javascript, except that it goes at the end not the beginning (in JS
you would spread and then replace, with this syntax you specify unique values
first and then tell Rust that you want to copy the rest of the values from an
existing instance)

```Rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

### Tuple Structs

Tuple structs are simply tuples that we want to give a unique type, since all
structs create their own type.  Here's an example where we define an RGB color,
which has 3 values that can range from 0 to 255, which is perfectly represented
by 3 fields have u8 types

```Rust
struct RGB(u8, u8, u8);
let black = RGB(0, 0, 0);
```

### Unit Like Structs

You can create structs with no fields (like a class with no attributes).  This
can be useful if you want to implement a _trait_ (like a method I think?) on
some type, but don't have any data to associate with it.

## Rectangles

I made the fun example program!  Here are some **new** things that I learned
(not already detailed above).

in Rust you cannot println! a class because how that class would be printed is ambigous.  For this reason if you want to look at a class instance then we have to opt in to include functionality that let's us print out **debugging information** by including it at the top of our code:

```Rust
#[derive(Debug)]
```

then we can print a nicely formatted version of an instance like this:

```Rust
println!("instance is: {:#?}", instance);
```

In the case of our rectangle instance, this printed out:

```
rect1 is Rectangle {
    width: 30,
    height: 50
}
```

### Method Syntax

Methods are exactly what they sound like - ways to describe methods of modifying
the data they are associated with.  In the case of Rust we declare these with an
implementation block `impl`.  Even though it's defined outside of the struct,
`impl` blocks reference the struct that they act on. 
