# Chapter 15

# Table of Contents
1. [Box<T>](#box<t>)
    1. [Using Box<T> to Store Data on Heap](#using-box<t>-to-store-data-on-heap)
    2. [Building a Recursive List](#building-a-recursive-list)

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



