# Chapter 17

# Table of Contents
1. [Object Oriented Rust](#object-oriented-rust)
    1. [Encapsulation](#encapsulation)
    2. [Inheritance](#inheritance)

# Object Orented Rust

Based on the definition of OOP, Rust is object oriented:

```
Object-oriented programs are made up of objects. An object packages both data
and the procedures that operate on that data. The procedures are typically
called methods or operations.
```

Rust allows you to provide methods on data via impl blocks on structs and enums.
According to many modern definitions though Rust is not an OOP language because
it lacks classes and traditional inheretence.  These things generally are
**unnecessary** and one of Rust's strengths is that it didn't follow the herd.
In that sense it didn't absorb some of the bad parts of OOP (Yes, it's not
actually a silver bullet).

Let's dig into a couple other ideas common to OOP design and see how it's
possible to achieve them in rust.

## Encapsulation

The idea of encapsulation is that the implementation details of an object are
not accessible to scopes external to the object itself.  In other words, I might
use methods on a class instance to get to some data I need to look at, but I
don't have the ability to directly mutate all the fields in the class instance.

The Rust Language book describes this as providing an interface to interact with
an object through a public API that doesn't change. We can then internally
change the code later if we need to - as long as the public facing API hasn't
changed then to the end user they are still providing some input to get a
predictable output.

I loved this part of the Rust Language book because coming from a
Javascript & Python background (where there's not a sense of pub and private at
all), the implementation of encapsulation seems superficial in those languages.
Ironically even though Rust does not have classes it does a much better job of
helping you achieve true encapsulation if you want to.  Let's look at a simple
example to illustrate how we could use encapsulation:

```Rust
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```

In this example we have a struct `AveragedCollection` which has a list field and
an average field.  The fields are private so the only way to add or remove from
the list is through our public API.  An add or remove will in turn re-run the
private update_average function which updates the statically stored average. In
the future if we wanted to change list to some other data structure, like a
`HashSet` we could do so, and as long as our public api still takes in the same
arguments and returns the same types the end user will be unaware of these
implementation details. This is the core idea behind _encapsulation_

Now that we've looked at encapsulation let's take a look at inheritence.

## Inheritance

Rust does not include traditional class based inheretence - and that's a good thing. I'm going to quote a paragraph from the book because it's so well written:

```
Inheritance has recently fallen out of favor as a programming design solution in many programming languages because it’s often at risk of sharing more code than
necessary. Subclasses shouldn’t always share all characteristics of their parent
class but will do so with inheritance. This can make a program’s design less
flexible. It also introduces the possibility of calling methods on subclasses
that don’t make sense or that cause errors because the methods don’t apply to
the subclass. In addition, some languages will only allow a subclass to inherit
from one class, further restricting the flexibility of a program’s design.
```

Inheritance serves to enable code re-use, which we can already do in Rust using
traits.  Traits are more flexible than classic inheritance and allow us to
re-use specific methods without inheriting state and methods that have no place
in a child class.  Anonther goal of inheritance is to enable polymorphism - but
in that sense it's simply a means to accomplish polymorphism - and it's not the
only way.  

In Rust polymorphism is implimented by using generics to abstract over different
possible types and trait bounds to impose constraits on what those types must
provide.  We can also enable polymorphism in Rust through the use of trait
objects.  Let's look at that now.
