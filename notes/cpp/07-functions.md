+++
title = "Functions & closures"
lang = "cpp"
part = 7
tags = ["functions", "lambdas", "closures", "default arguments"]
version = "C++20"
+++

## Lambda expressions and captures
Lambdas define anonymous functions. The capture clause `[ ]` specifies how surrounding variables are accessed.

```cpp
#include <iostream>
#include <vector>
#include <algorithm>

int main() {
    int multiplier = 3;
    auto scale = [multiplier](int x) { return x * multiplier; };

    std::vector<int> data{1, 2, 3, 4};
    std::transform(data.begin(), data.end(), data.begin(), scale);

    for (int val : data) {
        std::cout << val << " ";
    }
    std::cout << "\n";
    return 0;
}
```

## Function overloading and default arguments
Functions can be overloaded by parameter types and arity. Default values permit omitting optional arguments.

```cpp
#include <iostream>

int calculate(int base, int multiplier = 1, int offset = 0) {
    return (base * multiplier) + offset;
}

int main() {
    std::cout << calculate(10) << "\n";        // 10
    std::cout << calculate(10, 2) << "\n";     // 20
    std::cout << calculate(10, 2, 5) << "\n";  // 25
    return 0;
}
```
