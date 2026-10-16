+++
title = "File system & I/O"
lang = "cpp"
part = 18
tags = ["filesystem", "fstream", "files", "io"]
version = "C++20"
+++

## Reading and writing files with std::fstream
`std::ifstream` and `std::ofstream` provide stream-based file input and output.

```cpp
#include <iostream>
#include <fstream>
#include <string>

int main() {
    const std::string filename = "temp_cpp.txt";

    // Write file
    {
        std::ofstream out(filename);
        out << "Hello from C++ File I/O!\n";
    }

    // Read file
    {
        std::ifstream in(filename);
        std::string line;
        while (std::getline(in, line)) {
            std::cout << "Read line: " << line << "\n";
        }
    }
    return 0;
}
```

## Filesystem navigation with std::filesystem (C++17)
`std::filesystem` queries file attributes, directories, and paths portably.

```cpp
#include <iostream>
#include <filesystem>

namespace fs = std::filesystem;

int main() {
    fs::path current = fs::current_path();
    std::cout << "Current working directory: " << current.string() << "\n";
    std::cout << "Exists: " << fs::exists(current) << "\n";
    return 0;
}
```
