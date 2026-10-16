+++
title = "Concurrency & asynchronous programming"
lang = "js"
part = 15
tags = ["async", "await", "promises", "event loop", "allSettled"]
version = "ES2022+"
+++

## Promises and async/await syntax
`async`/`await` flattens asynchronous promise resolution into readable linear control flow.

```javascript
function delay(ms, value) {
    return new Promise(resolve => setTimeout(() => resolve(value), ms));
}

async function fetchUserData() {
    console.log("Fetching user...");
    const user = await delay(50, { id: 1, name: "Taylor" });
    console.log(`User loaded: ${user.name}`);
    return user;
}

fetchUserData();
```

## Concurrent operations with Promise.allSettled
`Promise.allSettled` waits for all promises to resolve or reject without aborting on initial failure.

```javascript
async function runBatch() {
    const p1 = Promise.resolve("Success 1");
    const p2 = Promise.reject(new Error("Failed 2"));
    const p3 = Promise.resolve("Success 3");

    const results = await Promise.allSettled([p1, p2, p3]);
    for (const res of results) {
        console.log(`Status: ${res.status}, Value: ${res.value ?? res.reason.message}`);
    }
}

runBatch();
```
