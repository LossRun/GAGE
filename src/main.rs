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
    println!("\x1b[1;36m⚡ Gage 2.0 Interactive Shell\x1b[0m");
    println!("\x1b[90mType code directly. Arrow keys navigate history. Type \x1b[33mexit\x1b[90m to quit.\x1b[0m\n");

    let mut rl = Editor::new().unwrap();
    rl.set_helper(Some(GageHelper));

    let history_path = format!("{}/.gage_history", env::var("HOME").unwrap_or_else(|_| "/data/data/com.termux/files/home".into()));
    let _ = rl.load_history(&history_path);

    let mut session_code = String::new();

    loop {
        let readline = rl.readline("\x1b[1;37m❯\x1b[0m ");
        match readline {
            Ok(line) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                let _ = rl.add_history_entry(line.as_str());

                if trimmed == "exit" || trimmed == "quit" {
                    println!("\x1b[90mGoodbye!\x1b[0m");
                    break;
                }
                if trimmed == "clear" {
                    print!("\x1b[H\x1b[2J");
                    let _ = io::stdout().flush();
                    continue;
                }

                let to_compile = if !trimmed.ends_with(';') && !trimmed.ends_with('}') {
                    if trimmed.starts_with("let ") || trimmed.contains('=') {
                        format!("{};\n", trimmed)
                    } else {
                        format!("println({});\n", trimmed)
                    }
                } else {
                    format!("{}\n", trimmed)
                };

                let candidate_code = format!("{}{}", session_code, to_compile);

                let mut lexer = Lexer::new(&candidate_code);
                let tokens = match lexer.tokenize() {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("\x1b[31m[Lex Error]\x1b[0m Line {}:{}: {}", e.line, e.column, e.message);
                        continue;
                    }
                };

                let mut parser = Parser::new(tokens);
                let program = match parser.parse() {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("\x1b[31m[Parse Error]\x1b[0m Line {}:{}: {}", e.line, e.column, e.message);
                        continue;
                    }
                };

                let mut checker = TypeChecker::new();
                if let Err(e) = checker.check(&program) {
                    eprintln!("\x1b[31m[Type Error]\x1b[0m {}", e);
                    continue;
                }

                let codegen = CodeGen::new();
                let c_code = codegen.generate(&program);

                let temp_c = "/data/data/com.termux/files/usr/tmp/gage_repl.tmp.c";
                let temp_bin = "/data/data/com.termux/files/usr/tmp/gage_repl.tmp";

                if fs::write(temp_c, &c_code).is_err() {
                    eprintln!("\x1b[31m[IO Error]\x1b[0m Failed to write temporary REPL source");
                    continue;
                }

                let clang_status = Command::new("clang")
                    .args(&["-O0", "-lm", temp_c, "-o", temp_bin])
                    .output();

                match clang_status {
                    Ok(out) if out.status.success() => {
                        let _ = Command::new(temp_bin).status();
                        session_code.push_str(&to_compile);
                    }
                    Ok(out) => {
                        eprintln!("\x1b[31m[Clang Error]\x1b[0m\n{}", String::from_utf8_lossy(&out.stderr));
                    }
                    Err(e) => {
                        eprintln!("\x1b[31m[Toolchain Error]\x1b[0m Failed to invoke clang: {}", e);
                    }
                }

                let _ = fs::remove_file(temp_c);
                let _ = fs::remove_file(temp_bin);
            }
            Err(ReadlineError::Interrupted) => {
                println!("\x1b[90m(Ctrl+C) Type exit to quit\x1b[0m");
            }
            Err(ReadlineError::Eof) => {
                println!("\n\x1b[90mGoodbye!\x1b[0m");
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

fn main() {
    let cli_args: Vec<String> = std::env::args().collect();
    if cli_args.len() >= 2 {
        let cmd = cli_args[1].as_str();
        match cmd {
            "--help" | "-h" | "help" => {
                print_custom_help();
                return;
            }
            "--version" | "-v" | "version" => {
                println!("\x1b[1;36mGAGE\x1b[0m version \x1b[1;32m0.1.0\x1b[0m (SIMD AOT native compiler)");
                return;
            }
            "--info" | "info" => {
                print_system_info();
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
                    eprintln!("\x1b[1;31mError:\x1b[0m --example requires an ID (e.g. \x1b[36mgage --example 51\x1b[0m)");
                    std::process::exit(1);
                }
                handle_run_example(&cli_args[2]);
                return;
            }
            _ => {
                if cmd.starts_with('-') && cmd != "--time" && cmd != "--vm" {
                    eprintln!("\x1b[1;31m[Error]\x1b[0m Unknown option: '{}'", cmd);
                    eprintln!("Run \x1b[1;33mgage --help\x1b[0m to see all available options.");
                    std::process::exit(1);
                }
            }
        }
    }

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        start_repl();
        return;
    }

    if args[1] == "-h" || args[1] == "--help" {
        print_usage();
        return;
    }

    if args[1] == "emit-c" {
        if args.len() < 3 {
            print_usage();
            return;
        }
        let in_file = &args[2];
        let out_file = if args.len() >= 5 && args[3] == "-o" {
            &args[4]
        } else {
            "out.c"
        };

        let source = fs::read_to_string(in_file).expect("Failed to read input gage file");
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
        fs::write(out_file, &c_code).expect("Failed to write output C file");
        println!("\x1b[32m✔ Emitted native C code to:\x1b[0m {}", out_file);
        return;
    }

    let mut benchmark = false;
    let mut use_vm = false;
    let mut file_idx = 1;

    if args[1] == "--time" {
        benchmark = true;
        file_idx = 2;
    } else if args[1] == "--vm" {
        use_vm = true;
        file_idx = 2;
    }

    if file_idx >= args.len() {
        print_usage();
        return;
    }

    let filepath = &args[file_idx];
    let source = match fs::read_to_string(filepath) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("\x1b[31m[Error] Could not find file:\x1b[0m {}", filepath);
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
            println!("\n⏱️  \x1b[1;36mBenchmark Results (VM):\x1b[0m");
            println!("  Frontend (Lex + Parse + Check): {:.2} ms", (t_frontend - t_start).as_secs_f64() * 1000.0);
            println!("  VM Execution Time:              {:.2} ms", (vm_end - vm_start).as_secs_f64() * 1000.0);
            println!("  Total Time:                     {:.2} ms", (vm_end - t_start).as_secs_f64() * 1000.0);
        }
        return;
    }

    let codegen = CodeGen::new();
    let c_code = codegen.generate(&program);

        let cache_dir = "/data/data/com.termux/files/usr/tmp/gage_cache";
    let _ = fs::create_dir_all(cache_dir);
    let mut h: u64 = 0xcbf29ce484222325;
    for b in source.bytes() { h = (h ^ (b as u64)).wrapping_mul(0x100000001b3); }
    let cached_bin = format!("{}/bin_{:x}", cache_dir, h);

    if !std::path::Path::new(&cached_bin).exists() {
        let codegen = CodeGen::new();
        let c_code = codegen.generate(&program);
        let temp_c = format!("{}/src_{:x}.c", cache_dir, h);
        let _ = fs::write(&temp_c, &c_code);
        let clang_status = Command::new("clang").args(&["-O1", "-lm", &temp_c, "-o", &cached_bin]).output();
        let _ = fs::remove_file(&temp_c);
        match clang_status {
            Ok(out) if out.status.success() => { let _ = Command::new("chmod").args(&["+x", &cached_bin]).status(); }
            Ok(out) => { eprintln!("Compilation error:
{}", String::from_utf8_lossy(&out.stderr)); return; }
            Err(e) => { eprintln!("Failed to run clang: {}", e); return; }
        }
    }

    let _ = Command::new(&cached_bin).status();
}
