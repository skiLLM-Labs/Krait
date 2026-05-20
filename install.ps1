Write-Host "🐍 Installing Krait Programming Language..." -ForegroundColor Cyan

# Setup Directories
$KraitDir = "$env:USERPROFILE\.krait"
$BinDir = "$KraitDir\bin"
If (!(Test-Path $BinDir)) { New-Item -ItemType Directory -Force -Path $BinDir | Out-Null }

$AssetName = "krait-windows-amd64.exe"
$ExePath = "$BinDir\krait.exe"

# Fetch latest release URL from GitHub API
Write-Host "Fetching latest release from skiLLM-Labs/Krait..."
$ApiUrl = "https://api.github.com/repos/skiLLM-Labs/Krait/releases/latest"
$Release = Invoke-RestMethod -Uri $ApiUrl
$DownloadUrl = ($Release.assets | Where-Object { $_.name -eq $AssetName }).browser_download_url

If (-not $DownloadUrl) {
    Write-Host "Error: Could not find binary for Windows." -ForegroundColor Red
    Exit
}

# Download binary
Write-Host "Downloading $AssetName..."
Invoke-WebRequest -Uri $DownloadUrl -OutFile $ExePath

# Add to PATH
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
If ($UserPath -notmatch [regex]::Escape($BinDir)) {
    $NewPath = "$UserPath;$BinDir"
    [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
    Write-Host "Added Krait to your User PATH." -ForegroundColor Yellow
}

Write-Host "✅ Krait installed successfully!" -ForegroundColor Green
Write-Host "Please restart your terminal (PowerShell/Command Prompt) and type 'krait'."