+++
title = "Modules, packages, imports"
lang = "go"
part = 16
tags = ["modules", "packages", "exports", "go.mod"]
version = "Go 1.22+"
+++

## Package export rules and identifiers
Identifiers starting with a capital letter are exported (public); lowercase identifiers are unexported (package-private).

```go
package mathutil

// Exported function (capitalized)
func Add(a, b int) int {
    return internalAdd(a, b)
}

// Unexported helper (lowercase)
func internalAdd(a, b int) int {
    return a + b
}
```

## Importing packages with aliases and blank import
Import paths can be renamed with aliases or imported for side effects using the blank identifier `_`.

```go
package main

import (
    f "fmt" // Alias import
    _ "image/png" // Side-effect import (registers format)
)

func main() {
    f.Println("Custom alias import")
}
```
