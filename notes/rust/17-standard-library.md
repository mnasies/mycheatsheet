+++
title = "Standard library essentials"
lang = "rust"
part = 17
tags = ["stdlib", "iterators", "option", "result", "env"]
version = "Rust 2021"
+++

## Iterator combinators: map, filter, fold
Iterators in Rust are zero-cost abstractions evaluated lazily until consumed by methods like `collect`.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let sum_of_even_squares: i32 = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum();

    println!("Sum of even squares: {}", sum_of_even_squares);
}
```

## Command line arguments and environment variables
The `std::env` module provides access to process arguments and environment variables.

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let current_dir = env::current_dir().unwrap();

    println!("Binary: {}, Cwd: {}", args[0], current_dir.display());
}
```
