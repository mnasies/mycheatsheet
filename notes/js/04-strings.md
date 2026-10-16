+++
title = "Strings & text manipulation"
lang = "js"
part = 4
tags = ["strings", "template literals", "regex", "unicode"]
version = "ES2022+"
+++

## Template literals and interpolation
Template literals enclosed in backticks support multi-line strings and arbitrary expression interpolation.

```javascript
const language = "JavaScript";
const version = "ES2022";
const message = `Language: ${language}
Target: ${version.toUpperCase()}
Evaluated: ${2 + 2}`;

console.log(message);
```

## Modern string manipulation methods
`String.prototype` provides search, padding, replacement, and splitting utilities.

```javascript
const text = "   modern-javascript-reference   ";
const cleaned = text.trim().replaceAll("-", " ");
console.log(`Formatted: "${cleaned}"`);
console.log(`Starts with modern? ${cleaned.startsWith("modern")}`);
console.log(`Padded: "${"42".padStart(5, "0")}"`);
```
