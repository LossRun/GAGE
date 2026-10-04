#![allow(warnings)]

mod token;
mod lexer;
mod ast;
mod parser;
mod types;
mod codegen;
mod bytecode;
mod vm;
mod compiler;

use std::borrow::Cow;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::Command;
use std::time::Instant;

use rustyline::completion::Completer;
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Editor, Helper};

use lexer::Lexer;
use parser::Parser;
use types::TypeChecker;
use codegen::CodeGen;
use compiler::Compiler;
use vm::VM;

struct GageHelper;

impl Completer for GageHelper {
    type Candidate = String;
}
impl Hinter for GageHelper {
    type Hint = String;
}
impl Validator for GageHelper {}
impl Helper for GageHelper {}

impl Highlighter for GageHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        let mut out = String::new();
        let words = [
            "let", "fn", "return", "class", "new", "this", "if", "else", 
            "while", "for", "in", "loop", "break", "step", "print", "println",
            "vec2", "vec3", "vec4", "dot", "cross", "length", "normalize",
            "read_file", "write_file", "input"
        ];

        let mut in_str = false;
        let mut cur = String::new();

        let flush_token = |buf: &mut String, target: &mut String| {
            if buf.is_empty() { return; }
            if words.contains(&buf.as_str()) {
                target.push_str("\x1b[1;35m");
                target.push_str(buf);
                target.push_str("\x1b[0m");
            } else if buf.chars().all(|c| c.is_digit(10) || c == '.') {
                target.push_str("\x1b[36m");
                target.push_str(buf);
                target.push_str("\x1b[0m");
            } else {
                target.push_str(buf);
            }
            buf.clear();
        };

        for ch in line.chars() {
            if ch == '"' {
                flush_token(&mut cur, &mut out);
                if !in_str {
                    out.push_str("\x1b[32m\"");
                    in_str = true;
                } else {
                    out.push_str("\"\x1b[0m");
                    in_str = false;
                }
                continue;
            }

            if in_str {
                out.push(ch);
                continue;
            }

            if ch.is_alphanumeric() || ch == '_' {
                cur.push(ch);
            } else {
                flush_token(&mut cur, &mut out);
                if "=+-*/%<>!&|".contains(ch) {
                    out.push_str("\x1b[33m");
                    out.push(ch);
                    out.push_str("\x1b[0m");
                } else {
                    out.push(ch);
                }
            }
        }
        flush_token(&mut cur, &mut out);
        Cow::Owned(out)
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _forced: bool) -> bool {
        true
    }
}

fn print_usage() {
    println!("\x1b[1;36m⚡ Gage Programming Language (v2.0)\x1b[0m");
    println!("Usage:");
    println!("  gage                                \x1b[90mLaunch interactive REPL\x1b[0m");
    println!("  gage <file.gage>                    \x1b[90mCompile and run natively (AOT Clang)\x1b[0m");
    println!("  gage --vm <file.gage>               \x1b[90mExecute via Bytecode VM\x1b[0m");
    println!("  gage --time <file.gage>             \x1b[90mBenchmark execution times\x1b[0m");
    println!("  gage emit-c <file.gage> -o <out.c>  \x1b[90mEmit generated C source file\x1b[0m");
}

fn start_repl() {
    println!("GAGE 0.2.0 (native-aot, Oct 2026) [Clang AOT on aarch64-linux-android]");
    println!("Type \"help\", \"copyright\", \"credits\" or \"license\" for more information.");

    let mut rl = Editor::new().unwrap();
    rl.set_helper(Some(GageHelper));

    let history_path = format!("{}/.gage_history", env::var("HOME").unwrap_or_else(|_| "/data/data/com.termux/files/home".into()));
    let _ = rl.load_history(&history_path);

    let mut session_code = String::new();
    let mut multi_buf: Vec<String> = Vec::new();

    loop {
        let is_continuation = !multi_buf.is_empty();
        // Bold Purple / Magenta ANSI: \x1b[1;35m
        let prompt = if is_continuation { "\x1b[1;35m... \x1b[0m" } else { "\x1b[1;35m>>> \x1b[0m" };

        match rl.readline(prompt) {
            Ok(line) => {
                let trimmed = line.trim();

                if trimmed.is_empty() {
                    if !is_continuation {
                        continue;
                    }
                } else {
                    let _ = rl.add_history_entry(line.as_str());
                }

                // Python-style Top Level Commands
                if !is_continuation {
                    match trimmed {
                        "exit" | "quit" | "exit()" | "quit()" => {
                            break;
                        }
                        "clear" | "clear()" => {
                            print!("\x1b[H\x1b[2J");
                            let _ = io::stdout().flush();
                            continue;
                        }
                        "help" => {
                            println!("Type help() for interactive help, or check out these basics:");
                            println!("  • Variables:     let x = 42;");
                            println!("  • SIMD Vectors:  let v = vec3(1.0, 2.0, 3.0) * 2.0;");
                            println!("  • Canvas:        let c = gage_canvas_create(40, 15);");
                            println!("  • Functions:     fn add(a, b) {{ return a + b; }}");
                            println!("  • Exit:          exit or Ctrl+D");
                            continue;
                        }
                        "help()" => {
                            println!("Welcome to GAGE 0.2.0 interactive help utility!");
                            println!("\nGAGE is a high-performance simulation language with native SIMD support.");
                            println!("Expressions typed directly at the '>>>' prompt are evaluated immediately.");
                            println!("Statements like 'let', 'fn', and assignments persist in session memory.");
                            continue;
                        }
                        "license" => {
                            println!("Type license() to see the full license text");
                            continue;
                        }
                        "license()" => {
                            println!("GAGE Software License");
                            println!("=====================");
                            println!("");
                            println!("MIT License");
                            println!("");
                            println!("Copyright (c) 2026 LossRun / GAGE Project Contributors");
                            println!("");
                            println!("Permission is hereby granted, free of charge, to any person obtaining a copy");
                            println!("of this software and associated documentation files (the \"Software\"), to deal");
                            println!("in the Software without restriction, including without limitation the rights");
                            println!("to use, copy, modify, merge, publish, distribute, sublicense, and/or sell");
                            println!("copies of the Software, and to permit persons to whom the Software is");
                            println!("furnished to do so, subject to the following conditions:");
                            println!("");
                            println!("The above copyright notice and this permission notice shall be included in all");
                            println!("copies or substantial portions of the Software.");
                            println!("");
                            println!("THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR");
                            println!("IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,");
                            println!("FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE");
                            println!("AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER");
                            println!("LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,");
                            println!("OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE");
                            println!("SOFTWARE.");
                            continue;
                        }
                        "copyright" | "copyright()" => {
                            println!("Copyright (c) 2026 LossRun / GAGE Project Contributors.");
                            println!("All Rights Reserved.");
                            continue;
                        }
                        "credits" | "credits()" => {
                            println!("Thanks to all contributors and the open-source systems community.");
                            println!("GAGE is powered by Rust, LLVM/Clang AOT compilation, and native SIMD.");
                            continue;
                        }
                        _ => {}
                    }
                }

                multi_buf.push(line);

                let combined_input = multi_buf.join("\n");
                let mut balance: i32 = 0;
                for c in combined_input.chars() {
                    match c {
                        '{' | '(' | '[' => balance += 1,
                        '}' | ')' | ']' => balance -= 1,
                        _ => {}
                    }
                }

                if balance > 0 {
                    continue;
                }

                let current_input = combined_input.trim().to_string();
                multi_buf.clear();

                if current_input.is_empty() {
                    continue;
                }

                let is_assignment = {
                    let chars: Vec<char> = current_input.chars().collect();
                    let mut found = false;
                    for i in 0..chars.len() {
                        if chars[i] == '=' {
                            let prev = if i > 0 { chars[i - 1] } else { ' ' };
                            let next = if i + 1 < chars.len() { chars[i + 1] } else { ' ' };
                            if prev != '=' && prev != '!' && prev != '<' && prev != '>' && next != '=' {
                                found = true;
                                break;
                            }
                        }
                    }
                    found
                };

                let is_explicit_print = current_input.starts_with("println(") || current_input.starts_with("print(");

                let is_stmt = current_input.starts_with("let ")
                    || current_input.starts_with("fn ")
                    || current_input.starts_with("class ")
                    || current_input.starts_with("while ")
                    || current_input.starts_with("for ")
                    || current_input.starts_with("if ")
                    || is_assignment;

                let (to_compile, persist) = if is_explicit_print {
                    let mut s = current_input.clone();
                    if !s.ends_with(';') { s.push(';'); }
                    (format!("{}\n", s), false)
                } else if is_stmt {
                    let mut s = current_input.clone();
                    if !s.ends_with(';') && !s.ends_with('}') { s.push(';'); }
                    (format!("{}\n", s), true)
                } else {
                    let mut expr = current_input.clone();
                    if expr.ends_with(';') { expr.pop(); }
                    (format!("println({});\n", expr), false)
                };

                let candidate_code = format!("{}{}", session_code, to_compile);

                let mut lexer = Lexer::new(&candidate_code);
                let tokens = match lexer.tokenize() {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("  File \"<stdin>\", line {}:{}", e.line, e.column);
                        eprintln!("\x1b[31mSyntaxError:\x1b[0m {}", e.message);
                        continue;
                    }
                };

                let mut parser = Parser::new(tokens);
                let program = match parser.parse() {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("  File \"<stdin>\", line {}:{}", e.line, e.column);
                        eprintln!("\x1b[31mSyntaxError:\x1b[0m {}", e.message);
                        continue;
                    }
                };

                let mut checker = TypeChecker::new();
                if let Err(e) = checker.check(&program) {
                    eprintln!("  File \"<stdin>\", line 1");
                    eprintln!("\x1b[31mTypeError:\x1b[0m {}", e);
                    continue;
                }

                let codegen = CodeGen::new();
                let c_code = codegen.generate(&program);

                let temp_c = "/data/data/com.termux/files/usr/tmp/gage_repl.tmp.c";
                let temp_bin = "/data/data/com.termux/files/usr/tmp/gage_repl.tmp";

                if fs::write(temp_c, &c_code).is_err() {
                    eprintln!("\x1b[31mOSError:\x1b[0m Failed to write temporary REPL source");
                    continue;
                }

                let clang_status = Command::new("clang")
                    .args(&["-O0", "-lm", temp_c, "-o", temp_bin])
                    .output();

                match clang_status {
                    Ok(out) if out.status.success() => {
                        let _ = Command::new(temp_bin).status();
                        if persist {
                            session_code.push_str(&to_compile);
                        }
                    }
                    Ok(out) => {
                        eprintln!("\x1b[31mRuntimeError:\x1b[0m Compilation failed\n{}", String::from_utf8_lossy(&out.stderr));
                    }
                    Err(e) => {
                        eprintln!("\x1b[31mToolchainError:\x1b[0m Failed to invoke clang: {}", e);
                    }
                }

                let _ = fs::remove_file(temp_c);
                let _ = fs::remove_file(temp_bin);
            }
            Err(ReadlineError::Interrupted) => {
                if !multi_buf.is_empty() {
                    multi_buf.clear();
                    println!("\nKeyboardInterrupt");
                } else {
                    println!("\nKeyboardInterrupt (Use exit or Ctrl+D to quit)");
                }
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                eprintln!("\x1b[31m[Readline Error]\x1b[0m {:?}", err);
                break;
            }
        }
    }

    let _ = rl.save_history(&history_path);
}

fn print_gage_banner() {
    println!("\x1b[1;36m   ⚡ GAGE v0.1.0\x1b[0m \x1b[90m— High-Performance SIMD Simulation Language\x1b[0m");
    println!("\x1b[90m   ──────────────────────────────────────────────────────────\x1b[0m");
}

fn print_custom_help() {
    print_gage_banner();
    println!("\x1b[1mUSAGE:\x1b[0m");
    println!("  gage [OPTIONS] <file.gage>");
    println!("  gage [COMMAND]\n");
    println!("\x1b[1mCORE COMMANDS:\x1b[0m");
    println!("  \x1b[1;32mgage <file.gage>\x1b[0m          Compile natively (Clang + FNV-1a cache) & run");
    println!("  \x1b[1;32mgage run <file.gage>\x1b[0m      Explicit native compile and execute");
    println!("  \x1b[1;32mgage vm <file.gage>\x1b[0m       Execute on stack bytecode virtual machine");
    println!("  \x1b[1;32mgage check <file.gage>\x1b[0m    Validate syntax and check types without compiling");
    println!("  \x1b[1;32mgage emit-c <file.gage>\x1b[0m   Transpile GAGE source to optimized C99\n");
    println!("\x1b[1mRAPID TOOLS & EXAMPLES:\x1b[0m");
    println!("  \x1b[1;33mgage --example <id>\x1b[0m       Run an example by ID (e.g. \x1b[36mgage --example 51\x1b[0m)");
    println!("  \x1b[1;33mgage --examples, --list\x1b[0m   List all available numbered examples");
    println!("  \x1b[1;33mgage --test\x1b[0m               Run automated regression test suite\n");
    println!("\x1b[1mDIAGNOSTICS & SYSTEM:\x1b[0m");
    println!("  \x1b[1;35mgage --info\x1b[0m               Display toolchain, SIMD vectors, and environment");
    println!("  \x1b[1;35mgage --version, -v\x1b[0m        Display GAGE compiler version");
    println!("  \x1b[1;35mgage --help, -h\x1b[0m           Show this documentation menu\n");
    println!("\x1b[1mINTERACTIVE MODE:\x1b[0m");
    println!("  \x1b[1;36mgage\x1b[0m                      Launch the interactive REPL\n");
}

fn print_system_info() {
    print_gage_banner();
    println!("\x1b[1mSystem & Toolchain Diagnostics:\x1b[0m");
    println!("  \x1b[1;32mHost Platform:\x1b[0m      {} ({})", std::env::consts::OS, std::env::consts::ARCH);
    println!("  \x1b[1;32mSIMD Pipeline:\x1b[0m      Clang ext_vector_type (vec2, vec3, vec4, mat4)");
    println!("  \x1b[1;32mCompilation Cache:\x1b[0m  FNV-1a content-addressed /tmp/gage_cache");

    let clang_out = std::process::Command::new("clang").arg("--version").output();
    match clang_out {
        Ok(out) => {
            let s = String::from_utf8_lossy(&out.stdout);
            let first_line = s.lines().next().unwrap_or("Detected");
            println!("  \x1b[1;32mBackend Toolchain:\x1b[0m  {}", first_line);
        }
        Err(_) => {
            println!("  \x1b[1;31mBackend Toolchain:\x1b[0m  clang not found in PATH!");
        }
    }
    println!();
}

fn find_examples_dir() -> Option<std::path::PathBuf> {
    let candidates = [
        std::path::PathBuf::from("examples"),
        std::path::PathBuf::from("/sdcard/GAGE/examples"),
    ];
    for p in &candidates {
        if p.is_dir() { return Some(p.clone()); }
    }
    if let Ok(mut exe) = std::env::current_exe() {
        exe.pop();
        let cand = exe.join("examples");
        if cand.is_dir() { return Some(cand); }
    }
    None
}

fn handle_list_examples() {
    print_gage_banner();
    let ex_dir = match find_examples_dir() {
        Some(d) => d,
        None => {
            eprintln!("\x1b[1;31mError:\x1b[0m Could not locate examples directory.");
            std::process::exit(1);
        }
    };

    let entries = match std::fs::read_dir(&ex_dir) {
        Ok(e) => e,
        Err(err) => {
            eprintln!("\x1b[1;31mError reading examples:\x1b[0m {}", err);
            std::process::exit(1);
        }
    };

    let mut files = Vec::new();
    for entry in entries.flatten() {
        let p = entry.path();
        if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
            if name.ends_with(".gage") {
                files.push(name.to_string());
            }
        }
    }
    files.sort();

    println!("\x1b[1mAvailable Examples ({})\x1b[0m:\n", files.len());
    for f in &files {
        let parts: Vec<&str> = f.splitn(2, '_').collect();
        if parts.len() == 2 {
            let id = parts[0];
            let name = parts[1].trim_end_matches(".gage").replace('_', " ");
            println!("  \x1b[1;33m[{:>2}]\x1b[0m \x1b[1m{:<28}\x1b[0m \x1b[90m({})\x1b[0m", id, name, f);
        } else {
            println!("  \x1b[1;33m[--]\x1b[0m {}", f);
        }
    }
    println!("\n\x1b[90m💡 Run any example via:\x1b[0m \x1b[1;36mgage --example <id>\x1b[0m\n");
}

fn handle_run_example(target: &str) {
    let ex_dir = match find_examples_dir() {
        Some(d) => d,
        None => {
            eprintln!("\x1b[1;31mError:\x1b[0m Could not locate examples directory.");
            std::process::exit(1);
        }
    };

    let entries = match std::fs::read_dir(&ex_dir) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("\x1b[1;31mError reading examples:\x1b[0m {}", e);
            std::process::exit(1);
        }
    };

    let target_prefix = if target.len() == 1 { format!("0{}_", target) } else { format!("{}_", target) };
    let target_raw = format!("{}_", target);
    let mut matched = None;

    for entry in entries.flatten() {
        let p = entry.path();
        if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
            if name.ends_with(".gage") && (name.starts_with(&target_prefix) || name.starts_with(&target_raw)) {
                matched = Some(p);
                break;
            }
        }
    }

    match matched {
        Some(file) => {
            let current_exe = std::env::current_exe().unwrap_or_else(|_| std::path::PathBuf::from("gage"));
            let status = std::process::Command::new(current_exe)
                .arg(file)
                .status();
            match status {
                Ok(s) => std::process::exit(s.code().unwrap_or(0)),
                Err(err) => {
                    eprintln!("\x1b[1;31mError executing example:\x1b[0m {}", err);
                    std::process::exit(1);
                }
            }
        }
        None => {
            eprintln!("\x1b[1;31mError:\x1b[0m No example found starting with id '{}'.", target);
            eprintln!("\x1b[90mRun \x1b[1;33mgage --examples\x1b[0m \x1b[90mto view all available options.\x1b[0m");
            std::process::exit(1);
        }
    }
}

fn handle_run_tests() {
    print_gage_banner();
    let candidates = [
        std::path::PathBuf::from("test_runner.py"),
        std::path::PathBuf::from("/sdcard/GAGE/test_runner.py"),
    ];
    let runner = candidates.iter().find(|p| p.exists());
    let path = match runner {
        Some(p) => p,
        None => {
            eprintln!("\x1b[1;31mError:\x1b[0m test_runner.py not found.");
            std::process::exit(1);
        }
    };

    println!("\x1b[1;33m==> Launching GAGE Regression Test Suite...\x1b[0m\n");
    let status = std::process::Command::new("python3")
        .arg(path)
        .status();

    match status {
        Ok(s) => std::process::exit(s.code().unwrap_or(0)),
        Err(err) => {
            eprintln!("\x1b[1;31mFailed to launch test_runner.py:\x1b[0m {}", err);
            std::process::exit(1);
        }
    }
}



fn handle_delete() {
    println!("\x1b[1;33m==> Deleting GAGE binaries and compiler artifacts...\x1b[0m");

    let mut removed_items = Vec::new();

    // 1. Remove the installed executable from Termux bin path
    if let Ok(prefix) = std::env::var("PREFIX") {
        let p = std::path::PathBuf::from(prefix).join("bin/gage");
        if p.exists() && std::fs::remove_file(&p).is_ok() {
            removed_items.push(p.to_string_lossy().to_string());
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if exe.exists() && std::fs::remove_file(&exe).is_ok() {
            removed_items.push(exe.to_string_lossy().to_string());
        }
    }

    // 2. Remove the Cargo release/debug build targets (~/.cargo_target_gage)
    if let Ok(home) = std::env::var("HOME") {
        let target_dir = std::path::PathBuf::from(home).join(".cargo_target_gage");
        if target_dir.exists() {
            let _ = std::fs::remove_dir_all(&target_dir);
            removed_items.push(target_dir.to_string_lossy().to_string());
        }
    }

    // 3. Remove all cached C runner binaries and scratch files
    let cache_dirs = [
        std::path::PathBuf::from("/data/data/com.termux/files/usr/tmp/gage_cache"),
        std::path::PathBuf::from("/tmp/gage_cache"),
    ];
    for dir in &cache_dirs {
        if dir.exists() {
            let _ = std::fs::remove_dir_all(dir);
            removed_items.push(dir.to_string_lossy().to_string());
        }
    }
    let _ = std::fs::remove_file("/data/data/com.termux/files/usr/tmp/gage_repl.tmp.c");
    let _ = std::fs::remove_file("/data/data/com.termux/files/usr/tmp/gage_repl.tmp");

    println!("\x1b[1;32m✔ All GAGE binaries and execution targets removed!\x1b[0m");
    for item in &removed_items {
        println!("  \x1b[90mRemoved:\x1b[0m {}", item);
    }
    println!("\n\x1b[1m📁 Source repository remains completely untouched:\x1b[0m /sdcard/GAGE");
    println!("\x1b[90mRun '\x1b[33mhash -r\x1b[90m' to clear the terminal command cache.\x1b[0m");
    std::process::exit(0);
}

fn handle_clean() {
    let cache_dirs = [
        std::path::PathBuf::from("/data/data/com.termux/files/usr/tmp/gage_cache"),
        std::path::PathBuf::from("/tmp/gage_cache"),
    ];
    let mut count = 0;
    for dir in &cache_dirs {
        if dir.exists() {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    if std::fs::remove_file(entry.path()).is_ok() {
                        count += 1;
                    }
                }
            }
            let _ = std::fs::remove_dir(dir);
        }
    }
    // Also remove temporary REPL scratch files
    let _ = std::fs::remove_file("/data/data/com.termux/files/usr/tmp/gage_repl.tmp.c");
    let _ = std::fs::remove_file("/data/data/com.termux/files/usr/tmp/gage_repl.tmp");

    println!("\x1b[1;32m✔ GAGE Cache Purged:\x1b[0m Removed {} cached binaries and scratch files.", count);
}

fn resolve_gage_file(target: &str) -> Option<String> {
    let p = std::path::Path::new(target);
    if p.is_file() {
        return Some(target.to_string());
    }
    let with_ext = format!("{}.gage", target);
    if std::path::Path::new(&with_ext).is_file() {
        return Some(with_ext);
    }
    if let Some(ex_dir) = find_examples_dir() {
        let inside = ex_dir.join(target);
        if inside.is_file() {
            return Some(inside.to_string_lossy().to_string());
        }
        let inside_ext = ex_dir.join(&with_ext);
        if inside_ext.is_file() {
            return Some(inside_ext.to_string_lossy().to_string());
        }
    }
    None
}

fn main() {
    let cli_args: Vec<String> = std::env::args().collect();

    // No arguments -> launch REPL
    if cli_args.len() == 1 {
        start_repl();
        return;
    }

    let first_arg = cli_args[1].as_str();

    // Handle Top-Level Commands
    match first_arg {
        "--help" | "-h" | "help" => {
            print_custom_help();
            return;
        }
        "--version" | "-v" | "version" => {
            println!("\x1b[1;36mGAGE\x1b[0m version \x1b[1;32m0.2.0\x1b[0m (SIMD AOT native compiler)");
            return;
        }
        "--info" | "info" => {
            print_system_info();
            return;
        }
        "--clean" | "clean" | "clear-cache" => {
            handle_clean();
            return;
        }
        "--delete" | "delete" | "--uninstall" | "uninstall" => {
            handle_delete();
            return;
        }
        "--test" | "test" => {
            handle_run_tests();
            return;
        }
        "--examples" | "--list" | "examples" | "list" => {
            handle_list_examples();
            return;
        }
        "--example" | "example" => {
            if cli_args.len() < 3 {
                eprintln!("\x1b[1;31mError:\x1b[0m 'example' requires an ID (e.g. \x1b[36mgage example 52\x1b[0m)");
                std::process::exit(1);
            }
            handle_run_example(&cli_args[2]);
            return;
        }
        "repl" | "shell" => {
            start_repl();
            return;
        }
        "emit-c" => {
            if cli_args.len() < 3 {
                print_usage();
                return;
            }
            let in_file = &cli_args[2];
            let out_file = if cli_args.len() >= 5 && cli_args[3] == "-o" {
                &cli_args[4]
            } else {
                "out.c"
            };

            let source = match fs::read_to_string(in_file) {
                Ok(s) => s,
                Err(_) => {
                    eprintln!("\x1b[31m[Error] Could not find file:\x1b[0m {}", in_file);
                    return;
                }
            };
            let mut lexer = Lexer::new(&source);
            let tokens = match lexer.tokenize() {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("[Lex Error] Line {}:{}: {}", e.line, e.column, e.message);
                    return;
                }
            };
            let mut parser = Parser::new(tokens);
            let program = match parser.parse() {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("[Parse Error] Line {}:{}: {}", e.line, e.column, e.message);
                    return;
                }
            };
            let mut checker = TypeChecker::new();
            if let Err(e) = checker.check(&program) {
                eprintln!("{}", e);
                return;
            }

            let c_code = CodeGen::new().generate(&program);
            let _ = fs::write(out_file, &c_code);
            println!("\x1b[32m✔ Emitted native C code to:\x1b[0m {}", out_file);
            return;
        }
        _ => {}
    }

    // Execution Flags
    let mut benchmark = false;
    let mut use_vm = false;
    let mut file_idx = 1;

    if first_arg == "--time" {
        benchmark = true;
        file_idx = 2;
    } else if first_arg == "--vm" {
        use_vm = true;
        file_idx = 2;
    } else if first_arg == "run" {
        file_idx = 2;
    }

    if file_idx >= cli_args.len() {
        print_usage();
        return;
    }

    let raw_target = &cli_args[file_idx];
    let resolved = match resolve_gage_file(raw_target) {
        Some(path) => path,
        None => {
            eprintln!("\x1b[1;31m[Error]\x1b[0m Unknown command or file not found: '\x1b[1;33m{}\x1b[0m'", raw_target);
            eprintln!("\n\x1b[1mAvailable GAGE Commands:\x1b[0m");
            eprintln!("  \x1b[36mgage clean\x1b[0m         Clear compiled cache binaries");
            eprintln!("  \x1b[36mgage test\x1b[0m          Run regression test suite");
            eprintln!("  \x1b[36mgage examples\x1b[0m      List all available demo programs");
            eprintln!("  \x1b[36mgage example <id>\x1b[0m  Run example by numeric ID");
            eprintln!("  \x1b[36mgage repl\x1b[0m          Launch interactive Python-style shell");
            eprintln!("  \x1b[36mgage help\x1b[0m          Display detailed usage guide");
            std::process::exit(1);
        }
    };

    let source = match fs::read_to_string(&resolved) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("\x1b[31m[Error] Failed to read file:\x1b[0m {}", resolved);
            return;
        }
    };

    let cache_dir = "/data/data/com.termux/files/usr/tmp/gage_cache";
    let _ = fs::create_dir_all(cache_dir);
    let mut h_fast: u64 = 0xcbf29ce484222325;
    for b in source.bytes() { h_fast = (h_fast ^ (b as u64)).wrapping_mul(0x100000001b3); }
    let fast_bin = format!("{}/bin_{:x}", cache_dir, h_fast);
    if !use_vm && !benchmark && std::path::Path::new(&fast_bin).exists() {
        let _ = Command::new(&fast_bin).status();
        return;
    }

    let t_start = Instant::now();

    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("\x1b[31m[Lex Error]\x1b[0m Line {}:{}: {}", e.line, e.column, e.message);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("\x1b[31m[Parse Error]\x1b[0m Line {}:{}: {}", e.line, e.column, e.message);
            return;
        }
    };

    let mut checker = TypeChecker::new();
    if let Err(e) = checker.check(&program) {
        eprintln!("\x1b[31m{}\x1b[0m", e);
        return;
    }

    let t_frontend = Instant::now();

    if use_vm {
        let mut comp = Compiler::new();
        let chunk = match comp.compile(&program) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("\x1b[31m[Compiler Error]\x1b[0m {}", e);
                return;
            }
        };

        let mut vm = VM::new(chunk);
        let vm_start = Instant::now();
        if let Err(e) = vm.run() {
            eprintln!("\x1b[31m[VM Runtime Error]\x1b[0m {:?}", e);
        }
        let vm_end = Instant::now();

        if benchmark {
            println!("\n⏱️️  \x1b[1;36mBenchmark Results (VM):\x1b[0m");
            println!("  Frontend (Lex + Parse + Check): {:.2} ms", (t_frontend - t_start).as_secs_f64() * 1000.0);
            println!("  VM Execution Time:              {:.2} ms", (vm_end - vm_start).as_secs_f64() * 1000.0);
            println!("  Total Time:                     {:.2} ms", (vm_end - t_start).as_secs_f64() * 1000.0);
        }
        return;
    }

    let codegen = CodeGen::new();
    let c_code = codegen.generate(&program);

    let temp_c = format!("{}/src_{:x}.c", cache_dir, h_fast);
    let _ = fs::write(&temp_c, &c_code);
    let clang_status = Command::new("clang").args(&["-O1", "-lm", &temp_c, "-o", &fast_bin]).output();
    let _ = fs::remove_file(&temp_c);
    match clang_status {
        Ok(out) if out.status.success() => {
            let _ = Command::new("chmod").args(&["+x", &fast_bin]).status();
        }
        Ok(out) => {
            eprintln!("Compilation error:\n{}", String::from_utf8_lossy(&out.stderr));
            return;
        }
        Err(e) => {
            eprintln!("Failed to run clang: {}", e);
            return;
        }
    }

    let _ = Command::new(&fast_bin).status();
}
