+++
title = "Generics & templates"
lang = "cpp"
part = 12
tags = ["templates", "concepts", "requires", "generics"]
version = "C++20"
+++

## Function and class templates
Templates allow writing type-agnostic code instantiated at compile time.

```cpp
#include <iostream>

template <typename T>
T find_max(T a, T b) {
    return (a > b) ? a : b;
}

int main() {
    std::cout << "Max int: " << find_max(10, 25) << "\n";
    std::cout << "Max double: " << find_max(3.14, 2.71) << "\n";
    return 0;
}
```

## C++20 Concepts and requires clauses
Concepts constrain template type arguments at compile time with readable error messages.

```cpp
#include <iostream>
#include <concepts>

template <typename T>
concept Arithmetic = std::integral<T> || std::floating_point<T>;

template <Arithmetic T>
T multiply(T a, T b) {
    return a * b;
}

int main() {
    std::cout << multiply(6, 7) << "\n";
    std::cout << multiply(2.5, 4.0) << "\n";
    return 0;
}
```
