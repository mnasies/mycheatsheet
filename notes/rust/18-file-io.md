+++
title = "File system & I/O"
lang = "rust"
part = 18
tags = ["files", "io", "fs", "buffered io"]
version = "Rust 2021"
+++

## Reading and writing files with std::fs
`std::fs` provides high-level helpers for reading strings or byte vectors from disk in a single call.

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    let file_path = "sample.txt";
    fs::write(file_path, "Hello, Rust File I/O!")?;

    let content = fs::read_to_string(file_path)?;
    println!("File Content: {}", content);

    fs::remove_file(file_path)?;
    Ok(())
}
```

## Buffered line-by-line reading
`BufReader` alongside `BufRead::lines` streams lines from large files without exhausting available memory.

```rust
use std::io::{BufRead, BufReader, Cursor};

fn main() {
    let mock_file = Cursor::new("alpha\nbeta\ngamma");
    let reader = BufReader::new(mock_file);

    for (line_num, line) in reader.lines().enumerate() {
        println!("{}: {}", line_num + 1, line.unwrap());
    }
}
```
