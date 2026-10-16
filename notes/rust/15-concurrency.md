+++
title = "Concurrency & asynchronous programming"
lang = "rust"
part = 15
tags = ["threads", "channels", "mutex", "tokio", "async"]
version = "Rust 2021"
+++

## Thread spawning and channel messaging
Threads run concurrently, communicating safely over `mpsc` (multi-producer, single-consumer) channels.

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("message from thread").unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}
```

## Mutual exclusion with Mutex and Arc
`Arc<Mutex<T>>` provides thread-safe shared mutable state with automatic lock acquisition and RAII unlocking.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("Final count: {}", *counter.lock().unwrap());
}
```
