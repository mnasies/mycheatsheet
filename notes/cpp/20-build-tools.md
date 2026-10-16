+++
title = "Build tools, package managers, and CLI commands"
lang = "cpp"
part = 20
tags = ["cmake", "ninja", "vcpkg", "clang-format", "cli"]
version = "C++20"
+++

## CMake configuration (CMakeLists.txt)
CMake generates native build systems across platforms.

```cmake
cmake_minimum_required(VERSION 3.20)
project(MyProject CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(my_app src/main.cpp)
```

## Standard CMake & Compiler CLI workflow
Building and running modern C++ projects via command line tools.

```bash
# Generate build configuration
cmake -B build -DCMAKE_BUILD_TYPE=Release

# Build project with all CPU cores
cmake --build build -j $(nproc)

# Direct compilation with g++ or clang++
g++ -std=c++20 -O3 -Wall -Wextra -Wpedantic main.cpp -o app
clang++ -std=c++20 -O3 main.cpp -o app

# Format code according to project style
clang-format -i src/*.cpp include/*.hpp

# Run sanitizers during testing (AddressSanitizer & UndefinedBehaviorSanitizer)
g++ -std=c++20 -fsanitize=address,undefined -g main.cpp -o app_sanitized
```
