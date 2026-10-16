+++
title = "Enums & pattern matching"
lang = "go"
part = 10
tags = ["enums", "iota", "constants", "type switch"]
version = "Go 1.22+"
+++

## Type-safe enums with iota
Go implements enums via typed constants and the `iota` enumerator.

```go
package main

import "fmt"

type LogLevel int

const (
    LevelDebug LogLevel = iota
    LevelInfo
    LevelWarn
    LevelError
)

func (l LogLevel) String() string {
    return [...]string{"DEBUG", "INFO", "WARN", "ERROR"}[l]
}

func main() {
    lvl := LevelWarn
    fmt.Printf("Log level: %s (%d)\n", lvl, lvl)
}
```

## Type switch pattern
Type switches test the concrete type of an interface value.

```go
package main

import "fmt"

func inspect(val any) {
    switch v := val.(type) {
    case int:
        fmt.Printf("Integer: %d\n", v*2)
    case string:
        fmt.Printf("String: %q (len %d)\n", v, len(v))
    default:
        fmt.Printf("Unknown type: %T\n", v)
    }
}

func main() {
    inspect(42)
    inspect("cheatsheet")
}
```
