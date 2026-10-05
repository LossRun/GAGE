with open("src/main.rs", "r") as f:
    code = f.read()

# 1. Sleek neon green ASCII logo
banner_fn = r'''fn print_gage_banner() {
    println!("\x1b[1;32m");
    println!(r#"  ██████╗   █████╗   ██████╗  ███████╗"#);
    println!(r#" ██╔════╝  ██╔══██╗ ██╔════╝  ██╔════╝"#);
    println!(r#" ██║  ███╗ ███████║ ██║  ███╗ █████╗  "#);
    println!(r#" ██║   ██║ ██╔══██║ ██║   ██║ ██╔══╝  "#);
    println!(r#" ╚██████╔╝ ██║  ██║ ╚██████╔╝ ███████╗"#);
    println!(r#"  ╚═════╝  ╚═╝  ╚═╝  ╚═════╝  ╚══════╝"#);
    println!("\x1b[0m");
}'''

# Replace existing print_gage_banner
import re
pattern = r"fn print_gage_banner\(\)\s*\{[\s\S]*?\n\}"
if re.search(pattern, code):
    code = re.sub(pattern, banner_fn, code, count=1)
else:
    # If not found, insert before print_custom_help
    code = code.replace("fn print_custom_help() {", banner_fn + "\n\nfn print_custom_help() {")

# 2. Make sure start_repl() calls print_gage_banner()
if "fn start_repl() {" in code:
    old_repl_header = 'fn start_repl() {\n    println!("GAGE 0.2.0'
    new_repl_header = 'fn start_repl() {\n    print_gage_banner();\n    println!("GAGE 0.2.0'
    if old_repl_header in code:
        code = code.replace(old_repl_header, new_repl_header, 1)

# 3. Make sure print_custom_help() calls print_gage_banner()
if "fn print_custom_help() {" in code:
    if "print_gage_banner();" not in code[code.find("fn print_custom_help() {"):code.find("fn print_custom_help() {") + 150]:
        code = code.replace("fn print_custom_help() {", "fn print_custom_help() {\n    print_gage_banner();", 1)

with open("src/main.rs", "w") as f:
    f.write(code)

print("Green ASCII banner successfully added to start_repl and help!")
