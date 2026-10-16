+++
title = "Modules, packages, imports"
lang = "rust"
part = 16
tags = ["modules", "mod", "use", "visibility", "pub"]
version = "Rust 2021"
+++

## Module hierarchy and visibility
Items in modules are private by default; `pub` exports them, and `pub(crate)` restricts visibility to the current crate.

```rust
mod network {
    pub fn connect() {
        internal_handshake();
    }

    fn internal_handshake() {
        println!("Connected");
    }
}

use network::connect;

fn main() {
    connect();
}
```

## Re-exporting items with pub use
`pub use` creates convenient public API surfaces while keeping internal file layout modular.

```rust
mod internal {
    pub struct Client;
    impl Client {
        pub fn ping(&self) { println!("pong"); }
    }
}

pub use internal::Client;

fn main() {
    let client = Client;
    client.ping();
}
```
