+++
title = "Memory management, ownership, lifetimes"
lang = "cpp"
part = 13
tags = ["smart pointers", "unique_ptr", "shared_ptr", "memory"]
version = "C++20"
+++

## Exclusive ownership with std::unique_ptr
`std::unique_ptr` manages sole ownership of a heap resource, freeing it when the pointer goes out of scope.

```cpp
#include <iostream>
#include <memory>

struct Resource {
    void ping() { std::cout << "Resource online\n"; }
    ~Resource() { std::cout << "Resource deleted\n"; }
};

int main() {
    std::unique_ptr<Resource> res = std::make_unique<Resource>();
    res->ping();

    // Moving ownership
    std::unique_ptr<Resource> res2 = std::move(res);
    std::cout << "Transferred ownership\n";
    return 0;
}
```

## Shared ownership with std::shared_ptr and weak_ptr
`std::shared_ptr` provides reference-counted ownership. `std::weak_ptr` observes without preventing deletion.

```cpp
#include <iostream>
#include <memory>

int main() {
    auto p1 = std::make_shared<int>(42);
    auto p2 = p1; // Count = 2

    std::cout << "Value: " << *p1 << ", Use count: " << p1.use_count() << "\n";
    return 0;
}
```
