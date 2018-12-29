# Dining Philosophers 

# Table of Contents
1. [Dining Philosophers](#dining-philosophers)

# Dining Philosophers

This is a project from the rust language book version 1.6.0 - not included in
the newest Rust Language Book.  I thought it would be a fun way to re-enforce
concepts around concurrency so I'll be including what I learned from working on
this project. If you'd like to make this project yourself you can read about it
[here](https://doc.rust-lang.org/1.6.0/book/dining-philosophers.html)

Here's an explanation of the problem from the book:

```
In ancient times, a wealthy philanthropist endowed a College to accommodate five eminent philosophers. Each philosopher had a room in which they could engage in their professional activity of thinking; there was also a common dining room, furnished with a circular table, surrounded by five chairs, each labelled by the name of the philosopher who was to sit in it. They sat anticlockwise around the table. To the left of each philosopher there was laid a golden fork, and in the center stood a large bowl of spaghetti, which was constantly replenished. A philosopher was expected to spend most of their time thinking; but when they felt hungry, they went to the dining room, sat down in their own chair, picked up their own fork on their left, and plunged it into the spaghetti. But such is the tangled nature of spaghetti that a second fork is required to carry it to the mouth. The philosopher therefore had also to pick up the fork on their right. When they were finished they would put down both their forks, get up from their chair, and continue thinking. Of course, a fork can be used by only one philosopher at a time. If the other philosopher wants it, they just have to wait until the fork is available again.
```

This problem can be broken down into 4 steps:

1. A philosopher picks up the fork on their left
2. A philosopher picks up the fork on their right
3. They eat.
4. They return their forks.

The issue here is one of concurrency.  If all the philosophers sit down at the
table at the same time and each philosophers right fork is another philosophers
left fork, then who gets the fork?  Furthermore we have a circular relationship
where we could get a deadlock.  There will be a lock on each fork and if no
philosopher gets two forks then no philosopher will finish eating and resolve
the locks on their respective forks and our program will hang indefinitely.

Let's look at a solution that should in theory work but causes a **deadlock**

```Rust
use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

struct Table {
    forks: Vec<Mutex<()>>,
}

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left,
            right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep(Duration::from_millis(150));
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{} is done eating.", self.name);
    }
}


fn main() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});
    
    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Emma Goldman", 3, 4),
        Philosopher::new("Michel Foucault", 4, 0),
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = Arc::clone(&table);

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
```

This gives us a deadlock for the reasons stated above.  I won't go through
explaining all this code because there are no new concepts here.  Read my readme
on Chapter 16 if you'd like to find out about `Arc::clone`, `thread::spawn`, and
`Mutex<T>`.

If we instead make `Michael Foucault` left handed we won't get a deadlock:

```Rust
let philosophers = vec![
    Philosopher::new("Judith Butler", 0, 1),
    Philosopher::new("Gilles Deleuze", 1, 2),
    Philosopher::new("Karl Marx", 2, 3),
    Philosopher::new("Emma Goldman", 3, 4),
    Philosopher::new("Michel Foucault", 0, 4),
];
```

One thing I will point out about the design in the book that I found interesting
is that it takes more of a Data Oriented Design approach.  Our table holds all
of our forks which are stored simply in a Vector.  Each philosopher holds an
index of which fork is on their left vs right, and then we have a function `eat`
which allows us to use that data.  This is a very nice piece in how this was
designed and is worth pointing out. I also modified their solution a little bit.
They had called `table.clone()`. Perhaps this was considered good practice when
this book (version 1.6) came out but currently it's considered better to use
`Arc::clone` as it clearly indicates that we aren't actually doing a deep clone
of the data, and instead are simply incrementing the reference count.
