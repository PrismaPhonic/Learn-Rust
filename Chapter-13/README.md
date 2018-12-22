# Chapter 13

# Table of Contents
1. [Closures](#closures)
    1. [Writing Closures](#writing-closures)
    2. [Function Caching with Closures](#function-caching-with-closures)
        1. [Modifying Cacher to use Generics](#modifying-cacher-to-use-generics)
    1. [Capturing Lexical Environment](#capturing-lexical-environment)
2. [Iterators](#iterators)

# Closures 
In Rust a closure is in anonymous function, similar to a lamda function in
python or an arrow function in javascript.  Let's jump right into how to write
them:

## Writing Closures

Let's look at an example of a closure in Rust:

```rust
let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

This is equivalent to the following in Javascript:

```javascript
let expensive_closure = (num) => {
  console.log("calculating slowly...");
  // sleep for 2 seconds - actually pretty
  // complicated in js with async nature
  // so ignoring code for this demo
  return num;
}
```

The difference is just syntactical.  The Rust version follows Ruby syntax more
closely by putting our arguments in between `|` bars.  Just like with js arrow
functions we can assign the function to a variable to be called on that variable
later.  

Let's look at an example of a good use case for closures in Rust by building a
function caching system.

### Function Caching with Closures

Let's jump right in.  We want to design a caching system that will cache the
results of our expensive function and serve those results if needed.  To make
this dynamic we also need a way to check if that function has been run for the
specific argument passed, and if not run it and cache the results.  Let's
approach this from a **data first** perspective.  

What data structure would be good to hold our cache?  If you were thinking of a
HashMap, you'd be right!  We can store the function argument as the key, and the
result of the expensive calculation as the value.  Then if we get a call to our
cacher with an argument we've seen before, we hand back the result, otherwise we
run our expensive anonymous function, cache the result and then return it.
Let's look at some code:

```Rust
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    // had to dereference with * so hashmap get returns
    // a copy of the int rather than a reference
    fn value(&mut self, arg: u32) -> u32 {
        let result = if self.value.contains_key(&arg) {
            *self.value.get(&arg).unwrap()
        } else {
            let v: u32 = (self.calculation)(arg);
            self.value.insert(arg, v);
            v
        };
        result
    }
}
```

In this code we've built a struct called `Cacher` that takes a generic type.  We
then use **trait bounds** to limit that generic type to be something that
implements `fn` (which our closure does) and specifically takes in a u32 and
outputs a u32 (like our anonymous function does).  If we don't do this then the
type system won't know how to determine the type of the closure, as every single
closure generates a unique type.

Our struct takes the calculation (the anonymous function itself) and a value of
a HashMap.  We write some methods for our Cacher.  We can instantiate it with
`Cacher::new(closure)` which will take one argument of our closure function.  It
will also generate a new hashmap for us.  Later when call the value method on
the cacher instance it will take the argument (this is the one argument the
closure itself takes) and sees if a value matches that key in our hashmap - if
not it will run our expensive function, cache the result and return it to the
user.  

Let's look at how this Cacher struct gets instantiated and used:

```Rust
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num * 2
    });
    
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
```

We define our expensive closure when we instantiate `Cacher`.  We can then call
this when needed.  You'll notice that even though we call `.value()` twice in
our first if arm that because it caches the result the first time it is near
instantaneous to get that result in the secondd `println!` statement because the
result has already been cached.  Also, in the first if of our else block we can
see that we don't even need to call this function - so there's a use case for
when our expensive closure will never need to run.

Our code is well optimized for this use case but it could be better. Right now
our Cacher only stores u32's and it also only takes closures that have a single
argument. Can we design this to be more versatile?


### Modifying Cacher to use Generics

We are off on our own now (the book doesn't cover this).  Let's modify our cache
function using generics so that it can take a closure whose input is any type
that implements the Copy trait, Eq trait and Hash trait.  The argument will be
the key in our HashMap so it needs the Eq + Hash trait bounds set on the
generic.  We add the Copy trait to keep our code sane so we aren't dealing with
owned data types we would have to explicitely clone:

```Rust
struct Cacher<T, V, Y>
    where T: Fn(V) -> Y,
          V: Eq + Copy + Hash,
          Y: Eq + Copy
{
    calculation: T,
    value: HashMap<V, Y>,
}
```

Let's look at this step by step starting with our struct `Cacher`.  We specify
that we are acting on **three** generic data types.  We already were using `T` for
our closure, and now are adding on `V` and `Y`. `V` will act as the input to our
function, or our function argument - which also happens to be the key to our
HashMap.  The output of our anonymous function, generic type `Y` also happens to
be the value we store in our HashMap so we use that generic for both.  To ensure
that V is a valid generic for our HashMap key we need to constraint it with
trait bounds that make sure we only accept types that implement `Eq + Hash`.  We
will also set `Copy` for both V and Y so we aren't dealing with complicated
ownership logic in our `value()` method.  Next let's look at how we need to
modify our `impl` block:

```Rust
impl<T, V, Y> Cacher<T, V, Y>
    where T: Fn(V) -> Y,
          V: Eq + Copy + Hash,
          Y: Eq + Copy,
{
    fn new(calculation: T) -> Cacher<T, V, Y> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    // had to dereference with * so hashmap get returns
    // a copy of the int rather than a reference
    fn value(&mut self, arg: V) -> Y {
        let arg = arg.clone();
        let result = if self.value.contains_key(&arg) {
            *self.value.get(&arg).unwrap()
        } else {
            let v: Y = (self.calculation)(arg);
            self.value.insert(arg, v);
            v
        };
        result
    }
}
```

We make sure our impl is defined for the same generics witht he same trait
bounds.  We update our `new` method to return a `Cacher` type that includes our
generics signature.  Next we simply update arg to be generic type `V` and output
to be `Y`.  We also changed the explicit type annotation for variable `v` from
`u32` to generic `Y`.  

Now let's add a test to make sure that our cacher works with closures that take
any type (that implements Copy) and can return any other unknown type (that also
implements Copy):

```Rust
#[test]
fn call_with_varying_types() {
    let mut c = Cacher::new(|a: &str| -> usize {a.len()});
    let mut c2 = Cacher::new(|a: char| -> usize {a.len_utf8()});

    let v1 = c.value("yes");
    let v2 = c2.value('A');

    assert_eq!(v1, 3);

    // assert 'A' char is 1 byte in size
    assert_eq!(v2, 1);
}
```

Our new test passes when we run `$ cargo test`!!

### Capturing Lexical Environment

Closures also can capture all variables in the scope they are declared in:

```Rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
```

While this actually works with closures, it does **not** work with a function
definition.  This breaks:

```Rust
fn main() {
    let x = 4;

    fn equal_to_x(z: i32) -> bool { z == x }

    let y = 4;

    assert!(equal_to_x(y));
}
```

How does it work that closures have access to variables declared in the closures
outer scope?  Essentially the closure captures those variables in the closure
body - which adds an overhead.  If you don't need this functionality you can
just declare a plain old function.  Normally closures just take an immutable
borrow of a value from it's outer scope.  IF you need a closure to take a
mutable borrow then use the `move` keyword in front of the parameter list:

```Rust
fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
```

In this example our closure has taken ownership of x making it unusable on the
next line in our `println!` statement.  

Now that we've covered closures well, let's move onto iterators!

# Iterators
