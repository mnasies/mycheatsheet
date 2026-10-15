+++
title = "Program structure & entry point"
lang = "go"
part = 1
tags = ["package", "main", "import", "entry point", "init"]
version = "Go 1.22+"
+++

## Minimal executable program
Every standalone Go executable requires `package main` and a `main()` function with no parameters or return values.

```go
package main

import "fmt"

func main() {
    fmt.Println("Hello, World!")
}
```

## Package declaration and grouped imports
Related packages are grouped in parenthesized `import` blocks. Standard library packages are listed first by convention.

```go
package main

import (
    "fmt"
    "os"
    "strings"
)

func main() {
    fmt.Println(strings.ToUpper(os.Args[0]))
}
```

## Package initialization with init()
An `init()` function runs automatically before `main()`. A package can declare multiple `init()` functions across files.

```go
package main

import "fmt"

func init() {
    fmt.Println("Initializing runtime...")
}

func main() {
    fmt.Println("Running main...")
}
```
