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

# Clean, razor-sharp GAGE banner
echo -e "${CYAN}"
echo "   ______      ___       ______   _______ "
echo "  / _____|    /   \     / _____| |  _____|"
echo " | |  __     / /_\ \   | |  __   | |____  "
echo " | | |_ |   / _____ \  | | |_ |  |  ____| "
echo " | |__| |  / /     \ \ | |__| |  | |_____ "
echo "  \_____/ /_/       \_\\_____/   |_______|"
echo -e "${RESET}"
echo -e "${BORDER}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${RESET}"
echo -e " ${BOLD}${WHITE}GAGE TOOLCHAIN INSTALLER${RESET} ${MUTED}|${RESET} Native Engine & VM"
echo -e "${BORDER}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${RESET}\n"

# Live Braille spinner function
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

# 1. Target Directory Detection
if [ -n "$PREFIX" ] && [ -d "$PREFIX/bin" ]; then
    INSTALL_DIR="$PREFIX/bin"
elif [ "$(id -u)" -eq 0 ]; then
    INSTALL_DIR="/usr/local/bin"
else
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
fi

# 2. Host Toolchain Validation
if ! command -v cargo &> /dev/null; then
    run_step "Installing Rust & Cargo toolchain" "pkg update -y && pkg install rust -y"
else
    printf " ${GREEN}✔${RESET}  ${WHITE}%-42s${RESET}\n" "Rust & Cargo compiler toolchain verified"
fi

if ! command -v clang &> /dev/null && ! command -v gcc &> /dev/null; then
    run_step "Installing Clang/LLVM native compiler backend" "pkg install clang -y"
else
    printf " ${GREEN}✔${RESET}  ${WHITE}%-42s${RESET}\n" "Clang/LLVM native backend verified"
fi

# 3. Source Discovery
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ ! -f "$SCRIPT_DIR/Cargo.toml" ] || [ ! -d "$SCRIPT_DIR/src" ]; then
    printf " ${RED}✖${RESET}  Cannot find valid Gage source directory in %s\n" "$SCRIPT_DIR"
    exit 1
fi
printf " ${GREEN}✔${RESET}  ${WHITE}%-42s${RESET}\n" "Source directory verified"

# 4. Sandbox Staging
BUILD_DIR="$HOME/.gage_build"
run_step "Synchronizing build workspace" "rm -rf '$BUILD_DIR' && mkdir -p '$BUILD_DIR' && cp -r '$SCRIPT_DIR/Cargo.toml' '$SCRIPT_DIR/src' '$SCRIPT_DIR/version.txt' '$BUILD_DIR/'"

# 5. Compilation
run_step "Compiling Gage toolchain binary" "cd '$BUILD_DIR' && cargo build --release"

# 6. Binary Deployment
run_step "Installing binaries to $INSTALL_DIR" "mkdir -p '$INSTALL_DIR' && cp '$BUILD_DIR/target/release/gage' '$INSTALL_DIR/gage' && chmod +x '$INSTALL_DIR/gage'"

if [ -n "$PREFIX" ]; then
    mkdir -p "$PREFIX/share/gage"
    cp "$BUILD_DIR/version.txt" "$PREFIX/share/gage/version.txt"
    echo "$SCRIPT_DIR" > "$PREFIX/share/gage/source_path.txt"
fi

# 7. Verification Test
run_step "Running system verification test" "$INSTALL_DIR/gage --info"

# Summary Card
echo ""
echo -e "${BORDER}┌─ INSTALLATION COMPLETE ───────────────────────────┐${RESET}"
echo -e "${BORDER}│${RESET}  ${WHITE}Binary Path  :${RESET} ${CYAN}$INSTALL_DIR/gage${RESET}"
echo -e "${BORDER}│${RESET}  ${WHITE}Architecture :${RESET} ${GREEN}Native LLVM AOT + Bytecode VM${RESET}"
echo -e "${BORDER}│${RESET}  ${WHITE}Quick Test   :${RESET} ${BOLD}gage run main.gage${RESET}"
echo -e "${BORDER}│${RESET}  ${WHITE}Build Binary :${RESET} ${BOLD}gage build main.gage -o myapp${RESET}"
echo -e "${BORDER}└───────────────────────────────────────────────────┘${RESET}\n"
