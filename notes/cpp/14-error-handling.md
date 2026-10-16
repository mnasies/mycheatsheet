+++
title = "Error handling"
lang = "cpp"
part = 14
tags = ["exceptions", "try-catch", "noexcept", "std::exception"]
version = "C++20"
+++

## Exception handling with standard exceptions
Exceptions throw derived `std::exception` objects caught by reference.

```cpp
#include <iostream>
#include <stdexcept>

double divide(double a, double b) {
    if (b == 0.0) {
        throw std::invalid_argument("Division by zero");
    }
    return a / b;
}

int main() {
    try {
        std::cout << divide(10.0, 0.0) << "\n";
    } catch (const std::exception& e) {
        std::cerr << "Caught exception: " << e.what() << "\n";
    }
    return 0;
}
```

## The noexcept specifier
`noexcept` guarantees a function will not propagate exceptions, enabling compiler optimizations.

```cpp
#include <iostream>

void safe_function() noexcept {
    std::cout << "Guaranteed not to throw\n";
}

int main() {
    safe_function();
    return 0;
}
```
