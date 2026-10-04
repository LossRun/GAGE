# GAGE

<div align="center">

<img src="logo.png" alt="GAGE Logo" width="200">

### High-performance, native-oriented programming language built in Rust.

<p>
  <b>Ahead-of-Time C CodeGen</b> · <b>SIMD Vectors</b> · <b>4×4 Transformation Pipelines</b> · <b>Bytecode VM</b> · <b>Simulation Runtime</b>
</p>

<p>
  <a href="https://github.com/LossRun/GAGE/actions">
    <img src="https://img.shields.io/badge/Tests-61%2F61%20passing-brightgreen?style=flat-square" alt="Test Suite">
  </a>
  <img src="https://img.shields.io/badge/Language-Rust-orange?style=flat-square&logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/Backend-C99%20%2B%20Clang-6b6b6b?style=flat-square" alt="C + Clang">
  <img src="https://img.shields.io/badge/Acceleration-Hardware%20SIMD-blueviolet?style=flat-square" alt="Hardware SIMD">
  <img src="https://img.shields.io/badge/License-MIT-blue?style=flat-square" alt="MIT License">
</p>

<p>
  <a href="https://github.codespaces.new/?hide_repo_select=true&ref=main&repo=LossRun/GAGE">
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

The compiler translates validated GAGE source programs through a complete front end consisting of lexical analysis, parsing, and semantic type checking. The resulting validated representation can then be lowered into either native C code for ahead-of-time compilation or instructions for the project's stack-based bytecode virtual machine.

The language is designed around native execution, high-throughput mathematical operations, physical simulation, coordinate transformations, vector mathematics, dynamic collections, and terminal-oriented rendering.

Rather than depending on a large external runtime or graphics framework, GAGE provides a compact language and compiler architecture where important mathematical and simulation primitives are part of the language/runtime itself.

---

## 🎯 Design Goals

GAGE is designed around several core ideas:

- Native-oriented execution.
- A compact and expressive language syntax.
- Direct lowering to C for native compilation.
- Hardware-accelerated vector operations where supported by the C backend.
- Built-in mathematical and transformation primitives.
- Simulation-oriented execution through `step(dt)`.
- A secondary bytecode execution path for experimentation and alternative execution.
- Lightweight objects and dynamic arrays.
- Terminal-based rendering and ANSI control.
- A small compiler architecture implemented in Rust.
- Clear separation between the compiler front end and execution backends.

The project is particularly suited to experimentation with programming-language implementation, native code generation, mathematical computation, simulation logic, and small interactive programs.

---

## 🏗️ Compiler Architecture

GAGE uses a shared compiler front end before selecting an execution backend.

```text
                         GAGE Source (.gage)
                                  │
                                  ▼
                         ┌─────────────────┐
                         │      Lexer      │
                         │                 │
                         │ Source → Tokens │
                         └────────┬────────┘
                                  │
                                  │ Tokens
                                  ▼
                         ┌─────────────────┐
                         │     Parser      │
                         │                 │
                         │ Tokens → AST    │
                         └────────┬────────┘
                                  │
                                  │ AST
                                  ▼
                         ┌─────────────────┐
                         │   Type Checker  │
                         │                 │
                         │ Semantic        │
                         │ Validation      │
                         └────────┬────────┘
                                  │
                                  │ Validated AST
                         ┌────────┴─────────┐
                         │                  │
                         ▼                  ▼
                ┌────────────────┐  ┌────────────────┐
                │  Native C Gen  │  │  Bytecode      │
                │                │  │  Compiler      │
                │  C99 + SIMD    │  │                │
                └───────┬────────┘  └───────┬────────┘
                        │                   │
                        │ C Source          │ Bytecode
                        ▼                   ▼
                ┌────────────────┐  ┌────────────────┐
                │     Clang      │  │   Stack VM     │
                │                │  │                │
                │ Native Binary  │  │   Execution    │
                └────────────────┘  └────────────────┘
```

Both execution paths share the same language front end.

The lexer converts source characters into tokens. The parser consumes those tokens and constructs the program's abstract syntax tree. The type checker then performs semantic validation before the validated representation is passed to one of the available execution backends.

This architecture allows the language frontend to remain independent from the final execution strategy.

---

## ⚡ Ahead-of-Time Native Code Generation

The primary execution path is the native C backend.

GAGE lowers its validated program representation into C99-compatible source code. The generated C code can then be compiled by Clang into native machine code.

This approach provides a relatively small compiler implementation while allowing GAGE programs to take advantage of the optimization capabilities of an established native compiler toolchain.

The native backend also provides the implementation used by the language's vector and mathematical facilities.

### Native Pipeline

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
Validated AST
    │
    ▼
C99 Code Generator
    │
    ▼
Generated C
    │
    ▼
Clang
    │
    ▼
Native Executable
```

---

## 🧮 SIMD Vector Mathematics

GAGE provides first-class vector primitives intended for mathematical computation, simulation, graphics-style calculations, and coordinate manipulation.

Supported vector types include:

- `vec2`
- `vec3`
- `vec4`

The native backend maps these vector representations to compiler-supported vector extensions where available.

This allows vector expressions to remain concise at the language level while being lowered toward hardware-oriented representations in generated native code.

### Vector Operations

The language provides mathematical operations including:

- `dot()`
- `cross()`
- `length()`
- `normalize()`
- `reflect()`

Example:

```gage
let position = vec3(0.0, 10.0, 0.0);
let velocity = vec3(1.0, -2.0, 0.5);

let speed = length(velocity);
let direction = normalize(velocity);

println(speed);
println(direction);
```

Vector arithmetic can be used directly inside simulation and transformation expressions.

```gage
let a = vec3(1.0, 2.0, 3.0);
let b = vec3(4.0, 5.0, 6.0);

let sum = a + b;
let difference = a - b;
let scaled = a * 2.0;
```

---

## 📐 4×4 Transformation Pipeline

GAGE includes built-in functionality for 4×4 transformation mathematics.

The transformation API is intended for coordinate manipulation, simulation, terminal graphics, and graphics-style programming.

Available transformation primitives include:

```text
mat4_identity()
mat4_translate()
mat4_scale()
mat4_rotate_y()
mat4_mul()
mat4_transform_vec3()
```

These operations allow programs to construct and combine transformation matrices and transform 3D positions.

A typical transformation pipeline can be represented conceptually as:

```text
Object Coordinates
        │
        ▼
Model Transformation
        │
        ▼
World Coordinates
        │
        ▼
Additional Transformation
        │
        ▼
Projected / Transformed Coordinates
```

---

## 📐 Mathematical Primitives

GAGE includes mathematical primitives intended for simulation and numerical programming.

Available functions include:

```text
sin()
cos()
tan()
sqrt()
pow()
floor()
ceil()
clamp()
lerp()
```

The language also provides global mathematical constants:

```text
PI
TAU
```

These primitives allow common calculations to remain inside GAGE programs without requiring an external mathematics library at the language level.

---

## 🔁 Simulation Runtime

Simulation is one of the central design targets of GAGE.

The language provides a dedicated `step(dt)` construct intended for deterministic delta-time based simulation updates.

A simulation can update state using the elapsed timestep:

```gage
let position = vec3(0.0, 50.0, 0.0);
let velocity = vec3(0.0, -9.8, 0.0);

step(dt) {
    position = position + (velocity * dt);
}
```

The `dt` value can be used to scale movement, acceleration, interpolation, and other time-dependent operations.

This makes the construct useful for physics-style calculations and simulation loops.

---

## 🧱 Classes and Objects

GAGE provides a lightweight object model with:

- Classes.
- Fields.
- Methods.
- `new` object construction.
- `this` instance access.

A class can group state and behavior into a reusable object.

Example:

```gage
class Player {
    health;
    speed;

    fn setup(hp, movement_speed) {
        this.health = hp;
        this.speed = movement_speed;
    }

    fn damage(amount) {
        this.health = this.health - amount;
    }
}

let player = new Player();

player.setup(100, 5.0);
player.damage(25);

println(player.health);
```

Objects can therefore be used to represent entities, simulation objects, game state, and other structured runtime data.

---

## 📦 Dynamic Arrays

GAGE provides dynamic arrays for storing collections of values.

Example:

```gage
let values = [10, 20, 30, 40];

println(values[0]);

values[1] = 99;
```

Arrays support indexed access and dynamic storage.

They can also be traversed using collection iteration:

```gage
let scores = [100, 250, 500, 1000];

for score in scores {
    println(score);
}
```

The runtime handles array bounds checking and dynamic reallocation.

---

## 🔄 Control Flow

GAGE provides conventional control-flow constructs for writing program logic.

Supported constructs include:

- `if`
- `else`
- `while`
- `loop`
- `break`
- `for ... in`

Example:

```gage
let score = 100;

if (score > 50) {
    println("High score");
} else {
    println("Low score");
}
```

A loop can be used for repeated execution:

```gage
let i = 0;

while (i < 10) {
    println(i);
    i = i + 1;
}
```

The `loop` construct can be terminated using `break`:

```gage
let i = 0;

loop {
    if (i >= 10) {
        break;
    }

    println(i);
    i = i + 1;
}
```

---

## 🧩 Functions

GAGE supports user-defined functions using the `fn` keyword.

Functions can accept parameters and return values.

Example:

```gage
fn add(a, b) {
    return a + b;
}

let result = add(10, 20);

println(result);
```

Functions can also call themselves recursively.

```gage
fn factorial(n) {
    if (n <= 1) {
        return 1;
    }

    return n * factorial(n - 1);
}
```

---

## 🖥️ Native Terminal Graphics and ANSI Support

GAGE contains terminal-oriented runtime functionality for interactive programs and simulation visualizations.

Terminal control primitives include:

```text
clear_screen()
gage_sleep()
```

The runtime also provides ANSI color controls including:

```text
color_cyan()
color_magenta()
color_yellow()
color_green()
color_reset()
```

These facilities make it possible to build lightweight terminal interfaces and visual simulations without requiring a large external graphics framework.

---

## 🧠 Bytecode Virtual Machine

In addition to the native C/Clang backend, GAGE contains a stack-based bytecode virtual machine.

The bytecode path provides an alternative execution model for development, experimentation, and runtime evaluation.

The general pipeline is:

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
Bytecode Chunks
    │
    ▼
Stack-Based VM
    │
    ▼
Program Execution
```

The VM and native compiler are separate execution backends that share the language's frontend architecture.

The native backend is intended for native compilation through generated C and Clang, while the VM provides a direct bytecode execution path.

---

## 🛠️ Installation & Building

### Prerequisites

GAGE is implemented in Rust and uses Cargo for building.

The native backend also requires a C compiler toolchain capable of compiling the generated C code.

Required development tools include:

- Rust and Cargo 1.70+
- Clang C compiler toolchain
- Git

### Clone the Repository

```bash
git clone https://github.com/LossRun/GAGE.git
cd GAGE
```

### Build a Development Version

```bash
cargo build
```

### Build an Optimized Release Version

```bash
cargo build --release
```

The optimized compiler binary will be generated at:

```text
target/release/gage
```

### Optional System-Wide Installation

On systems where `/usr/local/bin` is appropriate:

```bash
cp target/release/gage /usr/local/bin/gage
```

---

## 💻 CLI Usage

### Run a GAGE Program

The default execution path uses the native Clang AOT backend:

```bash
gage script.gage
```

### Run Through the Bytecode VM

```bash
gage --vm script.gage
```

### Measure Execution Time

```bash
gage --time script.gage
```

### Generate C Code

The native backend can expose the generated C representation:

```bash
gage emit-c script.gage -o output.c
```

This is useful when inspecting how a GAGE program is lowered into C.

The generated source can then be compiled independently with an appropriate C compiler toolchain.

### Start the Interactive REPL

```bash
gage
```

---

## 🧪 Test Suite

GAGE includes an automated regression test suite covering language behavior, mathematical functionality, vector operations, transformation pipelines, recursion, and complete example programs.

Run the complete test suite with:

```bash
python3 test_runner.py
```

The current project test summary is:

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

The repository therefore currently reports:

```text
61 / 61 tests passing
```

The test suite is intended to provide a regression layer across both individual language features and complete working programs.

---

## 📚 Examples

The `examples/` directory contains working GAGE programs demonstrating the language and runtime.

The examples cover areas such as:

- Basic language syntax.
- Functions.
- Recursion.
- Classes.
- Object state.
- Dynamic arrays.
- Vector mathematics.
- Physics calculations.
- Simulation loops.
- Matrix transformations.
- Terminal rendering.
- ANSI colors.
- Game-style logic.
- Coordinate transformations.
- 3D calculations.
- Integrated simulation programs.

A representative 3D example is:

```bash
./target/release/gage examples/51_rotating_cube_3d.gage
```

Examples are intended to serve both as demonstrations and as practical references for experimenting with the language.

---

## 🗂️ Repository Structure

```text
GAGE/
├── src/
│   ├── ast.rs
│   │   └── Abstract syntax tree nodes
│   │
│   ├── lexer.rs
│   │   └── Source tokenizer and lexical analysis
│   │
│   ├── parser.rs
│   │   └── Recursive descent parser
│   │
│   ├── types.rs
│   │   └── Semantic type checker and symbol tables
│   │
│   ├── codegen.rs
│   │   └── Native C99 and SIMD code generator
│   │
│   ├── bytecode.rs
│   │   └── Bytecode opcodes and instruction representations
│   │
│   ├── compiler.rs
│   │   └── Bytecode compiler and instruction emission
│   │
│   ├── vm.rs
│   │   └── Stack-based bytecode virtual machine
│   │
│   └── main.rs
│       └── Compiler driver and command-line pipeline
│
├── examples/
│   └── Working GAGE reference programs
│
├── tests_suite/
│   └── Core language and feature test specifications
│
├── test_runner.py
│   └── Automated regression test harness
│
├── logo.png
│   └── GAGE project logo
│
├── DOCS.md
│   └── Complete language grammar and API documentation
│
├── LICENSE
│   └── MIT License
│
└── README.md
    └── Project overview and high-level documentation
```

---

## 🔬 Compiler Front End

The compiler frontend is shared between GAGE's execution backends.

### Lexer

The lexer performs lexical analysis of GAGE source code.

It reads source characters and converts them into a stream of tokens that represent the meaningful lexical components of the program.

These tokens are then consumed by the parser.

### Parser

The parser uses recursive descent parsing to consume the token stream and construct the program's abstract syntax tree.

The AST provides a structured representation of the source program that can be analyzed and lowered by later compiler stages.

### Type Checker

The semantic type-checking stage validates the parsed program before code generation or bytecode compilation.

It is responsible for semantic validation and symbol-table handling required by the language implementation.

The validated program representation is then passed to the selected backend.

---

## ⚙️ Native Backend

The native backend is implemented in `src/codegen.rs`.

Its primary responsibility is translating validated GAGE program structures into C99-compatible source.

The generated C can use compiler-supported vector extensions for GAGE vector primitives.

The resulting C source is then suitable for compilation through Clang into a native executable.

Conceptually:

```text
GAGE AST
   │
   ▼
Native Code Generator
   │
   ▼
Generated C99
   │
   ▼
Clang
   │
   ▼
Native Machine Code
```

---

## ⚙️ Bytecode Backend

The bytecode execution path is implemented across the project's bytecode compiler and VM components.

The bytecode system contains:

```text
src/bytecode.rs
src/compiler.rs
src/vm.rs
```

The bytecode representation defines the instructions used by the VM.

The compiler translates validated program structures into bytecode instructions.

The VM then executes those instructions using a stack-based execution model.

This provides GAGE with an execution path that does not require the generated C source to be passed through Clang for every execution.

---

## 🧩 Backend Independence

One of the architectural characteristics of GAGE is the separation between language processing and execution.

The frontend performs:

```text
Source
  ↓
Lexing
  ↓
Parsing
  ↓
Semantic Validation
```

After that point, the execution strategy can diverge:

```text
                    Validated Program
                           │
                 ┌─────────┴─────────┐
                 │                   │
                 ▼                   ▼
          Native C Backend      Bytecode Backend
                 │                   │
                 ▼                   ▼
              Clang                  VM
                 │                   │
                 ▼                   ▼
          Native Execution      VM Execution
```

This allows the language implementation to experiment with different execution models without requiring a completely separate frontend for each backend.

---

## 📖 Language Documentation

The README intentionally provides a high-level overview of GAGE rather than duplicating the complete language specification.

For the complete language grammar, syntax reference, APIs, compiler details, runtime information, and additional technical documentation, see:

```text
DOCS.md
```

The documentation is kept separately so that the README can remain focused on introducing the project, its architecture, major capabilities, installation, and usage.

---

## 🚧 Project Status

GAGE is an experimental programming language and compiler project focused on native execution, mathematics, simulation, and language implementation.

The current repository includes:

- A Rust-based compiler frontend.
- Lexical analysis.
- Recursive descent parsing.
- Semantic type checking.
- Native C99 code generation.
- Clang-based native compilation.
- Hardware-oriented vector support.
- 4×4 transformation mathematics.
- Simulation-oriented `step(dt)` execution.
- Classes and objects.
- Dynamic arrays.
- A stack-based bytecode compiler.
- A bytecode virtual machine.
- Terminal graphics and ANSI functionality.
- More than 50 example programs.
- An automated regression suite.

GAGE should currently be considered a development and experimentation project rather than a production-ready replacement for established programming languages.

The architecture and language features may continue to evolve as the project develops.

---

## 📜 License

GAGE is released under the MIT License.

See the following file for the complete license text:

```text
LICENSE
```

---

## ⭐ Support the Project

If you find GAGE interesting, consider starring the repository and following its development.

```text
https://github.com/LossRun/GAGE
```

---

## ⚡ GAGE

A compact programming language built around native execution, mathematics, and simulation.