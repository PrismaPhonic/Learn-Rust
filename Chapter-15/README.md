# Chapter 15

# Table of Contents
1. [Box<T>](#box<t>)
    1. [Using Box<T> to Store Data on Heap](#using-box<t>-to-store-data-on-heap)
    2. [Building a Recursive List](#building-a-recursive-list)
1. [Deref Trait](#deref)
    1. [Deref Coercion](#deref-coercion)
1. [Drop Trait](#drop-trait)

# Smart Pointers
What are smart pointers? Two examples of smart pointers we've already seen are
`String` and `Vec<T>`. These are pointers that take ownership over the data they
point to (this isn't a requirement of all smart pointers). They also contain
extra metadata beyond what a normal pointer would hold, (such as `String`
unsuring all of it's data is always valid UTF-8).  By definitions all smart
pointers must implement `Deref` and `Drop` traits.  When a smart pointer goes
out of scope it will run `Drop` to clean up the data on the heap that the
pointer will point to.

# Box<T>

The simplest smart pointer is a _box_.  Boxes very simply are smart pointers
that allow us to store data on the heap rather than the stack.  They are useful
for solving three problems:

1. When you have a type whose size can’t be known at compile time and you want
to use a value of that type in a context that requires an exact size
2. When you have a large amount of data and you want to transfer ownership but
ensure the data won’t be copied when you do so
3. When you want to own a value and you care only that it’s a type that
implements a particular trait rather than being of a specific type


We'll look at solving problem #1 through a simple example, but first, let's see
a **very** simple example to demonstrate the core of how boxes work in rust.

## Using Box<T> to Store Data on Heap

In it's simplest sense a box is just a smart pointer to data we want to store on
the heap, and that data is cleaned up when our box goes out of scope.  Take this
example:

```Rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

This will print out `b = 5`.  Pretty useless to use a box this way but all
that's happening is we are allocating space to store the integer `5` on the heap
rather than the stack and then printing it's value.

Let's look at how we can solve problem #1 listed above by trying to create a
recursive list in Rust

## Building a Recursive List

Let's say we wanted to build a simple `Cons` list, which is a data type common
in functional programming languages.  We could write an enum like such:

```Rust
enum List {
    Cons(i32, List),
    Nil,
}
```

We could then call it like such:

```Rust
use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

This won't run!  We will get an error that we are trying to declare a recursive
type `List`that has an **infinite** size. That is because our enum type could go
on indefinitely - never terminating at `Nil` and so Rust doesn't know at compile
time how much data to allocate for our `List`.  We can instead use a `Box<T>`
for indirection - that is to say that our recursive list will instead use smart
pointers to point at the next iteration of the list.  The data then will be
stacked side by side with each box pointing to the next list.  Rust knows for
sure the size of a pointer and so it will compile if we re-write our recursive
list to use boxes like such:

```Rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil)
		))
	    ))
	);
}
```

Now any list value will take up the size of an i32 plus the size of one pointer!

When a Box<T> goes out of scope it runs `Drop` and cleans up the data on the
heap that it points to.  It also implements the `Deref` trait which allows it's
values to be treated like references.  Let's get into these two traits common to
all smart pointers next.


## Deref Trait

Let's cover the `Deref` trait in more detail now.  First, let's look at a simple
example of how the dereference `*` operator works:

```Rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

In this example we set y to be a pointer to x.  if we tried to compare it
directly to 5 we would get that 5 != &5 (basically). We use the dereference
operator to get the value that we are pointing at and compare the literal value.
Rust knows how to dereference any `&` reference, so why do we need a `Deref`
trait at all?  Well, `Deref` trait has a method called `deref` where we can
instruct Rust on how to **create** an `&` reference to the data in our custom
data type.  Then it can apply the dereference operator to get at the value
itself **without** taking ownership.  

Let's imagine that we tried to create our own box using a tuple struct:

```Rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

If we then try to use our box in place of our basic code above, we'll get an
error. The compiler will tell us that `MyBox` cannot be dereferenced - because
it doesn't implement the `Deref` trait yet!  Lets go ahead and do that:

```Rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
```

A few new things here - what the heck is `type Target = T`? Well, apparently
we'll get into it in Chapter 19, but for now it's a slightly different way of
declaring a generic parameter.  We implement a method called `deref` which
instructs Rust on how to create an `&` reference.  In this example our target is
some generic T which is located at `self.0` (because this is a tuple struct and
that's how you get at indexes in tuples).  Rust now know how to create a
reference to our generic.  Now if we use our box, Rust will know how to properly
dereference it:

```Rust
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

Under the hood Rust will run change the call to `*y` to this: `*(y.deref())`.
In other words, it will run our deref method so it can create an `&` reference
to the data stored in our struct tuple, and then subsequently use the derefence
operator to get at the data itself without taking ownership of the data.

### Deref Coercion

Rust handles coercing types in arguments passed to a function to the types wanted by that function if such coercion can take place through chaining of `deref` methods
(I think this definition is much better than the one in the book). This happens
at compile time and is hard coded for us so we don't pay a performance penalty
at runtime for deref coercion.  To wrap our heads around this let's look at a
simple function called hello which takes as an argument a string slice:

```Rust
fn hello(name: &str) {
    println!("Hello, {}!", name);
}
```

We could then call the function this way (yes, I know this seems strange):

```Rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

I know this looks like it shouldn't work, but it does!  What's happening here?
Well, when rust sees `&m` it runs the `deref` method we wrote for our tuple
struct and turns it into an `&String` which is still not the type that our
`hello` function accepts!  So then what happens?  Well, Rust will then look at
the `Deref` implementation on the `String` type and see that it essentially
helps rust to convert a `String` into a `&str` and so it coerces it by calling
the `String` types `deref` method and voila - our function call actually works!

This is pretty confusing, so play with this some and re-read this section a few
times until it starts to sink in.

Now that we've covered `Deref`, let's look at the other trait all smart pointers
implement: `Drop`.

## Drop Trait
