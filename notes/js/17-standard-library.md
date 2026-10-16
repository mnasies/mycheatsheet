+++
title = "Standard library essentials"
lang = "js"
part = 17
tags = ["json", "math", "date", "intl", "structuredClone"]
version = "ES2022+"
+++

## Deep cloning with structuredClone (ES2022)
`structuredClone` performs native deep copying of objects, maps, sets, and typed arrays.

```javascript
const original = { user: "Robin", tags: ["dev", "lead"] };
const cloned = structuredClone(original);
cloned.tags.push("admin");

console.log(`Original: ${original.tags.length}, Clone: ${cloned.tags.length}`);
```

## Internationalization with Intl API
The `Intl` namespace provides locale-sensitive number, currency, and date formatting.

```javascript
const price = 1250.5;
const usdFormatter = new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' });
const eurFormatter = new Intl.NumberFormat('de-DE', { style: 'currency', currency: 'EUR' });

console.log(usdFormatter.format(price));
console.log(eurFormatter.format(price));
```
