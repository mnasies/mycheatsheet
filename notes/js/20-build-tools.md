+++
title = "Build tools, package managers, and CLI commands"
lang = "js"
part = 20
tags = ["npm", "package.json", "node", "vite", "cli"]
version = "ES2022+"
+++

## Package.json configuration
`package.json` defines project metadata, npm scripts, and dependencies.

```json
{
  "name": "my-app",
  "version": "1.0.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "test": "node --test"
  },
  "dependencies": {}
}
```

## Essential CLI package manager commands
Everyday command-line workflow for Node.js development.

```bash
# Initialize a new package
npm init -y

# Install dependencies
npm install express
npm install -D typescript @types/node

# Execute scripts
npm run build
npm test

# Execute package binary without installing globally
npx prettier --write .
npx eslint .

# Check outdated dependencies
npm outdated
```
