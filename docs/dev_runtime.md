# EDL Dev Runtime – Feature Ideas & Roadmap

This document lists feature ideas and improvements for the EDL language runtime.  
It aims to help contributors and maintainers track, discuss, and prioritize enhancements.

---

## Core Language Features

- [x] **Comments support** (`//` and `#` single-line comments)
- [x] **Better error reporting** (line/column info, clearer messages)
- [ ] **Pattern matching** (`match` statement, Rust-style)
- [ ] **Enums and variants**
- [x] **Structs and custom types** with fields and methods
- [ ] **Type annotations** (optional, for TypeScript/Rust flavor)
- [x] **Constants** (`const` keyword)
- [x] **Lists/Arrays** and **Dictionaries/Maps** as first-class types
- [ ] **Tuples** support
- [ ] **Anonymous functions (lambdas)**
- [ ] **Block expressions** (return last value, Rust-style)
- [ ] **Import system** for modules/scripts
- [x] **Native functions** (e.g. `print`, `input`, `to_number`)
- [ ] **Function overloading** (optional)
- [ ] **Operator overloading** (optional)
- [ ] **Yield/generator support**
- [x] **Break/Continue** in loops
- [ ] **For loops** with ranges and iterators
- [x] **While loops**
- [x] **If/Else expressions**
- [x] **REPL improvements** (multi-line, variable history, help command)

---

## Interoperability & Extensibility

- [ ] **FFI/External function calls** (call Rust, Python, or TypeScript code)
- [ ] **Plugin/module system** for extending the language
- [ ] **Standard library** (math, string, file, etc.)

---

## Developer Experience

- [ ] **Unit tests for core (lexer, parser, runtime)**
- [x] **Better documentation and code comments**
- [ ] **Error messages in English and French**
- [x] **Examples and sample scripts**
- [ ] **Changelog and contribution guide**
- [ ] **Code formatting/linting tools**

---

## Advanced Ideas

- [ ] **Type inference**
- [ ] **Async/await support**
- [ ] **Pattern destructuring**
- [ ] **Macros or code generation**
- [ ] **Debugger integration**
- [ ] **Source maps for error reporting**

---

*Feel free to add, discuss, or vote for features! This list will evolve with the project and the community.*