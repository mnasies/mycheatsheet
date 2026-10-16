+++
title = "Control flow (if, switch/match, loops)"
lang = "cpp"
part = 6
tags = ["if", "switch", "loops", "range-for"]
version = "C++20"
+++

## If statements with initializer (C++17)
Variables can be scoped directly to the `if` statement to restrict variable lifetime.

```cpp
#include <iostream>
#include <map>

int main() {
    std::map<std::string, int> ratings{{"Go", 5}, {"Rust", 5}, {"Cpp", 5}};

    if (auto it = ratings.find("Rust"); it != ratings.end()) {
        std::cout << "Found " << it->first << " rating: " << it->second << "\n";
    }
    return 0;
}
```

## Range-based for loops
Range-based for loops iterate over any container providing `begin()` and `end()` iterators.

```cpp
#include <iostream>
#include <vector>

int main() {
    std::vector<int> numbers{10, 20, 30, 40};

    // By const reference to avoid copying
    for (const auto& num : numbers) {
        std::cout << num << " ";
    }
    std::cout << "\n";
    return 0;
}
```
