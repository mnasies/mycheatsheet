+++
title = "Program structure & entry point"
lang = "js"
part = 1
tags = ["entry point", "console", "strict mode", "node"]
version = "ES2022+"
+++

## Execution entry point and console output
JavaScript executes top-level statements sequentially in runtimes like Node.js or modern browsers.

```javascript
'use strict';

function main() {
    console.log("Hello, Modern JavaScript (ES2022+)!");
}

main();
```

## Environment inspection and process arguments
In Node.js, `process.argv` exposes CLI invocation arguments, and `process.env` accesses environment variables.

```javascript
console.log(`Node version: ${process.version}`);
console.log(`Arguments count: ${process.argv.length}`);
process.argv.forEach((arg, index) => {
    console.log(`  [${index}]: ${arg}`);
});
```
