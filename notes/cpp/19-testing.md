+++
title = "Testing & benchmarking"
lang = "cpp"
part = 19
tags = ["testing", "assert", "catch2", "googletest"]
version = "C++20"
+++

## Assertion checking with cassert
`assert` verifies invariants during development, compiling to no-ops when `NDEBUG` is defined.

```cpp
#include <iostream>
#include <cassert>

int add(int a, int b) { return a + b; }

int main() {
    assert(add(2, 2) == 4);
    assert(add(-1, 1) == 0);
    std::cout << "All assertions passed!\n";
    return 0;
}
```

## Modern Catch2 test structure
Catch2 provides BDD-style unit testing with natural expression capture.

```cpp
#define CATCH_CONFIG_MAIN
#include <catch2/catch.hpp>

int factorial(int n) { return (n <= 1) ? 1 : n * factorial(n - 1); }

TEST_CASE("Factorial computation", "[math]") {
    REQUIRE(factorial(1) == 1);
    REQUIRE(factorial(5) == 120);
}
```
