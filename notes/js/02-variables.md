+++
title = "Variables, constants, mutability"
lang = "js"
part = 2
tags = ["let", "const", "destructuring", "scoping"]
version = "ES2022+"
+++

## Block-scoped declarations with const and let
`const` binds immutable references; `let` declares mutable, block-scoped variables.

```javascript
const maxLimit = 100;
let currentCount = 0;

if (true) {
    let blockScoped = "isolated";
    currentCount += 5;
}

console.log(`Count: ${currentCount}`);
```

## Array and object destructuring
Destructuring extracts properties from objects and arrays with optional default values and renaming.

```javascript
const user = { id: 101, username: "alex", role: "admin" };
const { id, username: handle, active = true } = user;

const coordinates = [12.5, 45.2];
const [latitude, longitude] = coordinates;

console.log(`User: ${handle} (ID: ${id}, Active: ${active}) at [${latitude}, ${longitude}]`);
```
