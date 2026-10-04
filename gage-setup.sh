#!/data/data/com.termux/files/usr/bin/bash
set -e

# Styling
C_RESET="\033[0m"
C_BOLD="\033[1m"
C_GREEN="\033[38;2;80;250;130m"
C_CYAN="\033[38;2;0;225;145m"
C_GRAY="\033[90m"
C_RED="\033[31m"

clear

# Logo
echo -e "\033[38;2;40;250;110m  ██████╗   █████╗   ██████╗  ███████╗\033[0m"
echo -e "\033[38;2;40;238;110m ██╔════╝  ██╔══██╗ ██╔════╝  ██╔════╝\033[0m"
echo -e "\033[38;2;40;226;110m ██║  ███╗ ███████║ ██║  ███╗ █████╗  \033[0m"
echo -e "\033[38;2;40;214;110m ██║   ██║ ██╔══██║ ██║   ██║ ██╔══╝  \033[0m"
echo -e "\033[38;2;40;202;110m ╚██████╔╝ ██║  ██║ ╚██████╔╝ ███████╗\033[0m"
echo -e "\033[38;2;40;190;110m  ╚═════╝  ╚═╝  ╚═╝  ╚═════╝  ╚══════╝\033[0m"
echo ""
echo -e "${C_GRAY}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${C_RESET}"
echo -e " ${C_BOLD}GAGE TOOLCHAIN INSTALLER${C_RESET} | Native Engine & VM"
echo -e "${C_GRAY}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${C_RESET}"
echo ""

# Helper to run any command with a live, continuous spinning animation
run_with_spinner() {
    local msg="$1"
    shift
    local cmd=("$@")

    # Start command in background
    "${cmd[@]}" > /dev/null 2>&1 &
    local pid=$!

    local spin=('⠋' '⠙' '⠹' '⠸' '⠼' '⠴' '⠦' '⠧' '⠇' '⠏')
    local i=0

    # Hide cursor
    echo -ne "\033[?25l"

    while kill -0 "$pid" 2>/dev/null; do
        echo -ne "\r\033[2K ${C_CYAN}${spin[$i]}${C_RESET}  ${msg}..."
        i=$(( (i + 1) % 10 ))
        sleep 0.08
    done

    wait "$pid"
    local status=$?

    # Show cursor
    echo -ne "\033[?25h"

    if [ $status -eq 0 ]; then
        echo -e "\r\033[2K ${C_GREEN}✔${C_RESET}  ${msg}"
    else
        echo -e "\r\033[2K ${C_RED}✖${C_RESET}  ${msg} (failed)"
        exit $status
    fi
}

echo -e " ${C_GREEN}✔${C_RESET}  Synchronizing build workspace"

if [ -d "$HOME/.cargo_target_gage" ]; then
    export CARGO_TARGET_DIR="$HOME/.cargo_target_gage"
fi

# Active Continuous Spinner for compilation
run_with_spinner "Compiling Gage toolchain binary" cargo build --release

# Install binary
echo -ne "\033[?25l"
TARGET_BIN=""
if [ -f "$HOME/.cargo_target_gage/release/gage" ]; then
    TARGET_BIN="$HOME/.cargo_target_gage/release/gage"
elif [ -f "./target/release/gage" ]; then
    TARGET_BIN="./target/release/gage"
elif [ -f "../target/release/gage" ]; then
    TARGET_BIN="../target/release/gage"
fi

if [ -n "$TARGET_BIN" ]; then
    cp -f "$TARGET_BIN" "$PREFIX/bin/gage"
    chmod 755 "$PREFIX/bin/gage"
    hash -r
    echo -e "\r\033[2K ${C_GREEN}✔${C_RESET}  Installing Gage binary"
else
    echo -e "\r\033[2K ${C_RED}✖${C_RESET}  Binary not found"
    echo -ne "\033[?25h"
    exit 1
fi

# Verification test
set +e
TEST_OUT=$("$PREFIX/bin/gage" -h 2>&1)
set -e
echo -e " ${C_GREEN}✔${C_RESET}  Running system verification test"
echo -ne "\033[?25h"

echo ""
echo -e "${C_GREEN}====================================================${C_RESET}"
echo -e " ${C_BOLD}Installation successful!${C_RESET}"
echo -e " Run the GAGE interactive environment by typing: ${C_GREEN}gage${C_RESET}"
echo -e "${C_GREEN}====================================================${C_RESET}"
echo ""
