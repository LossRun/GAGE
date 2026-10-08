# GAGE

<div align="center">

<img src="logo.png" alt="GAGE Logo" width="200">

### A high-performance, native-oriented programming language built in Rust.

**AOT C CodeGen** · **SIMD Vectors** · **4×4 Transformations** · **Bytecode VM** · **Simulation Runtime**

<p>
  <img src="https://img.shields.io/badge/Language-Rust-orange?style=flat-square&logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/Backend-C99%20%2B%20Clang-6b6b6b?style=flat-square" alt="C99 + Clang">
  <img src="https://img.shields.io/badge/SIMD-Hardware%20Vector%20Support-blueviolet?style=flat-square" alt="Hardware SIMD">
  <img src="https://img.shields.io/badge/License-MIT-blue?style=flat-square" alt="MIT License">
</p>

</div>

---

## What is GAGE?

GAGE is a programming language built in Rust for native execution, mathematical computing, and simulation.

GAGE uses a compact high-level syntax while providing a native execution path through generated C99 code and Clang. It also includes a stack-based bytecode virtual machine for an alternative execution path.

The language is designed around vector mathematics, transformations, simulation logic, and lightweight runtime features.

## Why GAGE?

GAGE focuses on keeping the language small while making common mathematical and simulation operations part of the language itself.

The project combines:

- Native-oriented execution
- Ahead-of-Time C code generation
- First-class vector types
- Hardware-oriented SIMD support
- 4×4 transformation mathematics
- Simulation-oriented `step(dt)`
- Classes and objects
- Dynamic arrays
- A stack-based bytecode VM
- Interactive REPL
- Terminal and ANSI functionality
- Compilation caching

---

## Quick Example

A simple vector calculation in GAGE:

```gage
let position = vec3(0.0, 10.0, 0.0);
let velocity = vec3(1.0, -2.0, 0.5);

let speed = length(velocity);
let direction = normalize(velocity);

println(speed);
println(direction);
```

A simulation can be written directly using `step(dt)`:

```gage
let position = vec3(0.0, 50.0, 0.0);
let velocity = vec3(0.0, -9.8, 0.0);

step(dt) {
    position = position + (velocity * dt);
}
```

---

## Features

### Native Compilation

GAGE's primary execution path generates C99 source code and compiles it through Clang.

```text
GAGE Source
    ↓
Lexer
    ↓
Parser
    ↓
Type Checker
    ↓
C99 Code Generator
    ↓
Generated C
    ↓
Clang
    ↓
Native Executable
```

This keeps the GAGE compiler relatively compact while allowing generated programs to use an established native compiler toolchain.

### SIMD Vector Mathematics

GAGE provides first-class vector types:

```text
vec2
vec3
vec4
```

Vector operations include:

```text
dot()
cross()
length()
normalize()
reflect()
```

Example:

```gage
let a = vec3(1.0, 2.0, 3.0);
let b = vec3(4.0, 5.0, 6.0);

let sum = a + b;
let difference = a - b;
let scaled = a * 2.0;

let d = dot(a, b);
let n = normalize(a);
let len = length(a);
let r = cross(a, b);
```

The native backend can use compiler-supported vector extensions for hardware-oriented execution where supported.

### 4×4 Transformations

GAGE includes built-in 4×4 transformation operations for mathematical, simulation, and graphics-style workloads.

```text
mat4_identity()
mat4_translate()
mat4_scale()
mat4_rotate_y()
mat4_mul()
mat4_transform_vec3()
```

### Mathematical Functions

Common mathematical operations are available directly in GAGE:

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

Global constants:

```text
PI
TAU
```

### Simulation Runtime

Simulation is one of GAGE's main design targets.

The `step(dt)` construct provides a direct way to update simulation state using a delta-time value.

```gage
let position = vec3(0.0, 50.0, 0.0);
let velocity = vec3(0.0, -9.8, 0.0);

step(dt) {
    position = position + (velocity * dt);
}
```

### Classes and Objects

GAGE provides a lightweight object model with:

- Classes
- Fields
- Methods
- `new`
- `this`

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

### Dynamic Arrays

GAGE supports dynamic arrays with indexed access, bounds checking, and collection iteration.

```gage
let values = [10, 20, 30, 40];

println(values[0]);

values[1] = 99;
```

Arrays can also be iterated:

```gage
let scores = [100, 250, 500, 1000];

for score in scores {
    println(score);
}
```

### Bytecode Virtual Machine

GAGE includes a stack-based bytecode compiler and virtual machine.

```text
GAGE Source
    ↓
Lexer
    ↓
Parser
    ↓
Type Checker
    ↓
Bytecode Compiler
    ↓
Bytecode
    ↓
Stack VM
    ↓
Execution
```

The VM provides an alternative execution path for development, experimentation, and runtime evaluation.

### Terminal Runtime

GAGE includes lightweight terminal functionality for interactive programs and simulations.

Available functionality includes:

```text
clear_screen()
gage_sleep()

color_cyan()
color_magenta()
color_yellow()
color_green()
color_reset()
```

---

## Compiler Architecture

GAGE uses a shared compiler frontend for its execution backends.

```text
                    GAGE Source
                         │
                         ▼
                    ┌─────────┐
                    │  Lexer  │
                    └────┬────┘
                         │
                         ▼
                    ┌─────────┐
                    │ Parser  │
                    └────┬────┘
                         │
                         ▼
                  ┌──────────────┐
                  │ Type Checker │
                  └──────┬───────┘
                         │
                  Validated Program
                         │
              ┌──────────┴──────────┐
              │                     │
              ▼                     ▼
       ┌──────────────┐      ┌──────────────┐
       │  Native C    │      │   Bytecode   │
       │   Backend    │      │   Compiler   │
       └──────┬───────┘      └──────┬───────┘
              │                     │
              ▼                     ▼
          ┌───────┐             ┌────────┐
          │ Clang │             │ Stack  │
          │       │             │   VM   │
          └───┬───┘             └────┬───┘
              │                      │
              ▼                      ▼
       Native Execution        VM Execution
```

Both execution paths share the same frontend.

This keeps language processing separate from the final execution strategy.

---

## Compilation Cache

GAGE includes a compilation cache intended to reduce repeated native compilation overhead.

The cache uses a 64-bit FNV-1a hash derived from generated source and compilation information.

```text
GAGE Source
    ↓
Generated C
    ↓
FNV-1a Hash
    ↓
Cache Lookup
    │
    ├── Hit  → Reuse Binary
    │
    └── Miss → Compile with Clang
                    ↓
                Store Binary
```

When the relevant compilation inputs have not changed, GAGE can reuse the previously generated native binary.

---

## Installation

### Requirements

- Rust and Cargo 1.70+
- Clang
- Git

### Build from Source

```bash
git clone https://github.com/LossRun/GAGE.git
cd GAGE
cargo build --release
```

The release compiler will be available at:

```text
target/release/gage
```

### Linux

Optionally install the compiler system-wide:

```bash
cp target/release/gage /usr/local/bin/gage
```

### Termux

```bash
cp target/release/gage $PREFIX/bin/gage
chmod +x $PREFIX/bin/gage
```

---

## CLI Usage

### Run a Program

The default execution path uses the native C/Clang backend:

```bash
gage script.gage
```

### Use the Bytecode VM

```bash
gage --vm script.gage
```

### Measure Execution Time

```bash
gage --time script.gage
```

### Generate C Code

```bash
gage emit-c script.gage
```

Write the generated C code to a file:

```bash
gage emit-c script.gage -o output.c
```

### Start the REPL

```bash
gage
```

### Get Help

```bash
gage --help
```

### Get Version Info

```bash
gage --version
```

---

## Examples

The `examples/` directory contains working GAGE programs covering areas such as:

- Basic syntax
- Functions
- Recursion
- Classes
- Objects
- Dynamic arrays
- Vector mathematics
- Physics calculations
- Simulation loops
- Matrix transformations
- Terminal rendering
- ANSI colors
- Game-style logic
- Coordinate transformations
- 3D calculations

A 3D example is included with the project:

```bash
./target/release/gage examples/51_rotating_cube_3d.gage
```

The examples are intended to be both demonstrations and practical references for learning the language.

---

## Testing

GAGE includes an automated regression test suite covering language features, mathematical functionality, vectors, transformations, recursion, objects, simulations, and complete example programs.

Run the test suite with:

```bash
python3 test_runner.py
```

Current project test result:

```text
======================================================
      GAGE FULL TEST SUITE
======================================================
Phase 1: Feature & Math Primitives Units       [11/11 PASS]
Phase 2: Full Examples Suite (01 to 50)        [50/50 PASS]
------------------------------------------------------
  Executed: 61 | Passed: 61 | Failed: 0
======================================================
```

**61 / 61 tests passing**

---

## Project Structure

```text
GAGE/
├── src/
│   ├── ast.rs          # Abstract syntax tree
│   ├── lexer.rs        # Source tokenizer
│   ├── token.rs        # Token definitions
│   ├── parser.rs       # Recursive descent parser
│   ├── types.rs        # Type checker and symbols
│   ├── codegen.rs      # Native C99 / SIMD code generator
│   ├── bytecode.rs     # Bytecode definitions
│   ├── compiler.rs     # Bytecode compiler
│   ├── vm.rs           # Stack-based virtual machine
│   └── main.rs         # Compiler driver and CLI
│
├── examples/           # GAGE example programs
├── tests_suite/        # Language test specifications
├── test_runner.py      # Automated test runner
├── README.md           # Project overview
├── DOCS.md             # Detailed language documentation
├── Cargo.toml          # Rust project configuration
├── LICENSE             # MIT License
├── logo.png            # GAGE logo
└── version.txt         # Project version information
```

---

## Documentation

The README is intended to provide an overview, quick start, and introduction to GAGE.

For the complete language documentation, see:

**[DOCS.md](DOCS.md)**

The documentation covers:

- Language syntax
- Lexical structure
- Parser and AST
- Type system
- Variables and types
- Control flow
- Functions and recursion
- Classes and objects
- Dynamic arrays
- Vector operations
- `mat4` transformations
- Mathematical functions
- Terminal functionality
- Input and file I/O
- Simulation
- Native C generation
- Bytecode compilation
- Virtual machine execution
- Compilation caching
- Testing
- Compiler architecture

---

## Design Goals

GAGE is built around a few simple goals:

- Keep the language compact and expressive.
- Make native execution a core part of the language.
- Provide first-class mathematical and vector operations.
- Make simulation logic straightforward to express.
- Keep the compiler architecture small and understandable.
- Support both native and bytecode execution.
- Provide useful runtime functionality without requiring a large framework.

GAGE is primarily intended for experimentation with programming-language design, native code generation, mathematical computing, simulation, and interactive programs.

---

## Project Status

GAGE is an experimental and actively evolving programming language and compiler project.

The current implementation includes:

- Rust-based compiler frontend
- Lexical analysis
- Recursive descent parsing
- Semantic type checking
- Native C99 code generation
- Clang-based native compilation
- Vector and SIMD support
- 4×4 transformation mathematics
- Mathematical primitives
- Simulation-oriented `step(dt)`
- Classes and objects
- Dynamic arrays
- Terminal runtime functionality
- Bytecode compilation
- Stack-based virtual machine
- Compilation caching
- Interactive REPL
- Automated regression testing
- A growing collection of example programs

GAGE is not intended to replace established production languages at this stage. The language, compiler, runtime, and VM are still evolving.

---

## License

GAGE is released under the MIT License.

See [LICENSE](LICENSE) for the complete license text.

---

<div align="center">

### GAGE

**Native execution. Mathematics. Simulation.**

If you find the project interesting, consider giving it a ⭐ on GitHub.

</div>
