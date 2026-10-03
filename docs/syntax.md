# 📘 Gage 2.0 Syntax & Language Reference
This document provides a complete reference for all syntax, statements, expressions, and standard built-in functions in Gage.
1. Variables & Data Types
Declarations & Mutation
let x = 42;                 // Integer (i64)
let pi = 3.14159;           // Floating-point (f64)
let is_active = true;       // Boolean
let name = "Hero";          // String
let empty = nil;            // Nil pointer / Null

x = 100;                    // Variable reassignment

2. Functions & Recursion
Functions are declared with fn and can accept multiple parameters and return values.
fn add(a, b) {
    return a + b;
}

fn fibonacci(n) {
    if (n <= 1) {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

3. Object-Oriented Programming (Classes)
Classes encapsulate fields and methods. Fields are defined before methods.
class Rigidbody {
    mass;
    pos_x;
    pos_y;

    fn setup(m, x, y) {
        this.mass = m;
        this.pos_x = x;
        this.pos_y = y;
    }

    fn apply_force(fx, fy) {
        this.pos_x = this.pos_x + (fx / this.mass);
        this.pos_y = this.pos_y + (fy / this.mass);
    }
}

// Instantiation
let body = new Rigidbody();
body.setup(2.0, 0.0, 0.0);
body.apply_force(10.0, 0.0);

4. Vectors & Linear Algebra
Native vector types support direct SIMD addition, subtraction, scalar multiplication, and division:
let v2 = vec2(1.0, 2.0);
let v3 = vec3(0.0, 1.0, 0.0);
let v4 = vec4(1.0, 0.0, 0.0, 1.0);

// Linear algebra built-ins:
let d = dot(v3, vec3(0.0, 1.0, 0.0));       // Dot product
let c = cross(vec3(1.0, 0.0, 0.0), v3);     // Cross product (vec3)
let l = length(v3);                          // Vector magnitude
let norm = normalize(v3);                    // Unit vector

5. Arrays & Collections
Dynamic arrays store collections of numbers or object references.
let list = [10, 20, 30, 40];

// Index access
let first = list[0];
list[1] = 99;

// Iteration
for item in list {
    println(item);
}

6. Control Flow
If / Else
if (hp > 50) {
    println("Healthy");
} else if (hp > 0) {
    println("Danger");
} else {
    println("Dead");
}

While & Loop
let i = 0;
while (i < 10) {
    i = i + 1;
}

loop {
    if (i >= 20) {
        break;
    }
    i = i + 1;
}

Game Physics Loop (step)
Executes with simulated fixed delta-time integration:
step(dt) {
    velocity_y = velocity_y + (-9.8 * dt);
    position_y = position_y + (velocity_y * dt);
}

7. Input / Output & File Persistence
print("Loading...");                        // Print without newline
println("Done!");                           // Print with newline

let answer = input("Enter prompt: ");      // Flush stdout and read line

let ok = write_file("save.txt", "data");    // Write string to disk (returns bool)
let data = read_file("save.txt");          // Read string from disk
