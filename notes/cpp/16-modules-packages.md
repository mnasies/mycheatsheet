+++
title = "Modules, packages, imports"
lang = "cpp"
part = 16
tags = ["modules", "includes", "header guards", "pragma once"]
version = "C++20"
+++

## Header files and header guards
Header guards and `#pragma once` prevent duplicate symbol definitions when included across translation units.

```cpp
#pragma once

namespace Math {
    int add(int a, int b);
}
```

## C++20 Modules syntax
C++20 modules replace textual inclusion with compiled module interfaces.

```cpp
// math.ixx
export module math;

export namespace Math {
    int square(int x) { return x * x; }
}

// main.cpp
import math;
import <iostream>;

int main() {
    std::cout << Math::square(5) << "\n";
    return 0;
}
```
