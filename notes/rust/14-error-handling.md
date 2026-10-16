+++
title = "Error handling"
lang = "rust"
part = 14
tags = ["result", "question mark", "panic", "custom errors"]
version = "Rust 2021"
+++

## The ? operator and error conversion
The `?` operator unpacks `Ok` or converts and returns `Err` using the standard `From` trait.

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    match read_username_from_file("missing.txt") {
        Ok(data) => println!("Read: {}", data),
        Err(e) => println!("Expected error caught: {}", e),
    }
}
```

## Panic and unrecoverable errors
`panic!` initiates stack unwinding for unrecoverable bugs; `catch_unwind` safely intercepts panics when required.

```rust
fn main() {
    let result = std::panic::catch_unwind(|| {
        println!("Running safely...");
    });
    assert!(result.is_ok());
}
```
