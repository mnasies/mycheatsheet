+++
title = "Generics & templates"
lang = "go"
part = 12
tags = ["generics", "type parameters", "constraints", "comparable"]
version = "Go 1.22+"
+++

## Generic functions and comparable constraint
Generic functions define type parameters within brackets, constrained by interfaces such as `comparable`.

```go
package main

import "fmt"

func Contains[T comparable](slice []T, target T) bool {
    for _, item := range slice {
        if item == target {
            return true
        }
    }
    return false
}

func main() {
    nums := []int{10, 20, 30}
    words := []string{"go", "rust"}

    fmt.Println(Contains(nums, 20))
    fmt.Println(Contains(words, "cpp"))
}
```

## Custom union type constraints
Constraints can limit types using union expressions (`|`) or the tilde operator (`~`) for underlying types.

```go
package main

import "fmt"

type Number interface {
    ~int | ~int64 | ~float64
}

func Max[T Number](a, b T) T {
    if a > b {
        return a
    }
    return b
}

func main() {
    fmt.Println(Max(3, 7))
    fmt.Println(Max(3.14, 2.71))
}
```
