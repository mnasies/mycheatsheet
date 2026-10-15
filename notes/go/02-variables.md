+++
title = "Variables, constants, mutability"
lang = "go"
part = 2
tags = ["var", "const", "short declaration", "zero value", "iota"]
version = "Go 1.22+"
+++

## Short variable declaration
The `:=` operator declares and initializes variables with type inference inside function bodies.

```go
name := "Gopher"
count := 42
pi := 3.14159
```

## Explicit var declaration and zero values
Variables declared without an initial value receive their type's zero value (`0`, `""`, `false`, or `nil`).

```go
var count int       // 0
var active bool     // false
var message string  // ""
var ptr *int        // nil
```

## Constants and iota enumeration
Constants are immutable values known at compile time. `iota` simplifies sequential enumeration.

```go
const (
    StatusPending = iota // 0
    StatusActive         // 1
    StatusFailed         // 2
)
```
