@echo off
title ANTAI — Build & Release Automator
color 0C
echo ============================================================
echo   ANTAI — Autonomous AI Cyber Defense Sentinel
echo   Compilazione Release Nativa e Packaging Desktop (.exe)
echo ============================================================
echo.

set "PATH=%PATH%;%USERPROFILE%\.cargo\bin"

cd /d "%~dp0antai-core"
echo [1/3] Compilazione ANTAI Core Engine (Backend Rust)...
cargo build --release
if %errorlevel% neq 0 (
    echo [ERRORE] Compilazione di antai-core fallita.
    pause
    exit /b %errorlevel%
)

echo [2/3] Copia eseguibile nativo antai-core.exe...
copy /Y "target\release\antai-core.exe" "..\antai-core.exe"

cd /d "%~dp0src-tauri"
echo [3/3] Compilazione ANTAI Desktop App (.exe nativo Tauri UI)...
cargo build --release
if %errorlevel% neq 0 (
    echo [ERRORE] Compilazione dell'App Desktop fallita.
    pause
    exit /b %errorlevel%
)

copy /Y "target\release\antai-desktop.exe" "..\ANTAI-Sentinel-Desktop.exe"

cd /d "%~dp0"
echo.
echo ============================================================
echo   COMPILAZIONE NATIVA ED INSTALLATORE DESKTOP COMPLETATI!
echo ============================================================
echo   - App Desktop Nativa: ANTAI-Sentinel-Desktop.exe
echo   - Core Engine Backend: antai-core.exe
echo   - Dashboard & Assets: index.html + app.js + styles.css
echo   - Logo Ufficiale: antai_logo.png (Trasparente)
echo ============================================================
echo.
pause
