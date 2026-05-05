# 💻 cli_gateway — Supreme CLI Interface

## Chức năng
Cổng giao diện dòng lệnh tối cao — cho phép **Đại K điều phối toàn bộ hệ thống iZ.Life BOSS trực tiếp từ Terminal** mà không cần mở trình duyệt. Giao tiếp với `boss.iz.life` API qua HTTP.

## Công nghệ
- **Language**: Rust
- **CLI Framework**: `clap` v4 (derive macro)
- **HTTP Client**: `reqwest`
- **Output**: `colored` (terminal màu sắc chuyên nghiệp)

## Cấu trúc thư mục
```
cli_gateway/
├── src/
│   ├── main.rs                 # Entry point — CLI router với clap
│   ├── api_client.rs           # [TODO] HTTP client → boss.iz.life API
│   └── commands/
│       ├── status.rs           # [TODO] `boss status` — xem tổng quan hệ thống
│       ├── agent.rs            # [TODO] `boss agent list/start/stop <id>`
│       └── node.rs             # [TODO] `boss node fleet` — xem fleet & heartbeat
└── Cargo.toml
```

## Danh sách lệnh (CLI Commands)
```bash
boss status                     # Tổng quan: Nodes online, Agents active, D1 status
boss agent list                 # Liệt kê tất cả Agents và trạng thái
boss agent start <agent-id>     # Khởi động Agent
boss agent stop <agent-id>      # Dừng Agent
boss node fleet                 # Xem Fleet: MAC Mini, VPS, Mobile, Edge
boss node ping <node-id>        # Kiểm tra heartbeat của Node
boss deploy <module>            # Trigger OTA update cho module
boss ba "<chỉ thị>"             # Gửi lệnh thẳng đến BA (Boss Assistant)
```

## Trạng thái hiện tại
| Module | Trạng thái | Ghi chú |
|---|---|---|
| `Cargo.toml` | ✅ Done | clap + reqwest + colored |
| `main.rs` | 🔨 In Progress | Đang xây dựng |
| `api_client.rs` | 📋 Planned | Chưa bắt đầu |
| `commands/` | 📋 Planned | Chưa bắt đầu |

## Lệnh build & run
```bash
# Build
cargo build -p cli_gateway

# Chạy (dev)
cargo run -p cli_gateway -- status
cargo run -p cli_gateway -- agent list
cargo run -p cli_gateway -- ba "Phân tích tình hình Trading tuần này"

# Install binary vào PATH
cargo install --path cli_gateway
boss status  # Dùng trực tiếp
```

## Cấu hình
```bash
# Đặt biến môi trường cho API endpoint
export BOSS_API_URL="https://boss.iz.life"
export BOSS_API_KEY="your-api-key"
```
