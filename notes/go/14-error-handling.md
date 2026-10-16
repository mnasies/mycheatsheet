+++
title = "Error handling"
lang = "go"
part = 14
tags = ["errors", "wrapping", "errors.Is", "errors.As", "panic", "recover"]
version = "Go 1.22+"
+++

## Custom errors and wrapping with fmt.Errorf
Errors in Go are plain values implementing `error`. The `%w` verb wraps errors for inspection with `errors.Is`.

```go
package main

import (
    "errors"
    "fmt"
)

var ErrNotFound = errors.New("resource not found")

func fetch(id int) error {
    if id <= 0 {
        return fmt.Errorf("query id %d: %w", id, ErrNotFound)
    }
    return nil
}

func main() {
    err := fetch(-1)
    if errors.Is(err, ErrNotFound) {
        fmt.Println("Caught ErrNotFound:", err)
    }
}
```

## Panic and recover
`panic` halts normal execution and unwinds the stack. `recover` stops panic unwinding within deferred functions.

```go
package main

import "fmt"

func safeExecute() {
    defer func() {
        if r := recover(); r != nil {
            fmt.Println("Recovered from panic:", r)
        }
    }()

    panic("critical system failure")
}

func main() {
    safeExecute()
    fmt.Println("Execution continued normally.")
}
```
