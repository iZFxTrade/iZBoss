#!/bin/sh
# ============================================================
# iZ.Life BOSS — iZCore Universal Installer
# ============================================================
# Cài đặt: curl -fsSL https://boss.iz.life/install | sh
# ============================================================

set -e

BOSS_API="https://boss.iz.life"
GITHUB_REPO="iZFxTrade/iZBoss"
INSTALL_DIR="/usr/local/bin"
SERVICE_NAME="iZCore"
USERNAME=""

# Parse arguments
while [ "$#" -gt 0 ]; do
  case "$1" in
    --username) USERNAME="$2"; shift 2 ;;
    *) shift ;;
  esac
done

# ── Step 0: Get Latest Version from GitHub ───────────────────
log "Đang kiểm tra phiên bản mới nhất từ GitHub..."
LATEST_TAG=$(curl -s https://api.github.com/repos/${GITHUB_REPO}/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_TAG" ]; then
    warn "Không lấy được version từ GitHub — sử dụng v0.1.1 làm mặc định."
    LATEST_TAG="v0.1.1"
fi
VERSION="$LATEST_TAG"

# ── Colors ──────────────────────────────────────────────────
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
CYAN='\033[0;36m'; BOLD='\033[1m'; NC='\033[0m'

log()  { printf "${CYAN}[iZCore]${NC} %s\n" "$1"; }
ok()   { printf "${GREEN}[✓]${NC} %s\n" "$1"; }
warn() { printf "${YELLOW}[!]${NC} %s\n" "$1"; }
die()  { printf "${RED}[✗]${NC} %s\n" "$1"; exit 1; }

# ── Banner ───────────────────────────────────────────────────
printf "\n${BOLD}${CYAN}"
printf "╔══════════════════════════════════════════════╗\n"
printf "║       iZ.Life BOSS — iZCore Installer        ║\n"
printf "║         The DNA Kernel for Every Node        ║\n"
printf "╚══════════════════════════════════════════════╝\n"
printf "${NC}\n"

# ── Step 1: Detect OS & Architecture ─────────────────────────
log "Đang nhận diện thiết bị..."

OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$ARCH" in
    x86_64)  ARCH_SLUG="x86_64" ;;
    aarch64|arm64) ARCH_SLUG="aarch64" ;;
    armv7l)  ARCH_SLUG="armv7" ;;
    *)       die "Kiến trúc không được hỗ trợ: $ARCH" ;;
esac

case "$OS" in
    linux)   OS_SLUG="linux" ;;
    darwin)  OS_SLUG="macos" ;;
    android) OS_SLUG="android" ;;
    *)       die "Hệ điều hành không được hỗ trợ: $OS" ;;
esac

PLATFORM="${OS_SLUG}-${ARCH_SLUG}"
ok "Phát hiện thiết bị: ${BOLD}${OS_SLUG}${NC} / ${BOLD}${ARCH_SLUG}${NC} (${PLATFORM})"

# ── Step 2: Get device fingerprint ───────────────────────────
log "Đang tạo Device Fingerprint..."

CPU_INFO=$(cat /proc/cpuinfo 2>/dev/null | grep "model name" | head -1 | cut -d: -f2 | xargs || sysctl -n machdep.cpu.brand_string 2>/dev/null || echo "UnknownCPU")
MAC_ADDR=$(ip link 2>/dev/null | grep "link/ether" | head -1 | awk '{print $2}' || ifconfig 2>/dev/null | grep "ether" | head -1 | awk '{print $2}' || echo "00:00:00:00:00:00")
HOSTNAME=$(hostname)
DEVICE_ID=$(printf "%s_%s_%s" "$CPU_INFO" "$MAC_ADDR" "$HOSTNAME" | sha256sum 2>/dev/null | cut -c1-16 || echo "node-$(date +%s)")
DEVICE_ID="iznode-${DEVICE_ID}"

ok "Device ID: ${BOLD}${DEVICE_ID}${NC}"

# ── Step 3: Check dependencies ───────────────────────────────
log "Đang kiểm tra dependencies..."

for cmd in curl; do
    if ! command -v "$cmd" > /dev/null 2>&1; then
        die "Thiếu dependency: $cmd — vui lòng cài đặt trước."
    fi
done
ok "Dependencies OK"

# ── Step 4: Download binary ──────────────────────────────────
DOWNLOAD_URL="https://github.com/${GITHUB_REPO}/releases/download/${VERSION}/iZCore-${PLATFORM}"
FALLBACK_URL="${BOSS_API}/api/ota/download?platform=${PLATFORM}&version=${VERSION}"
BINARY_PATH="/tmp/iZCore_${PLATFORM}"

log "Đang tải binary cho ${PLATFORM}..."
log "GitHub: ${DOWNLOAD_URL}"

HTTP_STATUS=$(curl -fsSL -w "%{http_code}" -o "$BINARY_PATH" "$DOWNLOAD_URL" 2>/dev/null || echo "000")

if [ "$HTTP_STATUS" != "200" ]; then
    warn "Không tải được từ GitHub (HTTP $HTTP_STATUS) — Đang thử fallback về boss.iz.life..."
    log "Fallback: ${FALLBACK_URL}"
    HTTP_STATUS=$(curl -fsSL -w "%{http_code}" -o "$BINARY_PATH" "$FALLBACK_URL" 2>/dev/null || echo "000")
fi

if [ "$HTTP_STATUS" = "200" ] && [ -s "$BINARY_PATH" ]; then
    ok "Đã tải binary thành công ($(du -sh "$BINARY_PATH" | cut -f1))"
else
    warn "Binary chưa có trên server cho platform: ${PLATFORM}"
    warn "Đang thử build từ source (cần Rust toolchain)..."

    if command -v cargo > /dev/null 2>&1; then
        log "Đang build iZCore từ source..."
        TMP_DIR="/tmp/iZCore_src"
        rm -rf "$TMP_DIR"
        curl -fsSL "${BOSS_API}/api/source/download" -o "/tmp/iZCore_src.tar.gz" 2>/dev/null || true

        if [ -f "/tmp/iZCore_src.tar.gz" ]; then
            mkdir -p "$TMP_DIR"
            tar -xzf "/tmp/iZCore_src.tar.gz" -C "$TMP_DIR"
            cd "$TMP_DIR" && cargo build -p dna_iZCore --release 2>/dev/null
            cp "$TMP_DIR/target/release/dna_iZCore" "$BINARY_PATH"
            ok "Build từ source thành công!"
        else
            die "Không thể tải source. Vui lòng thử lại sau khi binary được phát hành cho ${PLATFORM}."
        fi
    else
        die "Rust/Cargo chưa được cài. Cài tại: https://rustup.rs — sau đó chạy lại installer này."
    fi
fi

# ── Step 5: Install binary ───────────────────────────────────
log "Đang cài đặt iZCore..."

chmod +x "$BINARY_PATH"

if [ -w "$INSTALL_DIR" ]; then
    mv "$BINARY_PATH" "${INSTALL_DIR}/iZCore"
else
    sudo mv "$BINARY_PATH" "${INSTALL_DIR}/iZCore" || {
        warn "Không có quyền sudo — cài vào ~/bin"
        mkdir -p "$HOME/bin"
        mv "$BINARY_PATH" "$HOME/bin/iZCore"
        INSTALL_DIR="$HOME/bin"
    }
fi

ok "iZCore đã cài tại: ${BOLD}${INSTALL_DIR}/iZCore${NC}"

# ── Step 6: Register with Command Center ─────────────────────
log "Đang đăng ký với Command Center..."

REG_PAYLOAD=$(printf '{"device_id":"%s","platform":"%s","hostname":"%s","version":"%s","username":"%s"}' \
    "$DEVICE_ID" "$PLATFORM" "$HOSTNAME" "$VERSION" "$USERNAME")

REG_STATUS=$(curl -s -o /dev/null -w "%{http_code}" \
    -X POST "${BOSS_API}/api/register" \
    -H "Content-Type: application/json" \
    -d "$REG_PAYLOAD" 2>/dev/null || echo "000")

if [ "$REG_STATUS" = "200" ] || [ "$REG_STATUS" = "201" ]; then
    ok "Đã đăng ký thành công với Command Center!"
else
    warn "Command Center chưa phản hồi (HTTP ${REG_STATUS}) — sẽ thử lại khi khởi động."
fi

# ── Step 7: Setup auto-start service ─────────────────────────
log "Đang thiết lập tự động khởi động..."

setup_systemd() {
    SERVICE_FILE="/etc/systemd/system/${SERVICE_NAME}.service"
    cat > /tmp/${SERVICE_NAME}.service << EOF
[Unit]
Description=iZ.Life BOSS — iZCore DNA Kernel
After=network.target
Wants=network-online.target

[Service]
Type=simple
ExecStart=${INSTALL_DIR}/iZCore --username "${USERNAME}"
Restart=always
RestartSec=10
Environment="BOSS_MASTER_KEY=env_key_placeholder"
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF

    if sudo mv /tmp/${SERVICE_NAME}.service "$SERVICE_FILE" 2>/dev/null; then
        sudo systemctl daemon-reload
        sudo systemctl enable "${SERVICE_NAME}"
        sudo systemctl start "${SERVICE_NAME}"
        ok "systemd service đã được kích hoạt: ${BOLD}${SERVICE_NAME}.service${NC}"
    else
        warn "Không đủ quyền để cài systemd service. Chạy thủ công: iZCore"
    fi
}

setup_launchd() {
    PLIST_DIR="$HOME/Library/LaunchAgents"
    PLIST_FILE="${PLIST_DIR}/life.iz.boss.iZCore.plist"
    mkdir -p "$PLIST_DIR"

    cat > "$PLIST_FILE" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>life.iz.boss.iZCore</string>
    <key>ProgramArguments</key>
    <array>
        <string>${INSTALL_DIR}/iZCore</string>
        <string>--username</string>
        <string>${USERNAME}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>EnvironmentVariables</key>
    <dict>
        <key>BOSS_MASTER_KEY</key>
        <string>env_key_placeholder</string>
    </dict>
    <key>StandardOutPath</key>
    <string>/tmp/iZCore.log</string>
    <key>StandardErrorPath</key>
    <string>/tmp/iZCore.err</string>
</dict>
</plist>
EOF

    launchctl load "$PLIST_FILE" 2>/dev/null && \
        ok "LaunchAgent đã được kích hoạt: ${BOLD}life.iz.boss.iZCore${NC}" || \
        warn "Không thể load LaunchAgent — chạy thủ công: iZCore"
}

case "$OS_SLUG" in
    linux)   setup_systemd ;;
    macos)   setup_launchd ;;
    android) warn "Android: chạy thủ công từ terminal hoặc Termux." ;;
esac

# ── Done ─────────────────────────────────────────────────────
printf "\n${BOLD}${GREEN}"
printf "╔══════════════════════════════════════════════╗\n"
printf "║         iZCore đã cài đặt thành công!       ║\n"
printf "╚══════════════════════════════════════════════╝\n"
printf "${NC}\n"

printf "  ${BOLD}Device ID :${NC} %s\n" "$DEVICE_ID"
printf "  ${BOLD}Platform  :${NC} %s\n" "$PLATFORM"
printf "  ${BOLD}Binary    :${NC} %s/iZCore\n" "$INSTALL_DIR"
printf "  ${BOLD}Network   :${NC} boss.iz.life\n"
printf "\n"
printf "  Kiểm tra trạng thái: ${CYAN}iZCore --status${NC}\n"
printf "  Xem logs           : ${CYAN}journalctl -u iZCore -f${NC} (Linux)\n"
printf "                       ${CYAN}tail -f /tmp/iZCore.log${NC} (macOS)\n\n"
