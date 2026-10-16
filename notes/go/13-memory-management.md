+++
title = "Memory management, ownership, lifetimes"
lang = "go"
part = 13
tags = ["pointers", "memory", "gc", "escape analysis"]
version = "Go 1.22+"
+++

## Pointers, address-of, and dereferencing
Go pointers store the memory address of a value. Go uses automatic garbage collection and disallows pointer arithmetic.

```go
package main

import "fmt"

func swap(a, b *int) {
    *a, *b = *b, *a
}

func main() {
    x, y := 10, 20
    swap(&x, &y)
    fmt.Printf("x=%d, y=%d\n", x, y)
}
```

## Stack allocation vs heap escape analysis
The Go compiler performs escape analysis; variables escaping their frame are moved to the garbage-collected heap.

```go
package main

import "fmt"

type Buffer struct {
    data [1024]byte
}

func NewBuffer() *Buffer {
    // Escapes to heap because pointer is returned
    return &Buffer{}
}

func main() {
    buf := NewBuffer()
    fmt.Printf("Allocated buffer at: %p\n", buf)
}
```
