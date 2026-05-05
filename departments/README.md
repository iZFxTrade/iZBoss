# 🏢 departments — Các Phòng Ban Tự Trị

## Chức năng
Tập hợp các **phòng ban tự trị** dưới quyền điều phối của AO (Administrative Officer). Mỗi phòng ban là một Rust crate độc lập, có thể được compile thành WebAssembly (Wasm) để chạy trên bất kỳ Node nào trong mạng lưới.

## Cấu trúc tổng quan
```
departments/
├── finance/            # 💰 Phòng Tài chính
├── trading/            # 📈 Phòng iZFx (Trading)
├── rd/                 # 🧬 Phòng Tiến hóa (R&D)
├── marketing/          # 📢 Phòng Marketing
└── sales_cs/           # 🤝 Phòng Sales & CSKH
```

## Nguyên tắc hoạt động
1. **Nhận lệnh từ AO** qua IACS Protocol (`GOAL_ASSIGN` message).
2. **Thực thi nghiệp vụ** theo chức năng chuyên biệt của từng phòng.
3. **Báo cáo kết quả** về AO qua `STATUS_REPORT` message.
4. Mỗi phòng ban có thể chạy **độc lập** trên node riêng hoặc trên cùng node với Executive Layer.

## Phân công Node
| Phòng ban | Node mặc định | Lý do |
|---|---|---|
| Finance | `vps-win` | Cần môi trường ổn định 24/7 |
| Trading | `vps-win` | Latency thấp cho execution |
| R&D | `mac-m4` | Cần RAM cao cho model inference |
| Marketing | `vps-win` | Batch processing, schedule |
| Sales CS | `note-10-p` | Mobile-first, chatbot |

## Lệnh build tất cả departments
```bash
# Build toàn bộ workspace (bao gồm tất cả departments)
cargo build --workspace

# Build riêng từng department
cargo build -p finance
cargo build -p trading
cargo build -p rd
cargo build -p marketing
cargo build -p sales_cs
```

> Xem README trong từng thư mục con để biết chi tiết từng phòng ban.
