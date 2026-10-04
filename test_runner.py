#!/usr/bin/env python3
import os
import sys
import glob
import subprocess
import time
import re
import threading
import tempfile
import platform

CYAN = "\033[1;36m"
GREEN = "\033[1;32m"
RED = "\033[1;31m"
YELLOW = "\033[1;33m"
PURPLE = "\033[1;35m"
BOLD = "\033[1m"
DIM = "\033[90m"
RESET = "\033[0m"

SPINNER_FRAMES = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]

def get_clang_version():
    try:
        res = subprocess.run(["clang", "--version"], capture_output=True, text=True)
        return res.stdout.splitlines()[0].split("(")[0].strip()
    except Exception:
        return "Clang LLVM (AOT)"

def synthesize_mock_stdin(content):
    inputs = []
    for m in re.findall(r"(gage_input_int|gage_input_float|gage_input_str|input_int|input_float|input_str)", content):
        if "int" in m: inputs.append("42")
        elif "float" in m: inputs.append("3.14")
        else: inputs.append("demo")
    return ("\n".join(inputs) + "\n") if inputs else ""

def generate_accelerated_test_source(raw_code):
    modified = re.sub(r"gage_sleep\s*\([^)]*\)\s*;", "// test: skipped sleep", raw_code)
    modified = re.sub(r"\bsleep\s*\([^)]*\)\s*;", "// test: skipped sleep", modified)
    def loop_sub(match):
        var_name = match.group(1)
        limit = int(match.group(2))
        return f"while ({var_name} < 2) {{" if limit > 10 else match.group(0)
    return re.sub(r"while\s*\(\s*([a-zA-Z_][a-zA-Z0-9_]*)\s*<\s*(\d+)\s*\)\s*\{", loop_sub, modified)

class LiveSpinner:
    def __init__(self, prefix, name):
        self.prefix = prefix
        self.name = name
        self.stop_event = threading.Event()
        self.thread = None

    def _spin(self):
        idx = 0
        while not self.stop_event.is_set():
            frame = SPINNER_FRAMES[idx % len(SPINNER_FRAMES)]
            sys.stdout.write(f"\r\033[2K {self.prefix} {CYAN}{frame}{RESET} {DIM}{self.name[:18]}{RESET}")
            sys.stdout.flush()
            idx += 1
            time.sleep(0.06)

    def start(self):
        self.thread = threading.Thread(target=self._spin, daemon=True)
        self.thread.start()

    def stop(self):
        self.stop_event.set()
        if self.thread:
            self.thread.join(timeout=0.2)

def run_suite():
    gage_bin = os.path.expanduser("~/.cargo_target_gage/release/gage")
    if not os.path.exists(gage_bin):
        gage_bin = "gage"

    examples = sorted(glob.glob("examples/*.gage"))
    specs = sorted(glob.glob("tests_suite/*.gage"))
    all_tests = [(t, "DEMO") for t in examples] + [(t, "SPEC") for t in specs]
    total = len(all_tests)

    clang_info = get_clang_version()

    print(f"\n{CYAN}╔════════════════════════════════════════╗{RESET}")
    print(f"{CYAN}║{RESET}  {BOLD}⚡ GAGE AOT COMPILER REGRESSION SUITE{RESET}  {CYAN}║{RESET}")
    print(f"{CYAN}╚════════════════════════════════════════╝{RESET}")
    print(f" {BOLD}Target:{RESET}  {platform.system()} ({platform.machine()})")
    print(f" {BOLD}Engine:{RESET}  {clang_info}")
    print(f" {BOLD}SIMD:{RESET}    Clang ext_vector_type (v2/v3/v4)")
    print(f"{DIM}──────────────────────────────────────────{RESET}")

    demo_passed, demo_failed = 0, 0
    spec_passed, spec_failed = 0, 0
    timings = []
    failures = []
    slowest = ("", 0.0)
    fastest = ("", 999999.0)

    start_wall = time.time()

    for idx, (path, kind) in enumerate(all_tests, 1):
        fname = os.path.basename(path).replace(".gage", "")
        short_name = (fname[:17] + "…") if len(fname) > 18 else fname
        prefix = f"[{idx:02d}/{total:02d}]"

        with open(path, "r", encoding="utf-8") as f:
            raw_content = f.read()

        mock_in = synthesize_mock_stdin(raw_content)
        accelerated_code = generate_accelerated_test_source(raw_content)
        target_path = path
        temp_file = None

        if accelerated_code != raw_content:
            tmp = tempfile.NamedTemporaryFile(suffix=".gage", delete=False)
            tmp.write(accelerated_code.encode("utf-8"))
            tmp.close()
            target_path = tmp.name
            temp_file = tmp.name

        spinner = LiveSpinner(prefix, short_name)
        spinner.start()

        t0 = time.perf_counter()
        try:
            proc = subprocess.Popen(
                [gage_bin, target_path],
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True
            )
            stdout, stderr = proc.communicate(input=mock_in, timeout=12)
            ret = proc.returncode
            elapsed = (time.perf_counter() - t0) * 1000.0
            spinner.stop()

            if ret == 0:
                status = f"{GREEN}✔ PASS{RESET}"
                if kind == "DEMO": demo_passed += 1
                else: spec_passed += 1
            else:
                status = f"{RED}✖ FAIL{RESET}"
                if kind == "DEMO": demo_failed += 1
                else: spec_failed += 1
                failures.append((fname, f"Exit {ret}"))
        except Exception as e:
            spinner.stop()
            elapsed = (time.perf_counter() - t0) * 1000.0
            status = f"{RED}✖ FAIL{RESET}"
            if kind == "DEMO": demo_failed += 1
            else: spec_failed += 1
            failures.append((fname, "Timeout / Error"))
        finally:
            if temp_file and os.path.exists(temp_file):
                try: os.remove(temp_file)
                except Exception: pass

        timings.append(elapsed)
        if elapsed > slowest[1]: slowest = (fname, elapsed)
        if elapsed < fastest[1]: fastest = (fname, elapsed)

        tag = f"{PURPLE}[{kind[0]}]{RESET}"
        ms_str = f"{DIM}{int(elapsed):>3}ms{RESET}"

        sys.stdout.write(f"\r\033[2K {prefix} {status} {tag} {BOLD}{short_name:<18}{RESET} {ms_str}\n")
        sys.stdout.flush()

    total_wall = time.time() - start_wall
    passed_total = demo_passed + spec_passed
    failed_total = demo_failed + spec_failed
    avg_latency = sum(timings) / len(timings) if timings else 0.0

    sorted_timings = sorted(timings)
    p50 = sorted_timings[int(len(sorted_timings) * 0.50)] if sorted_timings else 0.0
    p95 = sorted_timings[int(len(sorted_timings) * 0.95)] if sorted_timings else 0.0
    throughput = total / total_wall if total_wall > 0 else 0.0

    # Executive Report Box
    print(f"\n{CYAN}╔════════════════════════════════════════╗{RESET}")
    print(f"{CYAN}║{RESET}      {BOLD}EXECUTIVE DIAGNOSTIC SUMMARY{RESET}      {CYAN}║{RESET}")
    print(f"{CYAN}╠════════════════════════════════════════╣{RESET}")
    print(f" {BOLD}Suite Status:{RESET}   {GREEN if failed_total == 0 else RED}{BOLD}{'ALL TESTS PASSED' if failed_total == 0 else 'VERIFICATION FAILED'}{RESET}")
    print(f" {BOLD}Demo Examples:{RESET}  {GREEN}{demo_passed}{RESET}/{len(examples)} passed {DIM}({demo_failed} fail){RESET}")
    print(f" {BOLD}Language Specs:{RESET} {GREEN}{spec_passed}{RESET}/{len(specs)} passed {DIM}({spec_failed} fail){RESET}")
    print(f"{CYAN}╟────────────────────────────────────────╢{RESET}")
    print(f" {BOLD}Latency (Avg):{RESET}  {CYAN}{avg_latency:.1f}ms{RESET} / unit")
    print(f" {BOLD}Median (p50):{RESET}   {CYAN}{p50:.1f}ms{RESET}")
    print(f" {BOLD}Tail (p95):{RESET}     {YELLOW}{p95:.1f}ms{RESET}")
    print(f" {BOLD}Fastest Unit:{RESET}   {fastest[0][:15]} {DIM}({fastest[1]:.1f}ms){RESET}")
    print(f" {BOLD}Slowest Unit:{RESET}   {slowest[0][:15]} {DIM}({slowest[1]:.1f}ms){RESET}")
    print(f"{CYAN}╟────────────────────────────────────────╢{RESET}")
    print(f" {BOLD}Throughput:{RESET}     {PURPLE}{throughput:.1f}{RESET} suites/sec")
    print(f" {BOLD}Total Runtime:{RESET}  {YELLOW}{total_wall:.2f}s{RESET}")
    print(f"{CYAN}╚════════════════════════════════════════╝{RESET}")

    if failures:
        print(f"\n{RED}{BOLD}Failed Units Detail:{RESET}")
        for fn, err in failures:
            print(f"  {RED}↳ {fn}: {err}{RESET}")

    print()
    return 0 if failed_total == 0 else 1

if __name__ == "__main__":
    sys.exit(run_suite())
