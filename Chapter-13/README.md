# Chapter 13

# Table of Contents
1. [Closures](#closures)
    1. [Writing Closures](#writing-closures)
    2. [Function Caching with Closures](#function-caching-with-closures)
        1. [Modifying Cacher to use Generics](#modifying-cacher-to-use-generics)
    1. [Capturing Lexical Environment](#capturing-lexical-environment)
2. [Iterators](#iterators)
    1. [Creating Iterators](#creating-iterators)
    2. [Methods that Consume Iterators](#methods-that-consume-iterators)
    3. [Iterator Adapters](#iterator-adapters)
    4. [Using Closures to Capture](#using-closures-to-capture)
    5. [Creating our Own Iterators](#creating-our-own-iterators)
3. [Improving Minigrep](#improving-minigrep)
    1. [Using Iterator Adapters](#using-iterator-adapters)

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

Iterators are a way to iterate over each item of a sequence.  They are much
more efficient than loops and can be customized.  Iterators either return
`Some(&T)` when we are iterating over the sequence or `None` when we've gotten
to the end and there's nothing left to iterate over.  We can call the next
iteration (iterators are lazy loaded) using `.next()`.  

## Creating Iterators

Calling an interator with `iter()` method will give us an iterator that itself is mutable (the iterator consumes itself each round) but returns a reference (read only) to each item in our sequence.  If we want to get an owned copy of each item we can call `into_iter()`.  If we need mutable copies as we iterate we can call `iter_mut` instead.

## Methods that Consume Iterators

Iterators that call `next` are called _consuming adapters_ because calling them
uses up the iterator (stole this line straight from the book, but it's a good
one).  That means that we can't use the iterator again after using a consuming
adapter.  One such consuming adapter is the `sum` method:

```Rust
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();
let total: i32 = v1_iter.sum();))]

println!('Here's your iterator: {:?}', v1_iter) // this line won't run
```

See that last line I threw in?  It will keep rust from compiling this code
because we are trying to access an iterator after `sum` has already taken
**ownership** of it because `sum` is a _consuming adapter_.

## Iterator Adapters

Iterator adapters are methods that allow us to adapt an iterator to multiple use
cases.  Because of their chaining behavior they do not consume the iterator for
us, so we have to explicitely consume the iterator at the end of the chain.
Take the `map` method for example:

```Rust
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1);
```

This won't work right now because we have only adapted our iterator, not
consumed it.  We need to consume it by using `collect` to collect it into a
vector:

```Rust
let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```

### Using Closures to Capture 

Remember how we said that closures capture variables in their outer environment? We can use that functionality in conjunction with iterators.  Let's look
at the `filter` method and see how we can use it in a useful way to capture an
outer environment: 

```Rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );

    println!("This shouldn't work: {:?}", shoes);
}
```

In this example we are doing a few interesting things.  In the function
`shoes_in_my_size` we are taking **ownership** of Vec<Shoe>.  We then call
`into_iter` which gives us owned copies of the values by which we filter through
to match only shoes in my size (even though `shoe_size` is part of the outer
environment).  We then return an owned vector of shoes filtered by our size.
Take note the last line in this block that I've added of a print statement that
will fail.  Because we have taken ownership of shoes and that ownership drops at
the end of `shoes_in_my_size` we no longer have access to `shoes` after we call
our function. We could have designed it to not do this, but this shows off some
ways to work with Rusts ownership system.

## Creating our Own Iterators

We can create our own iterators by implementing the `Iterator` trait.  When we
implement the iterator trait we are required to define the body of the `next`
method for our custom iterator.  Let's build a simple counter that will always
count from 1 to 5 using an iterator by defining the counting in our `next`
method:

```Rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                             .map(|(a, b)| a * b)
                             .filter(|x| x % 3 == 0)
                             .sum();
            assert_eq!(18, sum);
}
```

What's going on here?  Well, we create a new struct called Counter that just
stores an integer value of our count.  We implement a new function to
instantiate the counter at a count of 0.  Then we implement the `Iterator` trait
for our Counter.  We tell it that the `Item` it returns will be of a `u32` of
some kind.  Then we define our `next` function with the standard signature `fn
next(&mut self) -> Option<Self::Item> {`.  We simply increment the count and
then as long as the count is less than 6 we return `Some` with the count wrapped
up in it.  Otherwise we return a `None`.  

What does this buy us?  Well, as you can see from our first test we now have a
counter that counts from 1 to 5 using an iterator.  Because we have implemented
`Iterator` on our custom type we can now chain other `Iterator` methods on top,
as evidenced by the last test.

## Improving Minigrep

Now that we've learned about closures and iterators let's use them to improve
our minigrep project.  Right now our `Config` struct takes in a string slice and
we use clone on the slices to get the arguments out so we can return an instance
of `Config` that owns it's values:  

```Rust
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
```

This is wastful. We have no need to keep two deep copies in memory of user supplied arguments. Let's improve upon this.  If we look back at what we are doing in `main.rs` we can see that we collect args into a vector and then pass a slice to the function. Why are we collecting it into a vector? Because `env::args()` returns an iterator! Let's just pass the iterator in and work with it directly so we can grab ownership over the arguments rather than cloning.  Here's what our `main.rs` looks like right now:

```Rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
}
```

We simply pass `env::args()` in directly instead:

```Rust
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
}
```

Now we need to change the function signature for `fn new` in our Config `impl` block so that args is of the type we are being given.  But what is that type anyways?  If we look at the docs we'll see that `std::env::args()` returns an iterator of type `std::env::Args` so we'll specify that as the type of our input.  
```Rust
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
```

We also put `mut` before args so we are storing a mutable copy of args so that way we can own the iterator (necessary or each call of next would not be able to consume the previous iteration).  Now we call next immediately to get passed the first argument which is always the filename.  We use a couple match statements to extract owned copies of the query and filename arguments - returning errors if the arguments were not supplied.  The rest of this function hasn't changed.  

### Using Iterator Adapters

Lastly let's clean up our `search` function by using iterator adapters.  Here's what it used to look like:

```Rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

We can clean this up and help to improve our state management.  Currently we create an intermediate `results` vector that is mutable. This could create issues with managing concurrancy if later on we decide to setup parallel processing for our search functionality. If we use iterator adapters we can do the work of returning matching lines without creating an intermediate mutable state that might need to be shared by multiple threads. Let's see what a better solution looks like:

```Rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

Here we use a string literal method called `lines` that returns an iterator over the lines of a string as string slices. We then use the `filter` iterator adapter to act on each of those lines and return the ones that contain our query. We lastly use collect to collect this into a vector because iterator adapters don't consume the iterator for us.  Because we didn't add a semicolon `;` at the end this output is implicitely returned.  Cool huh?

One final word on closures and iterators: They are highly optimized and though they feel like a higher level abstraction they are zero cost abstractions.  That means that they are just as performant as if you wrote the assembly code yourself in the most efficient way possible. So feel free to use them without worry about performance hits!
