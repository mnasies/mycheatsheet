+++
title = "Operators (arithmetic, logical, bitwise)"
lang = "rust"
part = 5
tags = ["operators", "bitwise", "question mark", "dereference"]
version = "Rust 2021"
+++

## Arithmetic and bitwise operations
Rust includes standard arithmetic with debug overflow checks, bitwise operations, and boolean logic.

```rust
fn main() {
    let a: u8 = 0b1100;
    let b: u8 = 0b1010;

    println!("AND: {:04b}", a & b);
    println!("OR:  {:04b}", a | b);
    println!("XOR: {:04b}", a ^ b);
    println!("NOT: {:04b}", !a);
    println!("Shift: {:04b}", a << 2);
}
```

## Dereferencing and the question mark operator
The `*` operator dereferences references, while `?` unwinds `Err` or returns early on `None`.

```rust
fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    let num = s.trim().parse::<i32>()?;
    Ok(num * 2)
}

fn main() {
    let mut val = 42;
    let r = &mut val;
    *r += 10;
    println!("Updated: {}", val);
}
```
