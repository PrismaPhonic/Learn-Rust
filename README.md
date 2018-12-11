# Learn-Rust
A repository to showcase projects I'm building to help me learn the Rust programming language

## Project 1: Guessing Game
Build a simple guessing game that makes a user guess a number between 1 and 100.
The program will respond if the user is too low or too high, and exit on a
correct guess.

#### What I Learned:
```
use std::io;
```
This brings the io sub-library of the standard library into scope for use in our program.
This gives us the ability to accept user input among other things.
```
fn main() {}
```  
main function needed in every rust program
```
println!
```
calls a ***macro*** (hint **!**) that prints to the screen
```
let foo = 'bar';
```
declare an unmutable variable (default) and assign it a string 'bar'
```
let mut foo = String::new();
```
declare a mutable variable 'foo' and set it equal to a instance of the String
class (growable UTF-8 text). The :: means that new is an _associated function_ of the String
'type' (pretty sure this means class).  It's called on the type rather than an
instance (in other languages called a _static method_)

```
io::stdin().read_line(&mut guess)
```
from the io library that we brought into scope, call the stdin() method which
generates a new instance of the Stdin class which is a handle that represents
input from the terminal. read_line is a method on the stdin handle that takes a
string as an argument, and uses that string to write the contents of input read
from the terminal. **&** indicates that this argument is a reference, and mut
makes sure that the reference will be mutable.  if we want to simply pass an
immutable reference, we would have passed **&guess**.

```
.expect("Failed to read line");
```
If the read_line method fails, then we will handle that by throwing the string,
"Failed to read line" and then exit the program.  This is how we handle an
Err Result type in Rust.

```
println("You guessed {}", guess)
```
This injects the value stored in variable 'guess' into the {}.  if we had more
than one {} we could inject more arguments in order.

## Functions

This details what I learned from the functions section of the "Rust Programming
Language" book.

```Rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

In the above example we can see that functions are declared with _fn_ and
parameters must have their types specified.  In **another_function** we see that
it takes one argument, the parameter x which is defined a signed (can be
positive or negative numbers) 32-bit int.  The main() function must always be
declared and it is the first function that is run.  main can call other
functions that have been declared or imported.

```Rust
x + 1
```
The above represents an _expression_ which returns a value.  If we put a
semicolon on the end it will turn it into a statement, which will **not** return
a value.  **This distinction is extremely important**.

In Rust, **the last expression in a function is returned implicitly**.  Read
that line again because it's **very** important.

```Rust
fn five() -> i32 {
    5
} 
```

When a function returns a value to whatever called that function, the _type_ is
declared with a -> before the block begins. Note in this example that because
there is no semicolon after the 5, that 5 is an expression and is returned
implicitly.  

```Rust
fn plus_one(x: i32) -> i32 {
    x + 1
}
```

Like the above examples, function parameters must have their types declared.  In
this example plus_one is a function that takes in a 32-bit signed integer and
retruns a 32 bit signed integer.  Because there is no semicolon after 'x + 1' it
is an expression, whose value is returned implicitly when the plus_one function
is called.

#### Styling

Note that in Rust function names use the snake_case convention

## Variables
#### Const vs. Let

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
not be garbage collected like let's.

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

### Compound Type

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

#### If 'expressions' (!statements)

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
