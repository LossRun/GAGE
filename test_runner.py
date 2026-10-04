import os, subprocess, time

CYAN = "\033[1;36m"
GREEN = "\033[1;32m"
RED = "\033[1;31m"
YELLOW = "\033[1;33m"
BOLD = "\033[1m"
RESET = "\033[0m"

test_dir = "/sdcard/GAGE/tests_suite"
examples_dir = "/sdcard/GAGE/examples"

example_files = sorted([os.path.join(examples_dir, f) for f in os.listdir(examples_dir) if f.endswith(".gage") and "31_interactive_input" not in f and "51_rotating_cube_3d" not in f])
unit_files = sorted([os.path.join(test_dir, f) for f in os.listdir(test_dir) if f.endswith(".gage")])

passed, failed = 0, 0
failures = []
total_time = 0.0

def run_test(label, filepath, verify_pass=False):
    global passed, failed, total_time
    t0 = time.time()
    res = subprocess.run(["gage", filepath], capture_output=True, text=True)
    dt = (time.time() - t0) * 1000.0
    total_time += dt
    has_err = res.returncode != 0 or len(res.stderr.strip()) > 0 or "error:" in res.stdout.lower()
    if not has_err and (not verify_pass or "PASS" in res.stdout):
        passed += 1
        print(f"  [{GREEN}✔ PASS{RESET}] {label:<38} {dt:>6.1f} ms")
    else:
        failed += 1
        err = res.stderr.strip() or res.stdout.strip() or "Assertion Failed"
        failures.append((label, err))
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

if failures:
    print(f"\n{BOLD}{RED}Failure Diagnostics:{RESET}")
    for lbl, err in failures:
        print(f"  • {BOLD}{lbl}{RESET}:\n    {err}")
else:
    print(f"\n{BOLD}{GREEN}✔ 100% of all tests passed cleanly!{RESET}")

print(f"{BOLD}{CYAN}======================================================{RESET}\n")
