+++
title = "Enums & pattern matching"
lang = "js"
part = 10
tags = ["enums", "object freeze", "dispatch tables", "symbols"]
version = "ES2022+"
+++

## Enum patterns with Object.freeze
Enums in JavaScript are traditionally constructed using frozen objects with unique strings or symbols.

```javascript
const Direction = Object.freeze({
    North: "NORTH",
    South: "SOUTH",
    East:  "EAST",
    West:  "WEST",
});

function move(dir) {
    if (!Object.values(Direction).includes(dir)) {
        throw new Error("Invalid direction");
    }
    console.log(`Moving ${dir}`);
}

move(Direction.North);
```

## Object lookup dispatch table
Object dispatch maps replace repetitive `switch` statements with concise dictionary dispatch.

```javascript
const statusHandlers = {
    200: () => "Success",
    404: () => "Not Found",
    500: () => "Internal Server Error",
};

const handleStatus = code => (statusHandlers[code] ?? (() => "Unknown Code"))();

console.log(handleStatus(200));
console.log(handleStatus(404));
```
