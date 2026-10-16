+++
title = "Functions & closures"
lang = "js"
part = 7
tags = ["functions", "arrow functions", "closures", "rest parameters"]
version = "ES2022+"
+++

## Arrow functions and lexical this
Arrow functions provide concise syntax and inherit the `this` binding from their enclosing lexical context.

```javascript
const add = (a, b) => a + b;
const square = x => x * x;

console.log(`Sum: ${add(10, 5)}, Square: ${square(6)}`);
```

## Closures and encapsulation
Closures retain access to outer variables even after the outer function has completed execution.

```javascript
function createCounter(initial = 0) {
    let count = initial;
    return {
        increment() { return ++count; },
        decrement() { return --count; },
        value() { return count; }
    };
}

const counter = createCounter(10);
console.log(counter.increment(), counter.increment(), counter.value());
```

## Rest parameters and default values
Rest parameters collect variable arguments into a real array.

```javascript
function sumAll(multiplier = 1, ...numbers) {
    return numbers.reduce((total, n) => total + n, 0) * multiplier;
}

console.log(`Sum: ${sumAll(2, 1, 2, 3, 4)}`);
```
