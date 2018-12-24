# Chapter 14

# Table of Contents
1. [Customizing Builds](#customizing-builds)

# Cargo and Crates

We'll be diving in deep on cargo and crates in this chapter. Let's get started

## Customizing Builds

Out of the  box Rust comes with two build profiles that we've already used:
`dev` and `release`. By default `dev` uses optimization level 0 which has the
fastest compile time while `release` uses optimization level 3 (the highest)
which has the slowest compile time but applies all optimizations.  We can
override these settings if we want in Cargo.toml:

```Rust
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

It's pretty easy to change these to whatever our heart desires, but they are set
sensibly already.


