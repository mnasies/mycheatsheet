+++
title = "Memory management, ownership, lifetimes"
lang = "rust"
part = 13
tags = ["ownership", "borrowing", "lifetimes", "box", "rc", "arc"]
version = "Rust 2021"
+++

## Ownership and borrowing rules
Values have a single owner. You can have any number of immutable references (`&T`) OR exactly one mutable reference (`&mut T`).

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 moved to s2; s1 cannot be accessed anymore

    let mut count = 10;
    {
        let r = &mut count;
        *r += 5;
    } // r drops here, allowing next borrow
    println!("s2: {}, count: {}", s2, count);
}
```

## Explicit lifetime annotations
Lifetime annotations (`'a`) specify relationships between reference parameters and returned references to prevent dangling pointers.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("short");
    let string2 = "much longer text";
    println!("Longest: {}", longest(&string1, string2));
}
```

## Smart pointers: Box and Arc
`Box<T>` allocates on the heap; `Arc<T>` provides thread-safe reference counting.

```rust
use std::sync::Arc;

fn main() {
    let boxed = Box::new(42);
    let shared = Arc::new(vec![1, 2, 3]);
    let shared_clone = Arc::clone(&shared);

    println!("Boxed: {}, Shared count: {}", *boxed, Arc::strong_count(&shared_clone));
}
```
