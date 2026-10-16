+++
title = "Generics & templates"
lang = "js"
part = 12
tags = ["generics", "jsdoc", "polymorphism", "typing"]
version = "ES2022+"
+++

## Generic programming via higher-order functions
Dynamic JavaScript functions operate generically across types; JSDoc annotations provide IDE type inference.

```javascript
/**
 * Generic identity function
 * @template T
 * @param {T} item
 * @returns {T}
 */
function identity(item) {
    return item;
}

/**
 * Generic container wrapper
 * @template T
 */
class Box {
    /** @param {T} val */
    constructor(val) {
        this.value = val;
    }

    /** @returns {T} */
    unwrap() {
        return this.value;
    }
}

const numBox = new Box(42);
const strBox = new Box("hello");
console.log(numBox.unwrap(), strBox.unwrap());
```
