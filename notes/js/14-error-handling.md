+++
title = "Error handling"
lang = "js"
part = 14
tags = ["errors", "try-catch", "error causes", "exceptions"]
version = "ES2022+"
+++

## Structured try/catch/finally
The `try/catch/finally` statement handles runtime exceptions and ensures cleanup runs unconditionally.

```javascript
function parseData(raw) {
    try {
        return JSON.parse(raw);
    } catch (err) {
        console.error(`Parse failed: ${err.message}`);
        return null;
    } finally {
        console.log("Parsing operation concluded.");
    }
}

parseData('{"valid": true}');
parseData("invalid json");
```

## Custom error classes with cause chaining (ES2022)
Custom errors can inherit from `Error` and attach an underlying `cause` for root-cause diagnosis.

```javascript
class DatabaseError extends Error {
    constructor(message, options) {
        super(message, options);
        this.name = "DatabaseError";
    }
}

try {
    try {
        throw new Error("Connection timed out");
    } catch (netErr) {
        throw new DatabaseError("Failed to fetch record", { cause: netErr });
    }
} catch (dbErr) {
    console.log(`Error: ${dbErr.message}`);
    console.log(`Root cause: ${dbErr.cause.message}`);
}
```
