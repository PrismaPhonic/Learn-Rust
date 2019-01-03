# Chapter 19

# Table of Contents
1. [Unsafe Rust](#unsafe-rust)
    1. [Dereference a Raw Pointer](#dereference-a-raw-pointer)
    2. [Call an Unsafe Function](#call-an-unsafe-function)
    3. [Use Mutable Static Variables](#use-mutable-static-variables)
    4. [Implement an Unsafe Trait](#implement-an-unsafe-trait)

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


