+++
title = "Variables, constants, mutability"
lang = "cpp"
part = 2
tags = ["auto", "const", "constexpr", "structured binding"]
version = "C++20"
+++

## Type deduction with auto and immutability with const
`auto` deduces the type from the initializer expression. `const` prevents reassignment.

```cpp
#include <iostream>

int main() {
    const int max_users = 100;
    auto score = 95.5; // deduced as double
    auto flag = true;   // deduced as bool

    std::cout << "Max: " << max_users << ", Score: " << score << ", Flag: " << flag << "\n";
    return 0;
}
```

## Compile-time evaluation with constexpr
`constexpr` marks variables and functions whose values can be evaluated strictly at compile time.

```cpp
#include <iostream>

constexpr int factorial(int n) {
    return (n <= 1) ? 1 : (n * factorial(n - 1));
}

int main() {
    constexpr int fact5 = factorial(5); // Evaluated during compilation
    static_assert(fact5 == 120, "Compile-time assertion check");
    std::cout << "5! = " << fact5 << "\n";
    return 0;
}
```

## Structured bindings (C++17)
Structured bindings unpack structs, pairs, and tuples directly into individual variables.

```cpp
#include <iostream>
#include <tuple>

struct Point { double x; double y; };

int main() {
    Point pt{3.5, 7.2};
    auto [x, y] = pt;
    std::cout << "Point: (" << x << ", " << y << ")\n";
    return 0;
}
```
