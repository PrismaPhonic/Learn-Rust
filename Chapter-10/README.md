# Chapter 10

# Table of Contents
1. [Generic Data Types](#generic-data-types)
    1. [In Function Definitions](#in-function-definitions)
    2. [In Struct Definitions](#in-struct-definitions)
    3. [In Enum Definitions](#in-enum-definitions)
    4. [In Method Definitions](#in-method-definitions)
2. [Traits](#traits)
    1. [Defining A Trait](#defining-a-trait)
    2. [Trait Bounds](#trait-bounds)
        1. [Conditionally Implement Methods](#conditionally-implement-methods)
3. [Lifetimes](#lifetimes)
    1. [Lifetime Annotation Syntax](#lifetime-annotation-syntax)
    2. [Lifetime Annotations in Structs](#lifetime-annotations-in-structs)
    3. [Lifetime Rules](#lifetime-rules)
    4. [Lifetime Parameters](#lifetime-parameters)

# Generic Data Types

We've seen generic data types in things like Option<T> or Result<T, E>.  Let's
look at how we can write our own functions, structs, enums etc. that can take in
generic data types.  First, declaring them in functions!

## In Function Definitions

Generic data types can be declared in function definitions like so:

```Rust
fn largest<T>(list: &[T]) -> T {
```

The <T> goes after the function name but before the parameter list to specify
that this function is a generic over some type T.  We then note that the
function has one parameter (list) and that it's a slice of values of type T, and
will return that same type T.  Note that all of these are the same type (we
can't mix types as long as we keep the same generic name).

## In Struct Definitions

Let's look at defining a struct that uses a generic type parameter:

```Rust
struct Point<T> {
    x: T,
    y: T,
}
```

In this example we need to remember that T is the same in all cases, so we could
**not** instantiate a Point type using a float for x and an int for y.  If we
wanted to use different types for X and Y, we could define our struct like such:

```Rust
struct Point<T, U> {
    x: T,
    y: U,
}
```

Now you could pass any varying types to x and y and they would be valid.

### In Enum Definitions

Let's look at an enum definition that takes a generic we are already familiar
with:

```Rust
enum Option<T> {
    Some(T),
    None,
}
```

It's as easy as that

### In Method Definitions

Methods can be more complicated than the other examples, but first, let's look
at an easy example:

```Rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

In this example the method `x` is simply returning a pointer  to whatever x in
the struct is.  notice how T passes all the way down?  Also take note of how we
need to put <T> after impl **and** after the struct being acted upon.  If we
wanted to write a method that only applies to struct's of certain types (even if
Struct still takes a generic) then we could define that like such:

```Rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

Here's where it gets pretty confusing.  the function in our `impl` block doesn't
need to operate on the same generics that the `imp` block itself acts on.  let's
see an example:

```
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
```

Weird huh?  In this case we can say that mixup acts on <V, W> because <V, W> are
two generics that apply to `other`, which is essentially another instance of
Point being passed in that we are swapping values with.

That's it for generics!  Let's talk about traits now

# Traits

A trait is just a way to help us to craft polymorphism among our various structs
and their impl's.  It's a way that we can define a shared method.  In its most
simplistic form it can just be a way to signify a method signature.  Let's look
at that now.

## Defining a Trait

We define a trait by declaring it similarly to how we would declare an `impl`
block, but we can write just the function signature without the function block
and leave the block declaration up to each Structs `impl` block to handle:

```Rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

Simple, right? In this case we would need to fill out the body when we decide to
use the trait, like such:

```Rust
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

Yep, that simple, just follow the same signature.  This would be relatively
useless, so let's look at a more useful case - where we define a default method
body:

```Rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

In the above we can see that we have defined a default method body for
summarize.  This means that anytime we implement the trait for a struct, it
inherets that method without us having to declare it ourselves.  cool huh?  In
this example, we still would need to declare a method body for summarize_author
(it's required actually).

## Trait Bounds

Trait bounds are kind of like a **filter** that we can apply when we use generic
types.  We can specify (when we declare that a function, struct, impl or enum
takes a generic type) that we want to only accept a generic type that has
implemented a certain trait (or group of traits). In fact if we try to define a
method that takes in a generic, and in that method we try to call a trait method
on the generic data type we are  passing in, we'll get a compile time error
unless we specifically use trait bounds to notate that we are looking for
generics that implement the relevant trait.  let's look at an example:

```Rust
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
```

currently we can only call pass notify an instance of `NewsArticle` or `Tweet`
because those are the only two types we have that implement `Summary`.  

We can specify that the generic must implement multiple traits as well (if we
plan to call methods relevant to multiple traits) by using the `+` operator:

```Rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
```

That's very long and unwieldy right?  Well, Rust let's us write this with a
where clause to make it more readable:

```Rust
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

### Conditionally Implement Methods

We can use trait bounds to conditionally implement a trait for any type that
implements another trait.  This is called a _blanket implementation_ and is how
to_string() is implemented in the standard library.  Let's take a look:

```Rust
imp<T: Display> ToString for T {
```

This specifies that any type that implements the Display trait can use the
to_string() method. 

# Lifetimes

What are lifetimes?  They are a way for us to explicitely tell the compiler how
long a variable will stay in scope. We only have to supply lifetimes when there
is ambiguity about how long a borrowed variable will stay _alive_.  Let's look
at an example:

```Rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

The above code will not compile because the compiler can't tell if the borrowed
reference being **returned** refers to x, or y.  We don't know either, and
because it's lifetime (how long it will stay in memory) is not explicitely
deterministic and can't be **garaunteed** the compiler throws up and tells us
that we need to specify how long the lifetime will be.  Let's fix this:

```Rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

This tells us that for some lifetime `'a`, our function takes two parameters
which both have the **same** lifetime, which is also shared by our return slice.
The compiler doesn't need to know exactly how long x and y will live for, just
that there is some scope that can be substituted for 'a that is truthy. Once
interpreted by the compiler, the compiler will assign a concrete lifetime that
is equal to the smaller of the two lifetimes of x and y, and if the return value
does not satisfy the smaller of the two lifetimes - it won't compile either.  We
need to be **truthful** and help to clear up ambiguity for our compiler.

Let's look at lifetime annotation syntax

## Lifetime Annotation Syntax

We specify lifetimes using an `'` followed by a simple lowercase english
character, such as `'a`.  Here are some examples:

```Rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

Let's look at how to specify lifetimes in struct definitions.

## Lifetime Annotations in Structs 

Indicating lifetimes in the definition of a struc  is a bit different.  We put
it in the same place we would declare a generic type.  Like such:

```Rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}
```

We need to speficy a lifetime here because we are feeding in a string slice
which is **not** owned, and so we have to assure rust that the slice will exist
for as long as the lifetime of the struct (and it does in this example.  The
struct and the slice go out of scope at the exact same time - at the end of the
function block)

### Lifetime Rules

Why haven't we had to supply lifetime annotations for every single function
until now?  Well, Rust's compiler will infer lifetime by applying **three**
simple rules:

1. Each parameter that is a reference gets it's own lifetime parameter

That means a function with one parameter gets one lifetime parameter `fn
foo<'a>(x: &'a i32)`, with two parameters it gets auto assigned two lifetimes
`fn foo<'a>(x: &'a i32, y: &'b i32) and so on.

2. If there is **exactly one** input lifetime parameter, that lifetime is assigned
   to all output lifetime parameters
3. If there are multiple input lifetime parameters, but one of them is `&self`
   (like in a method) then the lifetime of self is assigned to **all** output
lifetime parameters.

Because of rule #3 this means that we rarely need to specify lifetime parameters
on methods!  

### Lifetime Annoations on Methods

If we do need to be explicit about lifetimes in method definitions, we define
them similarly to how we define generic types in `impl` blocks:

```Rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

In this example (and nearly all cases where a method takes &self) we don't have
to specify lifetimes in the function itself because of rule #3.


})
}
