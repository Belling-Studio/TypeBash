# TypeBash

<img width="280" height="282" alt="download" src="https://github.com/user-attachments/assets/c77f611e-50ac-4598-8894-43f390be479a" />

![License](https://github.com/Belling-Studio/TypeBash/blob/main/LICENSE) ![Built with Rust](https://github.com/rust-lang/rust)

A Rust-powered static type checker and compiler that brings strict typing and safety features to Bash scripts [INDEX].

TypeBash transforms vulnerable, error-prone shell scripts into safe, robust, and highly predictable production code using strict compiler diagnostics heavily inspired by `rustc` [INDEX].

---

## ⚡ Key Features

* 🛡️ **Types and Safety** Our compiler ensures safety through types.
* 🛡️ **DevOps Shield:** Automatically verifies whether invoked system utilities (like `docker`, `git`, `curl`) exist in your host `PATH` during compilation [INDEX].
* 📐 **Strict Typing & Scoping:** Full support for `string`, `number`, `boolean`, generic arrays (`Array<T>`), and an intelligent path-validation type (`Path`) [INDEX].
* 🔒 **Secure Bash Output:** Automatically generates production-ready `.sh` files configured with `set -euo pipefail` and a native runtime error handler (`trap ERR`) [INDEX].
* 🌐 **Secure Bash Output:** Fully compatible with your existing code; our compiler translates into .sh files.
---

## 💻 Code Example

Write clean, typed, and structured code in `script.tb`:

```typebash
let backupDir: Path = "./backups"
let maxAttempts: number = 3

fn notify(msg: string) {
    print(msg)
}

notify("TypeBash v0.5.0-beta: Initializing secure script execution...")

let targetFiles: Array<string> = ["config.json", "database.sql"]
for file in targetFiles {
    print(file)
}

exec "mkdir -p ./backups"
```

### Attention

The compiler was written by an AI, but I oversee everything to ensure it’s safe and convenient for you; I conceived the entire project myself.

---

## 🚀 Help us create safety!!!
We strive to maintain security, but we welcome your help; we are always glad to see your contributions to our project—thank you!

## 📜 License
Distributed under the **MIT License**. See `LICENSE` for more information [INDEX].
