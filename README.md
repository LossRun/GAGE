# ⚡ GAGE

<p align="center">
  <img src="logo.png" alt="GAGE Logo" width="220">
</p>

<h1 align="center">GAGE</h1>

<p align="center">
  <strong>A compact, native-oriented programming language built in Rust.</strong>
</p>

<p align="center">
  Expressive syntax · Native compilation · Bytecode execution · Vector mathematics · Simulation-oriented design
</p>

<p align="center">
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/language-Rust-orange?style=flat-square&logo=rust" alt="Rust">
  </a>
  <a href="https://github.com/LossRun/GAGE/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-blue?style=flat-square" alt="MIT License">
  </a>
  <a href="https://github.com/LossRun/GAGE">
    <img src="https://img.shields.io/badge/platform-cross--platform-55bdf6?style=flat-square" alt="Cross Platform">
  </a>
  <a href="https://github.com/LossRun/GAGE/tree/main/src">
    <img src="https://img.shields.io/badge/backend-C%2FClang%2FGCC-6b6bd4?style=flat-square" alt="Native Backend">
  </a>
</p>

<p align="center">
  <a href="DOCS.md">Documentation</a> ·
  <a href="examples/">Examples</a> ·
  <a href="src/">Source</a> ·
  <a href="LICENSE">License</a>
</p>

---

## What is GAGE?

**GAGE** is an experimental programming language and compiler toolchain written in Rust.

It is designed around a compact language core with features that make it particularly suitable for **native programs, simulations, mathematical workloads, and game-oriented scripting**.

Rather than depending on an existing language runtime, GAGE implements its own front end, semantic analysis, native code generation, bytecode compiler, and virtual machine.

The compiler accepts `.gage` source files and processes them through a custom language pipeline before executing them through one of two available execution paths:

- **Native execution** — GAGE generates C code and invokes an available native C compiler such as Clang or GCC.
- **Bytecode execution** — GAGE compiles the program into its own bytecode representation and executes it through the built-in VM.

This dual-engine design makes GAGE useful both as a native-oriented compiler experiment and as a compact language runtime.

---

## ✨ Highlights

### 🦀 Built from Scratch in Rust

The core compiler infrastructure is implemented directly in Rust.

The repository contains its own:

- Lexer
- Token system
- Parser
- Abstract syntax tree
- Type checker
- Native code generator
- Bytecode compiler
- Bytecode instruction set
- Virtual machine
- Command-line toolchain

There is no dependency on a large external language framework for the core compiler pipeline.

---

### ⚡ Native Compilation

GAGE can translate `.gage` source code into generated C and then compile that generated C into a native executable.

The native path is intentionally straightforward:

```text
GAGE Source
     │
     ▼
   Lexer
     │
     ▼
   Parser
     │
     ▼
 Type Checker
     │
     ▼
  C CodeGen
     │
     ▼
 Clang / GCC
     │
     ▼
Native Binary
```

The native backend uses C as an intermediate representation and attempts to use an available compiler such as:

- `clang`
- `gcc`

On Windows, it can also attempt:

- `clang-cl`

Native compilation is performed with optimization enabled by the toolchain.

---

### 🚀 Built-in Bytecode VM

GAGE also contains an independent bytecode execution path.

Instead of producing a native executable, the compiler can translate the parsed program into GAGE bytecode and execute that bytecode through the built-in virtual machine.

```text
GAGE Source
     │
     ▼
   Lexer
     │
     ▼
   Parser
     │
     ▼
 Type Checker
     │
     ▼
Bytecode Compiler
     │
     ▼
 Bytecode Chunk
     │
     ▼
     VM
     │
     ▼
 Program Output
```

The VM uses a stack-based execution model with global values and an instruction pointer.

This gives GAGE two different execution models without requiring two separate language syntaxes.

---

## 🧠 Language Design

GAGE keeps its language surface relatively compact while providing several features useful for simulation-oriented programs.

### Variables

Variables use `let` declarations and can subsequently be reassigned.

```gage
let speed = 10.0;
speed = speed + 5.0;

println(speed);
```

---

### Functions

Functions are declared with `fn` and support parameters and return statements.

```gage
fn square(x) {
    return x * x;
}

let result = square(8);
println(result);
```

Functions can also be used recursively.

---

### Classes and Objects

GAGE includes a small object-oriented layer with:

- `class`
- fields
- methods
- `new`
- `this`
- member access
- method calls

Example:

```gage
class Player {
    health;
    power;

    fn setup(hp, attack) {
        this.health = hp;
        this.power = attack;
    }

    fn damage(amount) {
        this.health = this.health - amount;
    }
}

let player = new Player();

player.setup(100, 25);
player.damage(20);

println(player.health);
```

Classes are represented directly by the native backend rather than being delegated to an external object system.

---

## 📐 Vector Mathematics

Vector types are part of the language itself.

GAGE currently provides:

- `vec2`
- `vec3`
- `vec4`

Vector expressions can participate directly in arithmetic operations.

```gage
let position = vec3(0.0, 10.0, 0.0);
let velocity = vec3(1.0, 0.0, -2.0);
let gravity = vec3(0.0, -9.8, 0.0);

let next_velocity = velocity + gravity;
let next_position = position + next_velocity;

println(next_position);
```

The native backend represents these vectors using compiler-supported vector types, allowing vector arithmetic to map naturally onto native operations.

---

## 🔢 Vector Operations

GAGE includes built-in mathematical operations for vectors:

```gage
let a = vec3(1.0, 0.0, 0.0);
let b = vec3(0.0, 1.0, 0.0);

let d = dot(a, b);
let c = cross(a, b);
let l = length(a);
let n = normalize(a);
```

Available operations include:

| Operation | Purpose |
|---|---|
| `dot(a, b)` | Dot product |
| `cross(a, b)` | Cross product |
| `length(v)` | Vector magnitude |
| `normalize(v)` | Normalize a vector |

The current implementation provides these operations primarily around `vec3` mathematical behavior.

For the complete language rules and implementation details, see [`DOCS.md`](DOCS.md).

---

## 📦 Arrays

GAGE supports array literals, indexing, mutation, and iteration.

```gage
let scores = [450, 1200, 890, 2400];

scores[0] = 500;

for score in scores {
    println(score);
}
```

The native backend represents arrays using a small dynamic runtime structure containing:

- allocated data
- current length
- capacity

The implementation can automatically grow the backing storage when values are appended.

---

## 🔁 Control Flow

GAGE provides familiar control-flow constructs:

- `if`
- `else`
- `while`
- `loop`
- `for ... in`
- `break`

Example:

```gage
let health = 75;

if (health > 50) {
    println("Healthy");
} else {
    println("Danger");
}
```

Loops can be used for normal program logic as well as simulation-style workloads.

---

## ⏱️ Simulation Steps

One of GAGE's more distinctive constructs is the `step(dt)` statement.

```gage
let position = vec3(0.0, 50.0, 0.0);
let velocity = vec3(0.0, -1.0, 0.0);

step(dt) {
    position = position + (velocity * dt);
}

println(position);
```

`dt` is introduced as the local time-step variable for the block.

The native backend currently uses a fixed simulation interval for this construct, making it convenient for small physics and simulation examples.

---

## 💻 Built-in I/O

GAGE provides simple terminal I/O without requiring users to manually access C or Rust APIs.

### Output

```gage
print("Hello ");
println("GAGE!");
```

### Input

```gage
let name = input("Enter your name: ");
println(name);
```

### File Reading

```gage
let contents = read_file("data.txt");
println(contents);
```

### File Writing

```gage
let success = write_file("data.txt", "Hello from GAGE!");

println(success);
```

The native backend provides these operations through its generated C runtime helpers.

---

## 🧩 Type System

GAGE includes a semantic checking stage between parsing and code generation.

The current type system includes:

- `Int`
- `Float`
- `Bool`
- `Str`
- `Vec2`
- `Vec3`
- `Vec4`
- Arrays
- Custom class types
- `Nil`
- `Any`

The compiler maintains scoped symbol information for variables, functions, and classes.

For example, using an undeclared variable is detected during semantic checking:

```text
[Semantic Error] Undefined variable 'name'
```

The type checker also understands the special `this` binding inside class methods.

Detailed type behavior and diagnostics are documented in [`DOCS.md`](DOCS.md).

---

# 🔧 Compiler Architecture

GAGE is organized as a traditional compiler pipeline followed by two execution backends.

```text
                     ┌─────────────────┐
                     │   GAGE Source   │
                     │    .gage file   │
                     └────────┬────────┘
                              │
                              ▼
                     ┌─────────────────┐
                     │      Lexer      │
                     │   Source → AST  │
                     └────────┬────────┘
                              │
                              ▼
                     ┌─────────────────┐
                     │     Parser      │
                     │   Tokens → AST  │
                     └────────┬────────┘
                              │
                              ▼
                     ┌─────────────────┐
                     │   TypeChecker   │
                     │ Semantic checks │
                     └────────┬────────┘
                              │
                    ┌─────────┴─────────┐
                    │                   │
                    ▼                   ▼
          ┌─────────────────┐   ┌─────────────────┐
          │    CodeGen      │   │    Compiler     │
          │    AST → C      │   │  AST → Bytecode │
          └────────┬────────┘   └────────┬────────┘
                   │                     │
                   ▼                     ▼
          ┌─────────────────┐   ┌─────────────────┐
          │   Clang / GCC   │   │   Bytecode VM   │
          └────────┬────────┘   └────────┬────────┘
                   │                     │
                   ▼                     ▼
             Native Binary          Program Output
```

### Front End

The front end is responsible for transforming source text into a validated AST.

```text
Source
  ↓
Lexer
  ↓
Tokens
  ↓
Parser
  ↓
AST
  ↓
Type Checker
```

The lexer tracks line and column information, recognizes language keywords, literals, operators, identifiers, and punctuation, and reports lexical errors with source locations.

The parser converts those tokens into the language's AST.

The type checker then validates declarations, scopes, expressions, classes, functions, and other semantic constructs.

---

## Native Backend

The native backend lives primarily in `src/codegen.rs`.

It transforms the AST into C source code.

Generated C contains runtime support for features such as:

- scalar output
- string output
- boolean output
- vector output
- vector construction
- vector mathematics
- dynamic arrays
- terminal input
- file reading
- file writing
- class representations

The generated C is then passed to an external C compiler.

GAGE searches for an available compiler rather than requiring a single hard-coded compiler implementation.

---

## Bytecode Backend

The bytecode backend is split across:

```text
src/bytecode.rs
src/compiler.rs
src/vm.rs
```

The compiler converts supported AST constructs into bytecode instructions stored in a `Chunk`.

The VM then executes those instructions using:

- an instruction pointer
- an evaluation stack
- global values
- constants
- runtime value operations

The VM has native handling for scalar values and vector values and performs runtime validation for invalid operations.

---

# 📁 Repository Structure

```text
GAGE/
├── docs/
│   └── ...
│
├── examples/
│   └── *.gage
│
├── src/
│   ├── ast.rs
│   ├── bytecode.rs
│   ├── codegen.rs
│   ├── compiler.rs
│   ├── lexer.rs
│   ├── main.rs
│   ├── parser.rs
│   ├── token.rs
│   ├── types.rs
│   └── vm.rs
│
├── stdlib/
│   └── ...
│
├── Cargo.toml
├── Cargo.lock
├── DOCS.md
├── LICENSE
├── README.md
├── fix_codegen.py
├── gage-setup.sh
├── install.ps1
├── logo.png
└── version.txt
```

### `src/`

The compiler and runtime implementation.

Important components include:

| File | Responsibility |
|---|---|
| `main.rs` | CLI, compilation pipeline, command dispatch |
| `token.rs` | Token definitions |
| `lexer.rs` | Source-code tokenization |
| `ast.rs` | Abstract syntax tree structures |
| `parser.rs` | Parsing and AST construction |
| `types.rs` | Semantic and type checking |
| `codegen.rs` | Native C code generation |
| `bytecode.rs` | Bytecode representation and values |
| `compiler.rs` | AST-to-bytecode compilation |
| `vm.rs` | Bytecode virtual machine |

### `examples/`

Contains GAGE programs demonstrating language features and experimentation.

### `docs/`

Repository documentation and supporting material.

### `stdlib/`

Standard-library-related project files.

### `DOCS.md`

The main language and toolchain documentation.

It contains the detailed material that intentionally does not belong in this README, including:

- installation instructions
- CLI reference
- language syntax
- primitive types
- vectors
- operators
- control flow
- simulation steps
- functions
- diagnostics
- platform-specific setup
- quick-start instructions

**For the complete language reference, read [`DOCS.md`](DOCS.md).**

---

# 🚀 Quick Start

## Requirements

To build GAGE from source, you need:

- Rust
- Cargo

For native compilation, GAGE also needs an available C compiler such as:

- Clang
- GCC

The exact installation instructions for Termux, Linux, macOS, and Windows are available in [`DOCS.md`](DOCS.md).

---

## Build from Source

Clone the repository:

```bash
git clone https://github.com/LossRun/GAGE.git
cd GAGE
```

Build the compiler:

```bash
cargo build --release
```

The resulting executable will be located at:

```text
target/release/gage
```

For the recommended installation procedure on your platform, see [`DOCS.md`](DOCS.md).

---

# ▶️ Running GAGE

GAGE provides several command-line operations.

### Native build

```bash
gage build program.gage -o program
```

This validates the source, generates C, invokes the native compiler, and produces a native executable.

### Native run

```bash
gage run program.gage
```

This performs the native compilation pipeline and executes the resulting temporary binary.

### Bytecode VM

```bash
gage vm program.gage
```

This runs the program through GAGE's internal bytecode compiler and virtual machine.

### Validation

```bash
gage check program.gage
```

This runs the lexer, parser, and semantic/type validation without producing a native executable.

### Inspect generated C

```bash
gage emit-c program.gage
```

This prints the generated C representation, making the native backend easier to inspect and debug.

### Version

```bash
gage --version
```

### Toolchain information

```bash
gage --info
```

### Help

```bash
gage --help
```

For the complete CLI reference, see [`DOCS.md`](DOCS.md).

---

# 🧪 Example Program

A small GAGE program can be as simple as:

```gage
let position = vec3(0.0, 10.0, 0.0);
let velocity = vec3(1.0, 0.0, 0.0);

step(dt) {
    position = position + (velocity * dt);
}

println(position);
```

The same source can be sent through either execution engine:

```text
             program.gage
                  │
          ┌───────┴───────┐
          │               │
          ▼               ▼
      Native AOT       Bytecode VM
          │               │
          ▼               ▼
      C compiler       GAGE VM
          │               │
          └───────┬───────┘
                  ▼
               Output
```

---

# 📚 Documentation

The README intentionally focuses on the project itself rather than reproducing the complete language manual.

### Main Documentation

**[`DOCS.md`](DOCS.md)**

Contains the detailed GAGE language and toolchain reference.

### Examples

**[`examples/`](examples/)**

Contains example GAGE programs.

### Source

**[`src/`](src/)**

Contains the compiler, code generator, bytecode compiler, and virtual machine.

### License

**[`LICENSE`](LICENSE)**

Contains the complete MIT License text.

---

# 🛠️ Development

GAGE is structured as a Rust Cargo project.

Build the project in debug mode:

```bash
cargo build
```

Build an optimized release:

```bash
cargo build --release
```

Run the compiler directly through Cargo:

```bash
cargo run -- --help
```

The project is intentionally small enough that the major compiler stages can be explored directly in `src/`.

---

# 🔍 Exploring the Compiler

If you want to understand how GAGE works internally, a useful reading order is:

```text
src/token.rs
     ↓
src/lexer.rs
     ↓
src/ast.rs
     ↓
src/parser.rs
     ↓
src/types.rs
     ↓
 ┌───┴───────────────┐
 ▼                   ▼
src/codegen.rs    src/compiler.rs
                     ↓
                src/bytecode.rs
                     ↓
                  src/vm.rs
```

This mirrors the actual architecture of the compiler.

The native path is especially useful for understanding how GAGE language constructs are lowered into C, while the bytecode path shows how the same language can be represented and executed by a custom virtual machine.

---

# 🎯 Design Goals

GAGE is built around a few simple ideas:

### 1. Keep the language compact

The language should provide useful programming constructs without requiring a huge syntax surface.

### 2. Stay close to native execution

Programs can ultimately be lowered to native code through the generated C backend.

### 3. Keep execution flexible

The same language can be executed through either native compilation or the internal VM.

### 4. Make mathematics a first-class concern

Vectors and common vector operations are built into the language rather than requiring a separate mathematics layer.

### 5. Make simulation code readable

Constructs such as `step(dt)`, vectors, loops, and mutable state are designed to make small simulation programs straightforward to express.

### 6. Keep the implementation understandable

The compiler is split into relatively focused Rust modules so that the language implementation can be studied, modified, and extended.

---

# 🌱 Project Status

GAGE is an experimental programming language and compiler project.

The repository is structured around language implementation rather than presenting itself as a mature production ecosystem.

That makes it suitable for:

- experimenting with programming-language design
- studying compiler construction
- exploring native code generation
- experimenting with bytecode VMs
- writing small simulations
- experimenting with vector mathematics
- building game-oriented prototypes
- extending the language itself

Language features and compiler behavior may evolve as the project develops.

---

# 🤝 Contributing

Contributions, experiments, bug reports, and language-design ideas are welcome.

If you want to contribute:

1. Fork the repository.
2. Create a branch for your change.
3. Make the change in the appropriate compiler or runtime component.
4. Test the affected functionality.
5. Add or update examples when appropriate.
6. Open a pull request describing the change.

For compiler changes, keeping the implementation separated between the front end, native backend, and VM helps maintain the project's architecture.

---

# 📄 License

GAGE is released under the **MIT License**.

See [`LICENSE`](LICENSE) for the complete license text.

---

<p align="center">
  <img src="logo.png" alt="GAGE" width="100">
</p>

<p align="center">
  <strong>GAGE</strong>
  <br>
  A compact language for native-oriented programming and simulation.
</p>

<p align="center">
  <a href="https://github.com/LossRun/GAGE">GitHub</a>
  ·
  <a href="DOCS.md">Documentation</a>
  ·
  <a href="examples/">Examples</a>
</p>