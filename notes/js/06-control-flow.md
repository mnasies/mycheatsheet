+++
title = "Control flow (if, switch/match, loops)"
lang = "js"
part = 6
tags = ["if", "switch", "loops", "for of", "while"]
version = "ES2022+"
+++

## Conditional statements and ternary operator
`if`/`else` structures alongside ternary operators for compact expression evaluation.

```javascript
const score = 85;
const status = score >= 90 ? "Excellent" : score >= 70 ? "Passing" : "Review";

if (score >= 80) {
    console.log(`Qualified: ${status}`);
}
```

## Iteration with for...of and for...in
`for...of` iterates over iterable values (arrays, strings, sets); `for...in` iterates over enumerable property keys.

```javascript
const frameworks = ["React", "Vue", "Svelte"];

for (const framework of frameworks) {
    console.log(`Framework: ${framework}`);
}

const config = { host: "127.0.0.1", port: 7878 };
for (const key in config) {
    console.log(`${key} => ${config[key]}`);
}
```
