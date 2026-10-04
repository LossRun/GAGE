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
        let readline = rl.readline("\x1b[1;32mgage \x1b[1;34m❯\x1b[0m ");
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

fn main() {
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

    let temp_c = "/data/data/com.termux/files/usr/tmp/gage_exec.tmp.c";
    let temp_bin = "/data/data/com.termux/files/usr/tmp/gage_exec.tmp";

    fs::write(temp_c, &c_code).expect("Failed to write temporary C file");

    let t_compile_start = Instant::now();
    let clang_status = Command::new("clang")
        .args(&["-O2", "-lm", temp_c, "-o", temp_bin])
        .output();

    let t_compile_end = Instant::now();

    match clang_status {
        Ok(out) if out.status.success() => {
            let t_exec_start = Instant::now();
            let _ = Command::new(temp_bin).status();
            let t_exec_end = Instant::now();

            if benchmark {
                println!("\n⏱️  \x1b[1;36mBenchmark Results (Native AOT):\x1b[0m");
                println!("  Frontend (Lex + Parse + Check): {:.2} ms", (t_frontend - t_start).as_secs_f64() * 1000.0);
                println!("  Clang C Compilation (-O2):       {:.2} ms", (t_compile_end - t_compile_start).as_secs_f64() * 1000.0);
                println!("  Native Execution Time:           {:.2} ms", (t_exec_end - t_exec_start).as_secs_f64() * 1000.0);
                println!("  Total End-to-End:                {:.2} ms", (t_exec_end - t_start).as_secs_f64() * 1000.0);
            }
        }
        Ok(out) => {
            eprintln!("\x1b[31m  ✖ Native compilation failed.\x1b[0m\nDetails:\nclang error:\n{}", String::from_utf8_lossy(&out.stderr));
        }
        Err(e) => {
            eprintln!("\x1b[31m  ✖ Failed to execute clang: {}\x1b[0m", e);
        }
    }

    let _ = fs::remove_file(temp_c);
    let _ = fs::remove_file(temp_bin);
}
