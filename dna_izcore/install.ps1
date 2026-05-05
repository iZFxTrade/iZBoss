# ============================================================
# iZ.Life BOSS — iZCore Windows Installer (PowerShell)
# ============================================================
# Usage: irm https://boss.iz.life/install | iex
# ============================================================

$ErrorActionPreference = 'Stop'

$BOSS_API = "https://boss.iz.life"
$GITHUB_REPO = "iZFxTrade/iZBoss"
$INSTALL_DIR = "$env:USERPROFILE\bin"
$BINARY_NAME = "iZCore.exe"

# ── Step 0: Get Latest Version ───────────────────────────────
Write-Host "[iZCore] Checking latest version from GitHub..." -ForegroundColor Cyan
try {
    $Release = Invoke-RestMethod -Uri "https://api.github.com/repos/$GITHUB_REPO/releases/latest"
    $VERSION = $Release.tag_name
} catch {
    Write-Host "[!] Could not fetch version, defaulting to v0.1.1" -ForegroundColor Yellow
    $VERSION = "v0.1.1"
}

# ── Step 1: Detect Architecture ──────────────────────────────
$ARCH = $env:PROCESSOR_ARCHITECTURE
if ($ARCH -eq "AMD64") {
    $PLATFORM = "windows-x86_64"
} else {
    Write-Error "Unsupported architecture: $ARCH. Only x86_64 is supported on Windows for now."
    exit
}

Write-Host "[✓] Device: Windows / $ARCH ($PLATFORM)" -ForegroundColor Green

# ── Step 2: Device Fingerprint ───────────────────────────────
Write-Host "[iZCore] Generating Device Fingerprint..." -ForegroundColor Cyan
$CPU = Get-CimInstance Win32_Processor | Select-Object -ExpandProperty Name
$MAC = Get-CimInstance Win32_NetworkAdapterConfiguration | Where-Object { $_.IPEnabled -eq $true } | Select-Object -First 1 -ExpandProperty MACAddress
$HOST_NAME = $env:COMPUTERNAME
$RAW_ID = "$CPU`_$MAC`_$HOST_NAME"
$SHA = [System.Security.Cryptography.SHA256]::Create()
$BYTES = [System.Text.Encoding]::UTF8.GetBytes($RAW_ID)
$HASH = $SHA.ComputeHash($BYTES)
$DEVICE_ID = "iznode-" + ([System.BitConverter]::ToString($HASH).Replace("-", "").ToLower().Substring(0, 16))

Write-Host "[✓] Device ID: $DEVICE_ID" -ForegroundColor Green

# ── Step 3: Download Binary ──────────────────────────────────
$DOWNLOAD_URL = "https://github.com/$GITHUB_REPO/releases/download/$VERSION/iZCore-$PLATFORM.exe"
$TEMP_EXE = "$env:TEMP\iZCore_temp.exe"

Write-Host "[iZCore] Downloading binary from GitHub..." -ForegroundColor Cyan
Write-Host "URL: $DOWNLOAD_URL"

if (-not (Test-Path $INSTALL_DIR)) {
    New-Item -Path $INSTALL_DIR -ItemType Directory | Out-Null
}

Invoke-WebRequest -Uri $DOWNLOAD_URL -OutFile $TEMP_EXE

# ── Step 4: Install ──────────────────────────────────────────
Write-Host "[iZCore] Installing to $INSTALL_DIR..." -ForegroundColor Cyan
Move-Item -Path $TEMP_EXE -Destination "$INSTALL_DIR\$BINARY_NAME" -Force

# Add to PATH for current session
if ($env:PATH -notlike "*$INSTALL_DIR*") {
    $env:PATH += ";$INSTALL_DIR"
    [Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$INSTALL_DIR", "User")
}

Write-Host "[✓] iZCore installed at: $INSTALL_DIR\$BINARY_NAME" -ForegroundColor Green

# ── Step 5: Register ─────────────────────────────────────────
Write-Host "[iZCore] Registering with Command Center..." -ForegroundColor Cyan
$PAYLOAD = @{
    device_id = $DEVICE_ID
    platform  = $PLATFORM
    hostname  = $HOST_NAME
    version   = $VERSION
} | ConvertTo-Json

try {
    Invoke-RestMethod -Uri "$BOSS_API/api/register" -Method Post -Body $PAYLOAD -ContentType "application/json" | Out-Null
    Write-Host "[✓] Registered successfully!" -ForegroundColor Green
} catch {
    Write-Host "[!] Command Center unavailable, will retry on startup." -ForegroundColor Yellow
}

# ── Step 6: Start ────────────────────────────────────────────
Write-Host "[iZCore] Starting iZCore in background..." -ForegroundColor Cyan
Start-Process -FilePath "$INSTALL_DIR\$BINARY_NAME" -WindowStyle Hidden

Write-Host ""
Write-Host "╔══════════════════════════════════════════════╗" -ForegroundColor Green
Write-Host "║        iZCore Installed Successfully!        ║" -ForegroundColor Green
Write-Host "╚══════════════════════════════════════════════╝" -ForegroundColor Green
Write-Host ""
Write-Host "  Device ID : $DEVICE_ID"
Write-Host "  Platform  : $PLATFORM"
Write-Host "  Command   : iZCore --status"
Write-Host ""
