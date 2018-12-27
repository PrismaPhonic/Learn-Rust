# Chapter 15

# Table of Contents
1. [Box<T>](#box<t>)
    1. [Using Box<T> to Store Data on Heap](#using-box<t>-to-store-data-on-heap)
    2. [Building a Recursive List](#building-a-recursive-list)
2. [Deref Trait](#deref)
    1. [Deref Coercion](#deref-coercion)
3. [Drop Trait](#drop-trait)
    1. [Dropping Values Early](#dropping-values-early)
4. [Rc<T>](#rc<t>)
    1. [Reference Counts](#reference-counts)
5. [RefCell<T>](#refcell<t>)
    1. [Combining Rc<T> and RefCell<T>](#combining-rc<t>-and-refcell<t>)
6. [Reference Cycles](#reference-cycles)
    1. [Preventing Reference Cycles](#preventing-reference-cycles)
7. [Review](#review)

# Smart Pointers
What are smart pointers? Two examples of smart pointers we've already seen are
`String` and `Vec<T>`. These are pointers that take ownership over the data they
point to (this isn't a requirement of all smart pointers). They also contain
extra metadata beyond what a normal pointer would hold, (such as `String`
unsuring all of it's data is always valid UTF-8).  By definitions all smart
pointers must implement `Deref` and `Drop` traits.  When a smart pointer goes
out of scope it will run `Drop` to clean up the data on the heap that the
pointer will point to.

# Box<T>

The simplest smart pointer is a _box_.  Boxes very simply are smart pointers
that allow us to store data on the heap rather than the stack.  They are useful
for solving three problems:

1. When you have a type whose size can’t be known at compile time and you want
to use a value of that type in a context that requires an exact size
2. When you have a large amount of data and you want to transfer ownership but
ensure the data won’t be copied when you do so
3. When you want to own a value and you care only that it’s a type that
implements a particular trait rather than being of a specific type


We'll look at solving problem #1 through a simple example, but first, let's see
a **very** simple example to demonstrate the core of how boxes work in rust.

## Using Box<T> to Store Data on Heap

In it's simplest sense a box is just a smart pointer to data we want to store on
the heap, and that data is cleaned up when our box goes out of scope.  Take this
example:

```Rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

This will print out `b = 5`.  Pretty useless to use a box this way but all
that's happening is we are allocating space to store the integer `5` on the heap
rather than the stack and then printing it's value.

Let's look at how we can solve problem #1 listed above by trying to create a
recursive list in Rust

## Building a Recursive List

Let's say we wanted to build a simple `Cons` list, which is a data type common
in functional programming languages.  We could write an enum like such:

```Rust
enum List {
    Cons(i32, List),
    Nil,
}
```

We could then call it like such:

```Rust
use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

This won't run!  We will get an error that we are trying to declare a recursive
type `List`that has an **infinite** size. That is because our enum type could go
on indefinitely - never terminating at `Nil` and so Rust doesn't know at compile
time how much data to allocate for our `List`.  We can instead use a `Box<T>`
for indirection - that is to say that our recursive list will instead use smart
pointers to point at the next iteration of the list.  The data then will be
stacked side by side with each box pointing to the next list.  Rust knows for
sure the size of a pointer and so it will compile if we re-write our recursive
list to use boxes like such:

```Rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil)
		))
	    ))
	);
}
```

Now any list value will take up the size of an i32 plus the size of one pointer!

When a Box<T> goes out of scope it runs `Drop` and cleans up the data on the
heap that it points to.  It also implements the `Deref` trait which allows it's
values to be treated like references.  Let's get into these two traits common to
all smart pointers next.


## Deref Trait

Let's cover the `Deref` trait in more detail now.  First, let's look at a simple
example of how the dereference `*` operator works:

```Rust
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

In this example we set y to be a pointer to x.  if we tried to compare it
directly to 5 we would get that 5 != &5 (basically). We use the dereference
operator to get the value that we are pointing at and compare the literal value.
Rust knows how to dereference any `&` reference, so why do we need a `Deref`
trait at all?  Well, `Deref` trait has a method called `deref` where we can
instruct Rust on how to **create** an `&` reference to the data in our custom
data type.  Then it can apply the dereference operator to get at the value
itself **without** taking ownership.  

Let's imagine that we tried to create our own box using a tuple struct:

```Rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

If we then try to use our box in place of our basic code above, we'll get an
error. The compiler will tell us that `MyBox` cannot be dereferenced - because
it doesn't implement the `Deref` trait yet!  Lets go ahead and do that:

```Rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
```

A few new things here - what the heck is `type Target = T`? Well, apparently
we'll get into it in Chapter 19, but for now it's a slightly different way of
declaring a generic parameter.  We implement a method called `deref` which
instructs Rust on how to create an `&` reference.  In this example our target is
some generic T which is located at `self.0` (because this is a tuple struct and
that's how you get at indexes in tuples).  Rust now know how to create a
reference to our generic.  Now if we use our box, Rust will know how to properly
dereference it:

```Rust
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

Under the hood Rust will run change the call to `*y` to this: `*(y.deref())`.
In other words, it will run our deref method so it can create an `&` reference
to the data stored in our struct tuple, and then subsequently use the derefence
operator to get at the data itself without taking ownership of the data.

### Deref Coercion

Rust handles coercing types in arguments passed to a function to the types wanted by that function if such coercion can take place through chaining of `deref` methods
(I think this definition is much better than the one in the book). This happens
at compile time and is hard coded for us so we don't pay a performance penalty
at runtime for deref coercion.  To wrap our heads around this let's look at a
simple function called hello which takes as an argument a string slice:

```Rust
fn hello(name: &str) {
    println!("Hello, {}!", name);
}
```

We could then call the function this way (yes, I know this seems strange):

```Rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

I know this looks like it shouldn't work, but it does!  What's happening here?
Well, when rust sees `&m` it runs the `deref` method we wrote for our tuple
struct and turns it into an `&String` which is still not the type that our
`hello` function accepts!  So then what happens?  Well, Rust will then look at
the `Deref` implementation on the `String` type and see that it essentially
helps rust to convert a `String` into a `&str` and so it coerces it by calling
the `String` types `deref` method and voila - our function call actually works!

This is pretty confusing, so play with this some and re-read this section a few
times until it starts to sink in.

Now that we've covered `Deref`, let's look at the other trait all smart pointers
implement: `Drop`.

## Drop Trait

Smart pointers call the `Drop` trait automatically when they go out of scope
which does the work of cleaning up the data the pointer was pointing at on the
heap.  We can write our own implementation for the `Drop` trait, which requires
we write a method called `drop`.  Let's do that now to see when the `drop`
method gets called automatically for us by Rust: 

```Rust


struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
}
```

When we run the program we'll see the following:

```Rust
CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

This is evidence that our `drop` method was called automatically when our
`CustomSmartPointer` went out of scope!  You'll also notice that they get
dropped in reverse order - this is because variables are droped in the reverse
order of their creation in Rust.

### Dropping Values Early

We aren't actually allowed to call the `drop` method directly - because doing so
would result in a **double free** error - as rust will also still try to call
drop automatically when our current scope ends.  To get around this we can drop
out early by implementing `std::mem::drop` function.  This is actually brought
into scope for us automatically, so we can call it like such:

```Rust
fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
```

Running this code will print:

```
CustomSmartPointer created.
Dropping CustomSmartPointer with data `some data`!
CustomSmartPointer dropped before the end of main.
```

We are able to drop the data early and not run into the problem of rust calling
the `drop` method at the end of the scope.  

Now that we've covered `Deref` and `Drop` traits, let's cover other kinds of
smart pointers defined in the standard library.

# Rc<T>

What is Rc<T>?  It's the _referenece counted smart pointer_. Basically it's a
way for us to declare **shared** ownership over a value.  Everytime we point a
new reference at that data, rust increments the count. When the count hits zero
(there exists no valid references to it), Rust will then issue `drop` and clean
up the data in the heap. To get a sense of where this would be helpful, let's
imagine that for our recursive list we have a cons list that we want two other
source nodes to point to.  Let's try to implement this using `Box<T>`:

```Rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Cons(5,
        Box::new(Cons(10,
            Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}
```

This won't compile! The `box` will take ownership of a (when we decalre `b`) and
then c will not be able to acces `a` on the last line.  In this situation we
want to share ownership of `a` between `b` and `c` so let's use `Rc<T>`:

```Rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```

We've changed our `Cons` type to use an `Rc<List>` rather than `Box<List>`.
Note how we are using RC here.  We use `Rc::new` for each chaining when we
declare the cons list and point `a` at it.  Then when we call `b` and `c` we
have both issue an `Rc::clone` of `&a`. This might seem alarming at first since
`clone` is usually pretty expensive, especially for large pieces of data!  We
could have called `a.clone()` instead, but this would signify we are making a
**deep clone** and we aren't!  By convention we call `Rc::clone` because the
implementation of the `Clone` trait on `Rc` is extremely mild - it just
increments Rusts ownership reference counter.  When the counter hits 0, it
cleans up the data on the heap - it's that simple!  We call clone with
`Rc::clone` so we can visually distinguish that we are not making a deep clone
and that our call is very efficient and shallow (shouldn't they have called it
copy then?! or increment?! oh well)

## Reference Counts

Let's make a quick example to see how references increase and decrease in count
for `Rc<T>`'s:

```Rust
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

When we run the program, we'll see the following:

```
count after creating a = 1
count after creating b = 2
count after creating c = 3
count after c goes out of scope = 2
```

To put it simply - everytime we call Rc::clone we increase the reference count,
and anytime an Rc<T> goes out of scope, the count goes down automatically when
`drop` is called.  only on the **last** drop (where count becomes zero) does the
data on the heap then get cleaned up!

One **very** important note about Rc<T> - it only deals with multiple owners of
**immutable** data! If it allowed for multiple simultaneous owners of mutable
data then we could run into issues of data races and other heavoc! We sometimes
need to mutate data though and for that let's talk about `RefCell<T>`.

# RefCell<T> 

RefCell's offer interior mutability - that is, they allow us to mutate a value
that would otherwise be immutable. This sounds like a horrible idea to me - it
also has a large performance cost as the borrow rules must be checked at
_runtime_ rather than at compilation time.  It seems that it's useful for cases
where you know without a doubt that your code will not violate the borrow rules
at runtime and need a way around the borrow checkers conservative checks.  

A few things: RefCell, unlike Rc<T> represents single ownership over the data it
holds. And remember, it still enforces the borrow rules at runtime - even if it
allows you to get a mutable reference of an immutable.

Let's look at one use case for when this could be handy - in testing when we
need to create a _mock object_ to check that our function is doing what it
should.  Here's an example of a library that allows someone to implement a limit
tracker, like those seen in public APIs restricting daily user quotas:

```Rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
             self.messenger.send("Urgent warning: You've used up over 90% of
your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your
quota!");
        }
    }
}
```

Take note before we look at a sample test of our trait `Messenger` - it takes an
**immutable** borrow of self.  Now let's try to create a mock object that can
intercept the message initiated by the `set_value` method of our `LimitTracker`:

```Rust

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: vec![] }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
```

This test fails because our implementation of the `Messenger` trait with the
`send` method tries to take a mutable borrow of self when the trait requires
that `send` only take an immutable borrow of self.  How then can we create a
mock object to test our library?

We can use `RefCell<T>` to get inner mutability:

```Rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```

By using `RefCell` we can create a smart pointer to our vector and even though
in our `send` function we are taking an immutable reference of self we are able
to get at a mutable borrow by using `borrow_mut` method.  

When we use `borrow` we get a `Ref<T>` which is an immutable smart pointer and when we
use `borrow_mut` we get back a `RefMut<T>` which is a mutable smart pointer.  At
runtime rust will keep a count of how many mutable and immutable borrows we have
and still enforce the borrow rules - which may make our function panic.  For
this reason we should seriously consider limiting the use of `RefCell` (this is
just my opinion, not necessarily the opinion of the author of The Rust
Programming Langauge book).

and because it was included in the book, let's go over a pattern that makes me
seriously question the safety of this approach:

### Combining Rc<T> and RefCell<T>

We can combine RefCell and Rc to have multiple owners of mutable data (even if
the types are immutable).  To me this seems to defeat the entire point of Rusts
safety checks but here we go:

```Rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

The `value` is a variable that is an instance of `Rc<RefCell<i32>>`.  Remember
that Rc allows multiple ownership while RefCell enables interior mutability.  By
combining these two we are able to mutate the value even though it's shared by
multiple owners and it will update in all paths:

```
a after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
```

## Reference Cycles

Rust does not actually garauntee against memory leaks, but safe rust does
garauntee that memory leaks in Rust are memory safe. What is a memory leak
anyways? It's basically memory that never gets cleaned up! This can happen when
we create cyclical references in rust using strong counts - if two nodes for
instance point at each other using strong counts then at the end of their scope
they will never hit 0 and the memory will never be cleaned up.  Let's try to
force that to happen to demonstrate this behavior by modifying our cons list:

```Rust
use std::rc::Rc;
use std::cell::RefCell;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
```

In this example our cons list owns it's own `i32` value but we instead create
shared ownership to the List it links to, and wrap that in a `RefCell` so we can
get interior mutability to change what list our current list links to.  We then
write a `tail` method that returns an Option of a tail - essentially if this
list links to another list then we get back that reference wrapped in a `Some`
and otherwise if it points to `Nil` then we get back a `None`.  So what's the
problem here?  Well, potentially none, but let's write some code where we
intentionally create a memory cycle:

```Rust
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
```

What's going on here?  Well, let's take a look at our terminal output **before**
we uncomment the last lines:

```
a initial rc count = 1
a next item = Some(RefCell { value: Nil })
a rc count after b creation = 2
b initial rc count = 1
b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
b rc count after changing a = 2
a rc count after changing a = 2
```

So at the end of the scope our rc strong count for both b and a are 2!  Even if
we get a drop of both references at the end of the scope there will still be a
strong count of 1 for each and the memory will never be cleaned up!  What
happens if we uncomment the last line?  Well, we'll get an endless loop of the
two lists referencing each other until we hit a stack overflow and our program
panics!  How can we prevent this? Let's look at how we can utilize **weak**
counts instead of strong counts so we don't end up with memory cycles.

### Preventing Reference Cycles

As of now we know that using `Rc::clone` increases the `strong_count` of an
`Rc<T>` by one, and the value pointed at by an `Rc<T>` will only get cleaned up
when the strong count reaches zero.  Well, we can still create circular
references if we want and not run into the problem of a memory leak by utilizing
a _weak reference_ to the value inside the Rc<T>.  We can either generate a weak
reference with `Weak::new` (after bringing into scope `std::rc::Weak`) or we
call `Rc::downgrade(&ref)` to get a weak reference to the ref we pass as an
argument.

What's so special about these weak references? Well, the value is still only
cleaned up if the strong count hits 0 - it doesn't matter at all if there are
still weak references to that data! In fact, unlike an `Rc<T>`, a `Weak<T>` does not imply ownership at all. Let's take a look at how this works by trying to build a tree in rust, where nodes have parents (strong count ownership), but children have parents whom they do not own (weak). 

Before we move on, I do have an ommision to make. At this point in the book I started to seriously question the design patterns being used here. I decided instead to get rid of the use of RefCell entirely in the code they suggested to build a tree by which each node has a reference to both it's parent and children.  Here's what I came up with instead:

```Rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<Node>>,
    parent: Weak<Node>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: Weak::new(),
        children: vec![],
    });

    println!("leaf parent = {:?}", leaf.parent.upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: Weak::new(),
        children: vec![Rc::clone(&leaf)],
    });

    *leaf.parent = Rc::downgrade(&branch);
}
```

And that's where I started to finally understand the use case for Refcell.
`leaf` is immutable!  Furthermore, if we want to modify the parent, we would
have to change what the `Weak<Node>` is, but this isn't possible!  Using
`RefCell` we can provide _interior mutability_ to specific fields in our struct
and keep other fields completely immutable.  That's handy!  Let's look at their
suggested design, which does work:

```Rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
```

Now we are able to modify the leafs parent by asking for a mutable borrow from
`RefCell` with `borrow_mut` method.  We assign to that parent a `Weak<T>`
pointer to our branch which will not increase the strong count.

Whats with all these `upgrade` methods?  Well, calling `upgrade` on a `Weak<T>`
instance will return an `Option<Rc<T>>`.  Why?  Because it might be that we are
still holding a weak reference to a value that has already been cleaned up
because the strong count hit 0!  By using upgrade we can confirm if we have
`Some` valid value still being referenced or `None`.  

Lastly let's confirm all of our new found learnings around weak and strong
counts by creating some scopes and throwing in some print statements:

```Rust
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
```

What happens when we run this in the terminal?  We'll get:

```
leaf strong = 1, weak = 0
branch strong = 1, weak = 1
leaf strong = 2, weak = 0
leaf parent = None
leaf strong = 1, weak = 0
```

So what happened exactly?  Well, we create the leaf which has an initial strong
count of 1 (the variable leaf itself is an Rc<T> instance pointing at the Node).
Then we create a new scope and create the branch there and issue
`Rc::clone(&leaf)` which increases the strong count of the leaf by 1.  branch
itself now also has an initial strong count of 1.  We then borrow a mutable copy
of the leafs parent and assign it a `Weak<T>` pointing at `branch` which
increases the branch weak count by 1.  So at the end of this scope before Rust
issues the `drop` method on `branch` we have a strong count of `1` for branch
and a weak count of `1` as well.  This means that at the end of the scope Rust
drops the strong count to 0 and cleans up all that data.

Now we have a remaining `Weak<T>` pointing at a value that was already cleaned
up!  So when we check the leafs parent using `upgrade` we get back `None`.  We
lastly look at the leafs strong count and see that it has dropped back to 1
which means at the end of the function scope Rust can clean up that Node in
memory as well!

This was a very heavy chapter, so let's take a second to summarize the
difference between the smart pointers we've covered:

# Review

Let's quickly summarize our different smart pointers:

1. Box<T> - A type with a known size that points to data allocated on the heap
2. Rc<T> - A type that allows multiple owners of immutable data, and keeps track
   of the number of references to data.  When the strong count hits zero, the
   data on the heap is cleaned up.
3. RefCell<T> - A type that provides interior mutability even if applied to
   immutable types.  It still enforces all the borrowing rules, but enforces
   them at runtime which encures a performance cost - and can result in program
   panic if the borrow rules aren't met at runtime.

When would we use each?

1. Box<T>
    1. When you have a type whose size can't be known at compile time and
       you want to use a value of that type in a context that requires an exact
       size.
    2. When you have a large amount of data and want to transfer ownership but
       ensure the data won't be copied when you do so.
    3. When you want to own a value and you care only that it's a type that
       implements a particular trait rather than being of a specific type.
    4. Allows immutable or mutable borrows checked at **compile time**
2. Rc<T>
    1. When we want to allocate some data on the heap for multiple parts of our
       program to read and we can't determine at compile time which part will 
       finish using the data last
    2. Enables multiple ownership of the same data.  Box<T> and RefCell<T> have
       single owners.
    3. Allows only immutable borrows checked at **compile time**
3. RefCell<T>
    1. Allows _interior mutability_ which might be useful when mocking data in
       testing for complex scenarios, or allowing us to mutate specific fields
       of custom structs when we want most of our struct instance to be
       immutable.
    2. Allows mutable borrows checked **at runtime**. Still follows all borrow
       rules and has a performance cost of checking at runtime as opposed to
       compile time checking.
