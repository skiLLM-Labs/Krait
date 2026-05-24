@echo off
setlocal enabledelayedexpansion

echo Installing Krait Programming Language (v1.0.0)...

:: Setup Directories
set "KRAIT_DIR=%USERPROFILE%\.krait"
set "BIN_DIR=%KRAIT_DIR%\bin"
set "EXE_PATH=%BIN_DIR%\krait.exe"

if not exist "%BIN_DIR%" (
    mkdir "%BIN_DIR%" >nul 2>&1
)

:: Detect System Architecture
set "ASSET_NAME="
if "%PROCESSOR_ARCHITECTURE%"=="ARM64" (
    set "ASSET_NAME=krait-windows-arm64.exe"
) else if "%PROCESSOR_ARCHITECTURE%"=="AMD64" (
    set "ASSET_NAME=krait-windows-x64.exe"
) else (
    echo Error: Unsupported Windows architecture (%PROCESSOR_ARCHITECTURE%).
    exit /b 1
)

echo Fetching latest release from KraitDev/Krait (%ASSET_NAME%)...

:: Use PowerShell under the hood to securely grab the download URL from the GitHub API
set "API_URL=https://api.github.com/repos/KraitDev/Krait/releases/latest"
for /f "delims=" %%I in ('powershell -NoProfile -Command ^
    "[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; ^
     try { ^
         $Release = Invoke-RestMethod -Uri '%API_URL%' -Headers @{'User-Agent'='CMD-Krait-Installer'}; ^
         $Url = ($Release.assets | Where-Object { $_.name -eq '%ASSET_NAME%' }).browser_download_url; ^
         if ($Url) { Write-Output $Url } ^
     } catch { exit 1 }"') do (
    set "DOWNLOAD_URL=%%I"
)

if "%DOWNLOAD_URL%"=="" (
    echo Error: Could not find binary asset '%ASSET_NAME%' or failed to reach GitHub API.
    exit /b 1
)

:: Download the binary using PowerShell
echo Downloading from %DOWNLOAD_URL%...
powershell -NoProfile -Command "Invoke-WebRequest -Uri '%DOWNLOAD_URL%' -OutFile '%EXE_PATH%'"

:: Add to User PATH via PowerShell environment tracking to avoid messy registry hacks
powershell -NoProfile -Command ^
    "$UserPath = [Environment]::GetEnvironmentVariable('PATH', 'User'); ^
     if ($UserPath -notlike '*%BIN_DIR%*') { ^
         $UserPath = $UserPath -replace ';+$', ''; ^
         [Environment]::SetEnvironmentVariable('PATH', \"$UserPath;%BIN_DIR%\", 'User'); ^
         Write-Output 'Added Krait to your User PATH.' ^
     }"

echo.
echo Checkmark Krait installed successfully!
echo Please restart your Command Prompt window and type 'krait'.
echo.

pause
