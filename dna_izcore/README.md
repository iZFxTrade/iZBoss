# 🦀 dna_iZCore — The DNA Kernel

## Chức năng
Đây là **lõi hệ thống iZ.Life BOSS** — "mã gen" siêu nhẹ viết bằng Rust, được cài đặt trên mọi thiết bị tham gia mạng lưới. Kernel này chịu trách nhiệm định danh thiết bị, xác thực quyền truy cập, kết nối P2P và nhận bản cập nhật tự động (OTA).

## Công nghệ
- **Language**: Rust (async/await với Tokio)
- **P2P**: `libp2p` với mDNS và GossipSub Protocol
- **Auth**: ED25519 digital signature (`ed25519-dalek`)
- **Fingerprint**: SHA-256 hash từ CPU + MAC + Disk Serial (`sysinfo`, `sha2`, `mac_address`)
- **OTA**: Poll Cloudflare R2 để nhận binary mới

## Cấu trúc thư mục
```
dna_iZCore/
├── src/
│   ├── main.rs             # Entry point — Khởi động tuần tự 5 tầng Kernel
│   ├── fingerprint.rs      # Hardware fingerprinting (CPU+MAC+Disk → SHA256 hash)
│   ├── auth.rs             # ED25519 signature verify — Auth Gate
│   ├── bootstrap.rs        # Self-bootstrap: kết nối về boss.iz.life/api/register
│   ├── p2p.rs              # libp2p node: mDNS peer discovery + GossipSub
│   └── ota.rs              # OTA listener: poll Cloudflare R2 mỗi 60 giây
└── Cargo.toml              # Dependencies (workspace-managed)
```

## Luồng khởi động (Boot Sequence)
```
main()
  ├── 1. fingerprint::generate_device_id()    → SHA256(CPU+MAC+Disk)
  ├── 2. auth::verify_master_key()            → Chỉ Đại K mới pass
  ├── 3. bootstrap::connect_to_command_center() → Đăng ký device lên boss.iz.life
  ├── 4. p2p::start_p2p_network()             → Spawn async task (mDNS)
  └── 5. ota::start_ota_listener()            → Spawn async task (poll R2)
```

## Trạng thái hiện tại
| Module | Trạng thái | Ghi chú |
|---|---|---|
| `fingerprint.rs` | 🟡 Partial | Disk serial vẫn là mock |
| `auth.rs` | 🟡 Partial | Luôn trả về `true` — chưa verify thực |
| `bootstrap.rs` | 🟡 Partial | Log ra terminal, chưa gọi API thực |
| `p2p.rs` | 🟡 Skeleton | Loop giả, chưa khởi động libp2p node thực |
| `ota.rs` | 🟡 Skeleton | Loop giả, chưa poll R2 thực |

## Lệnh build & run
```bash
# Build từ workspace root
cargo build -p dna_iZCore

# Chạy Kernel trên thiết bị
cargo run -p dna_iZCore

# Build release (production binary)
cargo build -p dna_iZCore --release
# Output: target/release/dna_iZCore
```
