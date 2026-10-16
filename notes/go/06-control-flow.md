+++
title = "Control flow (if, switch/match, loops)"
lang = "go"
part = 6
tags = ["if", "switch", "for", "loops", "control flow"]
version = "Go 1.22+"
+++

## If statements with initialization
Go allows executing a short statement before the conditional expression, scoping variables to the `if`/`else` block.

```go
package main

import "fmt"

func main() {
    scores := map[string]int{"Alice": 95}

    if score, found := scores["Alice"]; found && score >= 90 {
        fmt.Printf("Alice honors score: %d\n", score)
    } else {
        fmt.Println("Not found or below honors")
    }
}
```

## Switch without condition (clean chained if-else)
A switch statement without an expression acts as an idiomatic replacement for long `if-else if-else` chains.

```go
package main

import "fmt"

func main() {
    hour := 14
    switch {
    case hour < 12:
        fmt.Println("Good morning")
    case hour < 18:
        fmt.Println("Good afternoon")
    default:
        fmt.Println("Good evening")
    }
}
```

## For loop variations
In Go, `for` is the only looping construct. It serves as traditional for, while, and infinite loops.

```go
package main

import "fmt"

func main() {
    // Traditional
    for i := 0; i < 3; i++ {
        fmt.Print(i, " ")
    }
    fmt.Println()

    // While-style
    count := 3
    for count > 0 {
        count--
    }

    // Range over slice
    items := []string{"go", "rust", "cpp"}
    for idx, item := range items {
        fmt.Printf("[%d: %s] ", idx, item)
    }
    fmt.Println()
}
```
