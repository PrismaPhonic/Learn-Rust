# Chapter 2

## Guessing Game
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
