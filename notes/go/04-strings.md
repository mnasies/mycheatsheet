+++
title = "Strings & text manipulation"
lang = "go"
part = 4
tags = ["strings", "runes", "utf8", "formatting", "builder"]
version = "Go 1.22+"
+++

## String literals and UTF-8 iteration
Strings in Go are immutable byte sequences. Iterating with `range` decodes UTF-8 runes, whereas index access yields raw bytes.

```go
package main

import (
    "fmt"
    "unicode/utf8"
)

func main() {
    greeting := "Hello, 世界"
    fmt.Printf("Byte length: %d, Rune count: %d\n", len(greeting), utf8.RuneCountInString(greeting))

    for idx, r := range greeting {
        fmt.Printf("Index %d: %c (U+%04X)\n", idx, r, r)
    }
}
```

## Efficient string concatenation with strings.Builder
`strings.Builder` minimizes memory allocations when building strings incrementally.

```go
package main

import (
    "fmt"
    "strings"
)

func main() {
    var b strings.Builder
    b.WriteString("Status: ")
    fmt.Fprintf(&b, "%s (%d)", "Active", 200)
    result := b.String()
    fmt.Println(result)
}
```
