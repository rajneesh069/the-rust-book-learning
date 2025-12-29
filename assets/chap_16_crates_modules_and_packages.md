# Rust Crates, Modules & Visibility — Mental Model Notes

> TL;DR: **Rust compiles module trees, not files.**
> Files and folders are just a way to *define* modules.

---

## 1. Package vs Crate vs Module

### Package
- A **Cargo project**
- Defined by `Cargo.toml`
- Can contain:
  - **one library crate** (`src/lib.rs`)
  - **zero or more binary crates** (`src/main.rs`, `src/bin/*.rs`)

---

### Crate
- A **single compilation unit**
- Has **exactly one root module**
- Root module:
  - `src/main.rs` → binary crate
  - `src/lib.rs`  → library crate
- Root module path is accessed via `crate::`

---

### Module
- A **namespace**
- Modules form a **tree**
- Rust reasons about **modules + paths + visibility**
- Files/folders are only storage, not semantics

---

## 2. Core Mental Model (Very Important)

```text
❌ Files are modules        (WRONG)
✅ Modules are namespaces  (CORRECT)

Rust does NOT compile files.
Rust compiles a MODULE TREE.
````

---

## 3. `mod` — Declare a Module Exists

```rust
mod user;
```

* `mod` does **NOT** import
* It declares:

  > “There exists a module named `user`”
* Rust will search for **exactly one**:

  * `user.rs`
  * `user/mod.rs`

### Inline module

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}
```

---

## 4. Folders ≠ Modules

```text
src/
└── services/
    └── auth.rs
```

⬆️ This folder is **ignored** unless declared.

You must write:

```rust
mod services;
```

And inside `services/mod.rs`:

```rust
pub mod auth;
```

👉 **Folders do nothing unless `mod` says they do.**

---

## 5. Module Paths

```text
crate::services::auth::login
```

* `crate`     → root module
* `services`  → submodule
* `auth`      → submodule
* `login`     → item inside module

---

## 6. `pub` — Visibility (Not Export)

* Everything in Rust is **private by default**
* `pub` allows access **from outside the module**

### Functions

```rust
pub fn login() {}
```

---

### Structs

```rust
pub struct User {
    pub id: i32,     // must be pub
    name: String,   // private field
}
```

➡️ Struct AND its fields must be `pub`

---

### Enums

```rust
pub enum Role {
    Admin,
    User,
}
```

➡️ `pub enum` automatically exposes **all variants**

---

## 7. `use` — Bring Names Into Scope

```rust
use crate::services::auth::login;
```

* `use` does **NOT** load files
* It shortens paths only

```rust
// instead of
crate::services::auth::login();

// you can write
login();
```

### Common patterns

```rust
use crate::models::User;
use crate::services::{auth, user};
use crate::errors::*;
```

---

## 8. Path Keywords: `crate`, `self`, `super`

### `crate`

* Absolute path from crate root
* Recommended for clarity

```rust
crate::services::auth::login();
```

---

### `self`

* Current module
* Similar to `./`

```rust
mod auth {
    pub fn login() {}

    pub fn handler() {
        self::login();
    }
}
```

---

### `super`

* Parent module
* Similar to `../`

```rust
mod services {
    pub fn common() {}

    mod auth {
        pub fn login() {
            super::common();
        }
    }
}
```

---

## 9. Scoped Visibility (`pub(...)`)

```rust
pub(crate) fn internal_api() {}
pub(super) fn parent_only() {}
pub(self) fn private_fn() {}
pub(in crate::services) fn service_only() {}
```

| Visibility     | Meaning         |
| -------------- | --------------- |
| `pub`          | Everywhere      |
| `pub(crate)`   | Same crate only |
| `pub(super)`   | Parent module   |
| `pub(self)`    | Same module     |
| `pub(in path)` | Limited to path |

---

## 10. Typical Production Backend Layout

```text
src/
├── main.rs
├── lib.rs
├── config/
│   ├── mod.rs
│   └── env.rs
├── db/
│   ├── mod.rs
│   └── pool.rs
├── routes/
│   ├── mod.rs
│   └── auth.rs
├── services/
│   ├── mod.rs
│   └── auth_service.rs
└── models/
    ├── mod.rs
    └── user.rs
```

```rust
// lib.rs
pub mod config;
pub mod db;
pub mod routes;
pub mod services;
pub mod models;
```

```rust
// main.rs
use my_app::routes::auth;

fn main() {
    auth::register();
}
```

---

## 11. Final Rule (Remember This)

```text
Rust does NOT care about files.
Rust cares about MODULES and VISIBILITY.
Files only define the module tree.
```

## `mod.rs` — How It Works & When to Declare `mod`

### 1. What is `mod.rs`?

`mod.rs` is **NOT special syntax**.
It is just a **convention** Rust uses to define a module **whose contents live in a folder**.

```text
services/
├── mod.rs   ← defines module `services`
├── auth.rs
└── user.rs
````

```rust
// services/mod.rs
pub mod auth;
pub mod user;
```

Effect:

```text
crate::services::auth
crate::services::user
```

👉 `mod.rs` is simply:

> “This folder is a module, and these are its children.”

---

### 2. Who is Allowed to Declare `mod something;`?

**Only the PARENT module may declare its children.**

This is a **hard rule**.

```text
lib.rs
└── services/
    ├── mod.rs
    └── auth.rs
```

```rust
// lib.rs (PARENT of `services`)
mod services;        // ✅ allowed
```

```rust
// services/mod.rs (PARENT of `auth`)
pub mod auth;        // ✅ allowed
```

```rust
// auth.rs
mod services;        // ❌ ILLEGAL (child cannot declare parent)
```

---

### 3. You Do NOT Declare `mod` Where You *Use* Things

❌ **WRONG (very common mistake)**

```rust
// main.rs
mod services;              // ❌ if already declared elsewhere
use services::auth::login;
```

Why wrong?

* `mod` = module declaration
* `use` = name import
* Declaring a module twice = illegal

---

### 4. Correct Rule (Memorize This)

```text
`mod` → declared ONCE, at the module boundary
`use` → used EVERYWHERE you want access
```

---

### 5. Who Declares What? (Authority Model)

```text
crate (lib.rs or main.rs)
 └── services (services/mod.rs)
     └── auth (services/auth.rs)
```

| File              | Responsibility                                |
| ----------------- | --------------------------------------------- |
| `lib.rs`          | declares `mod services;`                      |
| `services/mod.rs` | declares `mod auth;`                          |
| `auth.rs`         | defines items (no mod children unless needed) |

---

### 6. Modern Alternative to `mod.rs` (Rust 2018+)

Rust allows this **instead of `mod.rs`**:

```text
services.rs
services/
└── auth.rs
```

```rust
// services.rs
pub mod auth;
```

➡️ Both are equivalent
➡️ `mod.rs` is still very common in large codebases

---

### 7. Declaring `mod` in the SAME FILE (Inline)

```rust
mod auth {
    pub fn login() {}
}
```

This is valid **only when the module is inline**.

You CANNOT mix inline and file-based modules:

```rust
mod auth;     // from auth.rs
mod auth { }  // ❌ illegal — duplicate module
```

---

### 8. Visibility Rule Reminder (Critical)

Declaring a module ≠ exposing it

```rust
mod auth;     // private module
pub mod auth; // public module
```

If a parent module is private,
**everything inside is unreachable** from outside.

---

### 9. Mental Model (Final)

Folders store modules
Files define modules
`mod` declares structure
`use` imports names
Parents define children
Children NEVER define parents

## How Cargo decides “what is a crate?”

Cargo does **not** look for:

* `fn main()`
* “executable-looking code”
* magic annotations

Cargo looks for **specific file locations**.

## Rule 1: These locations create crates automatically

Inside **one package** (`Cargo.toml`):

```text
src/main.rs        → binary crate
src/lib.rs         → library crate
src/bin/*.rs       → binary crates
```

### Example

```text
src/
├── main.rs            ← binary crate
├── lib.rs             ← library crate
└── bin/
    ├── worker.rs      ← binary crate
    └── migrate.rs     ← binary crate
```

Cargo sees **4 crates** here:

* 1 library
* 3 binaries

You can run them because Cargo *knows they are crates*.

```bash
cargo run
cargo run --bin worker
cargo run --bin migrate
```

---

## Rule 2: Folder name `bin` is SPECIAL

```text
src/bin/
```

This folder name is **hard-coded into Cargo’s conventions**.

If you rename it:

```text
src/auth/
└── login.rs
```

❌ **This is NOT a crate**
❌ Cargo ignores it as an entry point
❌ Even if it has `fn main()`

Why?

Because Cargo never even *checks* that folder for crates.

---

## “But what if the file is executable?”

This does **not** matter:

```rust
fn main() {
    println!("I can run!");
}
```

If this file lives in:

```text
src/auth/login.rs
```

Cargo treats it as:

* a **module file**
* usable *only if declared with `mod`*
* **never** as a binary crate

---

## Rule 3: Arbitrary folders are for MODULES, not CRATES

```text
src/
├── auth/
│   └── login.rs
```

This means:

```rust
mod auth;
```

```rust
// auth/mod.rs
pub mod login;
```

Accessed as:

```rust
crate::auth::login
```

🧠 **Folders ≠ crates**
🧠 **Folders = module grouping only**

---

## “So how do I make a binary outside `bin/`?”

You have **two valid options**.

---

### ✅ Option 1: Use `src/bin/` (recommended)

```text
src/bin/auth.rs
```

This becomes:

```bash
cargo run --bin auth
```

Clean. Idiomatic. Simple.

---

### ✅ Option 2: Explicit binary in Cargo.toml (advanced)

```toml
[[bin]]
name = "auth"
path = "src/auth/main.rs"
```

Now this works:

```text
src/
└── auth/
    └── main.rs
```

Cargo now knows:

> “This file is a binary crate, even though it’s not in `bin/`”

⚠️ This is explicit and manual — **not automatic**.

---

## What Cargo will NEVER do

Cargo will **never**:

* scan folders for `fn main`
* guess intent
* auto-promote modules to crates

Rust avoids **implicit behavior by design**.

---

## Mental Model (lock this in)

```text
bin/ folder → crate factory
other folders → module storage
```

or even simpler:

```text
Crates are declared by LOCATION
Modules are declared by `mod`
```

---

## Final Yes/No Table

| Situation                       | Is it a crate? |
| ------------------------------- | -------------- |
| `src/main.rs`                   | ✅ Yes          |
| `src/lib.rs`                    | ✅ Yes          |
| `src/bin/foo.rs`                | ✅ Yes          |
| `src/auth/foo.rs`               | ❌ No           |
| `src/auth/foo.rs` + `fn main()` | ❌ No           |
| Declared in `[[bin]]`           | ✅ Yes          |
