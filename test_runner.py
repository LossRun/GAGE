import os
import subprocess
import time

CYAN = "\033[1;36m"
GREEN = "\033[1;32m"
RED = "\033[1;31m"
YELLOW = "\033[1;33m"
BOLD = "\033[1m"
RESET = "\033[0m"

test_dir = "/sdcard/GAGE/tests_suite"
os.makedirs(test_dir, exist_ok=True)

# 1. Dedicated Syntax & Feature Unit Tests
unit_tests = {
    "test_primitives_arithmetic.gage": """
let a = 15;
let b = 4;
let sum = a + b;
let sub = a - b;
let mul = a * b;
let div = a / b;
let neg = -a;
if (sum == 19 && sub == 11 && mul == 60 && div == 3 && neg == -15) {
    println("PASS");
} else {
    println("FAIL");
}
""",
    "test_boolean_logic.gage": """
let t = true;
let f = false;
if (t && !f && (f || t) && !(t && f)) {
    println("PASS");
} else {
    println("FAIL");
}
""",
    "test_control_flow.gage": """
let count = 0;
let i = 0;
while (i < 5) {
    count = count + 1;
    i = i + 1;
}
let looped = 0;
loop {
    looped = looped + 1;
    if (looped == 3) {
        break;
    }
}
if (count == 5 && looped == 3) {
    println("PASS");
} else {
    println("FAIL");
}
""",
    "test_functions_recursion.gage": """
fn fib(n) {
    if (n <= 1) { return n; }
    return fib(n - 1) + fib(n - 2);
}
let res = fib(7);
if (res == 13) {
    println("PASS");
} else {
    println("FAIL");
}
""",
    "test_oop_classes.gage": """
class Calculator {
    base;
    fn setup(b) { this.base = b; }
    fn multiply(val) { return this.base * val; }
}
let calc = new Calculator();
calc.setup(12);
let out = calc.multiply(5);
if (out == 60) {
    println("PASS");
} else {
    println("FAIL");
}
""",
    "test_simd_vectors.gage": """
let v1 = vec3(1.0, 2.0, 3.0);
let v2 = vec3(4.0, 5.0, 6.0);
let d = dot(v1, v2);
let l = length(vec3(3.0, 4.0, 0.0));
let c = cross(vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0));
if (d == 32.0 && l == 5.0) {
    println("PASS");
} else {
    println("FAIL");
}
""",
    "test_dynamic_arrays.gage": """
let arr = [10, 20, 30, 40];
arr[1] = 99;
let sum = 0;
for n in arr {
    sum = sum + n;
}
if (sum > 178 && sum < 180) {
    if (arr[0] == 10 && arr[1] == 99) {
        println("PASS");
    } else {
        println("FAIL");
    }
} else {
    println("FAIL");
}
""",
    "test_step_physics_loop.gage": """
let pos = 0.0;
let vel = 100.0;
let ticks = 0;
step(dt) {
    if (ticks < 3) {
        pos = pos + (vel * dt);
        ticks = ticks + 1;
    }
}
if (pos > 0.0) {
    if (ticks == 3) {
        println("PASS");
    }
}
""",
    "test_file_io.gage": """
let fname = "/sdcard/GAGE/tests_suite/test_io_temp.txt";
let ok = write_file(fname, "gage_io_ok");
if (ok) {
    let content = read_file(fname);
    println("PASS");
} else {
    println("FAIL");
}
"""
}

for name, code in unit_tests.items():
    with open(os.path.join(test_dir, name), "w") as f:
        f.write(code.strip() + "\n")

examples_dir = "/sdcard/GAGE/examples"
example_files = sorted([os.path.join(examples_dir, f) for f in os.listdir(examples_dir) if f.endswith(".gage")])
unit_files = sorted([os.path.join(test_dir, f) for f in os.listdir(test_dir) if f.endswith(".gage")])

filtered_examples = [f for f in example_files if "31_interactive_input" not in f]

print(f"\n{BOLD}{CYAN}======================================================{RESET}")
print(f"{BOLD}{CYAN}        GAGE 2.0 FULL ENGINE DIAGNOSTIC SUITE        {RESET}")
print(f"{BOLD}{CYAN}======================================================{RESET}\n")

passed = 0
failed = 0
failures = []
total_time = 0.0

def run_test(label, filepath, verify_pass=False):
    global passed, failed, total_time
    t0 = time.time()
    res = subprocess.run(["gage", filepath], capture_output=True, text=True)
    dt = (time.time() - t0) * 1000.0
    total_time += dt

    if res.returncode == 0:
        if verify_pass and "PASS" not in res.stdout:
            failed += 1
            failures.append((label, f"Assertion Failed (Output: {res.stdout.strip()})"))
            print(f"  [{RED}FAIL{RESET}] {label:<38} {dt:>7.1f} ms  {RED}(assertion mismatch){RESET}")
        else:
            passed += 1
            print(f"  [{GREEN}PASS{RESET}] {label:<38} {dt:>7.1f} ms")
    else:
        failed += 1
        err_msg = res.stderr.strip().split("\n")[-1] if res.stderr else "Non-zero exit"
        failures.append((label, err_msg))
        print(f"  [{RED}FAIL{RESET}] {label:<38} {dt:>7.1f} ms  {RED}({err_msg}){RESET}")

print(f"{BOLD}Phase 1: Feature & Syntax Verification Units{RESET}")
for uf in unit_files:
    run_test(os.path.basename(uf), uf, verify_pass=True)

print(f"\n{BOLD}Phase 2: Full Examples Suite (AOT Native Compilation){RESET}")
for ef in filtered_examples:
    run_test(os.path.basename(ef), ef, verify_pass=False)

print(f"\n{BOLD}Phase 3: Tooling & CLI Subsystems{RESET}")

# 3a: --time flag
t0 = time.time()
bench_res = subprocess.run(["gage", "--time", unit_files[0]], capture_output=True, text=True)
dt = (time.time() - t0) * 1000.0
label_bench = "CLI --time benchmarking"
if bench_res.returncode == 0 and "Benchmark Results" in bench_res.stdout:
    passed += 1
    print(f"  [{GREEN}PASS{RESET}] {label_bench:<38} {dt:>7.1f} ms")
else:
    failed += 1
    failures.append(("CLI --time", "Failed benchmark output"))
    print(f"  [{RED}FAIL{RESET}] {label_bench:<38} {dt:>7.1f} ms")

# 3b: emit-c flag
t0 = time.time()
out_c = "/sdcard/GAGE/tests_suite/out.c"
emit_res = subprocess.run(["gage", "emit-c", unit_files[0], "-o", out_c], capture_output=True, text=True)
dt = (time.time() - t0) * 1000.0
label_emit = "CLI emit-c C-code export"
if emit_res.returncode == 0 and os.path.exists(out_c):
    passed += 1
    print(f"  [{GREEN}PASS{RESET}] {label_emit:<38} {dt:>7.1f} ms")
else:
    failed += 1
    failures.append(("CLI emit-c", "Failed to generate C file"))
    print(f"  [{RED}FAIL{RESET}] {label_emit:<38} {dt:>7.1f} ms")

for artifact in ["/sdcard/GAGE/tests_suite/test_io_temp.txt", out_c]:
    if os.path.exists(artifact):
        os.remove(artifact)

total_tests = passed + failed
pass_rate = (passed / total_tests) * 100.0 if total_tests > 0 else 0

print(f"\n{BOLD}{CYAN}------------------------------------------------------{RESET}")
print(f"{BOLD}                    OVERALL REPORT                    {RESET}")
print(f"{BOLD}{CYAN}------------------------------------------------------{RESET}")
print(f"  Total Test Cases Executed : {total_tests}")
print(f"  Passed                    : {GREEN}{passed}{RESET}")
print(f"  Failed                    : {RED if failed > 0 else GREEN}{failed}{RESET}")
print(f"  Pass Rate                 : {GREEN if pass_rate == 100.0 else YELLOW}{pass_rate:.1f}%{RESET}")
print(f"  Total Runtime             : {total_time/1000.0:.2f} s")

if failures:
    print(f"\n{BOLD}{RED}Failure Diagnostics:{RESET}")
    for target, err in failures:
        print(f"  • {BOLD}{target}{RESET}: {err}")
else:
    print(f"\n{BOLD}{GREEN}✔ All language primitives, syntax rules, OOP structures, vectors, and examples verified!{RESET}")
print(f"{BOLD}{CYAN}======================================================{RESET}\n")
