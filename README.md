<div align="center">

<img src="logo.png" alt="GAGE Logo" width="180">

# GAGE

### GAGE UNIFIED TOOLCHAIN
**Native LLVM Compiler & Fast VM**

[![License](https://img.shields.io/badge/License-MIT-00f5ff?style=flat-square)](LICENSE)
[![Status](https://img.shields.io/badge/Status-Active-39ff14?style=flat-square)](#)
[![Backend](https://img.shields.io/badge/Backend-LLVM%20%2F%20Clang-00f5ff?style=flat-square)](#)
[![Architecture](https://img.shields.io/badge/Architecture-ARM64%20%7C%20x86__64-39ff14?style=flat-square)](#)
[![Platforms](https://img.shields.io/badge/Platforms-Linux%20%7C%20Windows%20%7C%20macOS%20%7C%20Android-orange?style=flat-square)](#)
[![Language](https://img.shields.io/badge/Language-GAGE-00f5ff?style=flat-square)](#)

**A high-performance, ahead-of-time compiled native systems programming language designed for simulations, game engines, graphics, vector mathematics, and low-latency systems.**

</div>

---

## ⚡ What is GAGE?

**GAGE** is a native systems programming language and unified toolchain built around two execution paths:

- **AOT Native Compilation** powered by LLVM/Clang
- **Fast Bytecode VM Execution** for rapid development and testing

GAGE is designed to combine the productivity of a modern language with the performance and control expected from native systems software.

Source files such as `.gage`, `.gg`, and `.gag` can be compiled directly into native machine-code binaries or executed through the integrated virtual machine.

---

## ✨ Core Features

### 🚀 Native AOT Compilation

GAGE can compile source code directly into native **ARM64** and **x86_64** binaries through an LLVM/Clang-based compilation pipeline.

- Native machine-code generation
- Optimized compilation
- `-O3` optimization support
- Standalone executable output
- No managed runtime required

### 📐 First-Class Vector Mathematics

Vectors are treated as first-class language primitives rather than external library abstractions.

Supported primitives include:

- `vec2`
- `vec3`
- `vec4`

Designed for:

- Physics
- Graphics
- Simulations
- Game engines
- Numerical computation

### ⏱️ Simulation-Oriented Design

GAGE provides primitives designed around deterministic simulation workloads.

```text
step(dt)
```

This makes the language suitable for:

- Physics simulations
- Game logic
- Animation systems
- Dynamic environments
- Real-time computation

### 🛡️ Static Semantic Checking

Before native compilation, GAGE performs semantic and type validation.

The compiler can detect problems such as:

- Undefined identifiers
- Invalid scopes
- Type conflicts
- Invalid expressions
- Semantic errors

### ⚡ Dual Execution Engine

GAGE provides two primary execution modes:

| Mode | Command | Purpose |
|---|---|---|
| Native AOT | `gage build` | Produce optimized native binaries |
| Native Run | `gage run` | Compile and execute natively |
| Bytecode VM | `gage vm` | Fast execution without an external C compiler |
| Semantic Check | `gage check` | Validate source without compiling |
| C Emission | `gage emit-c` | Inspect generated intermediate C |

### 🌐 Cross-Platform Toolchain

GAGE is designed to operate across:

- Android / Termux
- Linux
- Windows
- macOS
- ARM64
- x86_64

---

## 🏗️ Compilation Architecture

```text
                    ┌─────────────────────┐
                    │     GAGE SOURCE     │
                    │ .gage / .gg / .gag  │
                    └──────────┬──────────┘
                               │
                               ▼
                    ┌─────────────────────┐
                    │  Lexer / Tokenizer  │
                    │      lexer.rs       │
                    └──────────┬──────────┘
                               │
                               ▼
                    ┌─────────────────────┐
                    │      AST Parser     │
                    │      parser.rs      │
                    └──────────┬──────────┘
                               │
                               ▼
                    ┌─────────────────────┐
                    │ Semantic / Type     │
                    │      Checker        │
                    │      types.rs       │
                    └──────────┬──────────┘
                               │
                  ┌────────────┴────────────┐
                  │                         │
                  ▼                         ▼
        ┌──────────────────┐      ┌──────────────────┐
        │   NATIVE AOT     │      │    BYTECODE VM   │
        │      PATH        │      │       PATH       │
        └────────┬─────────┘      └────────┬─────────┘
                 │                         │
                 ▼                         ▼
        ┌──────────────────┐      ┌──────────────────┐
        │    codegen.rs    │      │   compiler.rs    │
        │    C / SIMD      │      │     Bytecode     │
        └────────┬─────────┘      └────────┬─────────┘
                 │                         │
                 ▼                         ▼
        ┌──────────────────┐      ┌──────────────────┐
        │   LLVM / Clang   │      │      vm.rs       │
        │     Backend      │      │  Virtual Machine │
        └────────┬─────────┘      └────────┬─────────┘
                 │                         │
                 ▼                         ▼
        ┌──────────────────┐      ┌──────────────────┐
        │ Native Machine   │      │   Direct VM      │
        │     Binary       │      │     Output       │
        └──────────────────┘      └──────────────────┘
```

> **Note:** Detailed grammar rules, language keywords, syntax references, and standard-library documentation are maintained separately in [`DOCS.txt`](DOCS.txt).

---

## 💻 Cross-Platform Installation

Clone the repository and run the setup script for your platform.

### 1. Android — Termux 📱

```bash
git clone https://github.com/akarshtyagi08-lgtm/GAGE.git
cd GAGE
bash gage-setup.sh
```

### 2. Linux — Ubuntu, Debian, Fedora, Arch 🐧

```bash
git clone https://github.com/akarshtyagi08-lgtm/GAGE.git
cd GAGE
sudo bash gage-setup.sh
```

### 3. macOS — Apple Silicon & Intel 🍎

```bash
git clone https://github.com/akarshtyagi08-lgtm/GAGE.git
cd GAGE
bash gage-setup.sh
```

### 4. Windows — PowerShell 🪟

```powershell
git clone https://github.com/akarshtyagi08-lgtm/GAGE.git
cd GAGE
powershell -ExecutionPolicy Bypass -File .\install.ps1
```

---

## 🛠️ Toolchain CLI Reference

The unified `gage` executable handles compilation, validation, execution, and toolchain management across supported platforms.

| Command | Description |
|---|---|
| `gage build <file> [-o bin]` | Compile source into a standalone native binary |
| `gage run <file>` | Compile and execute using the native backend |
| `gage vm <file>` | Execute source using the integrated bytecode VM |
| `gage check <file>` | Run syntax, semantic, and type validation |
| `gage emit-c <file>` | Generate and inspect intermediate C code |
| `gage delete` | Safely remove installed GAGE binaries and local data |
| `gage --info` | Display target architecture, OS, and toolchain information |
| `gage -v` | Display the compiler version |
| `gage --version` | Display the compiler version |
| `gage -h` | Display CLI help |
| `gage --help` | Display the complete CLI manual |

---

## 📁 Repository Structure

```text
GAGE/
│
├── src/
│   ├── ast.rs
│   │   └── Abstract Syntax Tree nodes
│   ├── token.rs
│   │   └── Lexical tokens and source spans
│   ├── lexer.rs
│   │   └── Scanner and tokenizer
│   ├── parser.rs
│   │   └── Recursive-descent parser
│   ├── types.rs
│   │   └── Semantic analyzer and type checker
│   ├── codegen.rs
│   │   └── Native AOT and SIMD code generation
│   ├── bytecode.rs
│   │   └── Bytecode instructions and chunks
│   ├── compiler.rs
│   │   └── Bytecode compiler
│   ├── vm.rs
│   │   └── Stack-based virtual machine
│   └── main.rs
│       └── Unified CLI entrypoint
│
├── examples/
│   └── Example programs and simulation tests
│
├── logo.png
│   └── GAGE project logo
│
├── gage-setup.sh
│   └── Unix and Android installer
│
├── install.ps1
│   └── Windows installer
│
├── Cargo.toml
│   └── Rust package and build configuration
│
├── LICENSE
│   └── MIT License
│
├── DOCS.txt
│   └── Language specification and syntax reference
│
└── README.md
    └── Project documentation
```

---

## 🧪 Example

A minimal GAGE program:

```text
fn main() {
    print("Hello from GAGE!");
}
```

Build it:

```bash
gage build hello.gage -o hello
```

Run it:

```bash
./hello
```

Or execute it directly:

```bash
gage run hello.gage
```

For rapid VM testing:

```bash
gage vm hello.gage
```

---

## 📐 Vector Example

GAGE is designed with vector-oriented workloads in mind.

```text
let position = vec3(10.0, 5.0, 2.0);
let velocity = vec3(1.0, 0.0, -1.0);

let next_position = position + velocity;
```

This style is intended to make common mathematical and simulation operations concise and readable.

---

## 🔬 Compiler Pipeline

```text
Source
  │
  ▼
Lexer
  │
  ▼
Parser
  │
  ▼
AST
  │
  ▼
Semantic Analysis
  │
  ├───────────────────┐
  │                   │
  ▼                   ▼
AOT Backend        VM Backend
  │                   │
  ▼                   ▼
LLVM / Clang       Bytecode
  │                   │
  ▼                   ▼
Native Binary      VM Runtime
```

The separation between the language frontend and execution backends allows GAGE to support both native production builds and rapid development-time execution.

---

## 🎯 Designed For

GAGE is intended for workloads where native performance, predictable execution, and efficient numerical operations are important.

- 🎮 Game engines
- 🧮 Physics simulations
- 🎨 Graphics computation
- 🌌 Scientific simulations
- 📐 Vector mathematics
- ⚙️ Systems programming
- 🚀 Performance-sensitive applications
- 📱 Native ARM64 applications
- 🖥️ Low-latency workloads

---

## 🔧 Toolchain Philosophy

### Native First

Programs should be capable of becoming real native machine-code executables.

### Fast Iteration

The integrated VM provides a quick execution path during development.

### Predictable Compilation

Semantic and type validation happens before code generation.

### Minimal Runtime Overhead

Native builds do not depend on a heavyweight managed runtime.

### Portable Tooling

The same unified `gage` interface is designed to work across supported platforms.

---

## 📊 Execution Modes

| Feature | AOT | VM |
|---|:---:|:---:|
| Native machine code | ✅ | ❌ |
| LLVM / Clang backend | ✅ | ❌ |
| Fast startup | — | ✅ |
| External compiler required | Usually | ❌ |
| Standalone executable | ✅ | ❌ |
| Development testing | ✅ | ✅ |
| Native performance | ✅ | — |

---

## 🧭 Roadmap

The GAGE architecture is designed to evolve toward a broader native development ecosystem.

Potential development areas include:

- Expanded standard library
- More vector and matrix primitives
- Additional optimization passes
- Improved compiler diagnostics
- More complete language tooling
- Debugging support
- Editor integration
- Expanded platform support
- Advanced SIMD code generation
- Richer simulation primitives

---

## 🤝 Contributing

Contributions, experiments, bug reports, and language-design discussions are welcome.

Before submitting major changes, review the compiler architecture and language documentation in [`DOCS.txt`](DOCS.txt).

---

## 📄 License

GAGE is distributed under the terms of the **MIT License**.

See [`LICENSE`](LICENSE) for the complete license text.

Copyright © 2026 LossRun.

---

<div align="center">

<img src="logo.png" alt="GAGE Logo" width="120">

**Native performance. Vector-first design. Unified tooling.**

</div>
