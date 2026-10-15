+++
title = "Program structure & entry point"
lang = "rust"
part = 1
tags = ["fn main", "entry point", "exit code", "args"]
version = "Rust 2021"
+++

## Minimal executable program
Every binary crate entry point begins execution at the `fn main()` function.

```rust
fn main() {
    println!("Hello, World!");
}
```

## Main returning a Result
The `main` function can return a `Result<(), E>` to allow using the `?` error operator and clean exit codes.

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let home = std::env::var("HOME")?;
    println!("Home directory: {}", home);
    Ok(())
}
```

## Command line arguments
Arguments are retrieved via `std::env::args()` as an iterator of `String` values.

```rust
use std::env;

fn main() {
    for (i, arg) in env::args().enumerate() {
        println!("Arg {}: {}", i, arg);
    }
}
```
