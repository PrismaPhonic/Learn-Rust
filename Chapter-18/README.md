# Chapter 18

# Table of Contents
1. [Patterns and Matching](#patterns-and-matching)
2. [Patterns in Use](#patterns-in-use)
    1. [Match Arms](#match-arms)
    2. [if let Expressions](#if-let-expressions)

# Patterns and Matching

Patterns are a type of syntax in Rust that can be used to match against the
structure of types. We've used patterns with `match` expressions. Patterns can
consist of the following:

1. Literals
2. Destructured arrays, enums, structs or tuples
3. Variables
4. Wildcards
5. Placeholders

Let's identify all the places patterns can be used in. 

# Patterns in Use

## Match Arms

We can use patterns in match arms (we've already been doing this).  Formally
this is the syntax - we match a pattern to an expression:

```Rust
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

One thing to remember about `match` expressions is that they **must** be
exhaustive.  A way around this is to use a **catchall pattern** by using the `_`
pattern.

## if let Expressions

We can use an `if let` expression as a short way to write a match that only
matches on case. This let's us get around having to use a catchall pattern when
we only care about matching one arm.  
