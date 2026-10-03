# 📘 GAGE Language & Toolchain Documentation

> Complete reference for the GAGE programming language, compiler pipeline, command-line interface, execution engines, language syntax, built-in functionality, and troubleshooting.

GAGE is a compact programming language and compiler toolchain implemented in Rust. It provides a custom lexer, parser, semantic/type-checking stage, native C code generation backend, bytecode compiler, and stack-based virtual machine.

This document describes the language and the current implementation.

---

## 📑 Table of Contents

- [1. Overview](#1-overview)
- [2. Architecture](#2-architecture)
- [3. Requirements](#3-requirements)
- [4. Installation](#4-installation)
  - [Android / Termux](#android--termux)
  - [Linux](#linux)
  - [macOS](#macos)
  - [Windows](#windows)
  - [Building from Source](#building-from-source)
- [5. Command-Line Interface](#5-command-line-interface)
- [6. Source Validation](#6-source-validation)
- [7. Native Compilation](#7-native-compilation)
- [8. Bytecode Virtual Machine](#8-bytecode-virtual-machine)
- [9. Language Fundamentals](#9-language-fundamentals)
  - [Comments](#comments)
  - [Statements](#statements)
  - [Identifiers](#identifiers)
  - [Variables](#variables)
  - [Assignments](#assignments)
- [10. Data Types](#10-data-types)
  - [Integer](#integer)
  - [Float](#float)
  - [Boolean](#boolean)
  - [String](#string)
  - [Nil](#nil)
  - [Vectors](#vectors)
  - [Arrays](#arrays)
  - [Classes](#classes)
- [11. Operators](#11-operators)
- [12. Expressions](#12-expressions)
- [13. Control Flow](#13-control-flow)
  - [`if` / `else`](#if--else)
  - [`while`](#while)
  - [`loop`](#loop)
  - [`for ... in`](#for--in)
  - [`break`](#break)
  - [`step(dt)`](#stepdt)
- [14. Functions](#14-functions)
- [15. Classes and Objects](#15-classes-and-objects)
  - [Class Fields](#class-fields)
  - [Methods](#methods)
  - [`this`](#this)
  - [`new`](#new)
  - [Member Access](#member-access)
- [16. Vector Mathematics](#16-vector-mathematics)
  - [`vec2`](#vec2)
  - [`vec3`](#vec3)
  - [`vec4`](#vec4)
  - [`dot`](#dot)
  - [`cross`](#cross)
  - [`length`](#length)
  - [`normalize`](#normalize)
- [17. Input and Output](#17-input-and-output)
  - [`print`](#print)
  - [`println`](#println)
  - [`input`](#input)
- [18. File Operations](#18-file-operations)
  - [`read_file`](#read_file)
  - [`write_file`](#write_file)
- [19. Expression Precedence](#19-expression-precedence)
- [20. Lexer](#20-lexer)
- [21. Parser](#21-parser)
- [22. Semantic and Type Checking](#22-semantic-and-type-checking)
- [23. Native Backend](#23-native-backend)
- [24. Bytecode Backend](#24-bytecode-backend)
- [25. Virtual Machine](#25-virtual-machine)
- [26. Runtime Behavior](#26-runtime-behavior)
- [27. Error Reference](#27-error-reference)
- [28. Debugging Generated C](#28-debugging-generated-c)
- [29. Project Structure](#29-project-structure)
- [30. Example Programs](#30-example-programs)
- [31. Development Guide](#31-development-guide)
- [32. Current Implementation Notes](#32-current-implementation-notes)
- [33. Troubleshooting](#33-troubleshooting)
- [34. License](#34-license)

---

# 1. Overview

GAGE is designed as a small, native-oriented programming language with a particular focus on:

- simple imperative programming
- mathematical expressions
- vector operations
- simulation-oriented code
- game-style state management
- object-oriented structures
- native compilation
- bytecode execution

A GAGE program uses the `.gage` file extension.

For example:

```gage
let message = "Hello from GAGE!";
println(message);
```

GAGE processes source code through the following stages:

```text
                ┌────────────────────┐
                │    .gage Source     │
                └──────────┬─────────┘
                           │
                           ▼
                ┌────────────────────┐
                │       Lexer        │
                │ Source → Tokens    │
                └──────────┬─────────┘
                           │
                           ▼
                ┌────────────────────┐
                │       Parser       │
                │ Tokens → AST       │
                └──────────┬─────────┘
                           │
                           ▼
                ┌────────────────────┐
                │   Type Checker     │
                │ Semantic Analysis  │
                └──────────┬─────────┘
                           │
                 ┌─────────┴─────────┐
                 │                   │
                 ▼                   ▼
        ┌─────────────────┐   ┌─────────────────┐
        │   Native Path   │   │  Bytecode Path  │
        └────────┬────────┘   └────────┬────────┘
                 │                     │
                 ▼                     ▼
        ┌─────────────────┐   ┌─────────────────┐
        │ Generated C     │   │ GAGE Bytecode   │
        └────────┬────────┘   └────────┬────────┘
                 │                     │
                 ▼                     ▼
        ┌─────────────────┐   ┌─────────────────┐
        │ Clang / GCC     │   │ GAGE VM         │
        └────────┬────────┘   └────────┬────────┘
                 │                     │
                 ▼                     ▼
           Native Binary            Execution
```

---

# 2. Architecture

The compiler is implemented as separate Rust modules.

The major components are:

```text
src/
├── token.rs
├── lexer.rs
├── ast.rs
├── parser.rs
├── types.rs
├── codegen.rs
├── bytecode.rs
├── compiler.rs
├── vm.rs
└── main.rs
```

Each module has a specific responsibility.

| Module | Purpose |
|---|---|
| `token.rs` | Token definitions |
| `lexer.rs` | Converts source text into tokens |
| `ast.rs` | Defines the abstract syntax tree |
| `parser.rs` | Converts tokens into AST nodes |
| `types.rs` | Semantic and type validation |
| `codegen.rs` | Generates native C code |
| `bytecode.rs` | Defines VM values, instructions, and chunks |
| `compiler.rs` | Converts AST into bytecode |
| `vm.rs` | Executes bytecode |
| `main.rs` | CLI and compiler orchestration |

The command-line driver first validates source code and then selects either the native or VM execution path.

---

# 3. Requirements

## Rust

GAGE is a Rust Cargo project.

You need:

```text
Rust
Cargo
```

To verify:

```bash
rustc --version
cargo --version
```

## Native C Compiler

Native execution requires a C compiler.

Supported compiler candidates are:

```text
clang
gcc
```

On Windows, GAGE additionally checks:

```text
clang-cl
```

The compiler is searched through the system `PATH`.

The bytecode VM does not require an external C compiler.

---

# 4. Installation

## Android / Termux

Install the required packages:

```bash
pkg update -y
pkg install git rust clang -y
```

Clone GAGE:

```bash
git clone https://github.com/LossRun/GAGE.git
cd GAGE
```

Run the repository setup script:

```bash
bash gage-setup.sh
```

Verify the installation:

```bash
gage --version
gage --info
gage --help
```

---

## Linux

### Ubuntu / Debian

Install the required tools:

```bash
sudo apt update
sudo apt install git curl clang build-essential -y
```

Clone GAGE:

```bash
git clone https://github.com/LossRun/GAGE.git
cd GAGE
```

Run:

```bash
bash gage-setup.sh
```

If the setup script requires elevated installation permissions:

```bash
sudo bash gage-setup.sh
```

---

### Arch Linux

Install:

```bash
sudo pacman -S git clang base-devel
```

Then:

```bash
git clone https://github.com/LossRun/GAGE.git
cd GAGE
bash gage-setup.sh
```

---

### Fedora

Install:

```bash
sudo dnf install git clang gcc
```

Then:

```bash
git clone https://github.com/LossRun/GAGE.git
cd GAGE
bash gage-setup.sh
```

---

## macOS

Install Apple's command-line development tools:

```bash
xcode-select --install
```

If Rust is not already installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone GAGE:

```bash
git clone https://github.com/LossRun/GAGE.git
cd GAGE
```

Run:

```bash
bash gage-setup.sh
```

---

## Windows

GAGE requires Rust and a usable C compiler.

Install Rust through `rustup` and install Clang through LLVM or the Visual Studio C++ toolchain.

Clone the repository:

```powershell
git clone https://github.com/LossRun/GAGE.git
cd GAGE
```

Run the installation script:

```powershell
powershell -ExecutionPolicy Bypass -File .\install.ps1
```

Verify:

```powershell
gage --version
gage --info
gage --help
```

---

## Building from Source

From the repository root:

```bash
cargo build
```

For an optimized build:

```bash
cargo build --release
```

The release executable is produced under:

```text
target/release/gage
```

On Windows:

```text
target/release/gage.exe
```

---

# 5. Command-Line Interface

The GAGE executable is called:

```text
gage
```

Running it without a command displays the usage information.

```bash
gage
```

---

## `gage build`

Compile a `.gage` source file into a native executable.

```bash
gage build program.gage
```

The output name is derived from the source filename.

For an explicit output name:

```bash
gage build program.gage -o program
```

The native build pipeline is:

```text
program.gage
     ↓
Lexer
     ↓
Parser
     ↓
Type Checker
     ↓
C Code Generation
     ↓
Clang / GCC
     ↓
program
```

---

## `gage run`

Compile and execute a GAGE program through the native backend.

```bash
gage run program.gage
```

The compiler creates a temporary executable, runs it, and removes the temporary binary afterward.

The native C compiler is therefore required for this command.

---

## `gage vm`

Execute a GAGE program through the built-in bytecode VM.

```bash
gage vm program.gage
```

The pipeline is:

```text
Source
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
VM
```

This path does not invoke Clang or GCC.

---

## `gage check`

Validate a source file without generating a native executable.

```bash
gage check program.gage
```

The command performs:

```text
Lexing
Parsing
Semantic validation
Type checking
```

A successful result reports that the source passed the validation stages.

---

## `gage emit-c`

Generate and display the intermediate C representation.

```bash
gage emit-c program.gage
```

This is useful when:

- debugging native code generation
- studying the compiler
- inspecting generated runtime code
- investigating native compilation errors

The generated C is printed to standard output.

You can save it to a file:

```bash
gage emit-c program.gage > generated.c
```

---

## `gage delete`

The `delete` command removes installed GAGE binaries and known GAGE installation directories.

```bash
gage delete
```

The command asks for confirmation before deleting anything.

Use:

```text
y
```

or:

```text
yes
```

to confirm.

Any other response aborts the operation.

---

## `gage --version`

Display the compiler version and target information:

```bash
gage --version
```

Short form:

```bash
gage -v
```

---

## `gage --info`

Display information about the execution engines and target platform:

```bash
gage --info
```

---

## `gage --help`

Display the CLI help:

```bash
gage --help
```

Short form:

```bash
gage -h
```

---

## Direct File Execution

A source file can also be supplied directly:

```bash
gage program.gage
```

When the first argument is an existing file rather than a recognized command, GAGE uses the native compilation path, executes the temporary binary, and removes it afterward.

---

# 6. Source Validation

Every native build begins with source validation.

The validation pipeline is:

```text
Source
  ↓
Lexer
  ↓
Parser
  ↓
Type Checker
  ↓
Validated AST
```

If any stage fails, native code generation does not continue.

For example:

```bash
gage check main.gage
```

can produce:

```text
✔ [Verified] 'main.gage' passed lexer, parser, and type checks.
```

---

# 7. Native Compilation

The native backend converts the GAGE AST into C source code.

The generated C is then compiled using the first suitable compiler found.

On non-Windows platforms, GAGE checks:

```text
clang
gcc
```

On Windows:

```text
clang-cl
clang
gcc
```

The compiler is invoked with optimization enabled.

The generated command uses:

```text
-O3
```

and suppresses compiler warnings using:

```text
-w
```

On non-Windows platforms, the math library is also linked with:

```text
-lm
```

---

## Native Build Flow

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
                    GAGE AST
                         │
                         ▼
                     CodeGen
                         │
                         ▼
                   Generated C
                         │
               ┌─────────┴─────────┐
               │                   │
             Clang                GCC
               │                   │
               └─────────┬─────────┘
                         ▼
                  Native Executable
```

---

# 8. Bytecode Virtual Machine

GAGE contains a separate bytecode execution engine.

The bytecode system consists primarily of:

```text
src/bytecode.rs
src/compiler.rs
src/vm.rs
```

The compiler creates a `Chunk`.

A chunk contains:

```text
instructions
constants
source line information
```

The VM executes the instruction stream using:

```text
instruction pointer
evaluation stack
global variable table
```

Conceptually:

```text
AST
 │
 ▼
Bytecode Compiler
 │
 ▼
Chunk
 ├── Code
 ├── Constants
 └── Line Information
 │
 ▼
Virtual Machine
 ├── Instruction Pointer
 ├── Stack
 └── Globals
```

---

# 9. Language Fundamentals

## Comments

GAGE supports single-line comments beginning with:

```text
//
```

Example:

```gage
// This is a comment.
let value = 42;
```

The lexer ignores everything from `//` until the end of the line.

---

## Statements

Most executable statements end with:

```text
;
```

Example:

```gage
let x = 10;
println(x);
```

Missing semicolons generally produce a parse error.

---

## Identifiers

Identifiers may contain:

- letters
- digits after the first character
- underscores

Examples:

```gage
player
player_health
velocity2
_position
```

An identifier cannot begin with a digit.

Reserved keywords cannot be used as ordinary identifiers.

---

## Reserved Keywords

The lexer recognizes the following language keywords and built-ins:

```text
let
fn
return

class
new
this

if
else
while
for
in
loop
break
step

true
false
nil

print
println
input
read_file
write_file

dot
cross
length
normalize

vec2
vec3
vec4
```

---

# 10. Data Types

The semantic type system defines the following types:

```text
Int
Float
Bool
Str
Vec2
Vec3
Vec4
Array
Custom
Nil
Any
```

---

## Integer

Integers are represented as signed 64-bit values.

Examples:

```gage
let count = 42;
let negative = -10;
let zero = 0;
```

---

## Float

Floating-point values use 64-bit floating-point representation.

Examples:

```gage
let pi = 3.14159;
let gravity = -9.8;
let dt = 0.016;
```

A decimal point followed by digits creates a floating-point literal.

---

## Boolean

Boolean values are:

```text
true
false
```

Example:

```gage
let running = true;

if (running) {
    println("Running");
}
```

---

## String

Strings use double quotes.

```gage
let name = "GAGE";
println(name);
```

Escape sequences supported by the lexer include:

```text
\n
\t
\\
\"
```

Example:

```gage
let message = "Hello\nGAGE";
println(message);
```

---

## Nil

`nil` represents the absence of a value.

```gage
let value = nil;
println(value);
```

---

## Vectors

GAGE provides three vector constructors:

```text
vec2
vec3
vec4
```

They are represented by dedicated vector types.

---

## Arrays

Array literals use square brackets:

```gage
let values = [10, 20, 30, 40];
```

Elements are separated by commas.

Arrays support indexing:

```gage
let first = values[0];
```

and assignment:

```gage
values[1] = 99;
```

Arrays can also be traversed with:

```gage
for value in values {
    println(value);
}
```

---

## Custom Types

Classes introduce custom types.

For example:

```gage
class Player {
    health;
}
```

An instance can be created using:

```gage
let player = new Player();
```

The semantic type is represented internally as a custom type associated with the class name.

---

# 11. Operators

## Arithmetic

GAGE supports:

| Operator | Meaning |
|---|---|
| `+` | Addition |
| `-` | Subtraction |
| `*` | Multiplication |
| `/` | Division |
| `%` | Modulo |

Examples:

```gage
let a = 20;
let b = 6;

println(a + b);
println(a - b);
println(a * b);
println(a / b);
println(a % b);
```

---

## Comparison

Supported comparison operators:

```text
==
!=
<
<=
>
>=
```

Example:

```gage
let health = 80;

if (health >= 50) {
    println("Healthy");
}
```

---

## Logical Operators

GAGE supports:

```text
&&
||
!
```

Example:

```gage
let alive = true;
let active = true;

if (alive && active) {
    println("Entity is active");
}
```

---

## Unary Operators

Unary negation:

```text
-
```

Logical negation:

```text
!
```

Example:

```gage
let value = -10;
let active = !false;
```

---

# 12. Expressions

Expressions can contain:

- literals
- identifiers
- arrays
- vectors
- function calls
- method calls
- member access
- indexing
- object construction
- unary operations
- binary operations
- built-in mathematical operations

Example:

```gage
let result = (10 + 5) * 2;
```

Nested expressions are supported:

```gage
let distance = length(vec3(3.0, 4.0, 0.0));
```

---

## Function Calls

```gage
let result = calculate(10, 20);
```

---

## Method Calls

```gage
player.attack(enemy);
```

---

## Member Access

```gage
player.health
```

---

## Array Indexing

```gage
scores[0]
```

---

## Object Construction

```gage
let player = new Player();
```

Constructor arguments are syntactically accepted:

```gage
let object = new Example(value);
```

---

# 13. Control Flow

## `if` / `else`

The current parser requires the condition to be enclosed in parentheses.

```gage
if (health > 50) {
    println("Healthy");
} else if (health > 0) {
    println("Danger");
} else {
    println("Dead");
}
```

Structure:

```text
if (condition) {
    statements
}
```

Optional `else`:

```text
if (condition) {
    statements
} else {
    statements
}
```

---

## `while`

A `while` loop repeats while its condition evaluates as true.

```gage
let i = 0;

while (i < 10) {
    println(i);
    i = i + 1;
}
```

Syntax:

```text
while (condition) {
    statements
}
```

---

## `loop`

`loop` creates an unconditional loop.

```gage
let counter = 0;

loop {
    counter = counter + 1;

    if (counter >= 10) {
        break;
    }
}
```

Syntax:

```text
loop {
    statements
}
```

---

## `for ... in`

GAGE provides an iterator-style loop:

```gage
let values = [10, 20, 30];

for value in values {
    println(value);
}
```

Syntax:

```text
for variable in expression {
    statements
}
```

The loop variable is scoped to the loop body during semantic checking.

---

## `break`

`break` exits the current loop in the native execution model.

Syntax:

```gage
break;
```

Example:

```gage
let i = 0;

loop {
    i = i + 1;

    if (i >= 5) {
        break;
    }
}
```

---

## `step(dt)`

`step` is a simulation-oriented construct.

Syntax:

```text
step(identifier) {
    statements
}
```

Example:

```gage
let position = vec3(0.0, 10.0, 0.0);
let velocity = vec3(1.0, 0.0, 0.0);

step(dt) {
    position = position + (velocity * dt);
}
```

The identifier inside `step(...)` becomes a local floating-point time-step variable.

The native implementation uses a fixed timestep value.

The bytecode compiler currently lowers the timestep to approximately:

```text
0.016667
```

which corresponds to roughly 60 simulation steps per second.

---

# 14. Functions

Functions are declared using:

```text
fn
```

Basic syntax:

```gage
fn add(a, b) {
    return a + b;
}
```

Call the function:

```gage
let result = add(10, 20);
println(result);
```

---

## Function Parameters

Multiple parameters are separated by commas:

```gage
fn calculate_damage(power, defense, multiplier) {
    return (power - defense) * multiplier;
}
```

---

## Return Values

Use:

```text
return
```

Example:

```gage
fn square(x) {
    return x * x;
}
```

A return statement may also omit an expression:

```gage
fn reset() {
    return;
}
```

---

## Recursion

Functions can call other functions, including themselves.

Example:

```gage
fn factorial(n) {
    if (n <= 1) {
        return 1;
    }

    return n * factorial(n - 1);
}

println(factorial(5));
```

---

## Function Scope

Function parameters are placed into a new semantic scope.

Local names can therefore be used inside function bodies without being confused with the outer scope.

---

# 15. Classes and Objects

Classes are declared using:

```text
class
```

Example:

```gage
class Player {
    health;
    power;

    fn setup(hp, attack) {
        this.health = hp;
        this.power = attack;
    }

    fn attack(target) {
        return target - this.power;
    }
}
```

---

## Class Fields

Fields are declared inside the class body.

Each field declaration ends with a semicolon:

```gage
class Player {
    health;
    power;
    speed;
}
```

---

## Methods

Methods are functions declared inside a class.

```gage
class Player {
    health;

    fn heal(amount) {
        this.health = this.health + amount;
    }
}
```

Methods use the same function syntax as normal functions.

---

## `this`

Inside a class method, `this` refers to the current object.

Example:

```gage
class Player {
    health;

    fn set_health(value) {
        this.health = value;
    }
}
```

The semantic checker introduces `this` into the method's scope as the class type.

Using `this` outside a class method produces a semantic error.

---

## `new`

Objects are created with:

```text
new
```

Example:

```gage
class Player {
    health;
}

let player = new Player();
```

Constructor-style arguments are accepted by the language grammar:

```gage
let player = new Player(100);
```

---

## Member Access

Fields can be accessed using `.`:

```gage
player.health
```

Methods can be called using the same member-access syntax:

```gage
player.heal(25);
```

---

## Member Assignment

Fields can be assigned:

```gage
player.health = 100;
```

The parser represents this as a member assignment rather than a normal variable assignment.

---

# 16. Vector Mathematics

Vectors are built into the language rather than being implemented as ordinary user-defined classes.

GAGE provides:

```text
vec2
vec3
vec4
```

---

## `vec2`

Create a two-dimensional vector:

```gage
let position = vec2(10.0, 20.0);
```

The components are:

```text
x
y
```

---

## `vec3`

Create a three-dimensional vector:

```gage
let position = vec3(10.0, 20.0, 30.0);
```

Components:

```text
x
y
z
```

Example:

```gage
let velocity = vec3(1.0, 0.0, -2.0);
```

---

## `vec4`

Create a four-dimensional vector:

```gage
let color = vec4(1.0, 0.0, 0.5, 1.0);
```

Components:

```text
x
y
z
w
```

---

## Vector Addition

Vectors of the same dimension can be added:

```gage
let a = vec3(1.0, 2.0, 3.0);
let b = vec3(4.0, 5.0, 6.0);

let result = a + b;
```

---

## Vector Subtraction

```gage
let a = vec3(5.0, 6.0, 7.0);
let b = vec3(1.0, 2.0, 3.0);

let result = a - b;
```

---

## Scalar Multiplication

Vector values can be multiplied by scalar numeric values in the native runtime.

```gage
let velocity = vec3(1.0, 2.0, 3.0);
let scaled = velocity * 2.0;
```

This is particularly useful for simulation code:

```gage
let position = position + velocity * dt;
```

---

## `dot`

Calculate the dot product:

```gage
let a = vec3(1.0, 0.0, 0.0);
let b = vec3(0.0, 1.0, 0.0);

let result = dot(a, b);
println(result);
```

The result is a floating-point value.

---

## `cross`

Calculate the cross product:

```gage
let a = vec3(1.0, 0.0, 0.0);
let b = vec3(0.0, 1.0, 0.0);

let result = cross(a, b);
println(result);
```

The semantic type of the result is `Vec3`.

---

## `length`

Calculate a vector's magnitude:

```gage
let velocity = vec3(3.0, 4.0, 0.0);

let speed = length(velocity);

println(speed);
```

The result is a floating-point value.

---

## `normalize`

Normalize a vector:

```gage
let direction = vec3(3.0, 4.0, 0.0);
let unit = normalize(direction);

println(unit);
```

The result retains the vector's type.

---

## Vector Simulation Example

```gage
let position = vec3(0.0, 20.0, 0.0);
let velocity = vec3(2.0, 0.0, 0.0);
let gravity = vec3(0.0, -9.8, 0.0);

step(dt) {
    velocity = velocity + gravity * dt;
    position = position + velocity * dt;
}

println(position);
```

---

# 17. Input and Output

## `print`

`print` writes an expression without automatically adding a newline.

```gage
print("Loading");
```

Example:

```gage
print("Hello ");
print("GAGE");
```

---

## `println`

`println` writes an expression followed by a newline.

```gage
println("Hello from GAGE!");
```

It can print values such as:

```gage
println(42);
println(3.14);
println(true);
println("text");
println(vec3(1.0, 2.0, 3.0));
```

---

## `input`

`input` reads a line from standard input.

With a prompt:

```gage
let name = input("Enter your name: ");
println(name);
```

The prompt expression is optional at the parser level.

---

# 18. File Operations

GAGE provides two file-related built-ins.

---

## `read_file`

Read the contents of a file:

```gage
let contents = read_file("data.txt");
println(contents);
```

Syntax:

```text
read_file(path)
```

---

## `write_file`

Write content to a file:

```gage
let success = write_file("data.txt", "Hello from GAGE!");
println(success);
```

Syntax:

```text
write_file(path, content)
```

The native backend exposes these operations through its generated runtime.

---

# 19. Expression Precedence

GAGE's parser evaluates expressions using a conventional precedence hierarchy.

From lowest precedence to highest:

| Level | Operators |
|---|---|
| 1 | `||` |
| 2 | `&&` |
| 3 | `==`, `!=` |
| 4 | `<`, `<=`, `>`, `>=` |
| 5 | `+`, `-` |
| 6 | `*`, `/`, `%` |
| 7 | unary `-`, `!` |
| 8 | function calls, method calls, member access, indexing |
| 9 | primary expressions |

Therefore:

```gage
let result = 2 + 3 * 4;
```

is interpreted as:

```text
2 + (3 * 4)
```

Parentheses can be used to explicitly control grouping:

```gage
let result = (2 + 3) * 4;
```

---

# 20. Lexer

The lexer is implemented in:

```text
src/lexer.rs
```

Its job is to convert source characters into tokens.

Example:

```gage
let value = 42;
```

Conceptually becomes:

```text
LET
IDENT(value)
ASSIGN
INT(42)
SEMICOLON
EOF
```

The lexer records:

```text
line
column
```

for tokens and lexical errors.

---

## Whitespace

The lexer ignores:

```text
space
tab
carriage return
newline
```

---

## Comments

The lexer recognizes:

```text
// comment
```

and ignores the comment until the newline.

---

## Numeric Literals

Integer:

```text
42
```

Floating-point:

```text
42.5
```

The lexer recognizes a decimal point only when it is followed by another digit.

---

## Strings

Strings begin and end with:

```text
"
```

Supported escapes include:

```text
\n
\t
\\
\"
```

An unterminated string produces:

```text
[Lexer Error]
```

---

# 21. Parser

The parser is implemented in:

```text
src/parser.rs
```

It consumes the token stream and constructs the abstract syntax tree.

The parser supports statements including:

```text
let
fn
class
return
print
println
if
while
for
loop
break
step
assignment
expression statements
```

Expressions include:

```text
literals
identifiers
arrays
new expressions
vectors
built-in math operations
function calls
method calls
member access
index access
unary expressions
binary expressions
```

---

## AST

The AST is defined in:

```text
src/ast.rs
```

The AST provides the intermediate representation shared by the semantic checker and the native/bytecode backends.

---

# 22. Semantic and Type Checking

Semantic validation is implemented in:

```text
src/types.rs
```

The type checker maintains:

```text
scopes
functions
classes
```

The checker validates the program after parsing and before native code generation.

---

## Variable Lookup

Variables are resolved through nested scopes.

For example:

```gage
let global_value = 10;

fn test(value) {
    println(value);
}
```

Function parameters are placed into the function scope.

---

## Undefined Variables

Using an unknown identifier:

```gage
println(unknown_value);
```

produces a semantic error similar to:

```text
[Semantic Error] Undefined variable 'unknown_value'
```

---

## `this` Validation

Using:

```gage
this
```

outside a class method produces a semantic error.

---

## Function Scopes

Function parameters are inserted into a new scope:

```gage
fn add(a, b) {
    return a + b;
}
```

Inside the function:

```text
a
b
```

are available as local names.

---

## Class Method Scopes

Class methods receive:

```text
this
```

as well as their declared parameters.

Example:

```gage
class Player {
    health;

    fn set(value) {
        this.health = value;
    }
}
```

---

## Type Conversion

The current semantic checker allows an integer value to be assigned to a floating-point variable.

Conceptually:

```text
Int → Float
```

is accepted in the relevant reassignment case.

---

# 23. Native Backend

The native backend is implemented primarily in:

```text
src/codegen.rs
```

It transforms the validated AST into C source code.

The generated C contains runtime structures and helper functions required by GAGE programs.

The backend handles language features including:

```text
primitive values
strings
vectors
arrays
classes
functions
control flow
I/O
file operations
```

The generated C is then passed to an external compiler.

---

## C Compiler Selection

On Linux, macOS, Android, and other non-Windows targets:

```text
1. clang
2. gcc
```

On Windows:

```text
1. clang-cl
2. clang
3. gcc
```

The first compiler that successfully produces an executable is used.

---

# 24. Bytecode Backend

The bytecode representation is defined in:

```text
src/bytecode.rs
```

The compiler is implemented in:

```text
src/compiler.rs
```

The runtime VM is implemented in:

```text
src/vm.rs
```

---

## Bytecode Values

The VM currently represents values including:

```text
Nil
Bool
Int
Float
Str
Vec2
Vec3
Vec4
```

---

## Bytecode Instructions

The bytecode instruction set contains categories for:

### Constants

```text
Constant
Nil
True
False
```

### Vectors

```text
MakeVec2
MakeVec3
MakeVec4
```

### Arithmetic and Comparison

```text
Add
Sub
Mul
Div
Negate
Not
Equal
NotEqual
Less
LessEqual
Greater
GreaterEqual
```

### Variables and Stack

```text
Pop
DefineGlobal
GetGlobal
SetGlobal
GetLocal
SetLocal
```

### Control Flow

```text
Jump
JumpIfFalse
Loop
```

### I/O

```text
Print
Println
```

### VM Control

```text
Return
Halt
```

---

# 25. Virtual Machine

The VM uses:

```text
Instruction Pointer
Evaluation Stack
Global Variable Table
```

The evaluation stack has an initial capacity and is used to perform expression evaluation.

For example:

```text
1 + 2
```

is conceptually executed as:

```text
push 1
push 2
add
```

resulting in:

```text
3
```

---

## VM Truthiness

The VM defines truthiness for runtime values.

The following values are false:

```text
nil
false
integer 0
float 0.0
empty string
```

Non-zero numeric values and non-empty strings are true.

Vector values are treated as truthy.

---

## VM Arithmetic

The VM supports numeric arithmetic and vector arithmetic for compatible values.

For example:

```text
Int + Int
Float + Float
Int + Float
Float + Int
```

are supported by the runtime.

Vector addition is supported for matching vector dimensions.

---

## VM Errors

Invalid runtime operations produce a VM runtime error.

For example, incompatible operands for an operator can produce:

```text
[Runtime Error] Invalid operands for '+'
```

---

# 26. Runtime Behavior

GAGE has two execution models and they should not be assumed to behave identically for every feature.

## Native Backend

The native backend is the more complete execution path and lowers the GAGE AST into generated C.

Use:

```bash
gage build program.gage
```

or:

```bash
gage run program.gage
```

when testing native behavior.

---

## VM Backend

The VM executes GAGE bytecode directly:

```bash
gage vm program.gage
```

The current VM implementation is smaller than the native backend.

When adding or testing a new language feature, verify that the feature is implemented in the desired backend.

---

# 27. Error Reference

GAGE errors are produced by multiple compiler stages.

---

## Lexer Errors

Example:

```text
[Lexer Error] Line 3:12 -> Unexpected character: '@'
```

### Common causes

- unsupported characters
- invalid punctuation
- unterminated strings

### Example

Invalid:

```gage
let value = @10;
```

---

## Unterminated String

Invalid:

```gage
let message = "Hello;
```

The lexer reports:

```text
Unterminated string literal
```

---

## Parse Errors

Example:

```text
[Parse Error] Line 4:10 -> Expected ';' after expression
```

Common causes:

- missing semicolon
- missing closing parenthesis
- missing brace
- malformed function declaration
- malformed class declaration
- invalid assignment target

---

## Missing Semicolon

Invalid:

```gage
let value = 10
println(value);
```

Correct:

```gage
let value = 10;
println(value);
```

---

## Semantic Errors

Example:

```text
[Semantic Error] Undefined variable 'player'
```

Cause:

```gage
println(player);
```

without declaring `player`.

Correct:

```gage
let player = 10;
println(player);
```

---

## `this` Outside a Class

Invalid:

```gage
println(this);
```

The semantic checker reports that `this` is being used outside a class method.

---

## Native Compilation Errors

If the generated C cannot be compiled, GAGE reports:

```text
Native compilation failed.
```

The first compiler failure is retained as diagnostic information.

Ensure that one of the following is installed and available through `PATH`:

```text
clang
gcc
```

On Windows, also consider:

```text
clang-cl
```

---

# 28. Debugging Generated C

The `emit-c` command is the primary tool for inspecting the native backend.

Run:

```bash
gage emit-c program.gage
```

Or save the result:

```bash
gage emit-c program.gage > generated.c
```

Then inspect:

```text
generated.c
```

This is useful when investigating:

- compiler errors
- type lowering
- vector operations
- generated runtime code
- class representations
- array handling
- native I/O
- generated control flow

---

# 29. Project Structure

The repository is organized approximately as follows:

```text
GAGE/
│
├── docs/
│
├── examples/
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

---

## `src/main.rs`

The command-line driver.

Responsible for:

- reading command-line arguments
- displaying help
- loading source files
- invoking validation
- invoking native code generation
- invoking the C compiler
- invoking the VM
- temporary executable management
- installation removal

---

## `src/token.rs`

Defines the token representation used by the lexer and parser.

---

## `src/lexer.rs`

Converts raw source text into tokens.

---

## `src/ast.rs`

Defines the abstract syntax tree used throughout the compiler.

---

## `src/parser.rs`

Converts tokens into the AST.

---

## `src/types.rs`

Performs semantic validation and tracks:

```text
variables
scopes
functions
classes
types
```

---

## `src/codegen.rs`

Generates C source for the native backend.

---

## `src/bytecode.rs`

Defines:

```text
Value
OpCode
Chunk
```

for the bytecode system.

---

## `src/compiler.rs`

Converts AST nodes into bytecode instructions.

---

## `src/vm.rs`

Executes bytecode.

---

## `examples/`

Contains example `.gage` programs covering language features and experimentation.

---

## `stdlib/`

Contains standard-library-related project files.

---

# 30. Example Programs

The repository contains example programs covering progressively more advanced features.

The examples include concepts such as:

```text
Hello World
variables
arithmetic
conditionals
while loops
loop statements
step blocks
functions
recursion
classes
object state
combat logic
vector mathematics
physics
arrays
inventory systems
interactive input
file persistence
raycasting calculations
orbit simulation
game loops
camera movement
projectile motion
logging
surface lighting
integrated gameplay examples
```

A useful way to learn GAGE is to start with the smallest examples and then move toward programs combining multiple language features.

---

# 31. Development Guide

GAGE is a Cargo project.

Build in debug mode:

```bash
cargo build
```

Build an optimized version:

```bash
cargo build --release
```

Run the compiler through Cargo:

```bash
cargo run -- --help
```

Run a source file through the native path:

```bash
cargo run -- run examples/example.gage
```

Run through the VM:

```bash
cargo run -- vm examples/example.gage
```

Check source:

```bash
cargo run -- check examples/example.gage
```

Inspect generated C:

```bash
cargo run -- emit-c examples/example.gage
```

---

## Recommended Development Flow

When modifying the language:

```text
1. Modify token definitions if needed
            ↓
2. Update lexer
            ↓
3. Update AST
            ↓
4. Update parser
            ↓
5. Update type checker
            ↓
6. Update native code generation
            ↓
7. Update bytecode compiler if supported
            ↓
8. Update VM if supported
            ↓
9. Add/update examples
            ↓
10. Run validation
```

For a new syntax feature, the parser and AST are normally the central points where the feature enters the compiler.

---

# 32. Current Implementation Notes

GAGE is an experimental language implementation, so the documentation describes the current implementation rather than promising that every feature behaves like a mature production language.

Important implementation details:

### Native and VM paths are separate

The native backend and VM are different execution engines.

A feature added to the language front end may require separate implementation work in:

```text
codegen.rs
compiler.rs
vm.rs
```

---

### The native backend uses generated C

GAGE does not directly emit machine instructions from its Rust compiler.

The native path is:

```text
GAGE AST
   ↓
Generated C
   ↓
Clang / GCC
   ↓
Native executable
```

---

### The VM is stack-based

The bytecode VM evaluates expressions using a stack and stores global variables in a runtime table.

---

### `step(dt)` uses a fixed timestep

The `step` construct provides a convenient simulation-oriented syntax.

The current bytecode compiler lowers its timestep value to:

```text
0.016667
```

The native implementation also uses a fixed simulation interval.

It should therefore be viewed as a simulation primitive rather than a complete wall-clock scheduler.

---

### Type checking is intentionally compact

The type system tracks the major built-in types, scopes, functions, and classes.

Some expressions are represented as `Any` during semantic analysis, particularly dynamic operations such as generic member access and function calls.

---

### Arrays are language-level constructs

Arrays are represented directly in the AST and are handled by the native backend.

The current bytecode implementation is smaller and does not provide complete bytecode equivalents for every native language feature.

---

# 33. Troubleshooting

## `gage: command not found`

The GAGE binary is not available through your `PATH`.

Check where the binary was installed and ensure its directory is included in:

```text
PATH
```

You can also run the binary directly from the build directory:

```bash
./target/release/gage --help
```

---

## `clang: command not found`

Install Clang.

### Termux

```bash
pkg install clang -y
```

### Ubuntu / Debian

```bash
sudo apt install clang -y
```

Then verify:

```bash
clang --version
```

---

## `gcc: command not found`

Install GCC through your operating system's package manager.

Verify:

```bash
gcc --version
```

---

## Rust is missing

Verify:

```bash
rustc --version
```

If it is unavailable, install Rust using the recommended Rust installation method for your operating system.

---

## Program fails during `gage check`

Run:

```bash
gage check program.gage
```

Read the reported stage:

```text
Lexer Error
Parse Error
Semantic Error
```

Fix the reported issue before attempting native compilation.

---

## Program works natively but not in the VM

The native and VM backends are separate.

Try:

```bash
gage run program.gage
```

and:

```bash
gage vm program.gage
```

If only one path works, the feature may currently be implemented in one execution engine but not the other.

---

## Native compiler error

First inspect the generated C:

```bash
gage emit-c program.gage > generated.c
```

Then inspect:

```text
generated.c
```

Also verify:

```bash
clang --version
```

or:

```bash
gcc --version
```

---

## Missing semicolon

GAGE statements normally terminate with `;`.

Incorrect:

```gage
let x = 10
println(x)
```

Correct:

```gage
let x = 10;
println(x);
```

---

## Invalid `if` syntax

The current parser expects parentheses around the condition.

Correct:

```gage
if (x > 10) {
    println(x);
}
```

---

## Invalid `while` syntax

The current parser expects parentheses around the condition.

Correct:

```gage
while (x < 10) {
    x = x + 1;
}
```

---

# 34. License

GAGE is released under the MIT License.

See:

```text
LICENSE
```

for the complete license text.

---

# 📌 Quick Reference

## CLI

```text
gage <file.gage>
gage build <file.gage> [-o output]
gage run <file.gage>
gage vm <file.gage>
gage check <file.gage>
gage emit-c <file.gage>
gage delete
gage --info
gage --version
gage --help
```

## Variables

```gage
let x = 10;
x = 20;
```

## Functions

```gage
fn add(a, b) {
    return a + b;
}
```

## Classes

```gage
class Player {
    health;

    fn set_health(value) {
        this.health = value;
    }
}

let player = new Player();
```

## Conditionals

```gage
if (x > 10) {
    println("large");
} else {
    println("small");
}
```

## While

```gage
while (x < 10) {
    x = x + 1;
}
```

## Loop

```gage
loop {
    if (done) {
        break;
    }
}
```

## For

```gage
for item in items {
    println(item);
}
```

## Simulation

```gage
step(dt) {
    position = position + velocity * dt;
}
```

## Vectors

```gage
let a = vec3(1.0, 2.0, 3.0);
let b = vec3(4.0, 5.0, 6.0);

let sum = a + b;
let d = dot(a, b);
let c = cross(a, b);
let l = length(a);
let n = normalize(a);
```

## Input

```gage
let name = input("Name: ");
```

## Output

```gage
print("Hello");
println("GAGE");
```

## Files

```gage
let data = read_file("data.txt");
let ok = write_file("data.txt", "GAGE");
```

---

# 🧭 Learning Path

If you are new to GAGE, a good progression is:

```text
1. Variables
      ↓
2. Arithmetic
      ↓
3. Conditions
      ↓
4. Loops
      ↓
5. Functions
      ↓
6. Arrays
      ↓
7. Vectors
      ↓
8. Classes
      ↓
9. Simulation with step(dt)
      ↓
10. Native compilation
      ↓
11. Bytecode VM
      ↓
12. Compiler internals
```

For compiler development, explore the implementation in this order:

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
src/codegen.rs
      │
      └──────────────┐
                     ↓
              src/compiler.rs
                     ↓
              src/bytecode.rs
                     ↓
                 src/vm.rs
```

---

# ⚡ GAGE

GAGE is intentionally compact: a language, a compiler front end, a native backend, and a bytecode virtual machine in one project.

The best way to understand the language is to read the examples, experiment with small `.gage` programs, inspect the generated C with `gage emit-c`, and explore the compiler implementation under `src/`.

```text
GAGE SOURCE
     │
     ▼
   LEXER
     │
     ▼
   PARSER
     │
     ▼
 TYPE CHECKER
     │
     ├──────────────────┐
     ▼                  ▼
 NATIVE PATH         BYTECODE PATH
     │                  │
     ▼                  ▼
 GENERATED C          BYTECODE
     │                  │
     ▼                  ▼
 CLANG / GCC           VM
     │                  │
     └────────┬─────────┘
              ▼
           EXECUTION
```

**GAGE — compact syntax, native-oriented execution, and simulation-focused programming.**