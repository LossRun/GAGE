with open("src/main.rs", "r") as f:
    code = f.read()

# Locate handle_delete definition
needle = "fn handle_delete() {"
idx = code.find(needle)

if idx == -1:
    print("Could not find fn handle_delete() in src/main.rs")
    exit(1)

# Find the next function or end of handle_delete block
# Look for where the actual deletion logic starts
clean_delete_fn = r'''fn handle_delete() {
    use std::io::{stdin, stdout, Write};

    // Print the recognizable GAGE logo
    print_gage_banner();

    println!("\x1b[1;31m  ⚠️  DANGER ZONE: UNINSTALL GAGE TOOLCHAIN\x1b[0m");
    println!("\x1b[90m  This will permanently delete the GAGE binary, cache, and compiled artifacts.\x1b[0m\n");
    print!("\x1b[1;33m  Are you sure you want to proceed? [y/N]: \x1b[0m");
    let _ = stdout().flush();

    let mut response = String::new();
    if stdin().read_line(&mut response).is_err() {
        println!("\n\x1b[31mAction aborted.\x1b[0m");
        return;
    }

    let trimmed = response.trim().to_lowercase();
    if trimmed != "y" && trimmed != "yes" {
        println!("\x1b[32m✔ Deletion cancelled. Your GAGE installation remains active.\x1b[0m\n");
        return;
    }

    println!("\n\x1b[33mUninstalling GAGE...\x1b[0m");

    // Remove binary from $PREFIX/bin
    let bin_path = format!("{}/bin/gage", std::env::var("PREFIX").unwrap_or_else(|_| "/data/data/com.termux/files/usr".into()));
    if std::path::Path::new(&bin_path).exists() {
        let _ = std::fs::remove_file(&bin_path);
        println!("  \x1b[32m✔ Removed executable:\x1b[0m {}", bin_path);
    }

    // Remove cargo target cache
    if let Ok(home) = std::env::var("HOME") {
        let cache_path = format!("{}/.cargo_target_gage", home);
        if std::path::Path::new(&cache_path).exists() {
            let _ = std::fs::remove_dir_all(&cache_path);
            println!("  \x1b[32m✔ Removed build cache:\x1b[0m {}", cache_path);
        }
    }

    println!("\x1b[1;32m✔ GAGE has been completely uninstalled.\x1b[0m\n");
}'''

# Replace the existing handle_delete function cleanly
import re
pattern = r"fn handle_delete\(\)\s*\{[\s\S]*?\n\}"
code = re.sub(pattern, clean_delete_fn, code, count=1)

with open("src/main.rs", "w") as f:
    f.write(code)

print("✔ Added GAGE logo to delete confirmation prompt!")
