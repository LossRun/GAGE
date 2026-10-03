# Gage Language Toolchain Installer for Windows
$ErrorActionPreference = "Stop"

Write-Host "`n  =========================================" -ForegroundColor Cyan
Write-Host "     GAGE COMPILER INSTALLER (WINDOWS)    " -ForegroundColor White
Write-Host "  =========================================`n" -ForegroundColor Cyan

# 1. Check for Rust / Cargo
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "[!] Cargo not detected. Please install Rust from https://rustup.rs" -ForegroundColor Yellow
    exit 1
}

# 2. Check for Clang or GCC
if (-not (Get-Command clang -ErrorAction SilentlyContinue) -and -not (Get-Command gcc -ErrorAction SilentlyContinue)) {
    Write-Host "[!] Neither Clang nor GCC found." -ForegroundColor Yellow
    Write-Host "[*] Installing LLVM Clang via winget..." -ForegroundColor Cyan
    winget install LLVM.LLVM --silent --accept-source-agreements --accept-package-agreements
}

# 3. Build release binary
Write-Host "[*] Compiling Gage native compiler with Cargo..." -ForegroundColor Cyan
cargo build --release

# 4. Install into %USERPROFILE%\.gage\bin
$InstallDir = "$env:USERPROFILE\.gage\bin"
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

Copy-Item ".\target\release\gage.exe" -Destination "$InstallDir\gage.exe" -Force
Write-Host "[✔] Binary installed to $InstallDir\gage.exe" -ForegroundColor Green

# 5. Add to User PATH if not present
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    Write-Host "[*] Adding Gage to user environment PATH..." -ForegroundColor Cyan
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    $env:Path += ";$InstallDir"
}

Write-Host "`n=========================================" -ForegroundColor Green
Write-Host "  GAGE SUCCESSFULLY INSTALLED! (Windows)" -ForegroundColor Green
Write-Host "=========================================" -ForegroundColor Green
Write-Host "Open a new terminal and try: gage --info" -ForegroundColor White
