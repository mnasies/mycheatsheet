+++
title = "Concurrency & asynchronous programming"
lang = "cpp"
part = 15
tags = ["concurrency", "threads", "mutex", "jthread", "async"]
version = "C++20"
+++

## Modern threading with std::jthread (C++20)
`std::jthread` automatically requests cancellation and joins upon destruction.

```cpp
#include <iostream>
#include <thread>
#include <chrono>

void task(int id) {
    std::cout << "Thread " << id << " executing\n";
}

int main() {
    std::jthread t1(task, 1);
    std::jthread t2(task, 2);
    // Automatically joined on scope exit
    return 0;
}
```

## Thread synchronization with std::mutex and std::lock_guard
`std::lock_guard` acquires a mutex on construction and releases it deterministically upon exit.

```cpp
#include <iostream>
#include <thread>
#include <mutex>

std::mutex mtx;
int counter = 0;

void increment() {
    for (int i = 0; i < 1000; ++i) {
        std::lock_guard<std::mutex> lock(mtx);
        counter++;
    }
}

int main() {
    std::jthread t1(increment);
    std::jthread t2(increment);
    t1.join();
    t2.join();
    std::cout << "Counter: " << counter << "\n";
    return 0;
}
```
