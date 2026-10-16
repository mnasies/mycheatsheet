+++
title = "Testing & benchmarking"
lang = "go"
part = 19
tags = ["testing", "benchmarks", "table-driven tests", "t.Run"]
version = "Go 1.22+"
+++

## Table-driven unit testing with subtests
Table-driven tests use slices of test cases executed via `t.Run` for isolated failure reporting.

```go
package math_test

import "testing"

func Add(a, b int) int { return a + b }

func TestAdd(t *testing.T) {
    tests := []struct {
        name     string
        a, b     int
        expected int
    }{
        {"positive", 2, 3, 5},
        {"negative", -1, -1, -2},
        {"zero", 0, 0, 0},
    }

    for _, tc := range tests {
        t.Run(tc.name, func(t *testing.T) {
            if got := Add(tc.a, tc.b); got != tc.expected {
                t.Errorf("Add(%d, %d) = %d; want %d", tc.a, tc.b, got, tc.expected)
            }
        })
    }
}
```

## Benchmark functions
Benchmark functions accept `*testing.B` and iterate `b.N` times to measure runtime performance.

```go
package math_test

import (
    "strings"
    "testing"
)

func BenchmarkStringConcat(b *testing.B) {
    for i := 0; i < b.N; i++ {
        var sb strings.Builder
        sb.WriteString("hello")
        sb.WriteString("world")
        _ = sb.String()
    }
}
```
