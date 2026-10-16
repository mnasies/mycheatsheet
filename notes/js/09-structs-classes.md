+++
title = "Structs, classes, and objects"
lang = "js"
part = 9
tags = ["classes", "inheritance", "private fields", "static"]
version = "ES2022+"
+++

## Modern ES6+ classes and private fields
Classes support constructors, inheritance, static members, and true private fields using the `#` prefix.

```javascript
class BankAccount {
    #balance; // Private field

    constructor(initial = 0) {
        this.#balance = initial;
    }

    deposit(amount) {
        if (amount <= 0) throw new Error("Invalid deposit");
        this.#balance += amount;
    }

    get balance() {
        return this.#balance;
    }
}

const account = new BankAccount(100);
account.deposit(50);
console.log(`Account balance: $${account.balance}`);
```

## Class inheritance with extends and super
Subclasses inherit methods and override behavior using `super`.

```javascript
class Animal {
    constructor(name) { this.name = name; }
    speak() { return `${this.name} makes a noise.`; }
}

class Dog extends Animal {
    speak() { return `${this.name} barks!`; }
}

const d = new Dog("Milo");
console.log(d.speak());
```
