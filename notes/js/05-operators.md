+++
title = "Operators (arithmetic, logical, bitwise)"
lang = "js"
part = 5
tags = ["operators", "strict equality", "nullish coalescing", "optional chaining"]
version = "ES2022+"
+++

## Strict equality and logical operators
Always prefer strict equality (`===`) over loose equality (`==`) to avoid surprising implicit type conversions.

```javascript
console.log(5 === 5);     // true
console.log(5 === "5");   // false
console.log(null === undefined); // false
console.log(null == undefined);  // true (loose coercion)
```

## Optional chaining (?.) and nullish coalescing (??)
Optional chaining safely reads nested properties without throwing TypeError; `??` falls back only on `null` or `undefined`.

```javascript
const response = {
    data: {
        profile: {
            name: "Sam"
        }
    }
};

const avatar = response?.data?.profile?.avatar ?? "default-avatar.png";
const missingVal = response?.data?.settings?.theme ?? "light";

console.log(`Avatar: ${avatar}, Theme: ${missingVal}`);
```
