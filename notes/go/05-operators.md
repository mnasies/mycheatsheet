+++
title = "Operators (arithmetic, logical, bitwise)"
lang = "go"
part = 5
tags = ["operators", "bitwise", "precedence", "logical"]
version = "Go 1.22+"
+++

## Arithmetic and bitwise operations
Go supports standard arithmetic along with bitwise operations, including bit-clear (`&^`).

```go
package main

import "fmt"

func main() {
    a, b := 12, 5
    fmt.Printf("Add: %d, Sub: %d, Mul: %d, Div: %d, Mod: %d\n", a+b, a-b, a*b, a/b, a%b)

    // Bitwise: AND, OR, XOR, Bit-clear (AND NOT), shifts
    x := 0b1100
    y := 0b1010
    fmt.Printf("AND: %04b, OR: %04b, XOR: %04b, BitClear: %04b\n", x&y, x|y, x^y, x&^y)
    fmt.Printf("Left shift: %04b, Right shift: %04b\n", x<<1, x>>1)
}
```

## Short-circuit logical evaluation
Logical AND (`&&`) and OR (`||`) short-circuit operand execution from left to right.

```go
package main

import "fmt"

func check(label string, val bool) bool {
    fmt.Println("evaluating:", label)
    return val
}

func main() {
    if check("A", false) && check("B", true) {
        fmt.Println("Both true")
    }
}
```
