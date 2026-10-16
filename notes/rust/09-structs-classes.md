+++
title = "Structs, classes, and objects"
lang = "rust"
part = 9
tags = ["structs", "impl", "methods", "constructors"]
version = "Rust 2021"
+++

## Struct definitions and impl blocks
Structs encapsulate named fields. Methods and associated functions are defined in separate `impl` blocks.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated constructor function
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // Method taking immutable reference
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle::new(10, 20);
    println!("Area: {}", rect.area());
}
```

## Tuple structs and unit structs
Tuple structs have unnamed fields accessed by index; unit structs carry no data but implement traits.

```rust
struct Meters(f64);
struct Marker;

fn main() {
    let dist = Meters(42.195);
    println!("Distance in meters: {}", dist.0);
}
```
