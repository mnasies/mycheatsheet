+++
title = "Operators (arithmetic, logical, bitwise)"
lang = "cpp"
part = 5
tags = ["operators", "spaceship", "bitwise", "three-way comparison"]
version = "C++20"
+++

## Arithmetic and bitwise operations
C++ provides standard arithmetic, relational, and bitwise manipulation operators.

```cpp
#include <iostream>

int main() {
    int a = 0b1100;
    int b = 0b1010;

    std::cout << "AND: " << (a & b) << ", OR: " << (a | b)
              << ", XOR: " << (a ^ b) << ", Shift: " << (a << 2) << "\n";
    return 0;
}
```

## Three-way comparison spaceship operator (<=> C++20)
The `<=>` operator computes less-than, equal, or greater-than in a single expression.

```cpp
#include <iostream>
#include <compare>

int main() {
    int x = 10, y = 20;
    auto cmp = (x <=> y);

    if (cmp < 0) {
        std::cout << "x is less than y\n";
    } else if (cmp == 0) {
        std::cout << "x is equal to y\n";
    } else {
        std::cout << "x is greater than y\n";
    }
    return 0;
}
```
