+++
title = "Primitive types & literals"
lang = "rust"
part = 3
tags = ["types", "integers", "floats", "booleans", "tuples", "arrays"]
version = "Rust 2021"
+++

## Numeric types and array primitives
Rust provides signed (`i8`..`i128`, `isize`) and unsigned (`u8`..`u128`, `usize`) fixed-width integer types and floats (`f32`, `f64`).

```rust
fn main() {
    let count: usize = 100;
    let delta: f64 = -0.05;
    let active: bool = true;
    let symbol: char = '🦀'; // 4-byte Unicode scalar value

    // Fixed-size stack array [T; N]
    let matrix: [i32; 4] = [1, 2, 3, 4];
    println!("{} {} {} {} {:?}", count, delta, active, symbol, matrix);
}
```

## Tuples and destructuring
Tuples group fixed quantities of heterogeneous values. Destructuring unpacks elements cleanly into named variables.

```rust
fn get_metrics() -> (&'static str, u32, bool) {
    ("healthy", 200, true)
}

fn main() {
    let (status, code, ok) = get_metrics();
    let point = (10.5, 20.0);

    println!("Status: {status} ({code}), ok={ok}, x={}", point.0);
}
```
