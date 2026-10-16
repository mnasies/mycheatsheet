+++
title = "Structs, classes, and objects"
lang = "go"
part = 9
tags = ["structs", "methods", "embedding", "composition"]
version = "Go 1.22+"
+++

## Struct declaration and pointer receivers
Methods are functions declared with a receiver argument. Pointer receivers allow mutating the underlying struct.

```go
package main

import "fmt"

type Point struct {
    X, Y float64
}

func (p Point) DistanceFromOrigin() float64 {
    return p.X*p.X + p.Y*p.Y
}

func (p *Point) Scale(factor float64) {
    p.X *= factor
    p.Y *= factor
}

func main() {
    pt := Point{X: 3, Y: 4}
    pt.Scale(2.0)
    fmt.Printf("Scaled Point: (%f, %f)\n", pt.X, pt.Y)
}
```

## Composition and struct embedding
Go uses composition rather than inheritance. Embedding a struct forwards its fields and methods.

```go
package main

import "fmt"

type Engine struct {
    Horsepower int
}

func (e Engine) Rev() {
    fmt.Printf("Vroom! %d HP\n", e.Horsepower)
}

type Car struct {
    Engine // Embedded struct
    Model  string
}

func main() {
    c := Car{Engine: Engine{Horsepower: 350}, Model: "Sedan"}
    c.Rev() // Directly calls Engine.Rev()
}
```
