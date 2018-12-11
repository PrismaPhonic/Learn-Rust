# Chapter 4

# Table of Contents
1. [Stack vs. Heap](#stack-vs-heap)
2. [Ownership Rules](#ownership-rules)
3. [Copy vs Move](#copy-vs-move)
4. [Clone](#clone)
5. [Transfering Ownership](#transfering-ownership)
6. [References and Borrowing](#references-and-borrowing)
7. [Mutation and Borrowing](#mutation-and-borrowing)
8. [Dangling References](#dangling-references)
9. [Slices](#slices)
    1. [String Literals](#string-literals)
    2. [Arrays](#arrays)

## Ownership
#### Stack vs Heap 

The stack is very fast because it always accesses what is on the top of the
stack.  It is for memory of a fixed size, like scalars.  If we don't know
exactly how much memory something needs to take up then it goes on the heap.
The heap is slower and when something is placed on the heap, it gets a pointer
put onto the stack - because a pointer is a known fixed size.  When we need to
use or modify the actual data in the heap, we need to follow a pointer to it -
which is slow.  

#### Ownership Rules

1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

For any piece of data, ownership is initially given to the variable that was
intially assigned that data.  That variable is the owner.  If ownership is not passed
along, then when that owner goes out of scope (the block ends and the variable
was not returned) the memory that was used to store that data automatically gets
returned.

#### Copy vs. Move

```Rust
let x = 5;
ley y = x;
```

In the above example, both x and y are equal to 5 and each version of the
integer 5 has a clear place in the stack - because 5 was copied when we assigned
x to y.  This is not true of data that's on the heap, which by default is passed
by reference.  What this means under the hood is that if we assign a new
variable to data on the heap then that variable is a pointer - that is data on
the stack that simply points to the real data on the heap. But remember that
when an owner goes out of scope the memory must be dropped!  What happens if one
pointer goes out of scope while another pointer stays in scope?  We might get a
_double free error_!  Rust solves this problem by **invalidating any previous
references when a new reference is made**.  That means that we can only have
**one** active pointer to mutable data!

#### Clone

If we want to deeply copy the heap data and not just the stack data, we can use
the clone method like such:

```Rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

The above works, and we won't get an error that we are calling an invalidated
pointer like we would if we didn't use clone!

#### Transfering Ownership

In Rust Ownership is transfered in two ways:
1. by passing a variable to another function
2. By returning a variable at the end of a function

If data is created inside a block, then that data will go out of scope (and have
it's memory freed) unless it is returned by the block or function.  If we pass a
variable into a function, it gains ownership and then either returns that
variable to keep it in memory, or runs the drop function at the end of it's
scope to drop the data from memory that the variable is in ownership of.

### References and Borrowing

The problem with transfering ownership to a function is that the variable must
be explicitely returned to keep it's data from dropping, and what if you have to
return some other value as well from the function?  Instead we can pass a
reference like so:

```Rust
let s1 = String::from("hello");

let len = calculate_length(&s1);

println!("The length of {} is {}.", s1, len);
```

In this example the function 'calculate_length' does not need to return the
variable passed to it to keep it from dropping because we are passing it a
reference using '&' syntax.  If we had instead passed just s1 then we would not
be able to print it out later along with the length, as it would have been
dropped from memory at the end of calculate_length's scope. Essentially we
create a pointer to the pointer (s1) which still maintains ownership over the
data it points to in the heap.

This is referred to as **borrowing**.

#### Mutation and Borrowing

You can't mutate a borrowed value, unless you pass the reference with the
following syntax:

```Rust
let mut s = String::from("hello");

change(&mut s);
```

The one caviat with mutable references is that **you can only have one mutable
reference to a particular piece of data in a particular scope**.  

One technique to mitigate this challenge is to define a new scope!  A new scope
can be defined by braces anywhere, like such:

```Rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
}

let r2 = &mut s;
```

Another rule is that you cannot combine mutable and immutable references to the
same piece of data.  Pick one and stick to it!  Either the data is mutable or
it's not.

#### Dangling References

A dangling reference is when a reference still exists while it's data has been
dropped.  This could happen if you have a function that returns a reference to a
variable that was declared in the function.  At the end of the function, the
variable goes out of scope and drops it's data, but if a reference is being
returned then it's a dankling reference.  

Instead, just return the variable itself so you transfer ownership to whomever
calls the function.

### Slices

Going to try to summarize this pretty quickly because there's not too much to
it.  A slice is pretty similar to other languages, but in Rust, slices are
references.  They have a start value (the pointer) and then a length.  You could
use a string slice to make a substring of a longer string.  For example:

```Rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

#### String literals

By definition all string literals are actually slices - when your code is
compiled to binary, your string literals are literally stored in binary, and so
string literals point to sections of the binary code.

String literals are of the type &str.  If you wanted to write a function that
returns a the first word of a String type you could write:

```Rust
fn first_word(s: &String) -> &str {
```

or if we wanted a function that could take slices as well we could write it in
such a way that if we need to analyze the entire string, we just send the
function a slice of the String like such:

```Rust
fn first_word(s: &str) -> &str {
```

#### Arrays

We can also take slices of arrays, but again, in Rust a slice will always be a
reference.  For an array filled with types i32, a slice would have a type of
&[i32]

```Rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```
