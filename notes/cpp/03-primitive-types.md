+++
title = "Primitive types & literals"
lang = "cpp"
part = 3
tags = ["types", "cstdint", "floats", "integers", "literals"]
version = "C++20"
+++

## Standard fixed-width integer types
The `<cstdint>` header provides unambiguous fixed-width integer types across architectures.

```cpp
#include <iostream>
#include <cstdint>

int main() {
    int8_t  s8  = -128;
    uint16_t u16 = 65535;
    int32_t s32 = 2147483647;
    uint64_t u64 = 18446744073709551615ULL;
    double   f64 = 3.14159265;

    std::cout << "Sizes: " << sizeof(s8) << ", " << sizeof(u16) << ", "
              << sizeof(s32) << ", " << sizeof(u64) << ", " << sizeof(f64) << "\n";
    return 0;
}
```

## Numeric literals and digit separators
C++ allows single quotes as digit separators for readability.

```cpp
#include <iostream>

int main() {
    auto hex = 0xDEAD'BEEF;
    auto bin = 0b1010'0101;
    auto million = 1'000'000;

    std::cout << "Hex: " << hex << ", Bin: " << bin << ", Val: " << million << "\n";
    return 0;
}
```
