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
gage --test
```

Expected test summary:

```bash
  ██████╗   █████╗   ██████╗  ███████╗
 ██╔════╝  ██╔══██╗ ██╔════╝  ██╔════╝
 ██║  ███╗ ███████║ ██║  ███╗ █████╗
 ██║   ██║ ██╔══██║ ██║   ██║ ██╔══╝
 ╚██████╔╝ ██║  ██║ ╚██████╔╝ ███████╗
  ╚═════╝  ╚═╝  ╚═╝  ╚═════╝  ╚══════╝
  ● GAGE 0.2.0 | native-aot [SIMD]
  Type "help", "license", or "exit" for more information.

==> Launching GAGE Regression Test Suite...


╔════════════════════════════════════════╗
║  ⚡ GAGE AOT COMPILER REGRESSION SUITE  ║
╚════════════════════════════════════════╝
 Target:  Android (aarch64)
 Engine:  clang version 21.1.8
 SIMD:    Clang ext_vector_type (v2/v3/v4)
──────────────────────────────────────────
 [01/123] ✔ PASS [D] 01_calculator      585ms
 [02/123] ✔ PASS [D] 02_bmi_calculator  256ms
 [03/123] ✔ PASS [D] 03_bms_battery_mo… 339ms
 [04/123] ✔ PASS [D] 04_fibonacci       359ms
 [05/123] ✔ PASS [D] 05_projectile_mot… 357ms
 [06/123] ✔ PASS [D] 06_pid_controller  350ms
 [07/123] ✔ PASS [D] 07_temperature_co… 359ms
 [08/123] ✔ PASS [D] 08_simple_interest 299ms
 [09/123] ✔ PASS [D] 09_vector_reflect… 264ms
 [10/123] ✔ PASS [D] 10_orbital_mechan… 395ms
 [11/123] ✔ PASS [D] 11_unit_converter  369ms
 [12/123] ✔ PASS [D] 12_discount_prici… 345ms
 [13/123] ✔ PASS [D] 13_game_player_st… 381ms
 [14/123] ✔ PASS [D] 14_prime_checker   352ms
 [15/123] ✔ PASS [D] 15_color_lerp      398ms
 [16/123] ✔ PASS [D] 16_server_rate_li… 379ms
 [17/123] ✔ PASS [D] 17_circle_physics  256ms
 [18/123] ✔ PASS [D] 18_linear_search   352ms
 [19/123] ✔ PASS [D] 19_camera_lookat   358ms
 [20/123] ✔ PASS [D] 20_leap_year       357ms
 [21/123] ✔ PASS [D] 21_fuel_efficiency 345ms
 [22/123] ✔ PASS [D] 22_matrix_trace    232ms
 [23/123] ✔ PASS [D] 23_hello_world     333ms
 [24/123] ✔ PASS [D] 24_functions_recu… 362ms
 [25/123] ✔ PASS [D] 25_classes_player  365ms
 [26/123] ✔ PASS [D] 26_enemy_ai        349ms
 [27/123] ✔ PASS [D] 27_vector_math     265ms
 [28/123] ✔ PASS [D] 28_physics_partic… 258ms
 [29/123] ✔ PASS [D] 29_dynamic_arrays  354ms
 [30/123] ✔ PASS [D] 30_inventory_syst… 344ms
 [31/123] ✔ PASS [D] 31_interactive_in… 353ms
 [32/123] ✔ PASS [D] 32_save_load_game  356ms
 [33/123] ✔ PASS [D] 33_raycast_distan… 355ms
 [34/123] ✔ PASS [D] 34_orbit_simulati… 356ms
 [35/123] ✔ PASS [D] 35_combat_arena    356ms
 [36/123] ✔ PASS [D] 36_score_leaderbo… 344ms
 [37/123] ✔ PASS [D] 37_game_loop_timer 336ms
 [38/123] ✔ PASS [D] 38_camera_follow   345ms
 [39/123] ✔ PASS [D] 39_projectile_mot… 349ms
 [40/123] ✔ PASS [D] 40_consumables     326ms
 [41/123] ✔ PASS [D] 41_game_logger     368ms
 [42/123] ✔ PASS [D] 42_surface_lighti… 356ms
 [43/123] ✔ PASS [D] 43_mini_rpg_dunge… 352ms
 [44/123] ✔ PASS [D] 44_tic_tac_toe     430ms
 [45/123] ✔ PASS [D] 45_physics_partic… 241ms
 [46/123] ✔ PASS [D] 46_flocking_boids  347ms
 [47/123] ✔ PASS [D] 47_elastic_collis… 338ms
 [48/123] ✔ PASS [D] 48_smooth_camera_… 347ms
 [49/123] ✔ PASS [D] 49_standard_math_… 379ms
 [50/123] ✔ PASS [D] 50_mat4_transform… 342ms
 [51/123] ✔ PASS [D] 51_rotating_cube_… 545ms
 [52/123] ✔ PASS [D] 52_canvas_ascii_a… 425ms
 [53/123] ✔ PASS [D] 53_matrix_perspec… 365ms
 [54/123] ✔ PASS [D] 54_cannon_ballist… 350ms
 [55/123] ✔ PASS [D] 55_physics_trajec…  23ms
 [56/123] ✔ PASS [D] 56_pulsar_oscilla…  25ms
 [57/123] ✔ PASS [D] 57_orbital_slings… 337ms
 [58/123] ✔ PASS [D] 58_elastic_bounce  359ms
 [59/123] ✔ PASS [D] 59_clock_divider   411ms
 [60/123] ✔ PASS [D] 60_lunar_lander_d… 314ms
 [61/123] ✔ PASS [D] 61_solar_orbit_ca… 362ms
 [62/123] ✔ PASS [D] 62_simd_vector_fi… 304ms
 [63/123] ✔ PASS [D] 63_braille_subpix…  26ms
 [64/123] ✔ PASS [D] 64_3d_planetary_g… 412ms
 [65/123] ✔ PASS [D] 65_3d_lorentz_mag… 359ms
 [66/123] ✔ PASS [D] 66_3d_double_pend… 354ms
 [67/123] ✔ PASS [D] 67_interactive_lu… 353ms
 [68/123] ✔ PASS [D] 68_simd_neon_part… 324ms
 [69/123] ✔ PASS [D] 69_terminal_radar… 347ms
 [70/123] ✔ PASS [D] 70_spacetime_curv… 391ms
 [71/123] ✔ PASS [D] 71_interactive_br… 459ms
 [72/123] ✔ PASS [D] 72_rotating_gage_… 431ms
 [73/123] ✔ PASS [S] test_boolean_logic 320ms
 [74/123] ✔ PASS [S] test_braille_canv… 240ms
 [75/123] ✔ PASS [S] test_canvas_buffer 349ms
 [76/123] ✔ PASS [S] test_collision_tu… 251ms
 [77/123] ✔ PASS [S] test_control_flow  373ms
 [78/123] ✔ PASS [S] test_dynamic_arra… 344ms
 [79/123] ✔ PASS [S] test_event_priori… 452ms
 [80/123] ✔ PASS [S] test_extension_ma… 681ms
 [81/123] ✔ PASS [S] test_field_gravit… 592ms
 [82/123] ✔ PASS [S] test_file_io       786ms
 [83/123] ✔ PASS [S] test_functions_re… 769ms
 [84/123] ✔ PASS [S] test_gate_propaga… 573ms
 [85/123] ✔ PASS [S] test_grid_still_l… 547ms
 [86/123] ✔ PASS [S] test_grid_toroida… 594ms
 [87/123] ✔ PASS [S] test_logic_trista… 552ms
 [88/123] ✔ PASS [S] test_math_and_mat4 786ms
 [89/123] ✔ PASS [S] test_nbody_gravit… 565ms
 [90/123] ✔ PASS [S] test_oop_classes   788ms
 [91/123] ✔ PASS [S] test_physics_rest… 581ms
 [92/123] ✔ PASS [S] test_primitives_a… 759ms
 [93/123] ✔ PASS [S] test_rk4_integrat… 595ms
 [94/123] ✔ PASS [S] test_sim_primitiv… 751ms
 [95/123] ✔ PASS [S] test_simd_batch_o… 611ms
 [96/123] ✔ PASS [S] test_simd_shading… 797ms
 [97/123] ✔ PASS [S] test_simd_vectors  786ms
 [98/123] ✔ PASS [S] test_step_physics… 947ms
 [99/123] ✔ PASS [S] test_strict_aster… 487ms
 [100/123] ✔ PASS [S] test_strict_brail… 339ms
 [101/123] ✔ PASS [S] test_strict_deep_…  19ms
 [102/123] ✔ PASS [S] test_strict_dt_ze…  36ms
 [103/123] ✔ PASS [S] test_strict_energ…  32ms
 [104/123] ✔ PASS [S] test_strict_exclu… 333ms
 [105/123] ✔ PASS [S] test_strict_exten… 358ms
 [106/123] ✔ PASS [S] test_strict_input… 357ms
 [107/123] ✔ PASS [S] test_strict_momen… 379ms
 [108/123] ✔ PASS [S] test_strict_neon_… 346ms
 [109/123] ✔ PASS [S] test_strict_neon_…  33ms
 [110/123] ✔ PASS [S] test_strict_numer…  30ms
 [111/123] ✔ PASS [S] test_strict_polar… 372ms
 [112/123] ✔ PASS [S] test_strict_repl_… 330ms
 [113/123] ✔ PASS [S] test_strict_resti…  32ms
 [114/123] ✔ PASS [S] test_strict_rotat… 332ms
 [115/123] ✔ PASS [S] test_strict_simd_…  35ms
 [116/123] ✔ PASS [S] test_strict_space… 343ms
 [117/123] ✔ PASS [S] test_strict_subpi…  27ms
 [118/123] ✔ PASS [S] test_strict_termi…  38ms
 [119/123] ✔ PASS [S] test_strict_toroi…  31ms
 [120/123] ✔ PASS [S] test_strict_trist…  26ms
 [121/123] ✔ PASS [S] test_strict_vecto…  34ms
 [122/123] ✔ PASS [S] test_waveform_sam… 236ms
 [123/123] ✔ PASS [S] test_zero_delay_c… 243ms

╔════════════════════════════════════════╗
║      EXECUTIVE DIAGNOSTIC SUMMARY      ║
╠════════════════════════════════════════╣
 Suite Status:   ALL TESTS PASSED
 Demo Examples:  72/72 passed (0 fail)
 Language Specs: 51/51 passed (0 fail)
╟────────────────────────────────────────╢
 Latency (Avg):  364.7ms / unit
 Median (p50):   353.0ms
 Tail (p95):     769.3ms
 Fastest Unit:   test_strict_dee (19.5ms)
 Slowest Unit:   test_step_physi (947.1ms)
╟────────────────────────────────────────╢
 Throughput:     2.4 suites/sec
 Total Runtime:  51.18s
╚════════════════════════════════════════╝
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
