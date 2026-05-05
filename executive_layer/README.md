# 🧠 executive_layer — AI Executive Intelligence

## Chức năng
Tầng điều hành trí tuệ nhân tạo của hệ thống — nơi **BA (Boss Assistant)** và **AO (Administrative Officer)** phối hợp để biến chỉ thị của Đại K thành hành động thực tế trong toàn bộ hệ thống.

## Công nghệ
- **Language**: Rust (async/await với Tokio)
- **LLM**: Cloudflare Workers AI (Llama 3.1 8B) / Gemini Pro
- **Communication**: IACS Protocol (Inter-Assistant Communication Schema)
- **API**: Giao tiếp với `boss.iz.life` qua HTTP

## Cấu trúc thư mục
```
executive_layer/
├── src/
│   ├── main.rs             # Entry point — khởi động BA và AO, vào vòng lắng nghe lệnh
│   ├── ba.rs               # Boss Assistant (BA) — Bộ não chiến lược
│   ├── ao.rs               # Administrative Officer (AO) — Giám đốc vận hành
│   ├── iacs.rs             # IACS Protocol — Message bus giữa BA và AO
│   └── llm_client.rs       # LLM Client — Gọi Cloudflare AI / Gemini API
└── Cargo.toml
```

## Kiến trúc BA ↔ AO
```
Đại K (Input)
    ↓
  [BA] Boss Assistant
    ├── Phân tích chiến lược (LLM call)
    ├── Gửi GOAL_ASSIGN → AO
    └── Nhận STATUS_REPORT ← AO
              ↓ IACS Protocol
  [AO] Administrative Officer
    ├── Nhận GOAL_ASSIGN từ BA
    ├── Phân bổ task xuống Departments
    ├── Theo dõi tiến độ
    └── Gửi STATUS_REPORT → BA
              ↓
  [Departments] Finance | Trading | R&D | Marketing | Sales
```

## IACS Message Types
| Message | Hướng | Mô tả |
|---|---|---|
| `GOAL_ASSIGN` | BA → AO | Giao mục tiêu, ngân sách, thời hạn |
| `RESOURCE_QUERY` | BA → AO | Hỏi về tài nguyên, trạng thái mạng |
| `STATUS_REPORT` | AO → BA | Báo cáo tiến độ và rủi ro |
| `ACTION_CONFIRM` | AO → BA | Xác nhận hoạt động đã khởi tạo |

## Trạng thái hiện tại
| Module | Trạng thái | Ghi chú |
|---|---|---|
| `iacs.rs` | ✅ Done | Message types, envelope, routing đầy đủ |
| `llm_client.rs` | ✅ Done | CF AI + mock mode |
| `ba.rs` | ✅ Done | process_input, assign_goal_to_ao |
| `ao.rs` | 🔨 In Progress | Đang xây dựng |
| `main.rs` | 🔨 In Progress | Đang xây dựng |

## Lệnh build & run
```bash
# Build từ workspace root
cargo build -p executive_layer

# Chạy Executive Layer
cargo run -p executive_layer

# Với env vars
CF_ACCOUNT_ID=xxx CF_API_TOKEN=yyy cargo run -p executive_layer
```
