+++
title = "Functions & closures"
lang = "go"
part = 7
tags = ["functions", "closures", "defer", "multiple returns", "variadic"]
version = "Go 1.22+"
+++

## Multiple returns and named return values
Go functions can return multiple values, standardizing error propagation alongside data.

```go
package main

import (
    "errors"
    "fmt"
)

func divide(a, b float64) (result float64, err error) {
    if b == 0 {
        return 0, errors.New("division by zero")
    }
    return a / b, nil
}

func main() {
    if res, err := divide(10, 2); err == nil {
        fmt.Println("Result:", res)
    }
}
```

## Variadic arguments and closures
Functions can accept variable numbers of arguments and capture surrounding lexical scope.

```go
package main

import "fmt"

func sum(nums ...int) int {
    total := 0
    for _, n := range nums {
        total += n
    }
    return total
}

func makeCounter(step int) func() int {
    val := 0
    return func() int {
        val += step
        return val
    }
}

func main() {
    fmt.Println("Sum:", sum(1, 2, 3, 4, 5))

    counter := makeCounter(5)
    fmt.Println(counter(), counter())
}
```

## Defer for resource management and cleanup
`defer` schedules execution of a function call until the surrounding function returns, executing in LIFO order.

```go
package main

import "fmt"

func worker() {
    defer fmt.Println("Deferred cleanup 1")
    defer fmt.Println("Deferred cleanup 2")
    fmt.Println("Executing body...")
}

func main() {
    worker()
}
```
