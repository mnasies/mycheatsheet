+++
title = "Build tools, package managers, and CLI commands"
lang = "go"
part = 20
tags = ["cli", "go build", "go test", "modules", "flags"]
version = "Go 1.22+"
+++

## Common Go toolchain CLI commands
The standard Go toolchain handles building, testing, linting, and dependency resolution.

```bash
# Initialize a new module
go mod init example.com/myproject

# Download and verify dependencies
go mod tidy

# Build an executable binary
go build -ldflags="-s -w" -o app .

# Run unit tests and show coverage
go test -v -race -cover ./...

# Run performance benchmarks
go test -bench=. -benchmem

# Cross-compile for Windows or Linux
GOOS=windows GOARCH=amd64 go build -o app.exe
GOOS=linux GOARCH=arm64 go build -o app_arm
```

## Command-line flags with the flag package
The standard `flag` package parses command-line flags into typed pointers.

```go
package main

import (
    "flag"
    "fmt"
)

func main() {
    port := flag.Int("port", 8080, "server port number")
    debug := flag.Bool("debug", false, "enable debug logging")
    flag.Parse()

    fmt.Printf("Configured port=%d, debug=%t\n", *port, *debug)
}
```
