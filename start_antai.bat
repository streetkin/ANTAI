@echo off
chcp 65001 >nul
title ANTAI Sentinel — Autonomous Cyber Defense
color 0C
echo ============================================================
echo   ANTAI — Autonomous AI Cyber Defense Sentinel
echo   Avvio del Motore Nativo Rust e della Control Room
echo ============================================================
echo.

if exist "%~dp0antai-core\target\release\antai-core.exe" (
    echo [ANTAI] Avvio antai-core\target\release\antai-core.exe...
    start "" "%~dp0antai-core\target\release\antai-core.exe"
) else if exist "%~dp0antai-core.exe" (
    echo [ANTAI] Avvio antai-core.exe...
    start "" "%~dp0antai-core.exe"
) else (
    echo [ERRORE] Eseguibile antai-core.exe non trovato. Esegui prima build_release_package.bat
    pause
    exit /b 1
)

timeout /t 2 /nobreak >nul

echo [ANTAI] Apertura Control Room Imperiale nel Browser...
start "" "%~dp0index.html"

echo.
echo [ANTAI] SHIELD ONLINE! Il proxy intercettore e in ascolto su porta 8090.
echo.
