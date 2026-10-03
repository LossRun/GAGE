# ⚡ Gage Programming Language (v2.0)

A high-performance, expressive programming language engineered for game development, physics simulations, and graphics scripting. Gage compiles directly to optimized native machine code via C/Clang and features a custom bytecode virtual machine.

---

## 🌟 Key Features in Gage 2.0

- 🏛️ **Object-Oriented Programming**: First-class `class` definitions, instance fields, member methods, and `new` instantiation with `this` pointer scoping.
- ⚡ **First-Class Functions**: Multi-argument functions (`fn`), return values (`return`), and recursion.
- 📐 **Native SIMD Vector Algebra**: Hardware-accelerated `vec2`, `vec3`, and `vec4` primitives with built-in `dot()`, `cross()`, `length()`, and `normalize()`.
- 📦 **Dynamic Collections**: Dynamic arrays (`[1, 2, 3]`), 0-based index subscripting (`arr[i]`), and `for item in array` iterators.
- 💾 **System I/O & Persistence**: Inline `print()`, newline `println()`, terminal `input("prompt")`, and native file operations (`read_file`, `write_file`).
- ⏱️ **Game Loop & Physics Primitives**: Dedicated `step(dt)` simulation blocks for delta-time integration.
- 🚀 **Dual Execution Targets**: Native ahead-of-time (AOT) compilation via Clang or instant bytecode VM execution.

---

## 🚀 Quick Start

### 1. Build & Install (Termux / Linux)
```bash
cargo build --release
cp target/release/gage "$PREFIX/bin/gage"
chmod +x "$PREFIX/bin/gage"

### 2. Run a Program
# Compile and run natively via Clang
gage main.gage

# Run via bytecode VM
gage --vm main.gage

💻 Code at a Glance
Classes & Methods
class Player {
    health;
    power;

    fn setup(hp, atk) {
        this.health = hp;
        this.power = atk;
    }

    fn attack(target_hp) {
        return target_hp - this.power;
    }
}

let hero = new Player();
hero.setup(100, 25);
let boss_hp = hero.attack(80);
println(boss_hp);

3D Vector Math & Lighting
let normal = normalize(vec3(0.0, 1.0, 0.0));
let light_dir = normalize(vec3(0.0, 1.0, 0.5));
let brightness = dot(normal, light_dir);
println(brightness);

Arrays & Iteration
let scores = [450, 1200, 890, 2400];
let high = 0;
for s in scores {
    if (s > high) {
        high = s;
    }
}
println(high);

File I/O & Input
let name = input("Enter player name: ");
let saved = write_file("save.txt", name);
if (saved) {
    println(read_file("save.txt"));
}

📂 Showcase Examples (1–43)
The repository includes 43 working example scripts in examples/:
 * 01–23: Hello world, basic control flow, arithmetic, while/loop, step blocks.
 * 24_functions_recursion.gage: Functions, returns, and recursive factorial.
 * 25_classes_player.gage: Classes, methods, and this state.
 * 26_enemy_ai.gage: State mutations and combat logic.
 * 27_vector_math.gage: Linear algebra operations (dot, cross, length, normalize).
 * 28_physics_particle.gage: Verlet/Euler delta-time gravity motion.
 * 29_dynamic_arrays.gage: Arrays and for-in traversal.
 * 30_inventory_system.gage: Item storage and value aggregation.
 * 31_interactive_input.gage: Real-time CLI terminal prompting.
 * 32_save_load_game.gage: Disk storage and data deserialization.
 * 33_raycast_distance.gage: 3D raycast distance checks.
 * 34_orbit_simulation.gage: Vector velocity integration in orbit.
 * 35_combat_arena.gage: Turn-based RPG boss battle loop.
 * 36_score_leaderboard.gage: High score calculation over collections.
 * 37_game_loop_timer.gage: Accumulator timing inside step(dt).
 * 38_camera_follow.gage: Smooth vector Lerp camera tracking.
 * 39_projectile_motion.gage: Ballistic gravity physics trajectory.
 * 40_consumables.gage: Object state healing and caps.
 * 41_game_logger.gage: File-based event logging.
 * 42_surface_lighting.gage: Lambertian shader diffuse computation.
 * 43_mini_rpg_dungeon.gage: Integrated Class, Array, Vector, and I/O gameplay demo.
📖 Documentation
Check the docs/ directory for detailed manuals:
 * Complete Syntax Reference
 * Architecture & VM Specification
📜 License
MIT License. Built with Rust and C.
