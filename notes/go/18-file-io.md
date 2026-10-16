+++
title = "File system & I/O"
lang = "go"
part = 18
tags = ["io", "os", "bufio", "files", "reader", "writer"]
version = "Go 1.22+"
+++

## Reading and writing entire files
The `os` package provides one-shot functions for reading and writing files directly.

```go
package main

import (
    "fmt"
    "os"
)

func main() {
    filename := "test.txt"
    err := os.WriteFile(filename, []byte("Hello, File I/O!"), 0644)
    if err != nil {
        return
    }
    defer os.Remove(filename)

    data, err := os.ReadFile(filename)
    if err == nil {
        fmt.Println("Read content:", string(data))
    }
}
```

## Buffered line reading with bufio.Scanner
`bufio.Scanner` reads text files line by line without loading the entire file into memory.

```go
package main

import (
    "bufio"
    "fmt"
    "strings"
)

func main() {
    input := "First line\nSecond line\nThird line"
    scanner := bufio.NewScanner(strings.NewReader(input))

    lineNum := 1
    for scanner.Scan() {
        fmt.Printf("%d: %s\n", lineNum, scanner.Text())
        lineNum++
    }
}
```
