+++
title = "Collections (arrays, slices, maps/dictionaries, sets)"
lang = "go"
part = 8
tags = ["arrays", "slices", "maps", "make", "append"]
version = "Go 1.22+"
+++

## Slices: allocation, append, and sub-slicing
Slices are dynamically sized views over underlying arrays, allocated via `make` or slice literals.

```go
package main

import "fmt"

func main() {
    s := make([]int, 0, 5) // len 0, cap 5
    s = append(s, 10, 20, 30)
    fmt.Printf("len=%d cap=%d slice=%v\n", len(s), cap(s), s)

    sub := s[1:3]
    fmt.Println("Subslice:", sub)
}
```

## Maps and key deletion
Maps are hash tables. Access returns two values: the value and a boolean indicating key presence.

```go
package main

import "fmt"

func main() {
    cache := make(map[string]int)
    cache["api"] = 200
    cache["db"] = 500

    val, exists := cache["api"]
    fmt.Printf("api exists? %t, val=%d\n", exists, val)

    delete(cache, "db")
    fmt.Printf("Count after delete: %d\n", len(cache))
}
```
