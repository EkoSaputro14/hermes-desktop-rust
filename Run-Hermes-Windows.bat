@echo off
title Hermes Desktop — Native Rust & Portable Launcher
echo ============================================================================
echo   HERMES DESKTOP (RUST ICED EDITION) — WINDOWS LAUNCHER
echo ============================================================================
echo.

REM 1. If native Rust release binary already exists, run it directly
if exist "%~dp0target\release\hermes-desktop.exe" (
    echo [OK] Launching compiled native Rust binary: target\release\hermes-desktop.exe
    start "" "%~dp0target\release\hermes-desktop.exe"
    exit /b 0
)

REM 2. Check if Rust Cargo toolchain is installed on this PC
where cargo >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    echo [INFO] Rust Cargo detected. Building optimized native release binary...
    cd /d "%~dp0"
    cargo build --release
    if exist "%~dp0target\release\hermes-desktop.exe" (
        echo [OK] Build complete! Launching native Rust binary...
        start "" "%~dp0target\release\hermes-desktop.exe"
        exit /b 0
    )
)

REM 3. Instant Zero-Install Fallback: Launch Standalone Desktop App Window (--app mode)
echo [INFO] Launching Hermes Desktop Standalone Native Window (Zero-Install Mode)...
if exist "%ProgramFiles(x86)%\Microsoft\Edge\Application\msedge.exe" (
    start "" "%ProgramFiles(x86)%\Microsoft\Edge\Application\msedge.exe" --app="file:///%~dp0Hermes-Desktop-Standalone.html" --window-size=1440,900
    exit /b 0
)
if exist "%ProgramFiles%\Google\Chrome\Application\chrome.exe" (
    start "" "%ProgramFiles%\Google\Chrome\Application\chrome.exe" --app="file:///%~dp0Hermes-Desktop-Standalone.html" --window-size=1440,900
    exit /b 0
)

start "" "%~dp0Hermes-Desktop-Standalone.html"
exit /b 0
