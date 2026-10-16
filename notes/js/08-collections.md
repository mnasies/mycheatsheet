+++
title = "Collections (arrays, slices, maps/dictionaries, sets)"
lang = "js"
part = 8
tags = ["arrays", "map", "set", "reduce", "filter"]
version = "ES2022+"
+++

## Array transformations: map, filter, reduce
Array methods chain cleanly for declarative data transformations.

```javascript
const numbers = [1, 2, 3, 4, 5, 6];

const processed = numbers
    .filter(n => n % 2 === 0)
    .map(n => n * 10);

const total = numbers.reduce((acc, curr) => acc + curr, 0);

console.log(`Processed: ${processed}, Total: ${total}`);
```

## Map and Set collections
`Map` supports arbitrary keys (including objects and functions); `Set` stores unique values.

```javascript
const userCache = new Map();
const key = { id: 1 };
userCache.set(key, { name: "Alice", role: "Developer" });

console.log(`Cached: ${userCache.get(key).name}`);

const uniqueTags = new Set(["go", "rust", "go", "js"]);
console.log(`Unique count: ${uniqueTags.size}`);
```
