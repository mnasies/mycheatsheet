+++
title = "Collections (arrays, slices, maps/dictionaries, sets)"
lang = "rust"
part = 8
tags = ["vec", "hashmap", "collections", "iterators"]
version = "Rust 2021"
+++

## Dynamic vectors (Vec)
Vectors are growable heap buffers managed with RAII semantics.

```rust
fn main() {
    let mut items = Vec::new();
    items.push("rust");
    items.push("go");
    items.push("cpp");

    for (i, lang) in items.iter().enumerate() {
        println!("{i}: {lang}");
    }
}
```

## Hash maps and entry API
`HashMap` stores key-value pairs with hashing. The `entry` API avoids double lookups on insert-or-modify.

```rust
use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    let words = ["apple", "banana", "apple"];

    for word in words {
        *word_counts.entry(word).or_insert(0) += 1;
    }

    println!("Counts: {:?}", word_counts);
}
```
