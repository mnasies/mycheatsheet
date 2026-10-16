+++
title = "Strings & text manipulation"
lang = "cpp"
part = 4
tags = ["strings", "string_view", "std::string", "formatting"]
version = "C++20"
+++

## std::string dynamic heap allocation
`std::string` manages dynamic character arrays with automatic resizing and null-termination compatibility.

```cpp
#include <iostream>
#include <string>

int main() {
    std::string text = "Modern";
    text += " C++";
    text.append(" Cheatsheet");

    std::cout << "Text: " << text << " (Length: " << text.length() << ")\n";
    return 0;
}
```

## Non-owning string slices with std::string_view (C++17)
`std::string_view` provides a zero-copy, read-only view of contiguous character sequences.

```cpp
#include <iostream>
#include <string_view>

void print_view(std::string_view sv) {
    std::cout << "View: " << sv << " (Size: " << sv.size() << ")\n";
}

int main() {
    const char* c_str = "Static literal";
    print_view(c_str);
    print_view("Slice substring");
    return 0;
}
```
