# GAGE Documentation

> Complete language, compiler, runtime, CLI, and development documentation for GAGE.

GAGE is a compact programming language and compiler toolchain implemented in Rust.

It is designed around a small language core, native-oriented execution, mathematical primitives, simulation-oriented constructs, and two execution backends:

- Native compilation through generated C and an external C compiler.
- Bytecode compilation and execution through the built-in GAGE virtual machine.

This document contains the detailed language and toolchain reference. The README is intentionally kept focused on introducing the project, while this document covers the practical and technical details required to use, understand, and develop GAGE.

---

# Table of Contents

1. [Overview](#overview)
2. [Project Status](#project-status)
3. [Requirements](#requirements)
4. [Building GAGE](#building-gage)
5. [Command-Line Interface](#command-line-interface)
6. [Compiler Pipeline](#compiler-pipeline)
7. [Lexer](#lexer)
8. [Tokens](#tokens)
9. [Parser](#parser)
10. [Abstract Syntax Tree](#abstract-syntax-tree)
11. [Semantic Analysis and Type Checking](#semantic-analysis-and-type-checking)
12. [Variables](#variables)
13. [Primitive Values](#primitive-values)
14. [Expressions](#expressions)
15. [Operators](#operators)
16. [Strings](#strings)
17. [Booleans](#booleans)
18. [Nil and Any](#nil-and-any)
19. [Functions](#functions)
20. [Return Values](#return-values)
21. [Recursion](#recursion)
22. [Classes](#classes)
23. [Fields](#fields)
24. [Methods](#methods)
25. [The `this` Binding](#the-this-binding)
26. [Object Construction](#object-construction)
27. [Member Access](#member-access)
28. [Arrays](#arrays)
29. [Array Indexing](#array-indexing)
30. [Array Mutation](#array-mutation)
31. [Array Iteration](#array-iteration)
32. [Conditional Execution](#conditional-execution)
33. [While Loops](#while-loops)
34. [Loop Statements](#loop-statements)
35. [For-In Iteration](#for-in-iteration)
36. [Break](#break)
37. [Simulation Steps](#simulation-steps)
38. [Vectors](#vectors)
39. [Vector Construction](#vector-construction)
40. [Vector Arithmetic](#vector-arithmetic)
41. [Vector Functions](#vector-functions)
42. [Input and Output](#input-and-output)
43. [File I/O](#file-io)
44. [Native Backend](#native-backend)
45. [Generated C](#generated-c)
46. [Bytecode Backend](#bytecode-backend)
47. [Bytecode Chunks](#bytecode-chunks)
48. [Virtual Machine](#virtual-machine)
49. [Execution Models](#execution-models)
50. [Diagnostics](#diagnostics)
51. [Repository Structure](#repository-structure)
52. [Source Code Guide](#source-code-guide)
53. [Examples](#examples)
54. [Testing](#testing)
55. [Development Workflow](#development-workflow)
56. [Extending GAGE](#extending-gage)
57. [Platform Notes](#platform-notes)
58. [Limitations](#limitations)
59. [License](#license)

---

# Overview

GAGE is an experimental programming language implemented from scratch in Rust.

The implementation contains the major components normally found in a programming language toolchain:

- Source lexer
- Token definitions
- Parser
- Abstract syntax tree
- Semantic analysis
- Type checking
- Native C code generation
- Bytecode representation
- Bytecode compiler
- Stack-based virtual machine
- Command-line interface
- Runtime support for I/O, vectors, arrays, and classes

A GAGE source file normally uses the `.gage` extension.

A source program can follow either of two execution paths.

## Native execution

The native pipeline transforms GAGE source into C source and then invokes an available C compiler.

```text
GAGE source
    |
    v
Lexer
    |
    v
Parser
    |
    v
AST
    |
    v
Type Checker
    |
    v
C Code Generator
    |
    v
Generated C
    |
    v
Clang / GCC / clang-cl
    |
    v
Native executable
```

## Bytecode execution

The VM path converts the validated program into GAGE bytecode and executes it directly using the built-in virtual machine.

```text
GAGE source
    |
    v
Lexer
    |
    v
Parser
    |
    v
AST
    |
    v
Type Checker
    |
    v
Bytecode Compiler
    |
    v
Bytecode Chunk
    |
    v
GAGE Virtual Machine
    |
    v
Program output
```

Both execution models operate on the same GAGE language syntax.

---

# Project Status

GAGE is an experimental language and compiler project.

The implementation is intentionally compact and is suitable for:

- Learning compiler implementation
- Experimenting with programming-language design
- Native-code-generation experiments
- Vector and mathematical programs
- Small simulations
- Game-oriented scripting experiments
- Exploring bytecode execution
- Studying compiler architecture

GAGE should not currently be treated as a drop-in replacement for mature production languages.

The language and implementation may continue to change as the project evolves.

---

# Requirements

## Rust

GAGE is implemented as a Rust Cargo project.

A working Rust installation with Cargo is required to build the compiler.

Check the installation with:

```bash
rustc --version
cargo --version
```

## Native C compiler

The native backend generates C source code and requires an external C compiler.

Supported compiler choices include:

- Clang
- GCC
- clang-cl on supported Windows environments

Check for Clang:

```bash
clang --version
```

Check for GCC:

```bash
gcc --version
```

The bytecode VM does not require an external C compiler for execution.

---

# Building GAGE

Clone the repository:

```bash
git clone https://github.com/LossRun/GAGE.git
cd GAGE
```

Build the debug version:

```bash
cargo build
```

Build the optimized release version:

```bash
cargo build --release
```

The release executable is produced under:

```text
target/release/gage
```

The debug executable is produced under:

```text
target/debug/gage
```

GAGE can also be executed directly through Cargo:

```bash
cargo run -- --help
```

---

# Command-Line Interface

The GAGE command-line tool provides several operations.

## Help

```bash
gage --help
```

Displays the available commands and command-line options.

## Version

```bash
gage --version
```

Displays the current GAGE version.

## Toolchain information

```bash
gage --info
```

Displays information about the GAGE toolchain and available native compilation environment.

## Native build

```bash
gage build program.gage -o program
```

The build command validates the GAGE source, generates C code, invokes the available native compiler, and produces a native executable.

## Native run

```bash
gage run program.gage
```

The run command performs the native compilation process and executes the resulting program.

## Bytecode VM

```bash
gage vm program.gage
```

The VM command sends the program through the bytecode compiler and executes it with the built-in GAGE virtual machine.

## Validation

```bash
gage check program.gage
```

The check command validates the program without producing a native executable.

The validation process includes lexical analysis, parsing, and semantic/type checking.

## Emit generated C

```bash
gage emit-c program.gage
```

This command prints the C source generated by the native backend.

It is useful for:

- Understanding the native backend
- Debugging code generation
- Inspecting how GAGE constructs are lowered
- Investigating generated runtime support

---

# Compiler Pipeline

GAGE is organized as a sequence of compiler stages.

```text
Source Characters
       |
       v
     Lexer
       |
       v
     Tokens
       |
       v
    Parser
       |
       v
      AST
       |
       v
 Type Checker
       |
       +----------------------+
       |                      |
       v                      v
   CodeGen                Compiler
       |                      |
       v                      v
 Generated C              Bytecode
       |                      |
       v                      v
 Clang / GCC                 VM
       |                      |
       v                      v
 Native Output            Program Output
```

Each stage has a separate responsibility.

The front end is responsible for understanding the language.

The native backend converts the language representation into C.

The bytecode backend converts the language representation into executable bytecode.

---

# Lexer

The lexer is implemented in:

```text
src/lexer.rs
```

Token definitions are implemented in:

```text
src/token.rs
```

The lexer converts raw source characters into tokens.

Conceptually:

```text
Source characters
       |
       v
     Lexer
       |
       v
     Tokens
```

The lexer tracks source locations so that lexical errors can report useful line and column information.

The lexer recognizes language elements including:

- Identifiers
- Keywords
- Integer literals
- Floating-point literals
- String literals
- Boolean literals
- Operators
- Punctuation
- Delimiters

For example:

```gage
let speed = 10.0;
println(speed);
```

The lexer breaks the source into a sequence of tokens representing:

```text
let
identifier(speed)
=
float(10.0)
;
identifier(println)
(
identifier(speed)
)
;
```

The exact internal representation is defined by the token structures in `src/token.rs`.

---

# Tokens

The token system provides the vocabulary consumed by the parser.

Tokens represent syntactic elements rather than complete program structures.

Examples include:

```text
let
fn
class
new
this
if
else
while
loop
for
in
break
return
step
true
false
nil
```

Operators and punctuation are also represented as tokens.

The separation between lexing and parsing allows the parser to work with structured token information rather than raw characters.

---

# Parser

The parser is implemented in:

```text
src/parser.rs
```

The parser receives the token stream produced by the lexer and constructs the abstract syntax tree.

Conceptually:

```text
Tokens
   |
   v
Parser
   |
   v
AST
```

The parser is responsible for recognizing language constructs such as:

- Variable declarations
- Expressions
- Assignments
- Function declarations
- Function calls
- Class declarations
- Object construction
- Member access
- Conditional statements
- Loops
- Array literals
- Index expressions
- Simulation steps
- Return statements

The parser therefore converts a flat sequence of tokens into a hierarchical representation of the program.

---

# Abstract Syntax Tree

The AST is implemented in:

```text
src/ast.rs
```

The abstract syntax tree represents the structure of a GAGE program.

For example:

```gage
let x = 10;
let y = x + 5;
```

is conceptually represented as a sequence containing:

```text
Variable declaration
    |
    +-- name: x
    +-- initializer: 10

Variable declaration
    |
    +-- name: y
    +-- initializer:
            |
            +-- x
            +-- +
            +-- 5
```

The AST provides a common representation consumed by later compiler stages.

Both the native code generator and bytecode compiler operate from this program representation.

---

# Semantic Analysis and Type Checking

Semantic and type checking are implemented in:

```text
src/types.rs
```

The type checker runs after parsing.

Its responsibility is to validate meaning rather than merely syntax.

The compiler maintains information about declarations, scopes, variables, functions, and classes.

The currently represented types include:

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

For example, referencing a variable that has not been declared can produce a semantic error.

```text
[Semantic Error] Undefined variable 'name'
```

Semantic checking also understands the special `this` binding available inside class methods.

---

# Variables

Variables are declared with `let`.

```gage
let x = 42;
let speed = 10.0;
let active = true;
let name = "GAGE";
```

Variables can be reassigned.

```gage
let speed = 10.0;

speed = speed + 5.0;

println(speed);
```

The declaration creates a binding in the current scope.

Assignments modify an existing binding.

---

# Primitive Values

GAGE supports several primitive value categories.

## Integer

```gage
let count = 42;
```

## Floating-point

```gage
let gravity = -9.8;
```

## Boolean

```gage
let running = true;
let finished = false;
```

## String

```gage
let name = "GAGE";
```

## Nil

```gage
let value = nil;
```

`nil` represents the absence of a normal value.

---

# Expressions

Expressions produce values.

Examples include:

```gage
10
3.14
true
"hello"
x
x + 10
x * speed
vec3(1.0, 2.0, 3.0)
foo(10)
player.health
array[0]
```

Expressions can be combined to construct larger expressions.

```gage
let result = (speed * time) + offset;
```

Expressions can also appear as conditions:

```gage
if (health > 0) {
    println("Alive");
}
```

---

# Operators

GAGE supports arithmetic and comparison operations used throughout the language.

Typical arithmetic operators include:

```text
+
-
*
/
```

Comparison operators include:

```text
==
!=
<
>
<=
>=
```

Operators may operate on scalar values and, where supported by the language implementation, vector values.

---

# Strings

Strings are written using double quotes.

```gage
let message = "Hello from GAGE";
println(message);
```

Strings can be passed to functions and I/O operations.

For example:

```gage
print("Hello ");
println("GAGE!");
```

---

# Booleans

Boolean values are represented by:

```gage
true
false
```

Example:

```gage
let active = true;

if (active) {
    println("Running");
}
```

Boolean expressions can also be produced by comparisons:

```gage
let healthy = health > 50;
```

---

# Nil and Any

GAGE contains `Nil` and `Any` concepts in its type system.

`nil` represents an empty or absent value:

```gage
let value = nil;
```

`Any` provides a more general type representation where the implementation requires a value without committing to a more specific type.

These types are primarily relevant to semantic analysis and runtime representation.

---

# Functions

Functions are declared using `fn`.

```gage
fn square(x) {
    return x * x;
}
```

Functions can accept multiple parameters.

```gage
fn add(a, b) {
    return a + b;
}
```

They can then be called using normal call syntax:

```gage
let result = add(10, 20);

println(result);
```

Functions can be used to separate reusable program logic into named operations.

---

# Return Values

A function can return a value using `return`.

```gage
fn multiply(a, b) {
    return a * b;
}
```

The returned value can be assigned:

```gage
let result = multiply(6, 7);

println(result);
```

Returning from a function terminates that function's current execution path.

---

# Recursion

GAGE functions can call themselves.

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

Recursive programs are subject to the limits of the selected execution backend and runtime.

---

# Classes

GAGE provides a compact object-oriented system.

A class can define fields and methods.

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
```

A class definition describes the structure and behavior of its instances.

---

# Fields

Fields are declared inside a class.

```gage
class Player {
    health;
    power;
}
```

Fields represent state belonging to an instance.

They can be accessed through an object:

```gage
player.health
player.power
```

---

# Methods

Methods are functions declared inside a class.

```gage
class Player {
    health;

    fn set_health(value) {
        this.health = value;
    }
}
```

Methods can modify object state through `this`.

---

# The `this` Binding

Inside a class method, `this` refers to the current object instance.

```gage
class Player {
    health;

    fn setup(value) {
        this.health = value;
    }
}
```

The expression:

```gage
this.health
```

refers to the `health` field belonging to the current instance.

The type checker recognizes `this` specially when analyzing class methods.

---

# Object Construction

Objects are created with `new`.

```gage
let player = new Player();
```

After construction, methods can be called:

```gage
player.setup(100);
```

A complete example:

```gage
class Player {
    health;

    fn setup(value) {
        this.health = value;
    }
}

let player = new Player();

player.setup(100);

println(player.health);
```

---

# Member Access

Members are accessed using the dot operator.

```gage
player.health
player.setup(100)
```

Member access can appear inside larger expressions:

```gage
let remaining = player.health - 10;
```

Member access is also used for method invocation.

---

# Arrays

GAGE supports array literals.

```gage
let numbers = [10, 20, 30, 40];
```

Arrays provide dynamic collection storage.

The native representation maintains storage information including:

- Data
- Current length
- Capacity

The backing storage can grow as required by supported operations.

---

# Array Indexing

Array elements can be accessed using an index.

```gage
let numbers = [10, 20, 30];

let first = numbers[0];

println(first);
```

Array indexing is zero-based.

Therefore:

```text
numbers[0]
```

refers to the first element.

---

# Array Mutation

Array elements can be changed through indexed assignment.

```gage
let numbers = [10, 20, 30];

numbers[1] = 99;

println(numbers[1]);
```

This modifies the element at the selected index.

---

# Array Iteration

Arrays can be traversed with `for ... in`.

```gage
let scores = [450, 1200, 890, 2400];

for score in scores {
    println(score);
}
```

The loop variable receives each element as iteration proceeds.

---

# Conditional Execution

GAGE provides `if` and `else`.

```gage
let health = 75;

if (health > 50) {
    println("Healthy");
} else {
    println("Danger");
}
```

Conditions can contain expressions:

```gage
if (health <= 0) {
    println("Defeated");
}
```

Multiple branches can be expressed using `else if` where supported by the parser.

```gage
if (health > 75) {
    println("High");
} else if (health > 25) {
    println("Medium");
} else {
    println("Low");
}
```

---

# While Loops

`while` repeats a block while its condition remains true.

```gage
let i = 0;

while (i < 10) {
    println(i);
    i = i + 1;
}
```

The condition is evaluated as part of each loop iteration.

---

# Loop Statements

GAGE also provides an unconditional `loop`.

```gage
let i = 0;

loop {
    i = i + 1;

    if (i >= 10) {
        break;
    }
}
```

The loop continues until control leaves it.

---

# For-In Iteration

The `for ... in` construct is designed for iterating over collections.

```gage
let values = [1, 2, 3, 4];

for value in values {
    println(value);
}
```

The loop variable represents the current element.

This construct is particularly useful for array-based simulation and game logic.

---

# Break

`break` exits the current loop.

```gage
let i = 0;

loop {
    i = i + 1;

    if (i >= 5) {
        break;
    }
}
```

`break` is useful when the termination condition is determined from inside the loop body.

---

# Simulation Steps

One of the distinctive language constructs in GAGE is:

```gage
step(dt) {
    ...
}
```

A `step` block provides a simulation-oriented execution construct.

Example:

```gage
let position = vec3(0.0, 50.0, 0.0);
let velocity = vec3(0.0, -1.0, 0.0);

step(dt) {
    position = position + (velocity * dt);
}

println(position);
```

The `dt` identifier represents the local time-step value available inside the step block.

The construct is intended for programs involving:

- Physics
- Motion
- Simulation
- Game state updates
- Repeated numerical integration

The current native implementation uses a fixed simulation interval for this construct.

---

# Vectors

Vectors are built into the GAGE language.

The current language includes:

```text
vec2
vec3
vec4
```

Vectors are intended to make mathematical and simulation-oriented code concise.

---

# Vector Construction

A two-dimensional vector:

```gage
let position = vec2(10.0, 20.0);
```

A three-dimensional vector:

```gage
let position = vec3(10.0, 20.0, 30.0);
```

A four-dimensional vector:

```gage
let color = vec4(1.0, 0.0, 0.0, 1.0);
```

---

# Vector Arithmetic

Vector values can participate directly in arithmetic operations supported by the implementation.

Example:

```gage
let position = vec3(0.0, 10.0, 0.0);
let velocity = vec3(1.0, 0.0, -2.0);
let gravity = vec3(0.0, -9.8, 0.0);

let next_velocity = velocity + gravity;
let next_position = position + next_velocity;

println(next_position);
```

Scalar multiplication can also be used:

```gage
let velocity = vec3(1.0, 2.0, 3.0);

let scaled = velocity * 2.0;

println(scaled);
```

The exact combinations supported are determined by the language type system and backend implementation.

---

# Vector Functions

GAGE provides built-in mathematical operations for vectors.

## Dot product

```gage
let a = vec3(1.0, 0.0, 0.0);
let b = vec3(0.0, 1.0, 0.0);

let result = dot(a, b);

println(result);
```

## Cross product

```gage
let a = vec3(1.0, 0.0, 0.0);
let b = vec3(0.0, 1.0, 0.0);

let result = cross(a, b);

println(result);
```

## Length

```gage
let velocity = vec3(3.0, 4.0, 0.0);

let speed = length(velocity);

println(speed);
```

## Normalize

```gage
let direction = vec3(10.0, 0.0, 0.0);

let normalized = normalize(direction);

println(normalized);
```

These operations are particularly useful for simulation, physics, geometry, and game-oriented programs.

---

# Input and Output

GAGE provides simple built-in terminal I/O.

The primary output functions are:

```gage
print("Hello ");
println("GAGE!");
```

`print` writes output without automatically adding a newline.

`println` writes output followed by a newline.

---

# Input

Interactive input can be read using `input`.

```gage
let name = input("Enter your name: ");

println(name);
```

The native runtime provides the necessary terminal handling through generated C runtime support.

---

# File I/O

GAGE provides basic file operations.

## Reading a file

```gage
let contents = read_file("data.txt");

println(contents);
```

## Writing a file

```gage
let success = write_file(
    "data.txt",
    "Hello from GAGE!"
);

println(success);
```

These operations are implemented through runtime helpers provided by the native backend.

File I/O is useful for simple persistence, logs, configuration data, and experimental game save systems.

---

# Native Backend

The native backend is implemented primarily in:

```text
src/codegen.rs
```

Its job is to transform the validated GAGE AST into C source code.

Conceptually:

```text
GAGE AST
   |
   v
Code Generator
   |
   v
C Source
   |
   v
External C Compiler
   |
   v
Native Executable
```

The backend provides generated runtime support for language features including:

- Scalar output
- String output
- Boolean output
- Vector output
- Vector construction
- Vector operations
- Dynamic arrays
- Terminal input
- File reading
- File writing
- Class representations

---

# Generated C

The generated C code can be inspected using:

```bash
gage emit-c program.gage
```

The generated representation is useful when investigating:

- Code-generation behavior
- Runtime representation
- Native data layout
- Vector lowering
- Array handling
- Class lowering
- Generated helper functions
- Native compiler compatibility

The generated C is an implementation detail of the native backend and should not normally be written manually.

---

# Native Compiler Selection

GAGE does not depend on one specific external C compiler.

The native toolchain can use an available compiler such as:

```text
clang
gcc
clang-cl
```

The exact compiler selected depends on the host platform and available toolchain.

This allows the same GAGE language implementation to target different environments through their native C compiler.

---

# Bytecode Backend

The bytecode backend is implemented across:

```text
src/bytecode.rs
src/compiler.rs
src/vm.rs
```

The bytecode path avoids generating an external native executable.

The pipeline is:

```text
AST
 |
 v
Bytecode Compiler
 |
 v
Bytecode Chunk
 |
 v
Virtual Machine
 |
 v
Program Output
```

---

# Bytecode Chunks

The bytecode compiler converts supported AST constructs into a bytecode representation stored in a `Chunk`.

A chunk contains the information required by the VM to execute the compiled program.

The bytecode representation is defined primarily in:

```text
src/bytecode.rs
```

The compiler responsible for producing the bytecode is implemented in:

```text
src/compiler.rs
```

---

# Virtual Machine

The virtual machine is implemented in:

```text
src/vm.rs
```

The VM uses a stack-oriented execution model.

Important runtime components include:

- Instruction pointer
- Evaluation stack
- Global values
- Constants
- Runtime values
- Bytecode instructions

Conceptually:

```text
Bytecode
   |
   v
Instruction Pointer
   |
   v
Evaluation Stack
   |
   v
Runtime Values
   |
   v
Program Output
```

The VM contains runtime handling for scalar values and supported vector values.

Invalid runtime operations can be detected by VM-side validation.

---

# Execution Models

GAGE provides two execution models from the same language.

```text
                 GAGE Program
                      |
             +--------+--------+
             |                 |
             v                 v
        Native Path        Bytecode Path
             |                 |
             v                 v
       Generate C         Compile Bytecode
             |                 |
             v                 v
       Clang / GCC            VM
             |                 |
             +--------+--------+
                      |
                      v
                   Output
```

The native path is designed for native execution through C compilation.

The VM path is designed for direct execution of GAGE bytecode.

This separation also makes the project useful for experimenting with different compiler and runtime strategies.

---

# Diagnostics

GAGE performs diagnostics at multiple stages.

## Lexical diagnostics

The lexer can report invalid source characters or malformed lexical constructs.

Source location information is retained so diagnostics can identify where an error occurred.

## Parse diagnostics

The parser reports invalid syntactic structures.

Examples include malformed expressions, declarations, or statements.

## Semantic diagnostics

The type checker validates program meaning.

Examples include:

- Undefined variables
- Invalid bindings
- Invalid operations
- Incorrect use of language constructs
- Invalid member access
- Type-related errors

Example:

```text
[Semantic Error] Undefined variable 'name'
```

Diagnostics are an important part of the compiler pipeline because they allow invalid programs to be rejected before native code generation or execution.

---

# Repository Structure

The repository is organized approximately as follows:

```text
GAGE/
├── docs/
│   └── supporting documentation
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
│   └── standard-library-related files
│
├── tests_suite/
│   └── test programs and test infrastructure
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
├── test_runner.py
└── version.txt
```

---

# Source Code Guide

The `src/` directory contains the compiler and runtime implementation.

## `main.rs`

```text
src/main.rs
```

Provides the command-line interface and connects the major compiler stages.

It is responsible for dispatching commands such as:

```text
build
run
vm
check
emit-c
--version
--info
--help
```

## `token.rs`

```text
src/token.rs
```

Contains the token definitions used by the lexer and parser.

## `lexer.rs`

```text
src/lexer.rs
```

Converts source characters into tokens.

It also handles source-location tracking and lexical diagnostics.

## `ast.rs`

```text
src/ast.rs
```

Contains the abstract syntax tree structures representing GAGE programs.

## `parser.rs`

```text
src/parser.rs
```

Converts the token stream into the AST.

## `types.rs`

```text
src/types.rs
```

Contains semantic analysis and type-checking functionality.

It tracks declarations and scopes and validates language constructs before code generation.

## `codegen.rs`

```text
src/codegen.rs
```

Implements the native C code generator.

The AST is lowered into C source code and the resulting C can then be compiled by the host C toolchain.

## `bytecode.rs`

```text
src/bytecode.rs
```

Defines the bytecode representation and runtime value structures used by the VM.

## `compiler.rs`

```text
src/compiler.rs
```

Converts AST structures into bytecode instructions.

## `vm.rs`

```text
src/vm.rs
```

Implements the bytecode virtual machine that executes compiled GAGE bytecode.

---

# Recommended Compiler Reading Order

For developers learning how GAGE works internally, the source is best explored in pipeline order:

```text
src/token.rs
      |
      v
src/lexer.rs
      |
      v
src/ast.rs
      |
      v
src/parser.rs
      |
      v
src/types.rs
      |
      +-----------------------+
      |                       |
      v                       v
src/codegen.rs          src/compiler.rs
                              |
                              v
                       src/bytecode.rs
                              |
                              v
                           src/vm.rs
```

This order follows the flow of information through the compiler.

Start with the language vocabulary, then understand tokenization, syntax construction, semantic checking, and finally the execution backends.

---

# Examples

The `examples/` directory contains GAGE source programs.

Examples are useful for understanding the language without reading the compiler implementation first.

Example categories include:

- Basic expressions
- Variables
- Control flow
- Functions
- Recursion
- Classes
- Objects
- Vector mathematics
- Physics
- Arrays
- Input
- File persistence
- Simulation logic
- Game-oriented experiments

A typical example can be executed with:

```bash
gage run examples/example.gage
```

or through the VM:

```bash
gage vm examples/example.gage
```

The exact filename depends on the example available in the repository.

---

# Example Program

A small simulation-oriented GAGE program can look like this:

```gage
let position = vec3(0.0, 10.0, 0.0);
let velocity = vec3(1.0, 0.0, 0.0);

step(dt) {
    position = position + (velocity * dt);
}

println(position);
```

The same source can be routed through either execution backend.

```text
              program.gage
                    |
          +---------+---------+
          |                   |
          v                   v
     Native AOT          Bytecode VM
          |                   |
          v                   v
     Generated C          Bytecode
          |                   |
          v                   v
      C Compiler              VM
          |                   |
          +---------+---------+
                    |
                    v
                  Output
```

---

# Testing

The repository contains a test suite and supporting test infrastructure.

The main test-related locations include:

```text
tests_suite/
test_runner.py
```

Rust-level tests can be run through Cargo:

```bash
cargo test
```

This is the preferred starting point for testing the Rust implementation.

Example GAGE programs can also be used to manually verify compiler behavior.

A useful development cycle is:

```text
Modify compiler
      |
      v
cargo build
      |
      v
cargo test
      |
      v
Run example
      |
      v
Check native output
      |
      v
Check VM output
```

---

# Development Workflow

Build the project in debug mode:

```bash
cargo build
```

Run the Rust test suite:

```bash
cargo test
```

Build an optimized release:

```bash
cargo build --release
```

Run the compiler through Cargo:

```bash
cargo run -- --help
```

Run a GAGE source file through Cargo:

```bash
cargo run -- run program.gage
```

Validate a program:

```bash
cargo run -- check program.gage
```

Inspect generated C:

```bash
cargo run -- emit-c program.gage
```

Run through the VM:

```bash
cargo run -- vm program.gage
```

---

# Extending GAGE

Adding a new language feature generally involves multiple compiler stages.

A typical feature-development process is:

```text
1. Define the syntax
        |
        v
2. Add or update tokens
        |
        v
3. Update the lexer if required
        |
        v
4. Update the parser
        |
        v
5. Add AST representation
        |
        v
6. Add semantic/type checking
        |
        +----------------------+
        |                      |
        v                      v
7. Add native lowering    8. Add bytecode
        |                      |
        v                      v
   codegen.rs             compiler.rs
                               |
                               v
                           vm.rs
        |
        +----------------------+
                       |
                       v
                 Add tests/examples
```

A feature that only works in one backend may behave differently depending on whether a program is executed natively or through the VM.

For this reason, backend parity should be considered when extending the language.

---

# Adding Syntax

When introducing new syntax, the lexer and parser must understand it.

For example, a new keyword normally requires:

```text
Token definition
      |
      v
Lexer recognition
      |
      v
Parser handling
      |
      v
AST representation
```

The type checker and execution backend then need to understand the resulting AST node if the feature has semantic behavior.

---

# Adding a Type

A new type generally affects:

```text
Type representation
      |
      v
Type checking
      |
      +------------------+
      |                  |
      v                  v
Native representation   VM representation
      |                  |
      v                  v
Code generation       Runtime handling
```

This is particularly important for vector-like or structured values because both execution paths need compatible semantics.

---

# Adding a Built-In Function

A built-in function may require changes to the runtime or compiler depending on how it is implemented.

For native execution, the required runtime support may need to be emitted into generated C.

For VM execution, equivalent runtime behavior may need to be implemented in the VM.

A built-in should therefore be tested against both execution paths whenever applicable.

---

# Native and VM Compatibility

GAGE has two execution backends.

They are separate implementations of the same language.

When developing a feature, verify both:

```bash
gage run program.gage
```

and:

```bash
gage vm program.gage
```

The intended behavior should remain consistent between the two paths.

Differences may exist for features that have not yet reached complete parity.

---

# Platform Notes

GAGE is implemented in Rust and is intended to be portable across environments supported by the Rust toolchain and an appropriate native C compiler.

## Linux

A typical environment requires:

```text
Rust
Cargo
Clang or GCC
```

Build:

```bash
cargo build --release
```

## Termux

Termux can be used as a development environment when the required Rust and native compiler toolchain are available.

The exact package setup depends on the Termux environment.

Once Rust and a C compiler are available, the project can be built normally:

```bash
cargo build --release
```

## macOS

A Rust installation and an available C compiler toolchain are required.

The native backend can use an available Clang toolchain.

## Windows

Rust and Cargo are required.

The native backend may use an available Windows-compatible C compiler, including `clang-cl` where supported.

The repository also contains:

```text
install.ps1
```

for Windows-oriented installation/setup work.

---

# Limitations

GAGE is an experimental language implementation.

Important considerations include:

- The language is still evolving.
- Native and VM execution paths may not have perfect feature parity.
- The bytecode VM is an independent execution implementation rather than simply executing generated native code.
- The native backend depends on an external C compiler.
- Some language features may be experimental.
- The runtime and language semantics may change as development continues.
- The project is better suited to experimentation, learning, simulations, and compiler development than as a production replacement for established programming languages.

---

# Architecture Summary

The complete GAGE architecture can be summarized as:

```text
                         GAGE
                           |
                           v
                    Source Program
                      .gage file
                           |
                           v
                        Lexer
                           |
                           v
                        Tokens
                           |
                           v
                        Parser
                           |
                           v
                         AST
                           |
                           v
                    Type Checker
                           |
                 +---------+---------+
                 |                   |
                 v                   v
             Native Path        Bytecode Path
                 |                   |
                 v                   v
             CodeGen              Compiler
                 |                   |
                 v                   v
            Generated C           Bytecode
                 |                   |
                 v                   v
          Clang / GCC /           VM
            clang-cl                |
                 |                   |
                 v                   v
          Native Program       VM Program
                 |                   |
                 +---------+---------+
                           |
                           v
                         Output
```

This architecture allows GAGE to remain small while still exposing the major stages of a traditional compiler.

---

# Design Goals

GAGE is built around several core ideas.

## Compact language surface

The language aims to provide useful programming constructs without requiring a huge syntax system.

## Native-oriented execution

The native backend allows GAGE programs to ultimately become native executables through generated C.

## Flexible execution

The same source language can be processed by either the native compiler path or the internal bytecode VM.

## Mathematics as a language feature

Vectors and common vector operations are integrated into the language rather than being treated purely as an external library concept.

## Simulation-friendly programming

The combination of:

```text
vectors
mutable variables
loops
functions
classes
step(dt)
```

makes small simulations and game-oriented experiments straightforward to express.

## Understandable implementation

The compiler is separated into focused Rust modules so that individual stages can be studied and modified independently.

---

# Suggested Learning Path

If you are new to compiler development, the following progression is recommended.

## Step 1 — Learn the language

Start with:

```text
examples/
```

Run small programs and observe their behavior.

## Step 2 — Read the token system

Open:

```text
src/token.rs
```

Understand how language symbols are represented.

## Step 3 — Read the lexer

Open:

```text
src/lexer.rs
```

Follow how source characters become tokens.

## Step 4 — Read the AST

Open:

```text
src/ast.rs
```

Understand how tokens eventually become structured program nodes.

## Step 5 — Read the parser

Open:

```text
src/parser.rs
```

Follow how syntax is converted into the AST.

## Step 6 — Read semantic analysis

Open:

```text
src/types.rs
```

Understand how declarations, scopes, and types are checked.

## Step 7 — Study native compilation

Open:

```text
src/codegen.rs
```

Follow how GAGE constructs become C code.

## Step 8 — Study bytecode

Open:

```text
src/bytecode.rs
src/compiler.rs
```

Understand how the AST becomes bytecode.

## Step 9 — Study the VM

Open:

```text
src/vm.rs
```

Follow how bytecode instructions are executed.

---

# Native Backend vs Bytecode VM

| Feature | Native Backend | Bytecode VM |
|---|---|---|
| Input | GAGE source | GAGE source |
| Front end | Lexer + Parser + Type Checker | Lexer + Parser + Type Checker |
| Intermediate representation | AST → C | AST → Bytecode |
| External compiler | Required | Not required |
| Runtime | Generated C runtime | Rust VM |
| Final execution | Native executable | VM |
| Main implementation | `src/codegen.rs` | `src/compiler.rs`, `src/bytecode.rs`, `src/vm.rs` |
| Inspection | Generated C | Bytecode/runtime implementation |

The native backend and VM provide different execution strategies while sharing the same language front end.

---

# Documentation Philosophy

`README.md` introduces the project.

`DOCS.md` contains the detailed language and toolchain information.

The source code under:

```text
src/
```

contains the actual implementation.

The examples under:

```text
examples/
```

provide executable demonstrations of the language.

These three layers serve different purposes:

```text
README.md
    |
    +-- What GAGE is
    +-- Why it exists
    +-- Major features
    +-- Architecture overview
    +-- Project introduction

DOCS.md
    |
    +-- How to build
    +-- How to use the CLI
    +-- Language reference
    +-- Compiler details
    +-- Runtime details
    +-- Development information

src/
    |
    +-- Actual compiler implementation

examples/
    |
    +-- Working language examples
```

---

# License

GAGE is released under the MIT License.

See:

```text
LICENSE
```

for the complete license text.

---

# GAGE

A compact programming language built around native execution, mathematics, and simulation.

Repository:

```text
https://github.com/LossRun/GAGE
```

If you find GAGE interesting, consider starring the repository and following its development.