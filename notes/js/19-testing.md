+++
title = "Testing & benchmarking"
lang = "js"
part = 19
tags = ["node:test", "node:assert", "testing", "assertions"]
version = "ES2022+"
+++

## Native Node.js test runner (node:test)
Modern Node.js includes a built-in test runner requiring zero external dependencies.

```javascript
import test from 'node:test';
import assert from 'node:assert/strict';

function sum(a, b) {
    return a + b;
}

test('sum computes accurate additions', () => {
    assert.equal(sum(2, 3), 5);
    assert.notEqual(sum(1, 1), 3);
});

test('async operations verify resolutions', async () => {
    const value = await Promise.resolve(42);
    assert.equal(value, 42);
});
```
