# Chapter 3

This chapter is pretty dense - we covered Variables, Data Types, Functions and Control
Flow.  Note that I did the extra suggested projects at the end which you can
find in this folder

# Table of Contents
1. [Const vs. Let](#const-vs-let)
2. [Shadowing](#shadowing)
3. [Data Types](#data-types)
    1. [Scalar](#scalar)
        1. [Integer](#integer)
        2. [Floating Point](#floating-point)
        3. [Boolean](#boolean)
        3. [Characters](#chars)
    2. [Compound Types](#compound-types)
        1. [Arrays](#arrays)
        2. [Tuples](#tuples)
4. [Control Flow](#control-flow)
    1. [If Expressions](#if-expressions)
    2. [Loops](#loops)
        1. [Loop](#loop)
        2. [While](#while)
        3. [For](#for)

## Variables
#### Const vs Let 

Yes, Rust uses **const** and **let**!  But they behave very differently than in
Javascript.  Remember that by default **all types in rust are immutable!**.  So
then why even bother with const?  Const will define a variable to a constant
expression at compile time.  If the value of the expression cannot be determined
at compile time, then your program won't compile!  Const's in Rust can also be
declared in the global scope while let's cannot.  Another difference: let
variables can have their type inferred by the compiler while const's must have
their type declared at assignment.

```Rust
const MAX_POINTS: u32 = 100_000;
```

Note how the common convention of ALL_CAPS snake cased?  Also note that in Rust,
like Ruby we can use _ to create visual space in our integers (because commas
are not allowed in ints).

Consts are also valid for **the entire time that your program runs** and will
not be dropped when they leave a scope the way that let will.

#### Shadowing

```Rust
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}
```

Note in the above example that we use let multiple times.  Everytime we are
creating a **new variable called x**.  When we print the value of x, we will get
the most recent declaration.  In this case that would be 12, as each new
declaration references the previous x (think of it like a stack)

```Rust
let mut spaces = "   ";
spaces = spaces.len();
```

The above code is **illegal** in Rust - even if a variable is declared as
mutable, we can never mutate a variables **type**.

## Data Types

### Scalar

A _scalar_ type represents a **single** value, and can be either: integers,
floating-point numbers, bools, or chars

#### Integer

Numbers without fractional components (non-floats). Can be signed (pos or neg)
which is i<bit-length> such as i8, i16 etc.  Unsigned (only pos) is u8, u16 etc.
i32 is the fastest.  Can use _ as visual separators instead of commas 100_000
rather than 100,000.

#### Floating Point 

Rust uses f32 and f64 for floating point numbers and defaults to f64.  virtually
the same speed so just use f64.

#### Boolean

Self explanatory - same as any other language

#### Chars

Single characters, like any character on the keyboard, or any unicode character.

### Compound Types

Different from Scalar in that it can represent multiple points of data.  The two
compound types are arrays or tuples which behave very differently in Rust than
in Javascript or Python.

#### Arrays

An array is a collection of variables which all have the **same type** and whose
number of elements is fixed at the time of creation.  elements are processed with bracket notation array[i].

#### Tuples

tuples are collections of variables which can all have different types, and
whose number of elements is fixed at time of creation. Because they can have
different types, their types must also be declared at the time of creation:

```Rust
let x: (i32, f64, u8) = (500, 6.4, 1);
```

Tuples in Rust are weirdly enough not accessed with bracket notation, but with
dot notation (why make it different?!).  tuple.i <-- I know, weird right?!

## Control Flow

Generally most things here are pretty sensible.  Just going to point out some
things that differentiate control flow in Rust from other common higher level languages like
Javascript, Ruby, Python etc.

#### If Expressions

```Rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```

In Rust, if's are not statements but **expressions**!  Like ternary's in other
languages - they return a value and therefore can be assigned to variables.
Because they are expressions, each **arm** of the if - else if - else
conditional must return the same type. Because of that, the following would not
be legal rust code:

```Rust
fn main() {
    let condition = true;

    let number = if condition {
        5
    } else {
        "six"
    };

    println!("The value of number is: {}", number);
}
```

^^ the above is **illegal** rust code and will not pass the compiler.  Remember:
**all arms must be of the same type**.

#### Loops

We can create loops in rust using loop, while, or for.

##### Loop

```Rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
```

With the **loop** construct, it's similar to a while (true) loop in other
languages.  In other words, it will continue forever until the code encounters a
**break**.  A loop is an expression and therefore can return a value to be
stored in a variable, as seenn above.

##### While

Works as expected

##### For

For loops behave almost identically to ruby syntatically and in function - which
is functionally very similar to python.  For loops always operate over a
**range**.  Two ways to do this.  

##### #1:
```Rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

In the above code we can see that we use the for...in syntax to iterate over an
array, by calling an iterator ( using .iter() ) on the array.  This ensures that
we loop through the array once and cover each item.  

#### #2:
```Rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

In the above we define a range using the (start..end) syntax that is also
commonly seen in ruby, and simply iterate over that range using for...in.
