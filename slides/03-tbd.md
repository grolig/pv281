---
marp: true
title: PV281 Programming in Rust
description: Programming in Rust The Basics
theme: rust
paginate: true
---
![w:512 h:512](./assets/rust-logo-1.png)
# <!--fit--> PV281: Programování v Rustu

---

# Obsah

Struktury

Enumny

Pattern Matching

---

# <!--fit--> Struktury

---

# Definice struktury

```rust 
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // ...
}
```

---

# Použití struktury

```rust 
struct User { 
    //... 
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}

```

---

# Funkce pro vytvoření struktury

```rust 
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

---

# Vytvoření z již existující struktury

```rust 
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ...user1
    };
}

```

---

# <!--fit--> Dotazy?

---

# <!--fit--> Děkuji za pozornost

