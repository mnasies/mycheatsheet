+++
title = "Interfaces, traits, polymorphism"
lang = "go"
part = 11
tags = ["interfaces", "polymorphism", "duck typing", "any"]
version = "Go 1.22+"
+++

## Implicit interface implementation
Types implement interfaces implicitly by implementing all required methods with matching signatures.

```go
package main

import "fmt"

type Speaker interface {
    Speak() string
}

type Dog struct{}
func (Dog) Speak() string { return "Woof!" }

type Cat struct{}
func (Cat) Speak() string { return "Meow!" }

func MakeNoise(s Speaker) {
    fmt.Println(s.Speak())
}

func main() {
    MakeNoise(Dog{})
    MakeNoise(Cat{})
}
```

## Empty interface and type assertions
The `any` alias (`interface{}`) holds values of any type, unpacked safely with type assertions.

```go
package main

import "fmt"

func printIfString(val any) {
    if str, ok := val.(string); ok {
        fmt.Printf("Extracted string: %s\n", str)
    } else {
        fmt.Println("Not a string")
    }
}

func main() {
    printIfString("golang")
    printIfString(123)
}
```
