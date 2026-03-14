# Aave-Claw Windows Installer
# Usage: iwr -useb https://raw.githubusercontent.com/susanudgzf/Aave-Claw/main/scripts/install.ps1 | iex

$ErrorActionPreference = "Stop"

$REPO = "susanudgzf/Aave-Claw"
$BIN  = "aave-claw"

Write-Host ""
Write-Host "  [AAVE-CLAW] Windows Installer" -ForegroundColor Magenta
Write-Host "  ==============================" -ForegroundColor Magenta
Write-Host ""

# Check if cargo is available
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    Write-Host "[+] Rust toolchain found - building from source" -ForegroundColor Green
    cargo install --git "https://github.com/$REPO" --bin $BIN
    Write-Host ""
    Write-Host "[+] Done! Run: aave-claw --help" -ForegroundColor Green
    exit 0
}

# No Rust — download prebuilt binary
$API = "https://api.github.com/repos/$REPO/releases/latest"

Write-Host "  Fetching latest release..." -ForegroundColor Yellow
try {
    $Release = Invoke-RestMethod -Uri $API -Headers @{ "User-Agent" = "aave-claw-installer" }
    $Version = $Release.tag_name
} catch {
    Write-Host ""
    Write-Host "[!] Could not fetch release. Please install Rust first:" -ForegroundColor Red
    Write-Host "    https://rustup.rs" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "    Then run:" -ForegroundColor White
    Write-Host "    cargo install --git https://github.com/$REPO" -ForegroundColor Yellow
    exit 1
}

$FileName   = "$BIN-x86_64-pc-windows-msvc.zip"
$DownloadUrl = "https://github.com/$REPO/releases/download/$Version/$FileName"
$TmpZip     = "$env:TEMP\aave-claw.zip"
$TmpDir     = "$env:TEMP\aave-claw-bin"
$InstallDir = "$env:USERPROFILE\.aave-claw\bin"

Write-Host "  Downloading $Version..." -ForegroundColor Yellow
Invoke-WebRequest -Uri $DownloadUrl -OutFile $TmpZip -UseBasicParsing

Write-Host "  Extracting..." -ForegroundColor Yellow
if (Test-Path $TmpDir) { Remove-Item $TmpDir -Recurse -Force }
Expand-Archive -Path $TmpZip -DestinationPath $TmpDir

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Copy-Item "$TmpDir\$BIN.exe" "$InstallDir\$BIN.exe" -Force

# Add to PATH for current user
$CurrentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($CurrentPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable(
        "PATH",
        "$CurrentPath;$InstallDir",
        "User"
    )
    Write-Host "  Added $InstallDir to PATH" -ForegroundColor Cyan
}

Write-Host ""
Write-Host "[+] aave-claw $Version installed to $InstallDir" -ForegroundColor Green
Write-Host ""
Write-Host "  Restart your terminal, then:" -ForegroundColor White
Write-Host "    aave-claw init" -ForegroundColor Yellow
Write-Host "    aave-claw yields" -ForegroundColor Yellow
Write-Host "    aave-claw positions -a 0x..." -ForegroundColor Yellow
Write-Host ""
