+++
title = "Primitive types & literals"
lang = "go"
part = 3
tags = ["types", "integers", "floats", "booleans", "constants"]
version = "Go 1.22+"
+++

## Numeric types and bit-sizes
Go provides explicit architecture-independent integer and floating-point types alongside pointer-sized `int` and `uint`.

```go
package main

import "fmt"

func main() {
    var a int8 = 127
    var b uint16 = 65535
    var c int64 = 9223372036854775807
    var f float64 = 3.141592653589793
    var ok bool = true
    var r rune = '⌘' // Unicode code point (int32)
    var bte byte = 0xFF // alias for uint8

    fmt.Printf("%d %d %d %f %t %c 0x%X\n", a, b, c, f, ok, r, bte)
}
```

## Type conversions and zero values
Go does not permit implicit type conversion between numeric types. Every conversion must be explicit.

```go
package main

import "fmt"

func main() {
    var i int = 42
    var f float64 = float64(i)
    var u uint = uint(f)

    var defaultInt int       // 0
    var defaultFloat float64 // 0.0
    var defaultBool bool     // false
    var defaultStr string    // ""

    fmt.Printf("%d %f %d -> %d %f %t %q\n", i, f, u, defaultInt, defaultFloat, defaultBool, defaultStr)
}
```
