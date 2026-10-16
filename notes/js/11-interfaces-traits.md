+++
title = "Interfaces, traits, polymorphism"
lang = "js"
part = 11
tags = ["prototypes", "duck typing", "mixins", "polymorphism"]
version = "ES2022+"
+++

## Prototype chain and inheritance
Every JavaScript object has an internal prototype link (`[[Prototype]]`), accessed via `Object.getPrototypeOf`.

```javascript
const speaker = {
    speak() {
        return `Hello from ${this.name}`;
    }
};

const bot = Object.create(speaker);
bot.name = "Assistant";

console.log(bot.speak());
```

## Mixins for shared behavior
Mixins compose reusable functional traits across unrelated classes without inheritance hierarchies.

```javascript
const SerializableMixin = Base => class extends Base {
    toJSON() {
        return JSON.stringify(this);
    }
};

class User {
    constructor(name) { this.name = name; }
}

class SerializableUser extends SerializableMixin(User) {}

const u = new SerializableUser("Jordan");
console.log(`Serialized: ${u.toJSON()}`);
```
