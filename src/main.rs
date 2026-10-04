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

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::Command;
use std::time::Instant;

use lexer::Lexer;
use parser::Parser;
use types::TypeChecker;
use codegen::CodeGen;
use compiler::Compiler;
use vm::VM;

fn print_usage() {
    println!("⚡ Gage Programming Language (v2.0)");
    println!("Usage:");
    println!("  gage                         Launch interactive REPL");
    println!("  gage <file.gage>             Compile and run natively (AOT Clang)");
    println!("  gage --vm <file.gage>        Execute via Bytecode VM");
    println!("  gage --time <file.gage>      Benchmark compilation and execution times");
    println!("  gage emit-c <file.gage> -o <out.c>  Emit generated C source file");
}

fn start_repl() {
    println!("⚡ Gage 2.0 Interactive REPL");
    println!("Type 'exit' or press Ctrl+D to quit.\n");

    let mut session_code = String::new();

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).unwrap() == 0 {
            println!("\nGoodbye!");
            break;
        }

        let trimmed = line.trim();
        if trimmed == "exit" || trimmed == "quit" {
            break;
        }
        if trimmed.is_empty() {
            continue;
        }

        // Auto-wrap bare expressions in println() for quick evaluation
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
                eprintln!("[Lex Error] Line {}:{}: {}", e.line, e.column, e.message);
                continue;
            }
        };

        let mut parser = Parser::new(tokens);
        let program = match parser.parse() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("[Parse Error] Line {}:{}: {}", e.line, e.column, e.message);
                continue;
            }
        };

        let mut checker = TypeChecker::new();
        if let Err(e) = checker.check(&program) {
            eprintln!("{}", e);
            continue;
        }

        let codegen = CodeGen::new();
        let c_code = codegen.generate(&program);

        let temp_c = "/data/data/com.termux/files/usr/tmp/gage_repl.tmp.c";
        let temp_bin = "/data/data/com.termux/files/usr/tmp/gage_repl.tmp";

        if fs::write(temp_c, &c_code).is_err() {
            eprintln!("[IO Error] Failed to write temporary REPL source");
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
                eprintln!("[Clang Error]\n{}", String::from_utf8_lossy(&out.stderr));
            }
            Err(e) => {
                eprintln!("[Toolchain Error] Failed to invoke clang: {}", e);
            }
        }

        let _ = fs::remove_file(temp_c);
        let _ = fs::remove_file(temp_bin);
    }
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

    // Command: gage emit-c <file.gage> -o <out.c>
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
        let tokens = lexer.tokenize().expect("Lex error");
        let mut parser = Parser::new(tokens);
        let program = parser.parse().expect("Parse error");
        let mut checker = TypeChecker::new();
        checker.check(&program).expect("Semantic error");

        let c_code = CodeGen::new().generate(&program);
        fs::write(out_file, &c_code).expect("Failed to write output C file");
        println!("✔ Emitted native C code to: {}", out_file);
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
            eprintln!("[Error] Could not find file: {}", filepath);
            return;
        }
    };

    let t_start = Instant::now();

    // 1. Lexing
    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("[Lex Error] Line {}:{}: {}", e.line, e.column, e.message);
            return;
        }
    };

    // 2. Parsing
    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("[Parse Error] Line {}:{}: {}", e.line, e.column, e.message);
            return;
        }
    };

    // 3. Type Checking
    let mut checker = TypeChecker::new();
    if let Err(e) = checker.check(&program) {
        eprintln!("{}", e);
        return;
    }

    let t_frontend = Instant::now();

    // VM Execution
    if use_vm {
        let mut comp = Compiler::new();
        let chunk = match comp.compile(&program) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[Compiler Error] {}", e);
                return;
            }
        };

        let mut vm = VM::new(chunk);
        let vm_start = Instant::now();
        if let Err(e) = vm.run() {
            eprintln!("[VM Runtime Error] {:?}", e);
        }
        let vm_end = Instant::now();

        if benchmark {
            println!("\n⏱️  Benchmark Results (VM):");
            println!("  Frontend (Lex + Parse + Check): {:.2} ms", (t_frontend - t_start).as_secs_f64() * 1000.0);
            println!("  VM Execution Time:              {:.2} ms", (vm_end - vm_start).as_secs_f64() * 1000.0);
            println!("  Total Time:                     {:.2} ms", (vm_end - t_start).as_secs_f64() * 1000.0);
        }
        return;
    }

    // Native Compilation
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
                println!("\n⏱️  Benchmark Results (Native AOT):");
                println!("  Frontend (Lex + Parse + Check): {:.2} ms", (t_frontend - t_start).as_secs_f64() * 1000.0);
                println!("  Clang C Compilation (-O2):       {:.2} ms", (t_compile_end - t_compile_start).as_secs_f64() * 1000.0);
                println!("  Native Execution Time:           {:.2} ms", (t_exec_end - t_exec_start).as_secs_f64() * 1000.0);
                println!("  Total End-to-End:                {:.2} ms", (t_exec_end - t_start).as_secs_f64() * 1000.0);
            }
        }
        Ok(out) => {
            eprintln!("  ✖ Native compilation failed.\nDetails:\nclang error:\n{}", String::from_utf8_lossy(&out.stderr));
        }
        Err(e) => {
            eprintln!("  ✖ Failed to execute clang: {}", e);
        }
    }

    let _ = fs::remove_file(temp_c);
    let _ = fs::remove_file(temp_bin);
}
