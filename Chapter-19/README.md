# Chapter 19

# Table of Contents
1. [Unsafe Rust](#unsafe-rust)
    1. [Dereference a Raw Pointer](#dereference-a-raw-pointer)
    2. [Call an Unsafe Function](#call-an-unsafe-function)
    3. [Use Mutable Static Variables](#use-mutable-static-variables)
    4. [Implement an Unsafe Trait](#implement-an-unsafe-trait)
2. [Advanced Lifetimes](#advanced-lifetimes)
    1. [Lifetime Subtyping](#lifetime-subtyping)
    2. [Lifetime Bounds](#lifetime-subtyping)
    3. [Inference of Trait Obj Lifetimes](#inference-of-trait-obj-lifetimes)
    4. [The anonymous lifetime](#the-anonymous-lifetime)
3. [Advanced Traits](#advanced-traits)
    1. [Placeholder Types in Trait Defs](#placeholder-types-in-trait-defs)
    2. [Default Generic Types](#default-generic-types)
    3. [Supertraits](#supertraits)
    4. [Implement External Traits on External Types](#implement-external-traits-on-external-types)
4. [Advanced Types](#advanced-types)
    1. [NewType Pattern](#newtype-pattern)
    2. [Type Aliases](#type-aliases)
    3. [The Never Type](#the-never-type)
    4. [Dynamically Sized Types](#dynamically-sized-types)
5. [Advanced Functions and Closures](#advanced-functions-and-closures)
    1. [Function Pointers](#function-pointers)
    2. [Returning Closures](#returning-closures)
6. [Macros](#macros)
    1. [Macros vs Functions](#macros-vs-functions)
    2. [Declarative Macros](#declarative-macros)
    3. [Procedural Macros](#procedural-macros)
    4. [Attribute Like Macros](#attribute-like-macros)
    5. [Function like macros](#function-like-macros)

# Advanced Features

# Unsafe Rust

Unsafe rust gives us the ability to do four things:

1. Dereference a raw pointer
2. Call an unsafe function or method
3. Access or modify a mutable static variable
4. Implement an unsafe trait

To use unsafe rust we use the `unsafe` keyword when starting a new block that
holds the unsafe code. The code might not actually be unsafe, but we are
garaunteeing that for the four action types listed above that we as programmers
have verified the code is safe. Why would we ever need this? The borrow checker
is very conservative and blocks certain actions if it can't deem them
to be absolutely safe - for this reason there are some cases where we as
programmers know that the code is safe but the compiler doesn't and we may need
to mark our code as `unsafe`.  This enables us to do one of the four abilities
listed above.

## Dereference a Raw Pointer

Rust doesn't let us use raw pointers in safe mode because they can easily lead
to dangling pointers.  We can have immutable raw pointers `*const T` or
immutable raw pointers `*mut T`.  In the context of a raw pointer immutable
means that it can't be assigned after it has been dereferenced.

Here are four differences between raw pointers and safe pointers in rust:

1. Raw pointers are alloweed to have both immutable and mutable pointers, or
   multiple mutable pointers to the same location
2. Are not garaunteed to point to valid memory
3. Are allowed to be null
4. Don't call the drop function (no auto cleanup)

Let's try to create some mutable and immutable raw pointers:

```rust
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}
```

Notice how `*const` and `*mut` are part of the variables **type**. In this case
we have two raw pointers, one mutable and one non-mutable pointing at the same value
in memory. Also we can create the raw pointers anywhere in rust but we can only
dereference raw pointers in `unsafe` blocks.

## Call an Unsafe Function

If we want to call an `unsafe` function we must do it inside an `unsafe` block:

```rust
unsafe fn dangerous() {}

unsafe {
    dangerous();
}
```

Usually this isn't necessary because safe abstractions are created over unsafe
code - we can call a safe function that in turn calls an unsafe function for us.
Let's look at that now.

We can also call an function from an external library of a different language
(C, Python etc). Because Rust can't check another language to make sure it meets
all of it's safeguards, we have to call these external function in unsafe
blocks:

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

We are pulling in an external function from the C standard library and calling
it inside an unsafe block.  The `"C"` part of `extern "C"` block defines which
_application binary interface (ABI)_ the external function uses. The `"C"` ABI
is the most common and works for C libraries. We can also call Rust functions
from other languages by defining them with the `extern` keyword:

```rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

### Creating a Safe Abstraction over Unsafe Code

Let's try to implement a function from the standard library that under the hood
uses some unsafe code and then create a safe abstraction over that unsafe code.
The function `split_at_mut` from the standard library takes a mutable slice and
returns two mutable slices that are separated by the index provided.  

Here's an example of how it works:

```rust
let mut v = vec![1, 2, 3, 4, 5, 6];

let r = &mut v[..];

let (a, b) = r.split_at_mut(3);

assert_eq!(a, &mut [1, 2, 3]);
assert_eq!(b, &mut [4, 5, 6]);
```

If we tried to implement this function using safe rust we would get an error
that we are mutably borrowing our slice twice (once for the first half and once
for the second half):

```rust
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    (&mut slice[..mid],
     &mut slice[mid..])
}
```

This is illegal under the borrow rules. But we know it's safe! The two borrows
references different portions of memory and there are no overlaps. In this case
we need to mark our code as **unsafe** and use raw pointers so we can create
this functionality.

```rust
use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}
```

We create a raw pointer outside of the unsafe block because that's allowed -
when we drop down to the `unsafe` block we create a raw pointer that starts at
the raw pointer we created and goes for a length up to the `mid` value provided
(since a pointer is just a memory address and a length).  Then our next mutable
slice starts it's pointers at an offset of the mid and goes up to the full
length - mid.  

Here's the full code(feel free to drop this into your `main.rs` and give it a
try):

```rust
let mut v = vec![1, 2, 3, 4, 5, 6];

let r = &mut v[..];

let (a, b) = r.split_at_mut(3);

assert_eq!(a, &mut [1, 2, 3]);
assert_eq!(b, &mut [4, 5, 6]);
```

## Use Mutable Static Variables

We can use mutable static variables in Rust which is unsafe because it can lead
to race conditions when multiple parts of your program are trying to access a
mutable shared variable at once. That's why rust doesn't allow it in safe rust.
Let's define a static mutable variable:

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

We define a **static** variable with `static` in front of the variable name.
Anytime we mutate that value we have to do it in an `unsafe` block. 

## Implement an Unsafe Trait

If a trait is unsafe then we have to implement it using an unsafe declaration.
A trait is unsafe if at least one of it's methods contain unsafe code.  Let's
see how to implement an unsafe trait:

```rust
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}
```

# Advanced Lifetimes

We learned in Chapter 10 about lifetimes, but there are some advanced lifetime
topics we didn't cover then:

1. Lifetime Subtyping: ensures one lifetime outlives another lifetime
2. Lifetime bounds: specifies a lifetime for a reference to a generic type
3. Inference of trait object lifetimes: allows the compiler to infer trait
   object lifetimes
4. The anonymous lifetime

Let's cover these now.

## Lifetime Subtyping

_Lifetime subtyping_ ensures that one lifetime should outlive another lifetime. I'm not actually sure if this is necessary anymore as of December 23rd, 2018 when the Rust team introduced non-lexical lifetimes. My understanding is that the compiler now can look across your library to determine lifetimes based on the control flow graph rather than lexical scopes.  Here's in their example where the author claimed this should still not compile, but it does on my system:

```rust
struct Context<'s>(&'s str);

struct Parser<'c, 's> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}
```

In this example we are creating a mock of a parser (that doesn't actually parse
anything, very simple mock).  We have a tuple struct called `Context` which
holds a string slice.  Our `Parser` struct has a field `context` which holds a
`Context` type.  Then we have an `impl` block for `Parser` with a `parse` method
that returns nothing on success or a sub-slice of the string slice being held by
the `Context` in the `Parser`.  We then have a totally separate function called
`parse_context` that takes ownership of a `Context` type and then constructs a
`Parser` instance right there on the spot and giving it a slice of this
`Context` that it has taken ownership of - running the `parse` method on the
just now instantiated `Parser` which would theoretically return a result.

This seems really hairy but all we care about is that the `&str` in the
`parse_context` function return is garaunteed to live as long as the `&str` that
the `Context` is holding onto which should live **longer** than either `Context`
or `Parse`.  As long as that holdds true it doesn't matter that we drop both
`Context` and `Parser` at the end of `parse_content`'s scope.

To handle this we create two lifetimes, one for `Context` type and one for the
`&str` it holds onto.  We then pass that down the line to make it clear that the
`&str` being returned by `parse` is tied to that original `&str` that `Context`
is holding onto which has a **different** lifetime (annotated with `'s` for
string slice lifetime and `'c` for `Context` type lifetime).  

The book says that this still shouldn't run because the rust compiler can't tell
that the lifetime `'s` we declare in the signature of our `Parser` struct is
tied to the same `'s` as the one we established for our string slice since these
are different lexical scopes and so we have to let Rust know that `'s` will live
longer than `'c` using subtyping.  We would do that by changing the signature of
our `Parser` struct to `struct Parser<'c, 's: 'c> {`.  When we do this we tell
rust that `'s` is some type that will live at least as long as `'c` but is not
tied directly to `'c`.  

In my experience the program compiled without this so I think the change to use
non-lexical lifetimes has perhaps made subtyping unneeded?  Let's keep this one
in our toolbox just in case.

## Lifetime Bounds

Lifetime bounds are similar to trait bounds. They are a way that we can tell
Rust to enforce that our references in generic types won't outlive the data they
are referencing.  

To put it more simply, it's a way for us to define a lifetime
for the data that a generic points at, and not simply define the lifetime of the
reference itself.

Let's look at an example that won't compile:

```rust
struct Ref<'a, T>(&'a T);
```

This won't compile because rust cannot ensure that our reference won't live
longer than the data it points at.  To fix this we can specify that our generic
`T` has a lifetime that is at least as long as our reference:

```rust
struct Ref<'a, T: 'a>(&'a T);
```

The key here is changing `T` to `T: 'a` that is that the generic (the value
itself in memory) has a lifetime of `'a` which is the same lifetime shared by
the reference to `T`. Now rust can enforce that our reference does not live
longer than the value it points at.

## Inference of Trait Obj Lifetimes

The compiler will automatically infer the lifetimes of trait objects for us.
Consider the following code:

```rust
trait Red { }

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> { }

fn main() {
    let num = 5;

    let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;
}
```

This code compiles fine even though we haven't explicitely annotated the
lifetimes involved for the trait object. The compiler follows the following four
rules when it comes to inferring the lifetimes of trait objects:

1. The default lifetime of a trait object is `'static`.
2. With `&'a Trait` or `&'a mut Trait` the default lifetime of the trait object
   is `'a`
3. With a single `T: 'a` clause, the default lifetime of the trait object is
   `'a`.
4. With multiple generic lifetime bounds there is no default lifetime so we have
   to be explicit

When #4 is true we can explicitely define lifetime bounds on a trait object like
`Box<dyn Red>` using the syntax `Box<dyn Red + 'static>` or `Box<dyn Red + 'a>`
(`'static` if it lives for the entire program or `'a` if not).

## The Anonymous Lifetime

There's an anonymous lifetime called with `'_` which tells the rust compiler to
use the elided lifetime (why wouldn't it just do this by default?!).

Here's the example the book gives. If we have a struct that wraps a string
slice:

```rust
struct StrWrap<'a>(&'a str);
```

and then we have a function that simply takes a string slice and returns the
struct `StrWrap` with that slice wrapped:;

```rust
fn foo<'a>(string: &'a str) -> StrWrap<'a> {
    StrWrap(string)
}
```

We are good to go - but apparently we can just write it with the anonymous
lifetime instead so we don't have to use `'a` in so many places:

```rust
fn foo(string: &str) -> StrWrap<'_> {
    StrWrap(string)
}
```

The book sadly does **not** explain why this works, or why we don't simply just
put these all over the place. Wish it went more in depth on this!

I found another resource that explained it like this: 

```
What exactly does `'_` mean? It depends on the context! In output contexts, as
in the return type, it refers to a single lifetime for all "output" locations.
In input contexts a fresh lifetime is generated for each "input location."
```

What doesn't really make sense to me here is why we would need it at all?
According to the lifetime ellision rules  you can elide lifetimes if you only
have one input, or one of your inputs is `self` - in either case the lifetime of
the single input, or the lifetime of self is applied to the output type as well.
Soooooo.... wtf?!

# Advanced Traits

## Placeholder Types in Trait Defs

We've already seen placeholder types in trait definitions. Let's look back at
the `Iterator` trait:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

There's a placeholder type of `Item` which is the same type that is returned
inside an `Option` from the `next` method. Later when we implement `Iterator` we
can specify the type of this placeholder:

```rust
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
```

That's really all there is to it. The reason why we might want to do it this way
is that by doing it this way rather than using a generic is that a generic might
end up getting multiple different hard coded types at runtime, but for this
single implimentation we only get one type to define for our custom struct.


## Default Generic Types

When using generic type parameters we can specify a default concrete type. If an
implimentor of the trait does not define a concrete type it will use a default
one. Let's look at a case of operator overloading to demonstrate this:

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
}
```

In this case we are overloading the `+` operator for our struct `Point`. We
specify that when we add two `Point` types together that we are summing the x
and y fields to produce a new `Point` type.  

Here's what the definition for `Add` trait looks like:

```rust
trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
```

It specifies a default type for `RHS` (stands for right hand side) of being
`Self` - thus we specify that the second argument to `add` method is the same
type as the first argument of `self`. The general syntax for default type
parameters is `<PlaceholderType=ConcreteType>`.

We can also overload the `+` operator such that the two arguments are of
different types and not use the default of the two arguments being the same
type:

```rust
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

cool!

I'm going to skip over the next section from the book on Fully Qualified Syntax
because I honestly can't think of a use case for when I would ever need it.
knock on wood I guess. 

## Supertraits

Supertraits are useful when we are designing a trait that needs another trait
for it's inherent functionality.  For instance, let's say we had a trait that
draws an outline around a `Point` struct coordinates on the screen.  In that
sense it would have to impliment `Display` trait itself to have access to
`println!`.  We could define it like such:

```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

Our trait `OutlinePrint` specifies that it needs the `Display` traits
functionality by specifying: `OutlinePrint: fmt::Display`.  This allows it to
print an outline:

```
**********
*        *
* (1, 3) *
*        *
**********
```

What this really does is enforce that anything that implements `OutlinePrint`
**must** also implement `Display`.  That means that this will not compile:

```rust
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}
```

until we implement `Display` like such:

```rust
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

Once we do this we can now implement `OutlinePrint`.

## Implement External Traits on External Types

The orphan rule keeps us from being able to implement external traits on external types but there's a pattern we can use to get around this called the _newtype pattern_. This pattern is essentially putting a wrapper around the external type of a tuple struct and then because we have defined a new type locally - we can apply an external trait to it. Let's try that now by wrapping a `Vec<T>` so we can implement our own version of `fmt::Display`:

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

The caviat with this is that we will not have the methods available to us from
`Vec<T>`. We can apparently still get access to them if we want by implementing
`Deref` to get at the value inside the wrapper and expose the methods there.

That's it for advanced traits! Now onto advanced methods of interacting with the
type system.

# Advanced Types

## NewType Pattern

The newtype pattern that we saw can also be used to create a new type around a
common type so that the rust compiler will enforce type checking and enforce
functional logic. An example would be if we created a `Meters(u32)` tuple struct
and a `Millimeter(u32)` tuple struct. They are functionally just `u32`'s under
the hood but we could enforce that a certain function only take a certain type
as an argument, or return one of the types.

We can also use it as a way to abstract logic away.  If we wrap a `Vec<T>` in a
tuple struct then we can choose which methods from `Vec<T>` to expose to the
public API of our new type.

## Type Aliases

I was actually a bit confused getting to this section because type aliases are
written identically to how you would write an associated type for a Trait. I've
tried searching far and wide across the web and can't seem to find any different
between the two - in fact maybe an associated type is just a type alias inside a
Trait? Might need to edit this doc later to update.

Essentially a type alias is just a way we can shorten a really long type into a
short name.  For instance, let's say we are making a tree and our nodes are of
the type `Option<Box<Tree<T>>>`.  We could store this as a shorter alias:

`type TreeNode<T> = Option<Box<Tree<T>>>`

We could then call it anywhere we like such as an input variable of type
`TreeNode<i32>`. Keep in mind that we are not creating a new type like we did
with the _newtype pattern_ but instead just creating an alias. We still have
access to all the methods we would on the type we have aliased.

## The Never Type

In Rust there's a special type called `!` that **never returns**.  Some
languages call this an _empty type_. We use this when we want a function to
never return:

```rust
fn bar() -> ! {
    // --snip--
}
```

This is useful if we have a match whose arms must all be the same type. Remember
back to our guessing game:

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

In this example `continue` has a return type of `!`.  Because of this we are
allowed to include it in a match arm even though it's not of the type `u32`. In
other words a never type can be coerced into any other type because it will
never itself return a type. Let's also look at the unwrap method on `Option`:

```rust
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```

What's useful to note here is that `panic!` macro has a type of `!` and
therefore it can be used in the match arm.

## Dynamically Sized Types

It might be that a values size is not known until runtime. Think back to string
lices `&str` where the size is known at compile time - all that `&` is in `&str`
is a pointer to a memory address with a length. There's also `str` in rust which
is a type whose size cannot be known at compile time. We had to put `str` behind
a pointer `&` to use it. In general anything where we don't know the size at
compile time must be put behind a pointer of sometime whether thaht's `&`, `Box`
or `Rc`. 

When we discussed trait objects we also saw this with declarations like `Box<dyn
Trait>`. In this case we don't know how big the trait is going to be at compile
time so we stick it behind a pointer. Under the hood rust has a trait called
`Sized` that determines whether a type's size is konwn at compile time. Rust
adds a trait bound of `Sized` on every function that uses a generic - it just
let's us omit having to write it:

```rust
fn generic<T: Sized>(t: T) {
    // --snip--
}
```

can instead be written:

```rust
fn generic<T>(t: T) {
    // --snip--
}
```

We can overwrite this behavior (and we can **only** do this trick for the
`Sized` trait) if we know our generic will have a size that can't be known at
compile time but using the trait bound `?Sized` which in this case means "we
don't have any idea how big this thing will be at compilation time":

```rust
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```

We also have to make our input variable some kind of pointer type -
in this case we just used a simple `&`.

# Advanced Functions and Closures

## Function Pointers

We can write a function that takes as an argument a function definition. All
functions in rust coerce to a type of `fn`. This is different from the `Fn`
trait that applies to both closures and functions in rust. If we wanted to write
a function that takes **only** other functions (and **not** closures) we could
write it like such:

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}
```

We are defining that our `do_twice` function will take one argument `f` of type
`fn(i32) -> i32`. Again this is different from the `Fn` trait bounds we used in
our earlier IO project.

This can be very limiting and it's usually better to write it with the trait
bounds since both functions and closures implement the `Fn` trait:

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice<T: (Fn(i32) -> i32)>(f: T, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    let add_two = |num| {
        num + 2
    };

    let answer2 = do_twice(add_two, 5);

    println!("The first answer is: {}", answer);
    println!("The second answer is: {}", answer2);
}
```

In this example our `do_twice` function can take either a closure or a function
as it's type because we used a generic that we bound to the `Fn` trait. In
`main` we successfully call it with our function and a closure we build inside
`main`.  This would not be possible with a function pointer - so why even use
function pointers?

There are some cases where a function pointer is necessary - like interfacing
with another language.  C for instance does not have closures so we might need
to specify an `fn` type rather than a generic bound by the `Fn` trait.

## Returning Closures

If we want to return a closure from a function whose job it is to generate a
closure, the return type is confusing because closures don't have known types.
All we know about closures is that they implement certain traits - we therefore
must implement them as trait objects:

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

Now our code will compile because we have provided a valid type for our return -
that is a box that points at something (our closure in this case) that
implements the `Fn` trait.

That's it for advanced function features, onto macros!

# Macros

## Difference between Macros and Functions

A primary difference between macros and functions is that functions must declare
how many arguments they take, and the types of the arguments whereas macros can
take any variable number of arguments. This makes macros harder to maintain as
their definitions have to be more complex to handle the extra functionality they
provide.

The book says that the difference between macros and functions is also that you
must define or bring macros into scope before you call them - whereas you can
define functions anywhere and call them anywhere (the functions are hoisted, but
the macros are not).

## Declarative Macros

Declarative macros are macros like `vec![]`.  They are defined using the
`macro_rules` construct:

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

The `#[macro_export]` annotation makes sure that this macro will be available
anywhere in the crate it's imported in (after the import line itself).
`macro_rules!` is put in front of the name of the macro we are defining, and our
macro name is written **without** the `!` (that's added for us).  This then
essentially looks like a match statement.  We are matching vec on which we have
some arms - well, in this case only one arm.  The first arm matches any pattern
that fits `$( $x:expr  ),*`.  What the hell does this mean?  Well, the first
dollar sign followed by parentheses captures the values that match the pattern
for use in the arm.  Then we have any rust expression `expr` and assign it to
the name `$x`.  The comma star `, *` means that we will match lazily - that is
we will match zero or more of things that also match what came before the comma.
In this case we will take all of those matches and push each into the vec.

If we call `vec![1, 2, 3]` it will be expanded to:

```rust
let mut temp_vec = Vec::new();
temp_vec.push(1);
temp_vec.push(2);
temp_vec.push(3);
temp_vec
```

That is that 1, 2, and 3 will match and then get pushed into a new vec and that
vec then returned to us. Declarative macros match against patterns and replace
the code with other code. Let's look now at procedural macros


## Procedural Macros

The example from the book (for derive macros) is extremely dense on procedural macros and introduces
wayyyyyyyy too many new concepts at once (please, heavily consder a re-write if
the author ever happens to read this).  I'm going to skip you the pain I went

Procedural macros are just functions that take a TokenStream and output a
TokenStream.  A TokenStream is basically what your code turns into by the
compiler right before it gets compiled.  It's a stream of "tokens" - so a
procedural macro intercepts your code on it's way to get compiled and mutates
something about the token stream - whereas a declarative macro literally
re-writes your code in place and happens well before compilation time.

## Attribute Like Macros

So attribute macros are like derive macros but instead of generating code for
the `derive` attribute, they allow you to create new attributes. Derive only
works for structs or enums while attribute-like macros can go on other items
like functions (honestly this whole section is so unbelievably confusing - the
team that writes the rust language book should heavily consider a re-write of
the macro section).

## Function-like Macros

function like macros essentially take arguments that look like function calls.
Here's an example from the book of a macro that would generate a call to a
relational database using some sql:

```rust
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

The macros definition would look like this:

```
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

In this sense it looks very similar to other procedural macros, but it looks
more like a function call.


Honestly I understand declarative macros very well but the sections on
procedural macros were very convoluted and confusing. If I find myself using
them in my actual code writing and I learn them a bit better I will come back
and re-write these sections.

Looks like we are done with these sectionns - next Chapter that ends the book is
a final project building a web server!
