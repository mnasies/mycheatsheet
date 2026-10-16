+++
title = "Testing & benchmarking"
lang = "rust"
part = 19
tags = ["testing", "assert", "integration tests", "should_panic"]
version = "Rust 2021"
+++

## Unit tests with #[test] attribute
Unit tests are written directly in module files within a `tests` sub-module guarded by `#[cfg(test)]`.

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
        assert_ne!(add(1, 1), 3);
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn test_panic() {
        panic!("divide by zero");
    }
}
```

## Custom test failure messages
`assert!` and `assert_eq!` support format strings for clear diagnostic failure reporting.

```rust
fn main() {
    let value = 100;
    assert!(value > 0, "Expected positive value, got {value}");
}
```
