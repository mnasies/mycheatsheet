+++
title = "Modules, packages, imports"
lang = "js"
part = 16
tags = ["modules", "esm", "export", "import", "dynamic import"]
version = "ES2022+"
+++

## ES Modules export and import
Named and default exports modularize functionality into encapsulated source files.

```javascript
// math.js
export const PI = 3.14159;
export function add(a, b) { return a + b; }
export default function multiply(a, b) { return a * b; }

// main.js
// import multiply, { PI, add } from './math.js';
console.log("Modules syntax ready.");
```

## Dynamic imports for code splitting
`import()` loads modules asynchronously on demand.

```javascript
async function loadFeature(name) {
    if (name === "chart") {
        // const { renderChart } = await import('./chart.js');
        console.log("Feature loaded dynamically");
    }
}

loadFeature("chart");
```
