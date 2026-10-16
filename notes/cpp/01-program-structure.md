+++
title = "Program structure & entry point"
lang = "cpp"
part = 1
tags = ["main", "headers", "namespaces", "iostream"]
version = "C++20"
+++

## Minimal executable program
Standard C++ programs start execution in the `main` function, returning an integer exit status.

```cpp
#include <iostream>

int main() {
    std::cout << "Hello, C++20!" << std::endl;
    return 0;
}
```

## Command line arguments and namespaces
Arguments are received via `argc` (count) and `argv` (argument vectors). Custom namespaces avoid symbol collisions.

```cpp
#include <iostream>

namespace Cheatsheet::Core {
    void print_status(int count) {
        std::cout << "Arguments received: " << count << "\n";
    }
}

int main(int argc, char* argv[]) {
    Cheatsheet::Core::print_status(argc);
    for (int i = 0; i < argc; ++i) {
        std::cout << "  argv[" << i << "] = " << argv[i] << "\n";
    }
    return 0;
}
```
