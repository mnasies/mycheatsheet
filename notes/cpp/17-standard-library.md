+++
title = "Standard library essentials"
lang = "cpp"
part = 17
tags = ["algorithms", "chrono", "ranges", "stdlib"]
version = "C++20"
+++

## Standard algorithms and C++20 ranges
`<algorithm>` provides utility functions like `std::sort`, modernized in C++20 with `std::ranges`.

```cpp
#include <iostream>
#include <vector>
#include <algorithm>
#include <ranges>

int main() {
    std::vector<int> numbers{5, 2, 8, 1, 9};
    std::ranges::sort(numbers);

    for (int n : numbers) {
        std::cout << n << " ";
    }
    std::cout << "\n";
    return 0;
}
```

## Time measurement with std::chrono
The `<chrono>` header provides type-safe duration and clock representations.

```cpp
#include <iostream>
#include <chrono>
#include <thread>

int main() {
    auto start = std::chrono::steady_clock::now();
    std::this_thread::sleep_for(std::chrono::milliseconds(20));
    auto end = std::chrono::steady_clock::now();

    auto elapsed = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
    std::cout << "Elapsed: " << elapsed.count() << " ms\n";
    return 0;
}
```
