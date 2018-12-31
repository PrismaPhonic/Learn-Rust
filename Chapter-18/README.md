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
4. [Pattern Syntax](#pattern-syntax)
    1. [Matching Literals](#matching-literals)
    2. [Variable Shadowing with Matches](#variable-shadowing-with-matches)
    3. [Multiple Patterns](#multiple-patterns)
    4. [Matching Ranges](#matching-ranges)
    5. [Destructuring with Patterns](#destructuring-with-patterns)
        1. [Destructuring Structs](#destructuring-structs)
        2. [Destructuring Enums](#destructuring-enums)
        3. [Destructuring Tuples](#destructuring-tuples)
        4. [Nested Destructuring](#nested-destructuring)
        5. [Destructuring References](#destructuring-references)
        6. [Complex Destructuring](#complex-destructuring)
    6. [Ignoring Values in a Pattern](#ignoring-values-in-a-pattern)
        1. [Ignoring Entire Value with _](#ignoring-entire-value-with-_)
        2. [Ignoring Remaining Parts with ..](#ignoring-remaining-parts-with-..)
    7. [Match Guards](#match-guards)
    8. [@ Bindings](#@-bindings)

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

# Pattern Synax

Let's now extensively cover pattern syntax

## Matching Literals

This method of using `match` is the most similar to using `switch` statements
in other languages - it literally matches the value:

```Rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

This code prints `one` (pretty obvious).

## Variable Shadowing with Matches

When we use a named variable in a pattern match (like we did above) we are also
creating a new scope with our braces. Just like anywhere else in rust variables
declared inside a new scope will shadow those with the same name in an outer
scope:

```Rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}
```

With this example the first arm won't run, but the next arm will.  The `y` in
the second arm is a newly created variable we are binding `5` too.  When we use
the print statement it is this `y` that we are referring to because it shadows
the `y` declared in the outer scope.

## Multiple Patterns

We can use the or `|` operator to match multiple patterns:

```Rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

## Matching Ranges

Just like how we can define an **inclusive** range using three dots `...`
between two integers, we can do the same thing in a pattern match:

```rust
let x = 5;

match x {
    1 ... 5 => println!("one through five"),
    _ => println!("something else"),
}
```

Rust either accepts numeric values or chars for ranges.  We can also define a
range of chars in a match:


```Rust
let x = 'c';

match x {
    'a' ... 'j' => println!("early ASCII letter"),
    'k' ... 'z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
```

## Destructuring with Patterns

We can use patterns to destructure structs, enums, and tuples and references.

### Destructuring Structs

We can destructure the fields in a struct into variables like such:

```Rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
```

We can also use patterns to match when a struct variables matches a specific
value and destructure the remaining struct values to use in the match arm
expression:

```rust
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
```

The first arm matches any point that lies on the x axis by matching when y == 0,
but then we destructure what is stored in the `x` field for use in that arms
expression.  The second arm matches values on the y axis and destructures y, and
when it's on neither axis we destructure both.  

In this example because `x` is `0` we will match the second arm and print `On
the y axis at 7`.

### Destructuring Enums

We can destructure an enum variant in a match pattern by using a pattern that
matches the variant type. If the variant type is tuple like then we would match
the inside content to a tuple like pattern.  Same with a struct or string like
pattern.  Let's take a look at an example:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    // tuple like enum variant
    let msg = Message::ChangeColor(0, 160, 255);

    // // struct like enum variant
    // let msg = Message::Move{x: 5, y: 7};
    
    // // string like enum variant
    // let msg = Message::Write("Written Message".to_string());

    // // nothing to destructure with this variant
    // let msg = Message::Quit;

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x,
                y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        }
    }
}
```

Feel free to comment out the various messages to see the output change in the
console.  In this example if we get a `Message::Quit` variant there is nothing
to destructure.  A `Message::Move` variant has a struct like type so we
destructure it's content as if it were a struct.  If we get a `Message::Write`
variant then we destructure it with a pattern that would have matched a string
(a simply variable assignment).  Lastly if we get a `Message::ChangeColor` that
is a tuple like variant so we use a pattern that would have matched a three
value long tuple.

### Nested Destructuring

We can also destructure nested data structures as well.  Let's image that we
have an enum of `Color` that contains two tuple types `Rgb` and `Hsv` and
`Message` still allows a `ChangeColor` of `Color` enum variants.  We can
destructure the nested data like such:

```Rust
enum Color {
   Rgb(i32, i32, i32),
   Hsv(i32, i32, i32)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )     
        },
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h,
                s,
                v
            )
        }
        _ => ()
    }
}
```

The above will match to `Hsv` variant of the `Color` enum and we'll get the
message `Change the color to hue 0, saturation 160, and value 255`. 

### Destructuring References

When we are trying to match a pattern to a reference we have to include the `&`
in our pattern.  This will let us get a variable (from the destructing) that
points at the value rather than getting a reference.  Let's look at the
following example:

```Rust
let points = vec![
    Point { x: 0, y: 0 },
    Point { x: 1, y: 5 },
    Point { x: 10, y: -3 },
];

let sum_of_squares: i32 = points
    .iter()
    .map(|&Point { x, y }| x * x + y * y)
    .sum();
```

What's happening here is because we are using `iter` rather than `into_iter` we
are getting references of each `Point` - in other words all `&Point`.  If we
want to destructure `x` and `y` from the fields of each `&Point` we need to make
sure our pattern matches accordingly - otherwise we would get a type mismatch
error from the compiler.

### Complex Destructuring

We can destructure multiple patterns all at once for multiple assignments:

```Rust
let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
```

We could have written this on multiple lines like such:

```Rust
let feet = 3;
let inches = 3;
let Point {x, y} = Point { x: 3, y: -10};
```

Using complex destructuring allows us to do interesting things like multiple
assignment on a single line and other useful pattern matching of nested data.

## Ignoring Values in a Pattern

We've already ignored values in match arm patterns using a catchall `_`.  We can
also use `..` to ignore the remaining parts of a value. Let's look at each
option in more detail.

### Ignoring Entire Value with _

We've already seen using an underscore `_` as a wildcard to match any remaining
arms and not bind a value. We can also use it in function parameters to not bind
a parameter:

```rust
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}
```

Why would this be useful? This comes into use if we need to implement a trait
which requires a specific type signature, but for our specific implimentation we
don't care about one of the method parameters.

We can also ignore parts of a value using nested underscores `_`.  Let's imagine
a scenario where we simply need to see if two varaibles are `Some` type and if
both are we do something but we don't care about what's in either:

```Rust

let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value");
    }
    _ => {
        setting_value = new_setting_value;
    }
}

println!("setting is {:?}", setting_value);)
}
```

We could also use it to skip over values in a tuple or other data structure:

```Rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth)
    },
}
```

We've also seen that we can tell the compiler to ignore an unused variable by
starting it's name with `_`.  Note that this still binds it to that variable!
There aren't too many use cases for this:

```rust
fn main() {
    let _x = 5;
    let y = 10;
}
```

In the above example we would get a warning that `y` has not been used by the
compiler will skip over warning us about an unused variable `_x`.  Because
putting an underscore in front of a variable name still binds it (unlike simply
using `_` by itself) this will produce an error:

```Rust
let s = Some(String::from("Hello!"));

if let Some(_s) = s {
    println!("found a string");
}

println!("{:?}", s);
```

We have taken ownership of the contents inside `Some` with our `_s` and then
tried to print s later which won't work!

We can fix this by using an underscore `_` by itself because doing so will not
bind the value and therefore we will not take ownership of `s`:

```rust
let s = Some(String::from("Hello!"));

if let Some(_) = s {
    println!("found a string");
}

println!("{:?}", s);
```

## Ignoring Remaining Parts with ..

We can use `..` to ignore the rest of the parts of a pattern. It acts as a
catchall match to the remaining portions (or as we'll see soon it can act to
catch intermediate values).  Let's look at a simple example where we only care
about matching one variable and not binding the others **without** using the
`..` syntax:

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, y: _, z: _ } => println!("x is {}", x),
}
```

That seems kind of annoying that we had to list `y` and `z` with nonbinding `_`
just so the pattern matches huh? Well, we can instead use `..` to catch the
remaining in a non-binding way:

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}
```

Great!  We can also use `..` to pattern match values between a first and last:

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }
}
```

Pretty cool.  There is one restriction though with `..`.  For any pattern match
we are only allowed to use it once.  This is to prevent the kind of ambiguity
that could come up from using it multiple times in a single pattern match. For
instance this won't compile: 

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }
}
```

It's very ambigous.  What in-between variable are we trying to match `second` to
exactly?  So we can either match it to the remainder or between the first and
last only (or the beginning only if we just want to match the last).

## Match Guards

We can use an extra `if` conditional with a _match guard_ which is an extra
layer of logic that a pattern must also match against to be valid for that arm
of the match:

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

In this example we match the first arm at `Some(x)` which binds `4` to `x` and
then we check if `x` is less than 5.  It is so we match the entire arm and print
out `less than five: 4`.  Now that we know about match guards we can also use
them to match against an outer variable:

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}
```

This will print out `Default case, x = Some(5)` because 5 != 10 nor does it
equal 50. We can also use match guards in combination with logic or `|` in our
match:

```rust
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
}
```

The important thing to note here is that `if y` applies to 4, 5, or 6 (not just
6 even though it confusingly looks like it might only apply to 6)

This prints out `no` even though x does match to 4 (because it doesn't pass the
match guard - in all cases `y` is `false`)

## @ Bindings

We can use the _at_ operator (`@`) to create a variable that holds a value in
that variable at the same time we are seeing if it matches a pattern:

```rust
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello { id: id_variable @ 3...7 } => {
        println!("Found an id in range: {}", id_variable)
    },
    Message::Hello { id: 10...12 } => {
        println!("Found an id in another range")
    },
    Message::Hello { id } => {
        println!("Found some other id: {}", id)
    },
}
```

In our first arm we are seeing if id is between the range of 3 and 7 inclusive
and if it does match then we bind it to id_variable using the `@` operator.  In
the second arm we see if we get a match between the range of `10...12` but don't
bind `id` to anything.

## Using Ref Mut for Partial Mut

We can use `ref mut` as a keyword to get mutable variables when destructuring
and can be used to get only partial mutability:

```rust
let (ref mut l, ref mut r, max) = (0, arr.len()-1, 10);
```

In this example we are getting mutable references to 0 for l, the last index for
mutable ref r, and an immutable max variable that holds the integer 10.

That's it for pattern matching!  Onto advanced Rust features next!


