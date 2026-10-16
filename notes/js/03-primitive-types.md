+++
title = "Primitive types & literals"
lang = "js"
part = 3
tags = ["types", "typeof", "primitives", "bigint", "symbol"]
version = "ES2022+"
+++

## JavaScript primitive types
JavaScript has seven primitive types: `number`, `string`, `boolean`, `null`, `undefined`, `symbol`, and `bigint`.

```javascript
const num = 42;
const str = "cheatsheet";
const isReady = true;
const missing = undefined;
const empty = null;
const sym = Symbol("uniqueId");
const large = 9007199254740991n; // BigInt literal

console.log(typeof num, typeof str, typeof isReady, typeof missing, typeof empty, typeof sym, typeof large);
```

## Type coercion and checking
Direct type coercion with constructors and `Number.isNaN` vs loose comparisons.

```javascript
const parsed = Number("123"); // 123
const boolVal = Boolean("");  // false

console.log(Number.isNaN(NaN)); // true
console.log(Array.isArray([1, 2, 3])); // true
```
