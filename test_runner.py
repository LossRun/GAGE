#!/usr/bin/env python3
import os
import sys
import glob
import subprocess
import time
import re
import threading
import tempfile

CYAN = "\033[1;36m"
GREEN = "\033[1;32m"
RED = "\033[1;31m"
PURPLE = "\033[1;35m"
BOLD = "\033[1m"
DIM = "\033[90m"
RESET = "\033[0m"

SPINNER_FRAMES = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]

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
            # Clean full-line erase with padded output to prevent ghost characters
            sys.stdout.write(f"\r\033[2K {self.prefix} {CYAN}{frame}{RESET} {DIM}{self.name[:20]}{RESET}")
            sys.stdout.flush()
            idx += 1
            time.sleep(0.065)

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

    print(f"\n{CYAN}⚡ GAGE TEST HARNESS{RESET} {DIM}v0.1.0{RESET}")
    print(f"{DIM}────────────────────────────────────────{RESET}")

    passed = 0
    failed = 0
    timings = []
    failures = []
    start_wall = time.time()

    for idx, (path, kind) in enumerate(all_tests, 1):
        fname = os.path.basename(path).replace(".gage", "")
        # Compact display name tailored for mobile screens
        short_name = (fname[:18] + "…") if len(fname) > 19 else fname
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
            stdout, stderr = proc.communicate(input=mock_in, timeout=10)
            ret = proc.returncode
            elapsed = (time.perf_counter() - t0) * 1000.0
            spinner.stop()

            if ret == 0:
                status = f"{GREEN}✔ PASS{RESET}"
                passed += 1
            else:
                status = f"{RED}✖ FAIL{RESET}"
                failed += 1
                failures.append((fname, f"Exit {ret}"))
        except Exception as e:
            spinner.stop()
            elapsed = (time.perf_counter() - t0) * 1000.0
            status = f"{RED}✖ FAIL{RESET}"
            failed += 1
            failures.append((fname, "Timed out / Error"))
        finally:
            if temp_file and os.path.exists(temp_file):
                try: os.remove(temp_file)
                except Exception: pass

        timings.append(elapsed)
        tag = f"{PURPLE}[{kind[0]}]{RESET}"
        ms_str = f"{DIM}{int(elapsed):>3}ms{RESET}"

        # Clears whole line (\033[2K) then writes clean, un-wrapped row
        sys.stdout.write(f"\r\033[2K {prefix} {status} {tag} {BOLD}{short_name:<19}{RESET} {ms_str}\n")
        sys.stdout.flush()

    total_wall = time.time() - start_wall
    avg_latency = sum(timings) / len(timings) if timings else 0.0

    print(f"{DIM}────────────────────────────────────────{RESET}")
    if failed == 0:
        print(f"{GREEN}{BOLD}✨ ALL {total} SYSTEM TESTS PASSED! ✨{RESET}")
    else:
        print(f"{RED}{BOLD}✖ {failed} FAILED / {passed} PASSED{RESET}")
        for fn, err in failures:
            print(f"  {RED}↳ {fn}: {err}{RESET}")

    print(f" {BOLD}Total Time:{RESET} {total_wall:.2f}s  {DIM}|{RESET}  {BOLD}Avg:{RESET} {avg_latency:.1f}ms/unit")
    print(f"{DIM}────────────────────────────────────────{RESET}\n")
    return 0 if failed == 0 else 1

if __name__ == "__main__":
    sys.exit(run_suite())
