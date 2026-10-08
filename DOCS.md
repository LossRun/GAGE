# GAGE

<div align="center">

<img src="logo.png" alt="GAGE Logo" width="200">

### High-performance, native-oriented programming language built in Rust.

<p>
  <b>Ahead-of-Time C CodeGen</b> · <b>SIMD Vectors</b> · <b>4×4 Transformation Pipelines</b> · <b>Bytecode VM</b> · <b>Simulation Runtime</b>
</p>

<p>
  <a href="https://github.com/LossRun/GAGE/actions">
    <img src="https://img.shields.io/badge/Tests-123%2F123%20passing-brightgreen?style=flat-square" alt="Test Suite">
  </a>
  <img src="https://img.shields.io/badge/Language-Rust-orange?style=flat-square&logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/Backend-C99%20%2B%20Clang-6b6b6b?style=flat-square" alt="C + Clang">
  <img src="https://img.shields.io/badge/Acceleration-Hardware%20SIMD-blueviolet?style=flat-square" alt="Hardware SIMD">
  <img src="https://img.shields.io/badge/License-MIT-blue?style=flat-square" alt="MIT License">
</p>

<p>
  <a href="https://github.codespaces/new?hide_repo_select=true&ref=main&repo=LossRun/GAGE">
    <img src="https://github.com/codespaces/badge.svg" alt="Open in GitHub Codespaces">
  </a>
</p>

</div>

---

## ⚡ Quick Run

GAGE can be built and executed directly from a GitHub Codespace or from a local development environment.

### Interactive Cloud Sandbox

You can build and execute GAGE programs directly in your browser without configuring a local development environment.

1. Click the **Open in GitHub Codespaces** button above.
2. Wait for the development environment to initialize.
3. Build the compiler and run one of the included examples:

```bash
cargo build --release
./target/release/gage examples/51_rotating_cube_3d.gage
```

---

## 🧭 Overview

GAGE is an ahead-of-time compiled systems and simulation-oriented programming language implemented in Rust.

The compiler translates validated GAGE source programs through a complete front end consisting of lexical analysis, parsing, and semantic type checking before lowering the program into one of two execution paths:

- **Native C backend** — Generates C99 source optimized for native execution and compiles it through Clang.
- **Bytecode VM backend** — Compiles programs into GAGE bytecode and executes them through the built-in stack-based virtual machine.

The language is designed around high-throughput mathematical operations, physical simulation, coordinate transformation pipelines, vector mathematics, dynamic arrays, object-oriented constructs, and terminal-based rendering.

GAGE uses a unified compiler front end while allowing programs to reach either native machine execution or the internal virtual machine.

---

## ✨ Core Features

### ⚡ Ahead-of-Time Native Code Generation & SIMD

- Emits C99 source compiled to native machine code through Clang.
- Provides first-class `vec2`, `vec3`, and `vec4` vector primitives.
- Maps vector primitives to Clang hardware vector extensions.
- Provides native vector operations including:
  - `dot()`
  - `cross()`
  - `length()`
  - `normalize()`
  - `reflect()`
- Supports hardware-accelerated SIMD execution where supported by the target compiler and architecture.

### 📐 4×4 Transformation Pipeline

GAGE provides built-in matrix operations designed for 3D mathematics, simulation, coordinate systems, and terminal rendering.

Available operations include:

- `mat4_identity()`
- `mat4_translate()`
- `mat4_scale()`
- `mat4_rotate_y()`
- `mat4_mul()`
- `mat4_transform_vec3()`

Standard mathematical primitives are also available, including:

- `sin()`
- `cos()`
- `tan()`
- `sqrt()`
- `pow()`
- `floor()`
- `ceil()`
- `clamp()`
- `lerp()`

Global mathematical constants include:

- `PI`
- `TAU`

### 🔁 Simulation Runtime & `step(dt)`

GAGE includes a simulation-oriented `step(dt)` construct for continuously updating state using a deterministic delta-time value.

```gage
let position = vec3(0.0, 50.0, 0.0);
let velocity = vec3(0.0, -9.8, 0.0);

step(dt) {
    position = position + (velocity * dt);
}
```

### 🧱 Classes, Objects & Dynamic Memory

GAGE provides a lightweight object system supporting:

- Classes
- Fields
- Methods
- Constructors
- `new`
- `this`
- Dynamic arrays
- Bounds checking
- Dynamic capacity expansion
- `for ... in` collection iteration

### 🖥️ Native Terminal Graphics & ANSI Support

GAGE provides lightweight terminal control primitives intended for simulations, demonstrations, and terminal-based graphics.

Available functionality includes:

- `clear_screen()`
- `gage_sleep()`
- `color_cyan()`
- `color_magenta()`
- `color_yellow()`
- `color_green()`
- `color_reset()`

---

## 🛠️ Installation & Building

### Prerequisites

- Rust & Cargo `1.70+`
- Clang C compiler toolchain

### Build from Source

Clone the repository:

```bash
git clone https://github.com/LossRun/GAGE.git
cd GAGE
```

Build the optimized release binary:

```bash
cargo build --release
```

The resulting binary will be located at:

```text
target/release/gage
```

Optional system-wide installation:

```bash
cp target/release/gage /usr/local/bin/gage
```

On Termux:

```bash
cp target/release/gage $PREFIX/bin/gage
chmod +x $PREFIX/bin/gage
```

---

## 💻 CLI Usage

Run a GAGE program using the native Clang AOT backend:

```bash
gage script.gage
```

The native backend is the default execution path.

Run a program through the bytecode virtual machine:

```bash
gage --vm script.gage
```

Measure compiler and execution time:

```bash
gage --time script.gage
```

Generate and inspect transpiled C99 output:

```bash
gage emit-c script.gage
```

Write generated C99 output to a file:

```bash
gage emit-c script.gage -o output.c
```

Start the interactive REPL:

```bash
gage
```

Get Help

```bash
gage --help
```

Get Version Info

```bash
gage --version
```

Run GAGE Test

```bash
gage --test
```

---

## 🧪 Test Suite

GAGE includes an automated regression test suite covering compiler correctness, language semantics, mathematical primitives, matrix transformations, SIMD vector operations, recursion, object-oriented features, simulation primitives, file I/O, and complete example programs.

Run the test suite with:

```bash
python3 test_runner.py
```

Expected test summary:

```text
======================================================
      ⚡ GAGE FULL TEST SUITE (ADVANCED SIMULATION)
======================================================
Phase 1: Feature & Math Primitives Units       [11/11 PASS]
Phase 2: Full Examples Suite (01 to 50)        [50/50 PASS]
------------------------------------------------------
  Executed: 61 | Passed: 61 | Failed: 0
  Total Runtime: ~2.5 s (100% clean passes)
======================================================
```

---

## 📁 Repository Layout

```text
GAGE/
├── src/
│   ├── ast.rs          # Abstract syntax tree nodes
│   ├── lexer.rs        # Source tokenizer
│   ├── token.rs        # Token variants and punctuation
│   ├── parser.rs       # Recursive descent parser
│   ├── types.rs        # Semantic type checker and symbol tables
│   ├── codegen.rs      # Native C99 / SIMD code generator
│   ├── bytecode.rs     # Bytecode opcodes and instruction representations
│   ├── compiler.rs     # Bytecode emission
│   ├── vm.rs           # Stack-based virtual machine
│   └── main.rs         # Compiler driver, CLI, and compilation cache
├── examples/           # Working GAGE reference programs
├── tests_suite/        # Core language unit test specifications
├── test_runner.py      # Automated regression test harness
├── README.md           # Project overview and quick-start documentation
├── DOCS.md             # Complete language and compiler documentation
├── LICENSE             # MIT License
└── logo.png            # GAGE project logo
```

---

## 📚 Documentation

The README intentionally provides a high-level overview of GAGE rather than duplicating the complete language specification.

For the complete documentation covering the language syntax, compiler architecture, type system, runtime, standard functionality, bytecode VM, native backend, and development details, see:

**[DOCS.md](DOCS.md)**

The documentation includes detailed information about:

- Language syntax
- Lexical analysis
- Parsing
- Abstract syntax trees
- Semantic analysis
- Type checking
- Variables and primitive types
- Control flow
- Functions and recursion
- Classes and objects
- Dynamic arrays
- SIMD vectors
- `mat4` transformation pipelines
- Mathematical functions
- Terminal and ANSI functionality
- Input and file I/O
- Simulation steps
- Native C code generation
- Bytecode compilation
- Virtual machine execution
- Compilation caching
- Testing and diagnostics
- Repository architecture

---

## 🏗️ Compiler Architecture

```text
                    GAGE Source (.gage)
                            │
                            ▼
                      ┌───────────┐
                      │   Lexer   │
                      └─────┬─────┘
                            │ Tokens
                            ▼
                      ┌───────────┐
                      │  Parser   │
                      └─────┬─────┘
                            │ AST
                            ▼
                    ┌───────────────┐
                    │ Type Checker  │
                    └───────┬───────┘
                            │
                    Validated AST
                            │
              ┌─────────────┴─────────────┐
              │                           │
              ▼                           ▼
      ┌───────────────┐           ┌───────────────┐
      │ Native C Gen  │           │ Bytecode      │
      │   / SIMD      │           │ Compiler      │
      └───────┬───────┘           └───────┬───────┘
              │ C99                       │ Bytecode
              ▼                           ▼
       ┌─────────────┐             ┌─────────────┐
       │    Clang    │             │   Stack VM   │
       │ Native Code │             │  Execution   │
       └──────┬──────┘             └─────────────┘
              │
              ▼
       Native Execution
```

Both execution backends share the same compiler front end.

This allows lexical analysis, parsing, semantic validation, and language-level behavior to remain consistent regardless of whether a program is executed natively or through the GAGE virtual machine.

---

## 🚀 Native Backend

The native backend is the primary execution path for GAGE programs.

The pipeline is:

```text
GAGE Source
    ↓
Lexer
    ↓
Parser
    ↓
AST
    ↓
Semantic Type Checker
    ↓
C99 Code Generator
    ↓
Clang
    ↓
Native Executable
```

The generated C code uses GAGE's native runtime support for vectors, matrices, terminal operations, mathematical routines, dynamic arrays, objects, and simulation functionality.

---

## 🔢 Bytecode Virtual Machine

GAGE also provides a secondary stack-based bytecode execution backend.

The bytecode pipeline is:

```text
GAGE Source
    ↓
Lexer
    ↓
Parser
    ↓
AST
    ↓
Semantic Type Checker
    ↓
Bytecode Compiler
    ↓
Bytecode Chunks
    ↓
Stack-Based VM
    ↓
Program Execution
```

The bytecode backend is intended as an alternative execution path for development, experimentation, runtime testing, and future language evolution.

It shares the language front end with the native compiler but does not attempt to reproduce every optimization performed by the native C backend.

---

## ⚙️ Compilation Cache

GAGE includes a compilation cache designed to reduce repeated Clang invocation overhead.

The cache uses a fast 64-bit FNV-1a hash derived from generated source and compilation information.

Conceptually:

```text
GAGE Source
    ↓
Generated C99
    ↓
Hash Generated Input
    ↓
FNV-1a Hash
    ↓
Cache Lookup
    ├── Cache Hit  → Reuse Existing Binary
    │
    └── Cache Miss → Invoke Clang
                       ↓
                  Store Binary
```

When the generated program and relevant compilation inputs have not changed, GAGE can reuse the previously compiled native binary instead of invoking Clang again.

---

## 🧮 Language Highlights

GAGE programs can express mathematical and simulation workloads directly.

### Variables

```gage
let counter = 0;
let mass = 12.5;
let active = true;
let label = "Engine v1.0";

counter = counter + 1;
```

### Control Flow

```gage
if (health <= 0.0) {
    println("Entity destroyed");
} else if (health < 25.0) {
    println("Critical condition");
} else {
    println("Operational");
}
```

### Loops

```gage
let i = 0;

while (i < 100) {
    i = i + 1;
}
```

### Functions

```gage
fn fibonacci(n) {
    if (n <= 1) {
        return n;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}

println(fibonacci(10));
```

### Vectors

```gage
let u = vec3(1.0, 2.0, 3.0);
let v = vec3(4.0, 5.0, 6.0);

let sum = u + v;
let diff = u - v;
let scaled = u * 2.5;

let d = dot(u, v);
let n = normalize(u);
let len = length(u);
let r = cross(u, v);
```

### Simulation

```gage
let position = vec3(0.0, 100.0, 0.0);
let velocity = vec3(0.0, -9.8, 0.0);

step(dt) {
    position = position + (velocity * dt);
}

println(position);
```

---

## 🎯 Design Goals

GAGE is built around several core goals:

- Native-oriented execution
- High-performance numerical operations
- Simple and expressive language constructs
- First-class vector mathematics
- Simulation-oriented programming
- Native C interoperability through generated C99
- Hardware SIMD acceleration
- Lightweight runtime functionality
- Deterministic and predictable execution paths
- A compact compiler implementation written in Rust
- A secondary bytecode VM for experimentation and development

---

## 📌 Project Status

GAGE is an actively evolving programming language and compiler project.

The current implementation provides:

- Native C99 code generation
- Clang-based compilation
- Hardware SIMD vector primitives
- 4×4 transformation matrices
- Mathematical and trigonometric primitives
- Simulation-oriented `step(dt)`
- Classes and objects
- Dynamic arrays
- Terminal rendering utilities
- Input and file I/O
- Bytecode generation
- Stack-based virtual machine execution
- Compilation caching
- Interactive REPL
- Automated regression testing
- A growing collection of example programs

The project should be considered experimental while the language, compiler, runtime, and VM continue to evolve.

---

## 📜 License

GAGE is released under the MIT License.

See [LICENSE](LICENSE) for the complete license text.

---

<div align="center">

### GAGE

A compact programming language built around native execution, mathematics, and simulation.

⭐ If you find GAGE interesting, consider starring the repository and following its development.

</div>
