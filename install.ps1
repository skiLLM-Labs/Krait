Write-Host "Installing Krait Programming Language (v1.0.0)..." -ForegroundColor Red

# Setup Directories
$KraitDir = "$env:USERPROFILE\.krait"
$BinDir = "$KraitDir\bin"
If (!(Test-Path $BinDir)) { New-Item -ItemType Directory -Force -Path $BinDir | Out-Null }

$ExePath = "$BinDir\krait.exe"

# Detect System Architecture
$Arch = $env:PROCESSOR_ARCHITECTURE
If ($Arch -eq "ARM64") {
    $AssetName = "krait-windows-arm64.exe"
} ElseIf ($Arch -eq "AMD64") {
    $AssetName = "krait-windows-x64.exe"
} Else {
    Write-Host "Error: Unsupported Windows architecture ($Arch)." -ForegroundColor Red
    Exit
}

# Fetch latest release URL from GitHub API
Write-Host "Fetching latest release from skiLLM-Labs/Krait ($AssetName)..."
$ApiUrl = "https://api.github.com/repos/skiLLM-Labs/Krait/releases/latest"

try {
    $Release = Invoke-RestMethod -Uri $ApiUrl -Headers @{"User-Agent"="PowerShell-Krait-Installer"}
    $DownloadUrl = ($Release.assets | Where-Object { $_.name -eq $AssetName }).browser_download_url
} catch {
    Write-Host "Error: Failed to reach GitHub API." -ForegroundColor Red
    Exit
}

If (-not $DownloadUrl) {
    Write-Host "Error: Could not find binary asset '$AssetName' in the latest release." -ForegroundColor Red
    Exit
}

# Download binary
Write-Host "Downloading from $DownloadUrl..."
Invoke-WebRequest -Uri $DownloadUrl -OutFile $ExePath

# Add to PATH
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
If ($UserPath -notmatch [regex]::Escape($BinDir)) {
    # Strip any trailing semicolons from the existing path cleanly before appending
    $UserPath = $UserPath -replace ';+$', ''
    $NewPath = "$UserPath;$BinDir"
    [Environment]::SetEnvironmentVariable("PATH", $NewPath, "User")
    Write-Host "Added Krait to your User PATH." -ForegroundColor Yellow
}

Write-Host "✅ Krait installed successfully!" -ForegroundColor Green
Write-Host "Please restart your terminal (PowerShell/Command Prompt) and type 'krait'."
