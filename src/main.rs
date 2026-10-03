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
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

use codegen::CodeGen;
use compiler::Compiler;
use lexer::Lexer;
use parser::Parser;
use types::TypeChecker;
use vm::VM;

const VERSION: &str = "0.1.0-cross-platform";

fn print_banner() {
    println!("\x1b[38;5;51m");
    println!("   ______      ___       ______   _______ ");
    println!("  / _____|    /   \\     / _____| |  _____|");
    println!(" | |  __     / /_\\ \\   | |  __   | |____  ");
    println!(" | | |_ |   / _____ \\  | | |_ |  |  ____| ");
    println!(" | |__| |  / /     \\ \\ | |__| |  | |_____ ");
    println!("  \\_____/ /_/       \\_\\_____/   |_______|");
    println!("\x1b[0m");
    println!("\x1b[38;5;238m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\x1b[0m");
    println!(" \x1b[1m\x1b[38;5;255mGAGE UNIFIED TOOLCHAIN\x1b[0m \x1b[38;5;244m|\x1b[0m Native LLVM Compiler & Fast VM");
    println!("\x1b[38;5;238m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\x1b[0m\n");
}

fn print_usage() {
    print_banner();
    println!("\x1b[1mUSAGE:\x1b[0m");
    println!("  \x1b[38;5;48mgage build\x1b[0m \x1b[38;5;220m<file.gage>\x1b[0m [-o <bin>]    Compile source to native ELF/EXE binary");
    println!("  \x1b[38;5;48mgage run\x1b[0m   \x1b[38;5;220m<file.gage>\x1b[0m                  Compile and execute at native CPU speed");
    println!("  \x1b[38;5;48mgage vm\x1b[0m    \x1b[38;5;220m<file.gage>\x1b[0m                  Execute instantly via bytecode VM (no clang required)");
    println!("  \x1b[38;5;48mgage check\x1b[0m \x1b[38;5;220m<file.gage>\x1b[0m                  Run full semantic and type validation");
    println!("  \x1b[38;5;48mgage emit-c\x1b[0m\x1b[38;5;220m<file.gage>\x1b[0m                  Inspect generated intermediate C code");
    println!("  \x1b[38;5;48mgage delete\x1b[0m                            Safely uninstall Gage binaries");
    println!("\n\x1b[1mFLAGS:\x1b[0m");
    println!("  \x1b[38;5;81m-v, --version\x1b[0m                          Show compiler version");
    println!("  \x1b[38;5;81m-h, --help\x1b[0m                             Display this help manual");
    println!("  \x1b[38;5;81m--info\x1b[0m                                 Display architecture and pipeline stats\n");
}

fn parse_and_validate(source: &str) -> Result<ast::Program, String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().map_err(|e| format!("[Lexer Error] Line {}:{} -> {}", e.line, e.column, e.message))?;

    let mut parser = Parser::new(tokens);
    let program = parser.parse().map_err(|e| format!("[Parse Error] Line {}:{} -> {}", e.line, e.column, e.message))?;

    let mut type_checker = TypeChecker::new();
    type_checker.check(&program)?;

    Ok(program)
}

fn compile_to_c(source: &str) -> Result<String, String> {
    let program = parse_and_validate(source)?;
    let codegen = CodeGen::new();
    Ok(codegen.generate(&program))
}

fn run_via_vm(source: &str) -> Result<(), String> {
    let program = parse_and_validate(source)?;
    let mut comp = Compiler::new();
    let chunk = comp.compile(&program).map_err(|e| format!("[Bytecode Error] {:?}", e))?;
    let mut vm = VM::new(chunk);
    vm.run().map_err(|e| format!("[Runtime Error] {:?}", e))?;
    Ok(())
}

fn get_temp_binary_path() -> PathBuf {
    let mut temp = env::temp_dir();
    let ext = if cfg!(target_os = "windows") { "gage_exec.exe" } else { "gage_exec" };
    temp.push(ext);
    temp
}

fn invoke_c_compiler(c_code: &str, output_binary: &Path) -> Result<(), String> {
    let temp_c = output_binary.with_extension("tmp.c");
    fs::write(&temp_c, c_code).map_err(|e| format!("Failed to create intermediate C file: {}", e))?;

    let compilers = if cfg!(target_os = "windows") { vec!["clang-cl", "clang", "gcc"] } else { vec!["clang", "gcc"] };
    let mut success = false;
    let mut primary_err = String::new();

    for cc in compilers {
        let mut cmd = Command::new(cc);
        cmd.arg("-O3").arg("-w").arg(&temp_c).arg("-o").arg(output_binary);
        if !cfg!(target_os = "windows") { cmd.arg("-lm"); }

        match cmd.output() {
            Ok(output) if output.status.success() => { success = true; break; }
            Ok(output) => {
                if primary_err.is_empty() {
                    primary_err = format!("{} error:
{}", cc, String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(e) => {
                if primary_err.is_empty() {
                    primary_err = format!("Could not launch {}: {}", cc, e);
                }
            }
        }
    }

    let _ = fs::remove_file(temp_c);

    if !success {
        return Err(format!("Native compilation failed.
Details:
{}", primary_err));
    }
    Ok(())
}

fn handle_delete() {
    println!("\x1b[38;5;196m\x1b[1m⚠️  WARNING: Uninstalling Gage Toolchain\x1b[0m");
    print!("Are you sure you want to proceed? [y/N]: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() { return; }
    let trimmed = input.trim().to_lowercase();
    if trimmed != "y" && trimmed != "yes" {
        println!("\x1b[38;5;81mAborted. No files were removed.\x1b[0m");
        return;
    }

    if let Ok(current_exe) = env::current_exe() {
        let _ = fs::remove_file(&current_exe);
    }
    if let Ok(prefix) = env::var("PREFIX") {
        let _ = fs::remove_file(format!("{}/bin/gage", prefix));
        let _ = fs::remove_dir_all(format!("{}/share/gage", prefix));
    }
    if let Ok(home) = env::var("HOME") {
        let _ = fs::remove_dir_all(format!("{}/.gage_build", home));
        let _ = fs::remove_file(format!("{}/.local/bin/gage", home));
    }
    println!("\x1b[38;5;48m✔ Gage has been cleanly uninstalled.\x1b[0m");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        exit(0);
    }

    match args[1].as_str() {
        "delete" | "-d" | "--delete" => handle_delete(),
        "-v" | "--version" => {
            println!("\x1b[38;5;48m\x1b[1mgage version {}\x1b[0m ({}-{})", VERSION, env::consts::OS, env::consts::ARCH);
        }
        "--info" => {
            println!("\x1b[38;5;51mEngine Pipelines\x1b[0m       : Native AOT (LLVM/Clang) + Fast Bytecode VM");
            println!("\x1b[38;5;51mSemantic Validation\x1b[0m    : Active (types.rs)");
            println!("\x1b[38;5;51mTarget Architecture\x1b[0m    : {} ({})", env::consts::ARCH, env::consts::OS);
        }
        "-h" | "--help" => print_usage(),
        "check" => {
            if args.len() < 3 {
                eprintln!("\x1b[38;5;196mError:\x1b[0m Source file required. Example: gage check main.gage");
                exit(1);
            }
            let source = fs::read_to_string(&args[2]).expect("Cannot read input file");
            match parse_and_validate(&source) {
                Ok(_) => println!("  \x1b[38;5;48m✔ [Verified]\x1b[0m '{}' passed lexer, parser, and type checks.", args[2]),
                Err(e) => {
                    eprintln!("  \x1b[38;5;196m✖ {}\x1b[0m", e);
                    exit(1);
                }
            }
        }
        "vm" => {
            if args.len() < 3 {
                eprintln!("\x1b[38;5;196mError:\x1b[0m Source file required. Example: gage vm main.gage");
                exit(1);
            }
            let source = fs::read_to_string(&args[2]).expect("Cannot read input file");
            if let Err(e) = run_via_vm(&source) {
                eprintln!("  \x1b[38;5;196m{}\x1b[0m", e);
                exit(1);
            }
        }
        "emit-c" => {
            if args.len() < 3 {
                eprintln!("\x1b[38;5;196mError:\x1b[0m Source file required. Example: gage emit-c main.gage");
                exit(1);
            }
            let source = fs::read_to_string(&args[2]).expect("Cannot read input file");
            match compile_to_c(&source) {
                Ok(c) => println!("{}", c),
                Err(e) => eprintln!("  \x1b[38;5;196m✖ {}\x1b[0m", e),
            }
        }
        "build" => {
            if args.len() < 3 {
                eprintln!("\x1b[38;5;196mError:\x1b[0m Source file required. Example: gage build main.gage -o app");
                exit(1);
            }
            let input_path = &args[2];
            let raw_out_name = if args.len() >= 5 && args[3] == "-o" {
                args[4].clone()
            } else {
                Path::new(input_path).file_stem().unwrap().to_str().unwrap().to_string()
            };
            let out_name = if cfg!(target_os = "windows") && !raw_out_name.ends_with(".exe") {
                format!("{}.exe", raw_out_name)
            } else {
                raw_out_name
            };

            let source = fs::read_to_string(input_path).expect("Could not open source file");
            println!("  \x1b[38;5;51m➜\x1b[0m  Validating types and generating machine code for \x1b[1m{}\x1b[0m...", input_path);

            let c_code = match compile_to_c(&source) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("  \x1b[38;5;196m✖ {}\x1b[0m", e);
                    exit(1);
                }
            };

            if let Err(e) = invoke_c_compiler(&c_code, Path::new(&out_name)) {
                eprintln!("  \x1b[38;5;196m✖ {}\x1b[0m", e);
                exit(1);
            }
            println!("  \x1b[38;5;48m✔\x1b[0m  Native binary compiled: \x1b[1m./{}\x1b[0m\n", out_name);
        }
        "run" => {
            if args.len() < 3 {
                eprintln!("\x1b[38;5;196mError:\x1b[0m Source file required. Example: gage run main.gage");
                exit(1);
            }
            let input_path = &args[2];
            let temp_bin = get_temp_binary_path();
            let source = fs::read_to_string(input_path).expect("Could not open source file");

            let c_code = match compile_to_c(&source) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("  \x1b[38;5;196m✖ {}\x1b[0m", e);
                    exit(1);
                }
            };

            if let Err(e) = invoke_c_compiler(&c_code, &temp_bin) {
                eprintln!("  \x1b[38;5;196m✖ {}\x1b[0m", e);
                exit(1);
            }

            let _ = Command::new(&temp_bin).status();
            let _ = fs::remove_file(temp_bin);
        }
        _ => {
            if Path::new(&args[1]).exists() {
                let temp_bin = get_temp_binary_path();
                let source = fs::read_to_string(&args[1]).expect("Could not open source file");
                let c_code = match compile_to_c(&source) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("  \x1b[38;5;196m✖ {}\x1b[0m", e);
                        exit(1);
                    }
                };
                if let Err(e) = invoke_c_compiler(&c_code, &temp_bin) {
                    eprintln!("  \x1b[38;5;196m✖ {}\x1b[0m", e);
                    exit(1);
                }
                let _ = Command::new(&temp_bin).status();
                let _ = fs::remove_file(temp_bin);
            } else {
                eprintln!("\x1b[38;5;196mUnknown command or file:\x1b[0m '{}'. Run \x1b[38;5;48mgage --help\x1b[0m", args[1]);
                exit(1);
            }
        }
    }
}