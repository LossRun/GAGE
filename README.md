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

### Run test

```bash
gage --test
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

```bash
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

**123 / 123 tests passing**

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
