+++
title = "Standard library essentials"
lang = "go"
part = 17
tags = ["stdlib", "json", "time", "context", "http"]
version = "Go 1.22+"
+++

## JSON encoding and decoding with struct tags
The `encoding/json` package uses struct tags to map struct fields to JSON keys.

```go
package main

import (
    "encoding/json"
    "fmt"
)

type User struct {
    Name  string `json:"name"`
    Email string `json:"email,omitempty"`
    Age   int    `json:"age"`
}

func main() {
    u := User{Name: "Alice", Age: 30}
    bytes, _ := json.Marshal(u)
    fmt.Println(string(bytes))

    var decoded User
    _ = json.Unmarshal(bytes, &decoded)
    fmt.Printf("Decoded: %s (%d)\n", decoded.Name, decoded.Age)
}
```

## Context for timeout and cancellation
`context.Context` propagates deadlines, cancellation signals, and request-scoped values across API boundaries.

```go
package main

import (
    "context"
    "fmt"
    "time"
)

func execute(ctx context.Context) {
    select {
    case <-time.After(100 * time.Millisecond):
        fmt.Println("Completed successfully")
    case <-ctx.Done():
        fmt.Println("Halted:", ctx.Err())
    }
}

func main() {
    ctx, cancel := context.WithTimeout(context.Background(), 20*time.Millisecond)
    defer cancel()
    execute(ctx)
}
```
