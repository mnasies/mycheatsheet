+++
title = "Variables, constants, mutability"
lang = "rust"
part = 2
tags = ["let", "mut", "const", "shadowing", "static"]
version = "Rust 2021"
+++

## Immutable by default and mut keyword
Bindings are immutable by default in Rust. Use `mut` to allow modification.

```rust
let x = 5;       // immutable
let mut y = 10;  // mutable
y += 5;
```

## Variable shadowing
A new binding with the same name shadows the previous one, allowing type transformation.

```rust
let spaces = "   ";
let spaces = spaces.len(); // shadows 'spaces' as usize
```

## Compile-time constants
Declared with `const`, must have explicit type annotations, and can be evaluated at compile time.

```rust
const MAX_BUFFER_SIZE: usize = 1024 * 64;
```
