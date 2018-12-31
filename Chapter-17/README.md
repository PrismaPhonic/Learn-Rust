# Chapter 17

# Table of Contents
1. [Object Oriented Rust](#object-oriented-rust)
    1. [Encapsulation](#encapsulation)
    2. [Inheritance](#inheritance)
2. [Trait Objects](#trait-objects)
    1. [Performance Hits](#performance-hits)
    2. [Polymorphism](#polymorphism)
    3. [Trait Objects](#trait-objects)

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

# Trait Objects

While the rest of the Rust Language Book has been written exceptionally well, I
found that the language in this chapter was unbelievably difficult to
understand.  If it's possible to petition for a re-write I think this chapter
needs it. After reading around a few more resources online I'm going to do my
best to try to explain this section.

FYI, I think [this explanation](http://gradebot.org/doc/ipur/traitobject.html) of trait objects is far clearer.

Let's get something out of the way. The Rust Language Book claims that _trait
objects_ are called objects because they behave much more like objects in OOP
languages than anything else in Rust. I call bullshit on this as someone who's
written in a lot of object oriented langauages - they really should have **not**
called it _trait objects_ which is an unbelievably confusing name.  Let's get
into it.

Remember when we learned about generics? The Rust compiler will take those
generics, look at our code during compilation time and re-write it so we have a
bunch of versions of our methods for all the given types we will be using. It
turns them from a generic into a bunch of methods all with static types. This
allows us to use generics without incuring a runtime overhead.

But what if we want to define a Struct whose field is a vector of unknown types.
We don't care about what the types are, we just want them all to implement a
specific trait:

```Rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Draw>,
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
	// code that will draw a button
    }
}

impl Screen {
    pub fn run(&self) {
	for component in self.components.iter() {
	    component.draw();
	}
    }
}
```

Will this work?  We are trying to tell rust that we want to make a Vector of
types that implement the `Draw` trait. The issue here is that it's impossible to
know the **size** of any potential type that might choose to implement the draw
trait! Remember what we used when we don't know the size something will be at
compile time? That's right! A `Box<T>` which is a pointer. Rust knows at compile
time the size of a pointer:

```Rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code that will draw a button
    }
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

The one new thing here is the `dyn` keyword. This just tells rust that the
components field will be a vector of smart pointers whose type they are pointing
at is a type that implements the `Draw` trait. Again, this is unbelievably
stupid to call this an object. Nothing about this is any more like an
object than anything else we've already seen in Rust. If anything it's more
similar to **trait bounds**. It is cool, but let's use a better name for this
please.  

Back to our example - now anything that implements the `Draw` trait (which
requires implementing a `draw` method) will be able to construct a `Screen` of
their various components that all need to be drawn on the screen.  We don't know
what another user might want  to implement `Draw` on, but we do know that we
only want our `Screen` component field to be composed of drawable things. This
also means that our type system will now enforce this at compile time!

But let's talk about some performance issues.

## Performance Hits

By using trait objects we suffer a performance hit. Unlike when we used generics
Rust cannot know at compile time the exact size of any type that implements
`Trait`, and it can't know at compile time which `draw` method we are going to
call. All it knows is that it has a collection of some pointers - which it will
follow **at runtime** to find the appropriate draw method - this incurs a
runtime cost because we don't get to take advantage of memorphization. This is a
trade off for the flexibility that trait objects offer us. Before we get into
some specific gotchas for trait objects, let's go over how this helps to enable
polymorphism.

## Polymorphism

Ok, to be honest this one was a bit hard for me to get at. The last section
ended by telling us that we'd learn about how trait objects **enable**
polymorphism in rust, but then the word polymorphism doesn't even come up once
in this section of the Rust Language Book (seriously, do a word search on the
page). It seemed to me that there was already plenty of polymorphism you could achieve using traits, generics and trait bounds. I would not say that trait objects strictly enables polymorphism but it does add some extra flexibility. Let's look at the exact definition of polymorphism:

```
Polymorphism is the provision of a single interface to entities of different
types
```

A traditional example of polymorphism is in dynamically typed languages you
might have a `+` operator that given two integers will add them together, but
given two strings will concatenate them together. In Javascript this gets even
more loopy if you try to add an integer and a string - you might be surprised to
see the number coerced into a string and then concatenated with the other
string.  

In the example we've used of a screen that draws components we have exposed a
single interface (the screen) to entities (the components) of different types
(unknown at compilation time, anything at all that implements the method we
wrote).  If we were to write our own javascript style `+` operator we would need
to have it take any type in existence that implemented some sort of an `Add`
trait that had an `add` method on it.  That `add` method might tell us **how**
that operator would act on this specific type.  

The funny thing is we could already do that before we learned about _trait
objects_. We could have simply written an `Add` trait with a method `add` that
took one generic type and a different generic type and then depending on what
unknown types they are... oh wait - that wouldn't work right? How would we
possibly know the way that two completely unknown types would interact? What if
we are working with custom types in Rust.  Perhaps we know that we want to
create a struct that itself can hold anything that implements our `Add` trait
because when we create an instance of our struct all we intend to do with it is
add stuff together.

So I'm starting to finally see how this could be a useful tool for extending the
functionality of polymorphism - but I wish the book had used more concrete
examples of code we could actually run that would be **useful** to help solidify
why trait objects are so important/useful. Now that I've finished ranting let's
look at some gotchas concerning trait objects.

## Gotchas

We can only make trait objects from _object-safe_ traits. What does that mean?
Well, it means that the trait has the following two properties:

1. The return type is **not** `Self`
2. There are **no** generic type parameters

Let's go over each of these. Why can't the return type be `Self`?  Because
`Self` indicates that we know what the type is, but with trait objects we don't
know!  It's totally unknown at compilation time and so a return type of `Self`
means that trait can't be turned into a trait object.  One example is the
`Clone` trait, which must know at compile time what the type of `Self` is so it
can clone it:

```Rust
pub trait Clone {
    fn clone(&self) -> Self;
}
```

Similarly we can't use generic type parameters in a trait that we want to use as
a trait object because generics are turned into concrete types at commpile time,
but we intentionally forget the type of our trait objects and simply follow the
pointers at runtime.

Now that we've finished covering trait objects we'll implement a blog post using
an OOP design pattern.


