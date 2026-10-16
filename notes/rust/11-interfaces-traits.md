+++
title = "Interfaces, traits, polymorphism"
lang = "rust"
part = 11
tags = ["traits", "polymorphism", "dyn", "generics", "derive"]
version = "Rust 2021"
+++

## Trait definition and implementation
Traits define shared behavior analogous to interfaces, implemented explicitly per type.

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

fn main() {
    let post = Article {
        title: "Systems Rust".into(),
        author: "Dev".into(),
    };
    println!("{}", post.summarize());
}
```

## Static vs dynamic trait dispatch
`impl Trait` uses monomorphization at compile time, whereas `dyn Trait` uses virtual method table (vtable) dispatch.

```rust
trait Greeter {
    fn greet(&self);
}

struct Robot;
impl Greeter for Robot {
    fn greet(&self) { println!("Beep boop"); }
}

fn greet_static(g: impl Greeter) { g.greet(); }
fn greet_dynamic(g: &dyn Greeter) { g.greet(); }

fn main() {
    let bot = Robot;
    greet_static(bot);
    greet_dynamic(&bot);
}
```
