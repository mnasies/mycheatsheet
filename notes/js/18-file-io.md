+++
title = "File system & I/O"
lang = "js"
part = 18
tags = ["fs", "promises", "node:fs", "streaming"]
version = "ES2022+"
+++

## Reading and writing files with node:fs/promises
The promise-based `node:fs` module supports asynchronous file operations with `async/await`.

```javascript
import fs from 'node:fs/promises';

async function handleFile() {
    const filename = "temp_js.txt";
    await fs.writeFile(filename, "Hello from Node.js File System!", "utf8");

    const content = await fs.readFile(filename, "utf8");
    console.log(`Read file: ${content}`);

    await fs.unlink(filename);
}

handleFile().catch(console.error);
```

## Checking file existence and directory listing
Querying directories and metadata stats asynchronously.

```javascript
import fs from 'node:fs/promises';

async function listCurrentDir() {
    const files = await fs.readdir(".");
    console.log(`Found ${files.length} items in current directory.`);
}

listCurrentDir().catch(console.error);
```
