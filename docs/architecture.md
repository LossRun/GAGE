# ⚙️ Gage Engine Architecture
Gage employs a dual-execution pipeline designed for flexibility during development and peak performance in production:
[ .gage Source Code ]
         │
         ▼
     [ Lexer ]  ──────► Token Stream
         │
         ▼
     [ Parser ] ──────► Abstract Syntax Tree (AST)
         │
         ▼
  [ Type Checker ] ───► Semantic Validation & Symbol Table
         │
    ┌────┴───────────────────────────┐
    ▼                                ▼
[ CodeGen ]                   [ Compiler ]
    │                                │
    ▼                                ▼
[ Clang AOT ]                 [ Bytecode VM ]
(Native Binary)            (Interactive Bytecode)

1. Native AOT Engine (src/codegen.rs)
 * Translates Gage AST into ANSI/C11 code.
 * Utilizes Clang __attribute__((ext_vector_type(N))) for SIMD operations.
 * Employs C11 _Generic function pointer dispatch for type-safe polymorphic print().
 * Classes lower into native C heap structures with zero abstraction overhead.
2. Bytecode VM (src/compiler.rs & src/vm.rs)
 * Emits high-level bytecode chunks (Chunk).
 * Global variable lookups and jump-instruction tables for instant REPL execution without invoking an external C toolchain.
