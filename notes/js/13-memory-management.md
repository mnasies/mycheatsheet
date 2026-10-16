+++
title = "Memory management, ownership, lifetimes"
lang = "js"
part = 13
tags = ["memory", "garbage collection", "weakmap", "weakref"]
version = "ES2022+"
+++

## Garbage collection and closures
V8 automatically frees unreachable objects through generational mark-and-sweep garbage collection. Avoid accidental closure leaks.

```javascript
function setupHandler() {
    const heavyBuffer = new Uint8Array(1024 * 1024); // 1 MB
    return function getLength() {
        // Only retain what is necessary
        return heavyBuffer.length;
    };
}

const fn = setupHandler();
console.log(`Buffer size: ${fn()} bytes`);
```

## WeakMap and WeakSet for leak-free caching
Keys in `WeakMap` are held weakly, permitting garbage collection when no other references remain.

```javascript
const cache = new WeakMap();

function trackAccess(obj) {
    const count = cache.get(obj) ?? 0;
    cache.set(obj, count + 1);
}

let session = { id: "sess_123" };
trackAccess(session);
console.log(`Access count: ${cache.get(session)}`);
session = null; // Eligible for garbage collection
```
