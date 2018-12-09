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

