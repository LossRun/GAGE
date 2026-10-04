import os, subprocess, time

CYAN = "\033[1;36m"
GREEN = "\033[1;32m"
RED = "\033[1;31m"
YELLOW = "\033[1;33m"
BOLD = "\033[1m"
RESET = "\033[0m"

test_dir = "/sdcard/GAGE/tests_suite"
os.makedirs(test_dir, exist_ok=True)

unit_tests = {
    "test_primitives_arithmetic.gage": "let a = 15; let b = 4; if (a+b == 19 && a*b == 60) { println(\"PASS\"); }",
    "test_boolean_logic.gage": "let t = true; let f = false; if (t && !f) { println(\"PASS\"); }",
    "test_control_flow.gage": "let i = 0; while (i < 5) { i = i + 1; } if (i == 5) { println(\"PASS\"); }",
    "test_functions_recursion.gage": "fn fib(n) { if (n <= 1) { return n; } return fib(n-1) + fib(n-2); } if (fib(7) == 13) { println(\"PASS\"); }",
    "test_oop_classes.gage": "class Calc { v; fn set(x) { this.v = x; } fn get() { return this.v * 2; } } let c = new Calc(); c.set(21); if (c.get() == 42) { println(\"PASS\"); }",
    "test_simd_vectors.gage": "let v1 = vec3(1.0, 2.0, 3.0); let v2 = vec3(4.0, 5.0, 6.0); if (dot(v1, v2) == 32.0 && length(vec3(3.0, 4.0, 0.0)) == 5.0) { println(\"PASS\"); }",
    "test_dynamic_arrays.gage": "let arr = [10.0, 20.0, 30.0, 40.0]; arr[1] = 99.0; let s = 0.0; for x in arr { s = s + x; } if (s > 178.0) { if (s < 180.0) { println(\"PASS\"); } }",
    "test_step_physics_loop.gage": "let ticks = 0; step(dt) { if (ticks < 3) { ticks = ticks + 1; } } if (ticks == 3) { println(\"PASS\"); }",
    "test_file_io.gage": "let f = \"/sdcard/GAGE/tests_suite/tmp.txt\"; write_file(f, \"ok\"); if (read_file(f) != \"\") { println(\"PASS\"); }",
    "test_sim_primitives.gage": "let n = vec3(0.0, 1.0, 0.0); let v = vec3(5.0, -5.0, 0.0); let ref = reflect(v, n); let ry = ref.y; let d = distance(vec3(0.0, 0.0, 0.0), vec3(3.0, 4.0, 0.0)); let l = lerp(0.0, 100.0, 0.5); let c = clamp(150.0, 0.0, 100.0); if (ry == 5.0) { if (d == 5.0) { if (l == 50.0) { if (c == 100.0) { println(\"PASS\"); } } } }"
}

for name, code in unit_tests.items():
    with open(os.path.join(test_dir, name), "w") as f:
        f.write(code + "\n")

examples_dir = "/sdcard/GAGE/examples"
example_files = sorted([os.path.join(examples_dir, f) for f in os.listdir(examples_dir) if f.endswith(".gage") and "31_interactive_input" not in f])
unit_files = sorted([os.path.join(test_dir, f) for f in os.listdir(test_dir) if f.endswith(".gage")])

passed, failed = 0, 0
total_time = 0.0

def run_test(label, filepath, verify_pass=False):
    global passed, failed, total_time
    t0 = time.time()
    res = subprocess.run(["gage", filepath], capture_output=True, text=True)
    dt = (time.time() - t0) * 1000.0
    total_time += dt
    if res.returncode == 0 and (not verify_pass or "PASS" in res.stdout):
        passed += 1
        print(f"  [{GREEN}✔ PASS{RESET}] {label:<38} {dt:>6.1f} ms")
    else:
        failed += 1
        print(f"  [{RED}✖ FAIL{RESET}] {label:<38} {dt:>6.1f} ms")

print(f"\n{BOLD}{CYAN}======================================================{RESET}")
print(f"{BOLD}{CYAN}      ⚡ GAGE FULL TEST SUITE (ADVANCED SIMULATION)   {RESET}")
print(f"{BOLD}{CYAN}======================================================{RESET}\n")

print(f"{BOLD}Phase 1: Feature & Math Primitives Units{RESET}")
for uf in unit_files: run_test(os.path.basename(uf), uf, verify_pass=True)

print(f"\n{BOLD}Phase 2: Full Examples Suite (01 to 48){RESET}")
for ef in example_files: run_test(os.path.basename(ef), ef, verify_pass=False)

print(f"\n{BOLD}{CYAN}------------------------------------------------------{RESET}")
print(f"  Executed: {BOLD}{passed + failed}{RESET} | Passed: {GREEN}{passed}{RESET} | Failed: {RED if failed else GREEN}{failed}{RESET}")
print(f"  Total Runtime: {YELLOW}{total_time/1000.0:.2f} s{RESET}")
print(f"{BOLD}{CYAN}======================================================{RESET}\n")
