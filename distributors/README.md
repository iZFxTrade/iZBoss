# 📦 distributors — OTA Binary Storage

## Chức năng
Kho chứa các file thực thi (binary) đã được compile sẵn cho từng nền tảng — dùng để phân phối qua cơ chế **OTA (Over-The-Air Update)**. Khi `dna_iZCore` nhận tín hiệu cập nhật từ Cloudflare R2, nó sẽ tải binary tương ứng từ kho này về và tự cài đặt.

## Cấu trúc
```
distributors/
├── README.md               # File này
├── linux-aarch64/          # [TODO] Binary cho ARM64 (Mac Mini M4, Raspberry Pi)
├── linux-x86_64/           # [TODO] Binary cho x86_64 (VPS Windows/Linux)
├── android-aarch64/        # [TODO] Binary cho Android (Samsung Note 9/10+)
└── edge-armv7/             # [TODO] Binary cho ARMv7 (FPT Play Box 2020)
```

## Quy trình OTA
```
1. Developer build binary mới
   └─ cargo build -p dna_iZCore --release --target <platform>

2. Upload lên Cloudflare R2
   └─ wrangler r2 object put boss-ota/<version>/<platform>/dna_iZCore <binary>

3. Cập nhật manifest version trên boss.iz.life/api/ota/latest

4. dna_iZCore trên các thiết bị poll mỗi 60 giây
   └─ Phát hiện version mới → tải về → verify SHA256 → restart
```

## Targets hỗ trợ
| Platform | Target Triple | Node |
|---|---|---|
| Mac Mini M4 | `aarch64-apple-darwin` | `mac-m4` |
| VPS Windows | `x86_64-pc-windows-msvc` | `vps-win` |
| Samsung Note 9/10+ | `aarch64-linux-android` | `note-9`, `note-10-p` |
| FPT Play Box | `armv7-linux-androideabi` | `playbox-2020` |

## Lệnh build cross-platform
```bash
# Cài cross-compilation toolchain
cargo install cross

# Build cho từng platform
cross build -p dna_iZCore --release --target aarch64-apple-darwin
cross build -p dna_iZCore --release --target x86_64-pc-windows-msvc
cross build -p dna_iZCore --release --target aarch64-linux-android
```

> ⚠️ **Lưu ý bảo mật**: Không commit binary files lên Git. Chỉ upload lên Cloudflare R2 qua wrangler CLI.
