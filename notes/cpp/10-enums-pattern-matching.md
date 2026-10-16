+++
title = "Enums & pattern matching"
lang = "cpp"
part = 10
tags = ["enum class", "variant", "visit", "enums"]
version = "C++20"
+++

## Scoped enums (enum class)
`enum class` creates strongly-typed, non-polluting scoped enumerations.

```cpp
#include <iostream>

enum class Status : uint8_t {
    Pending,
    Active,
    Failed,
};

int main() {
    Status s = Status::Active;
    if (s == Status::Active) {
        std::cout << "System is active\n";
    }
    std::cout << "Underlying value: " << static_cast<int>(s) << "\n";
    return 0;
}
```

## Type-safe unions with std::variant and std::visit (C++17)
`std::variant` holds one of several alternatives, inspected via `std::visit`.

```cpp
#include <iostream>
#include <variant>

int main() {
    std::variant<int, double, std::string> v = "C++ Variant";

    std::visit([](const auto& val) {
        std::cout << "Stored variant value: " << val << "\n";
    }, v);
    return 0;
}
```
