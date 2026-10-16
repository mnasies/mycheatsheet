+++
title = "Functions & closures"
lang = "rust"
part = 7
tags = ["functions", "closures", "Fn", "FnMut", "FnOnce"]
version = "Rust 2021"
+++

## Function signatures and explicit returns
Functions require typed arguments and return types. The last expression without a semicolon is returned implicitly.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("Sum: {}", add(5, 7));
}
```

## Closures and environment capture
Closures capture variables from their scope by reference (`&T`), mutable reference (`&mut T`), or by value (`move`).

```rust
fn main() {
    let factor = 3;
    let multiply = |x: i32| x * factor;

    let numbers = vec![1, 2, 3];
    let doubled: Vec<i32> = numbers.into_iter().map(|n| n * 2).collect();

    println!("Result: {}, Doubled: {:?}", multiply(10), doubled);
}
```
