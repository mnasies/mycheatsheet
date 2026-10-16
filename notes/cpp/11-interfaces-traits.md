+++
title = "Interfaces, traits, polymorphism"
lang = "cpp"
part = 11
tags = ["polymorphism", "virtual", "override", "interfaces"]
version = "C++20"
+++

## Abstract interfaces and dynamic polymorphism
Interfaces are modeled as abstract classes with pure virtual functions (`= 0`).

```cpp
#include <iostream>
#include <memory>

class Shape {
public:
    virtual ~Shape() = default;
    virtual double area() const = 0; // Pure virtual function
};

class Circle : public Shape {
    double r_;
public:
    explicit Circle(double r) : r_(r) {}
    double area() const override { return 3.14159 * r_ * r_; }
};

int main() {
    std::unique_ptr<Shape> shape = std::make_unique<Circle>(5.0);
    std::cout << "Circle Area: " << shape->area() << "\n";
    return 0;
}
```

## Final and override specifiers
`override` verifies virtual function matching at compile time; `final` prohibits further overriding or inheritance.

```cpp
#include <iostream>

class Base {
public:
    virtual void run() {}
};

class Derived final : public Base {
public:
    void run() override {} // Compiler verifies signature matches Base::run
};
```
