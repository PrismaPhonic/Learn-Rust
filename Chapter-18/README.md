# Chapter 18

# Table of Contents
1. [Patterns and Matching](#patterns-and-matching)
2. [Patterns in Use](#patterns-in-use)
    1. [Match Arms](#match-arms)
    2. [if let Expressions](#if-let-expressions)
    3. [while let Pattern Matching](#while-let-pattern-matching)
    4. [for Loops](#for-loops)
    5. [let statements](#let-statements)
    6. [Function Parameters](#function-parameters)
3. [Refutable vs Irrefutable Patterns](#refutable-vs-irrefutable-patterns)

# Patterns and Matching

Patterns are a type of syntax in Rust that can be used to match against the
structure of types. We've used patterns with `match` expressions. Patterns can
consist of the following:

1. Literals
2. Destructured arrays, enums, structs or tuples
3. Variables
4. Wildcards
5. Placeholders

Let's identify all the places patterns can be used in. 

# Patterns in Use

## Match Arms

We can use patterns in match arms (we've already been doing this).  Formally
this is the syntax - we match a pattern to an expression:

```Rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

One thing to remember about `match` expressions is that they **must** be
exhaustive.  A way around this is to use a **catchall pattern** by using the `_`
pattern.

## if let Expressions

We can use an `if let` expression as a short way to write a match that only
matches on case. This let's us get around having to use a catchall pattern when
we only care about matching one arm. What we haven't seen yet is that if let can
have an else arm that deals with all other cases (like a catch all) or we can
pair it with an else if that is _unrelated_.  We can even use an `else if let`
that deals with a completely different `VALUE`:

```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```

## while let pattern matching

This one is super cool - we can use a `while let` conditional loop that runs a
`while` loop for as long as the pattern continues to match:

```rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

In this example we will print 3, 2, then 1.  The while loop tries to `pop` an
element of the stack - which returns an `Option<T>`. If that matches to `Some`
then we print the value inside.  Once `pop` returns `None` instead of `Some`
the loop stops!

## for Loops

for loops in rust take patterns, we just haven't realized it yet.  `for x in y`
the `x` is the pattern (whatever directly follows `for`).  We can (and have)
used tuple destructuring in a for loop to get the index and value out of each
iteration of an `enumerate()`:

```rust
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```

## let Statements

By default we always use pattern matching in a let statement by the form of:

```
let PATTERN = EXPRESSION
```

This happens with regular assignment, but is more obvious when destructuring:

```rust
let (x, y, z) = (1, 2, 3);
```

Here Rust will compare the values (1, 2, 3) to the pattern (x, y, z) and see
that the values match the patterns, so Rust binds 1 to x, 2 to y and 3 to z.
Had we used a non-matching pattern we would have gotten a compiler error, like
this invalid pattern match:

```Rust
let (x, y) = (1, 2, 3);
```

## Function Parameters

Function parameters are also patterns. Here's a very simple example:

```Rust
fn foo(x: i32) {

}
```

The x is the pattern in this example. Just like with let we can destructure a
tuple in a function signature by using a pattern:

```Rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

This will print `Current location: (3, 5)`.  We were able to destructure our
arguments right in the function signature and then use those variables in our
function. Pretty sweet!

Now let's look at why some patterns might fail to match.

# Refutable vs Irrefutable Patterns

Patterns are either refutable or irrefutable. The simplest pattern is
irrefutable (there is only the potential for a successful match).  If we were to
write `let x = 7`, we are giving rust an irrefutable pattern because x will
match anything on the right of the assignment operator, therefore it's an
irrefutable pattern.  If innstead we said `if let Some(x) = a_value` we are
providing a refutable pattern - there is some case when we get a `None` rather
than a `Some`.  The `if` before `let` ensures that we can pass a refutable
pattern in.

The caviet to that is that function parameters, let statements and for loops can
only accept irrefutable patterns.  On the flip side `if let` and `while let`
**only** accept refutable patterns.  If we were to type:

```Rust
let Some(x) = some_option_value;
```

We would not be able to compile - we have tried to pass a refutable pattern to a
statement that only takes irrefutable patterns.  Inversely if we were to pass an
irrefutable pattern to a statement that only takes refutable patterns we would
fail to comile as well:

```Rust
if let x = 7 {
    println!("{}", x);
}
```

To expand on this, match arms must use refutable patterns except for the last
arm which must match all remaining values with an irrefutable pattern.

Now that we've gotten that out of the way, let's get deeper into pattern syntax.


