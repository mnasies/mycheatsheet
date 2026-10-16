+++
title = "Generics & templates"
lang = "rust"
part = 12
tags = ["generics", "trait bounds", "where", "monomorphization"]
version = "Rust 2021"
+++

## Generic functions with trait bounds
Type parameters can be constrained using trait bounds with `+` syntax or clean `where` clauses.

```rust
use std::fmt::Display;

fn print_largest<T: PartialOrd + Display>(a: T, b: T) {
    if a > b {
        println!("Largest: {a}");
    } else {
        println!("Largest: {b}");
    }
}

fn main() {
    print_largest(10, 20);
    print_largest("apple", "banana");
}
```

## Generic structs with where clauses
`where` clauses keep complex trait constraints readable when generic parameters multiply.

```rust
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U>
where
    T: std::fmt::Debug,
    U: std::fmt::Debug,
{
    fn dump(&self) {
        println!("{:?} and {:?}", self.first, self.second);
    }
}

fn main() {
    let p = Pair { first: 42, second: "ok" };
    p.dump();
}
```
