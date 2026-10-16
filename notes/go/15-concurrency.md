+++
title = "Concurrency & asynchronous programming"
lang = "go"
part = 15
tags = ["goroutines", "channels", "select", "sync", "waitgroup"]
version = "Go 1.22+"
+++

## Goroutines and WaitGroup synchronization
Goroutines are lightweight threads managed by the Go runtime. `sync.WaitGroup` coordinates concurrent task completion.

```go
package main

import (
    "fmt"
    "sync"
)

func worker(id int, wg *sync.WaitGroup) {
    defer wg.Done()
    fmt.Printf("Worker %d finished\n", id)
}

func main() {
    var wg sync.WaitGroup
    for i := 1; i <= 3; i++ {
        wg.Add(1)
        go worker(i, &wg)
    }
    wg.Wait()
    fmt.Println("All workers complete.")
}
```

## Channels and multiplexing with select
Channels provide typed synchronization between goroutines. `select` waits on multiple channel operations.

```go
package main

import (
    "fmt"
    "time"
)

func main() {
    c1 := make(chan string)
    c2 := make(chan string)

    go func() {
        time.Sleep(50 * time.Millisecond)
        c1 <- "one"
    }()
    go func() {
        time.Sleep(100 * time.Millisecond)
        c2 <- "two"
    }()

    for i := 0; i < 2; i++ {
        select {
        case msg1 := <-c1:
            fmt.Println("Received:", msg1)
        case msg2 := <-c2:
            fmt.Println("Received:", msg2)
        }
    }
}
```
