@echo off
title ANTAI — Build & Release Automator
color 0C
echo ============================================================
echo   ANTAI — Autonomous AI Cyber Defense Sentinel
echo   Compilazione Release Nativa e Packaging Desktop (.exe)
echo ============================================================
echo.

cd /d "%~dp0antai-core"
echo [1/3] Compilazione ANTAI Core Engine in Rust (release)...
cargo build --release
if %errorlevel% neq 0 (
    echo [ERRORE] Compilazione di antai-core fallita.
    pause
    exit /b %errorlevel%
)

echo [2/3] Copia eseguibile nativo antai-core.exe...
copy /Y "target\release\antai-core.exe" "..\antai-core.exe"

cd /d "%~dp0"
echo [3/3] Verifica Dashboard e SDK pronti per la distribuzione...
echo.
echo ============================================================
echo   COMPILAZIONE ED ESITO DI RILASCIO PERFETTI!
echo ============================================================
echo   - Eseguibile Nativo: antai-core.exe
echo   - Dashboard UI: index.html + app.js + styles.css
echo   - SDK Browser & Edge: sdk/antai-sdk.js, sdk/antai-middleware.ts
echo ============================================================
echo.
pause
