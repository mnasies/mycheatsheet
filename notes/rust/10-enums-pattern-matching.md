+++
title = "Enums & pattern matching"
lang = "rust"
part = 10
tags = ["enums", "option", "result", "if let"]
version = "Rust 2021"
+++

## Enums with data payloads
Rust enums can attach diverse data payloads to each variant, creating algebraic data types.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn process(msg: Message) {
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("Move to ({x}, {y})"),
        Message::Write(text) => println!("Message: {text}"),
    }
}

fn main() {
    process(Message::Write(String::from("Ready")));
}
```

## Option, Result, and if let sugar
`Option<T>` and `Result<T, E>` eliminate null pointer exceptions. `if let` matches a single pattern concisely.

```rust
fn main() {
    let score: Option<i32> = Some(95);

    if let Some(val) = score {
        println!("Score recorded: {val}");
    }
}
```
