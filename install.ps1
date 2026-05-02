# ⬡ POLYGONE — Smart Installer for Windows
# Requires: PowerShell 5.0+ (Windows 10/11)
# Run: irm https://raw.githubusercontent.com/lvs0/Polygone/main/install.ps1 | iex

$VERSION = "1.0.0"
$INSTALL_DIR = "$env:USERPROFILE\\.local\\bin"
$BINARY_URL = "https://github.com/lvs0/Polygone/releases/download/v${VERSION}/polygone.exe"

Write-Host ""
Write-Host "  ⬡ POLYGONE v${VERSION}" -ForegroundColor Cyan
Write-Host "  Post-quantum ephemeral privacy network" -ForegroundColor Cyan
Write-Host ""

# Create install dir
New-Item -ItemType Directory -Force -Path $INSTALL_DIR | Out-Null

# ─── Method 1: Download pre-built binary ────────────────────────
$downloaded = $false
Write-Host "  Downloading pre-built binary..." -ForegroundColor Cyan

try {
    Invoke-WebRequest -Uri $BINARY_URL -OutFile "$INSTALL_DIR\\polygone.exe" -UseBasicParsing
    & "$INSTALL_DIR\\polygone.exe" --version | Out-Null
    Write-Host "  ✓ Binary installed from GitHub Releases" -ForegroundColor Green
    $downloaded = $true
} catch {
    Remove-Item -Force "$INSTALL_DIR\\polygone.exe" -ErrorAction SilentlyContinue
}

# ─── Method 2: Build from source (fallback) ────────────────────
if (-not $downloaded) {
    Write-Host "  ! Pre-built not available, building from source..." -ForegroundColor Yellow
    
    # Check Rust
    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Host "  ↓ Installing Rust..." -ForegroundColor Cyan
        Invoke-WebRequest -Uri https://sh.rustup.rs -OutFile /tmp/rustup-init.sh -UseBasicParsing
        bash /tmp/rustup-init.sh -y
        & "$env:USERPROFILE\\.cargo\\env"
    }
    
    Write-Host "  ⚙ Building release binary (this may take a while)..." -ForegroundColor Cyan
    git clone https://github.com/lvs0/Polygone.git "$env:TEMP\\polygone-build"
    Set-Location "$env:TEMP\\polygone-build"
    cargo build --release
    Copy-Item "target\\release\\polygone.exe" "$INSTALL_DIR\\polygone.exe"
    Write-Host "  ✓ Built from source and installed" -ForegroundColor Green
}

# ─── Post-install ───────────────────────────────────────────────
Write-Host ""
Write-Host "  ✓ POLYGONE v${VERSION} installed!" -ForegroundColor Green
Write-Host "  Location: $INSTALL_DIR\\polygone.exe"
Write-Host ""

# Run self-test
Write-Host "  Running self-test..." -ForegroundColor Cyan
& "$INSTALL_DIR\\polygone.exe" self-test

Write-Host ""
Write-Host "  ⬡ POLYGONE is ready." -ForegroundColor Green
Write-Host ""
