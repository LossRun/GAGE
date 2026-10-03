#!/usr/bin/env bash
set -e

CYAN="\033[38;5;51m"
GREEN="\033[38;5;48m"
WHITE="\033[1;37m"
MUTED="\033[38;5;244m"
BORDER="\033[38;5;238m"
RED="\033[38;5;196m"
RESET="\033[0m"
BOLD="\033[1m"

clear
tput civis 2>/dev/null || printf "\033[?25l"

cleanup() {
    tput cnorm 2>/dev/null || printf "\033[?25h"
}
trap cleanup EXIT INT TERM

echo -e "${CYAN}"
echo "   ______      ___       ______   _______ "
echo "  / _____|    /   \     / _____| |  _____|"
echo " | |  __     / /_\ \   | |  __   | |____  "
echo " | | |_ |   / _____ \  | | |_ |  |  ____| "
echo " | |__| |  / /     \ \ | |__| |  | |_____ "
echo "  \_____/ /_/       \_\_____/   |_______|"
echo -e "${RESET}"
echo -e "${BORDER}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${RESET}"
echo -e " ${BOLD}${WHITE}GAGE TOOLCHAIN INSTALLER${RESET} ${MUTED}|${RESET} Native Engine & VM"
echo -e "${BORDER}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${RESET}\n"

run_step() {
    local task_name="$1"
    local cmd="$2"
    local spin_chars="⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"

    eval "$cmd" > /dev/null 2>&1 &
    local pid=$!

    local i=0
    while kill -0 "$pid" 2>/dev/null; do
        local spin="${spin_chars:i++%${#spin_chars}:1}"
        printf "\r\033[2K ${CYAN}%s${RESET}  ${WHITE}%s...${RESET}" "$spin" "$task_name"
        sleep 0.08
    done

    wait "$pid"
    local exit_code=$?

    if [ $exit_code -eq 0 ]; then
        printf "\r\033[2K ${GREEN}✔${RESET}  ${WHITE}%-42s${RESET}\n" "$task_name"
    else
        printf "\r\033[2K ${RED}✖${RESET}  ${WHITE}%-42s${RESET} ${RED}[failed]${RESET}\n" "$task_name"
        exit 1
    fi
}

# 1. Automated Platform Dependency Resolution
if [ -n "$PREFIX" ] && [ -d "$PREFIX/bin" ]; then
    INSTALL_DIR="$PREFIX/bin"
    # Ensure Termux has all C standard library headers (ndk-sysroot), Clang, and Rust
    if ! command -v clang &> /dev/null || [ ! -f "$PREFIX/include/stdio.h" ]; then
        run_step "Installing Termux C toolchain & sysroot headers" "pkg update -y && pkg install -y ndk-sysroot clang"
    fi
    if ! command -v cargo &> /dev/null; then
        run_step "Installing Rust & Cargo compiler" "pkg install -y rust"
    fi
elif [ "$(uname -s)" = "Darwin" ]; then
    INSTALL_DIR="/usr/local/bin"
    if ! command -v clang &> /dev/null; then
        run_step "Checking Xcode Command Line Tools" "xcode-select --install || true"
    fi
    if ! command -v cargo &> /dev/null; then
        run_step "Installing Rust via rustup" "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && source \$HOME/.cargo/env"
    fi
else
    # Linux (Debian/Ubuntu/Arch/Fedora)
    if [ "$(id -u)" -eq 0 ]; then
        INSTALL_DIR="/usr/local/bin"
    else
        INSTALL_DIR="$HOME/.local/bin"
        mkdir -p "$INSTALL_DIR"
    fi

    if command -v apt-get &> /dev/null; then
        if ! command -v clang &> /dev/null || ! command -v cargo &> /dev/null; then
            run_step "Installing system build dependencies (apt)" "sudo apt-get update -y && sudo apt-get install -y clang build-essential rustc cargo"
        fi
    elif command -v pacman &> /dev/null; then
        if ! command -v clang &> /dev/null || ! command -v cargo &> /dev/null; then
            run_step "Installing system build dependencies (pacman)" "sudo pacman -Sy --noconfirm clang base-devel rust"
        fi
    elif command -v dnf &> /dev/null; then
        if ! command -v clang &> /dev/null || ! command -v cargo &> /dev/null; then
            run_step "Installing system build dependencies (dnf)" "sudo dnf install -y clang gcc rust cargo"
        fi
    fi
fi

# 2. Source Discovery
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ ! -f "$SCRIPT_DIR/Cargo.toml" ] || [ ! -d "$SCRIPT_DIR/src" ]; then
    printf " ${RED}✖${RESET}  Cannot find valid Gage source directory in %s\n" "$SCRIPT_DIR"
    exit 1
fi

# 3. Workspace Staging
BUILD_DIR="$HOME/.gage_build"
run_step "Synchronizing build workspace" "rm -rf '$BUILD_DIR' && mkdir -p '$BUILD_DIR' && cp -r '$SCRIPT_DIR/Cargo.toml' '$SCRIPT_DIR/src' '$BUILD_DIR/'"

# 4. Compilation
run_step "Compiling Gage toolchain binary" "cd '$BUILD_DIR' && cargo build --release"

# 5. Deployment
run_step "Installing binaries to $INSTALL_DIR" "mkdir -p '$INSTALL_DIR' && cp '$BUILD_DIR/target/release/gage' '$INSTALL_DIR/gage' && chmod +x '$INSTALL_DIR/gage'"

# 6. Verification
run_step "Running system verification test" "$INSTALL_DIR/gage --info"

echo ""
echo -e "${BORDER}┌─ INSTALLATION COMPLETE ───────────────────────────┐${RESET}"
echo -e "${BORDER}│${RESET}  ${WHITE}Binary Path  :${RESET} ${CYAN}$INSTALL_DIR/gage${RESET}"
echo -e "${BORDER}│${RESET}  ${WHITE}Architecture :${RESET} ${GREEN}Native LLVM AOT + Bytecode VM${RESET}"
echo -e "${BORDER}│${RESET}  ${WHITE}Quick Test   :${RESET} ${BOLD}gage examples/23_hello_world.gage${RESET}"
echo -e "${BORDER}│${RESET}  ${WHITE}Build Binary :${RESET} ${BOLD}gage build main.gage -o myapp${RESET}"
echo -e "${BORDER}└───────────────────────────────────────────────────┘${RESET}\n"
