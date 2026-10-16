+++
title = "Collections (arrays, slices, maps/dictionaries, sets)"
lang = "cpp"
part = 8
tags = ["vector", "map", "unordered_map", "span"]
version = "C++20"
+++

## Dynamic array with std::vector
`std::vector` is the default contiguous sequential container in C++.

```cpp
#include <iostream>
#include <vector>

int main() {
    std::vector<std::string> langs{"C++", "Rust", "Go"};
    langs.push_back("JavaScript");

    std::cout << "Size: " << langs.size() << ", Capacity: " << langs.capacity() << "\n";
    std::cout << "First: " << langs.front() << ", Last: " << langs.back() << "\n";
    return 0;
}
```

## Hash maps with std::unordered_map
`std::unordered_map` provides hash table storage with average $O(1)$ lookup complexity.

```cpp
#include <iostream>
#include <unordered_map>

int main() {
    std::unordered_map<std::string, int> port_map{{"http", 80}, {"https", 443}};
    port_map["ssh"] = 22;

    for (const auto& [service, port] : port_map) {
        std::cout << service << " -> " << port << "\n";
    }
    return 0;
}
```
