+++
title = "Control flow (if, switch/match, loops)"
lang = "rust"
part = 6
tags = ["if", "match", "loops", "while", "for"]
version = "Rust 2021"
+++

## If expressions
In Rust, `if` is an expression that yields a value, requiring branches to have compatible types.

```rust
fn main() {
    let threshold = 50;
    let grade = if threshold >= 90 {
        "A"
    } else if threshold >= 50 {
        "Pass"
    } else {
        "Fail"
    };

    println!("Grade: {}", grade);
}
```

## Exhaustive pattern matching with match
`match` expressions enforce compile-time exhaustiveness across all possible enum variants or values.

```rust
fn main() {
    let code = 200;
    match code {
        200 => println!("OK"),
        400..=499 => println!("Client Error"),
        500..=599 => println!("Server Error"),
        _ => println!("Other Code"),
    }
}
```

## Loop expressions and break with value
The `loop` keyword creates infinite loops that can yield values directly from `break`.

```rust
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result from loop: {}", result);
}
```
