# Chapter 6

Let's get into Enums!

# Table of Contents
1. [Enums](#enums)
    1. [Null vs Option](#null-vs-option)
    2. [Match](#match)
        1. [Patterns that Bind to Values](#patterns-that-bind-to-values)
        2. [Matches MUST be Exhausted](#matches-must-be-exhausted)
            1. [The _ Placeholder](#the-_-placeholder)
            2. [If Let](#if-let)

## Enums

Let's dive right into Enums.  What are Enums?  They are enumerable groupings of
data. It allows us too group multiple structs together (and other data types)
into logical groupings.  Here's an example:

```Rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
}
```

What's happening here?  Well, we are defining an enum that describes two types
of IP addresses: IPv4 and IPv6.  They are different structs (V4, and V6), but
each type is related to a parent type of IpAddr.  We are also saying here that
both V4 and V6 structs can take a String type - which we demonstraate when we
instantiate each struct.  They are namespaced from their parent with double ::

Enums can handle structs that themselves are composed of different types, like
such:

```Rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

Let's look at one more example where we can define variants of a 'message' enum.  

```Rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

In this example we see 4 different types of messages - a quit mesasge, move
message, write message or changecolor message.  By grouping these together it
means that we can write functions that type a type of Message - which accepts
any of the variant Message types.

The **great** thing about enums is that you can apply an implementation (method)
among all the variants in the enum.  Case in point:

```Rust
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

In this example we are creating an implementation block with a 'call' method.
When we create the instance 'm', we are passing the write message type with it's
string body of "hello" along to the call method.  This would work for any of the
other message types as well.

### Null vs Option

Option was implemented in Rust rather than null.  Null apparently has been the
cause of many issues since it's creation - but the general idea is good (to
check if there is some value present or not).  Instead Rust uses the enum type
`Option<T>`

```Rust
enum Option<T> {
    Some(T),
    None,
}
```

This is included in the prelude, so you don't have to bring it into scope.  You
also don't need to explicitely bring it's variants `Some(T)` or `None` into
scope. The `<T>` syntax is a generic type parameter.  It means that some can
hold one parameter of any type.

Apparently for some convoluted reason I can't seem to grasp well, using
Option<T> is somehow better than null because it "limits the pervasiveness of
null" and forces you to use Option<T> for null values.  If something is any type
other than Option<T> than it garauntees that it's not null.  Well, ok?!  This is
very confusing - maybe I'll come back and update this part of the readme when I
have a better grasp over this.  For now it seems that if it's a type None then
we know it's null and if it's Some<T> then it's Not Null and we have to use some
methods to get T out of Some(T)

### Match

Match is very similar to switch but more powerful.  It allows us to operate
through variants in an enum and produce different results depending on the
match.  Let's see an example:

```Rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

So far it's not obvious why this is more powerful than a simple switch
statements in most languages, but presumably we will cover that later on.  So in
this example value_in_cents is a function that takes in as a type Coin, which is
an enum.  It then compares that against a match statement that checks for what
variety of coin it is, the sub-type if you will and returns a value for the
first type to match.

#### Patterns that Bind to Values

Match arms can bind to the parts of the values that match the pattern (this
sentence was taken directly from the book.  It's an extremely confusing sentence
and I don't understand it to be honest so I'm just regurgitating it for now).
Beyond this sentence I have somewhat of a grasp of what's going on.  Lets take a
look at their example:

```Rust
#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

In this example if we call value_in_cents(Coin::Quarter(UsState::Alabama)) we
will see printed in the terminal "State quarter from Alabama!" - in other words,
we were able to bind the match to the specific variant that was passed in
because we specified that Quarter would take a variable of 'state' and that
state becomes bound to Alabama when we pass in UsState:Alabama.

Let's look at another example using Option<T>:

```Rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

In this example we take in an Enum of type Option and give it a type of an i32.
if that matches None type (variant of Option<T>) then we return None right away
and stop comparing (fail fast).  Next we see if it matches Some type (the other
variant of Option<T>) - if it does match (in the case of five), then we get
access to the contents of Some, which in this case if bound to variable 'i'.  We
can then execute code based on it - in this case we are returning a new Option
type with i incremented by 1 and then bound inside of it.

For some reason this is supposed to be awesome - perhaps I will find out why as
I write more rust - for now this seems unbelievably convoluted.

#### Matches MUST be Exhausted

In rust we must exhaust all match options.  that means that we **must** have a
match case for every variant of an enum.  Here's an example of code that will
**not** compile:

```Rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

We are missing an _arm_ for None, which is a valid variant of the Option<T>
enum.  

We can solve this with the _ placeholder:

##### The _ Placeholder

To take care of the problem of matches needing to be exhaustive, we can use the
_ placeholder at the end of a match list to take care of all other matches.
Example:

```Rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

In the above example, any cases that don't match 1, 3, 5, or 7 will hit the _
placeholder arm.

#### If Let

`if let` is a pattern we can use to avoid the `_` placeholder and is useful when
we only want to match a couple things.  Take this match example for instance:

```Rust
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
```

In this example we are only trying to match for one arm.  We could instead write
it like this:

```Rust
if let Some(3) = some_u8_value {
    println!("three");
}
```
