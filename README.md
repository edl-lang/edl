# EDL (Essential Development Language)

EDL is a simple, domain-oriented programming language designed to enable quick scripting and automation with a clear and modern syntax.

---

## Table of Contents

* [Overview](#overview)
* [Installation](#installation)
* [Syntax](#syntax)

  * [Variables](#variables)
  * [Functions](#functions)
  * [Control Flow](#control-flow)
  * [Data Types](#data-types)
* [Examples](#examples)
* [Using the CLI](#using-the-cli)
* [Contribution](#contribution)
* [License](#license)

---

## Overview

EDL is a simple and lightweight language designed for quickly writing scripts and automations. Its interpreter is written in Rust and allows running scripts via command line or in REPL mode.

---

## Installation

1. **Prerequisites**: Rust and Cargo installed on your machine.
2. Clone the repository:

   ```bash
   git clone https://github.com/edl-lang/edl.git
   cd edl
   ```
3. Build the CLI binary:

   ```bash
   cargo build --release -p cli
   ```
4. Install the binary:

   ```bash
   mkdir -p ~/.local/bin
   cp target/release/cli ~/.local/bin/edl
   chmod +x ~/.local/bin/edl
   ```
5. Add `~/.local/bin` to your PATH if not already done:

   ```bash
   echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```
6. Verify the installation:

   ```bash
   edl --help
   ```

---

## Syntax

### Variables

Declared with `let`:

```edl
let x = 42;
let name = "EDL";
let is_ready = true;
```

---

### Functions

Defined with `fn`:

```edl
fn greet(name) {
    return "Hello, " + name + "!";
}

let message = greet("World");
```

---

### Control Flow

* `if` / `else` condition:

```edl
if x > 10 {
    print("x is greater than 10");
} else {
    print("x is 10 or less");
}
```

* `while` loop:

```edl
let i = 0;
while i < 5 {
    print(i);
    i = i + 1;
}
```

* `for` loop with `in` (if supported):

```edl
for item in [1, 2, 3, 4, 5] {
    print(item);
}
```

---

### Data Types

* Numbers: `42`, `3.14`
* Strings: `"text"`
* Booleans: `true`, `false`
* Lists (example): `[1, 2, 3]` (if supported)

---

## Examples

```edl
let x = 10;
let y = 20;
let sum = x + y;

if sum > 25 {
    print("Sum is large");
} else {
    print("Sum is small");
}
```

---

## Using the CLI

* Run a script file:

  ```bash
  edl run my_script.edl
  ```

* Interactive REPL mode:

  ```bash
  edl repl
  ```

* Show help:

  ```bash
  edl --help
  ```

---

## Contribution

Contributions welcome! To contribute:

1. Fork this repository
2. Create a feature or bugfix branch
3. Open a Pull Request with a clear description

---

## License

Project licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
