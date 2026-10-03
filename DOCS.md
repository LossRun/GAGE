# 📘 Gage Language Specification & Documentation

Welcome to the official documentation for **Gage**, a compiled, high-performance systems language with first-class SIMD vector math, simulation loops, and a dual-engine architecture (Native AOT LLVM compiler + Bytecode VM).

---

## 📑 Table of Contents

1. [Platform Setup & Installation](#1-platform-setup--installation)
2. [CLI Toolchain Reference](#2-cli-toolchain-reference)
3. [Language Syntax & Grammar](#3-language-syntax--grammar)
   - [Variables & Mutability](#variables--mutability)
   - [Primitive Types](#primitive-types)
   - [First-Class Vectors](#first-class-vectors)
   - [Operators](#operators)
   - [Control Flow](#control-flow)
   - [Simulation Step Loops](#simulation-step-loops)
   - [Functions](#functions)
4. [Compiler Diagnostics & Error Reference](#4-compiler-diagnostics--error-reference)

---

## 1. Platform Setup & Installation

Gage runs natively on all major operating systems. Ensure you have Git installed before proceeding.

### 📱 Android (Termux)

Termux requires `rust` (for Cargo) and `clang` (for the native AOT backend):

```bash
pkg update -y
pkg install git rust clang -y
git clone https://github.com/LossRun/GAGE.git
cd GAGE
bash gage-setup.sh
```

### 🐧 Linux (Ubuntu, Debian, Fedora, Arch)

Ensure a C compiler (clang or gcc) and cargo are available.

**Ubuntu / Debian:**

```bash
sudo apt update
sudo apt install git curl clang build-essential -y
```

**Arch Linux:**

```bash
sudo pacman -S git clang base-devel
```

**Fedora:**

```bash
sudo dnf install git clang gcc
```

Then clone and install Gage:

```bash
git clone https://github.com/LossRun/GAGE.git
cd GAGE
sudo bash gage-setup.sh
```

### 🍏 macOS (Apple Silicon & Intel)

Requires Xcode Command Line Tools and Rust:

```bash
xcode-select --install
```

If Rust is missing:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then:

```bash
git clone https://github.com/LossRun/GAGE.git
cd GAGE
bash gage-setup.sh
```

### 🪟 Windows (PowerShell)

Requires Rust installed via rustup and Clang via LLVM or Visual Studio C++ Build Tools.

```powershell
git clone https://github.com/LossRun/GAGE.git
cd GAGE
powershell -ExecutionPolicy Bypass -File .\install.ps1
```

---

## 2. CLI Toolchain Reference

The `gage` binary exposes several commands for building, inspecting, and executing programs.

| Command | Description | Example |
|---|---|---|
| `gage build <file> [-o bin]` | Compiles source directly to a standalone native binary (`-O3` optimization). | `gage build app.gage -o app` |
| `gage run <file>` | Compiles in memory and runs the native binary immediately. | `gage run sim.gage` |
| `gage vm <file>` | Runs code through the internal bytecode virtual machine. | `gage vm script.gage` |
| `gage check <file>` | Runs static type checks and syntax validation without compiling. | `gage check main.gage` |
| `gage emit-c <file>` | Outputs intermediate C code for debugging low-level output. | `gage emit-c main.gage` |
| `gage delete` | Safely uninstalls the Gage binary from your machine. | `gage delete` |
| `gage --info` | Displays architecture, target operating system, and backend status. | `gage --info` |
| `gage -v` / `gage --version` | Prints the current compiler version. | `gage -v` |
| `gage -h` / `gage --help` | Displays compiler help documentation. | `gage -h` |

---

## 3. Language Syntax & Grammar

### Variables & Mutability

Variables are declared using `let`. In Gage, variables can be reassigned directly:

```text
let counter = 0;
counter = counter + 1;
println(counter);
```

### Primitive Types

Gage natively handles the following fundamental types:

- **Int:** 64-bit signed integers (`42`, `-10`)
- **Float:** 64-bit IEEE floating-point numbers (`3.14159`, `-0.05`)
- **Bool:** Boolean flags (`true`, `false`)
- **Str:** UTF-8 string literals (`"Hello, Gage!"`)
- **Nil:** Absence of a value (`nil`)

Example:

```text
let count = 100;
let speed = 24.5;
let active = true;
let title = "Game Engine";
```

### First-Class Vectors

Unlike languages that require external math libraries, Gage has hardware-aligned SIMD vectors built directly into the grammar:

- `vec2(x, y)` — 2D vector
- `vec3(x, y, z)` — 3D vector
- `vec4(x, y, z, w)` — 4D vector

Vector arithmetic supports addition, subtraction, and scalar multiplication directly:

```text
let position = vec3(0.0, 10.0, 0.0);
let velocity = vec3(1.5, 0.0, -2.0);
let gravity = vec3(0.0, -9.8, 0.0);

// Vector math operations
let new_velocity = velocity + (gravity * 0.016);
let new_position = position + (new_velocity * 0.016);

println(new_position);
```

### Operators

#### Arithmetic Operators

- `+` — Addition (numbers, vectors)
- `-` — Subtraction (numbers, vectors)
- `*` — Multiplication (numbers, vector-by-scalar)
- `/` — Division
- `%` — Modulo

#### Comparison Operators

- `==` — Equals
- `!=` — Not equals
- `<` — Less than
- `<=` — Less than or equal
- `>` — Greater than
- `>=` — Greater than or equal

#### Logical Operators

- `&&` — Logical AND
- `||` — Logical OR
- `!` — Logical NOT

### Control Flow

#### `if` / `else` Conditional

```text
let health = 75;

if health <= 0 {
    println("Entity destroyed");
} else if health < 25 {
    println("Warning: Low health!");
} else {
    println("Status nominal");
}
```

#### `while` Loops

```text
let step_count = 0;

while step_count < 10 {
    println(step_count);
    step_count = step_count + 1;
}
```

#### `loop` (Infinite / Raw Loop)

```text
let frame = 0;

loop {
    frame = frame + 1;

    if frame >= 60 {
        break;
    }
}
```

### Simulation Step Loops

Gage introduces the native `step(dt)` construct designed specifically for game engines and physics engines:

```text
let pos = vec3(0.0, 50.0, 0.0);
let vel = vec3(0.0, -1.0, 0.0);

// step(dt) binds dt (time delta) as a local float variable
step(dt) {
    pos = pos + (vel * dt);
}
```

### Functions

Functions are declared using `fn` and support parameters and return values:

```text
fn calculate_momentum(mass, velocity) {
    return velocity * mass;
}

let m = 5.0;
let v = vec3(10.0, 0.0, 0.0);
let p = calculate_momentum(m, v);

println(p);
```

---

## 4. Compiler Diagnostics & Error Reference

Gage performs multi-stage static checks before generating code. Below are common errors and how to resolve them.

### 1. `[Lexer Error] Line X:Y -> Unexpected character`

**Cause:** The scanner encountered an invalid symbol not recognized by the language.

**Fix:** Check for missing quotes around strings or illegal punctuation characters.

### 2. `[Parse Error] Line X:Y -> Expected ';' after expression`

**Cause:** Statements in Gage must terminate with a semicolon `;`.

**Fix:** Append a semicolon `;` at the end of the line.

### 3. `[Semantic Error] Undefined variable '<name>'`

**Cause:** You tried to access or assign to a variable that has not been declared using `let`.

**Fix:** Declare the variable first:

```text
let count = 0;
```

### 4. `[Type Error] Cannot assign <TypeB> to variable '<name>' of type <TypeA>`

**Cause:** The static type checker detected an invalid type change during reassignment, such as assigning a string to an integer variable.

**Fix:** Maintain consistent types across variable lifetimes.

### 5. Native compilation failed. Please ensure Clang or GCC is installed.

**Cause:** The AOT native compiler could not locate `clang` or `gcc` on your system `PATH`.

**Fix:** Install Clang through your package manager.

On Termux:

```bash
pkg install clang -y
```

On Ubuntu/Debian:

```bash
sudo apt update
sudo apt install clang -y
```

Alternatively, use:

```bash
gage vm <file>
```

to run scripts using the built-in virtual machine without requiring an external compiler.

---

## 📦 Quick Termux Setup

For a fresh Termux installation:

```bash
pkg update -y
pkg upgrade -y
pkg install git rust clang -y
git clone https://github.com/LossRun/GAGE.git
cd GAGE
bash gage-setup.sh
```

After installation, verify the toolchain:

```bash
gage --version
gage --info
gage --help
```

---

## 🚀 Quick Start

Create a Gage source file:

```bash
nano hello.gage
```

Add:

```text
fn main() {
    println("Hello from Gage!");
}
```

Check the source:

```bash
gage check hello.gage
```

Build a native executable:

```bash
gage build hello.gage -o hello
```

Run the native executable:

```bash
./hello
```

Or use the native runner:

```bash
gage run hello.gage
```

For VM execution:

```bash
gage vm hello.gage
```

---

## 📄 Documentation Status

This document describes the current Gage language syntax, compiler architecture, CLI interface, supported platforms, and common diagnostics.

For repository-level information, see [`README.md`](README.md).

