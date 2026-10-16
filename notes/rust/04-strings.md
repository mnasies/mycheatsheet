+++
title = "Strings & text manipulation"
lang = "rust"
part = 4
tags = ["strings", "str", "slices", "formatting", "utf8"]
version = "Rust 2021"
+++

## String slices (&str) vs owned String
`&str` is an immutable borrowed view into UTF-8 bytes; `String` is a heap-allocated, growable UTF-8 buffer.

```rust
fn main() {
    let literal: &'static str = "Hello, world!";
    let mut owned: String = String::from(literal);
    owned.push_str(" 🦀");

    // Substring slicing requires valid UTF-8 character boundaries
    let slice: &str = &owned[0..5];
    println!("Slice: {}, Full: {}", slice, owned);
}
```

## String formatting and interpolation
The `format!` macro produces formatted strings without printing directly to standard output.

```rust
fn main() {
    let lang = "Rust";
    let year = 2021;
    let formatted = format!("{lang} edition {year}: {0} is memory-safe", lang);
    println!("{}", formatted);
}
```
