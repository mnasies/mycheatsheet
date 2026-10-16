+++
title = "Structs, classes, and objects"
lang = "cpp"
part = 9
tags = ["classes", "structs", "raii", "constructors", "destructors"]
version = "C++20"
+++

## Class declaration, encapsulation, and RAII
Classes default to private access. Destructors guarantee deterministic cleanup (RAII).

```cpp
#include <iostream>

class Session {
private:
    int id_;
public:
    explicit Session(int id) : id_(id) {
        std::cout << "Session " << id_ << " started\n";
    }
    ~Session() {
        std::cout << "Session " << id_ << " terminated (RAII cleanup)\n";
    }
    int id() const { return id_; }
};

int main() {
    {
        Session s(42);
        std::cout << "Inside scope: " << s.id() << "\n";
    } // s destructs automatically here
    std::cout << "Scope exited\n";
    return 0;
}
```

## Move semantics and Rule of Five
Move semantics transfer heap resource ownership without deep copying via rvalue references (`&&`).

```cpp
#include <iostream>
#include <utility>

class Buffer {
public:
    int* data;
    explicit Buffer(int val) : data(new int(val)) {}
    ~Buffer() { delete data; }

    // Move constructor
    Buffer(Buffer&& other) noexcept : data(other.data) {
        other.data = nullptr;
    }
};

int main() {
    Buffer b1(100);
    Buffer b2 = std::move(b1); // Ownership moved to b2
    std::cout << "Moved value: " << *b2.data << "\n";
    return 0;
}
```
