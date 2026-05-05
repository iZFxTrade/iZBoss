# 💰 finance — Phòng Tài Chính

## Chức năng
Quản lý tài chính tự trị của hệ thống iZ.Life BOSS — theo dõi ví, hạch toán lợi nhuận, phân bổ ngân sách và báo cáo tài chính định kỳ về AO.

## Cấu trúc
```
departments/finance/
├── src/
│   └── lib.rs      # Logic tài chính (đang xây dựng)
└── Cargo.toml
```

## Nhiệm vụ chính
| Task | Mô tả | Trạng thái |
|---|---|---|
| Wallet Monitor | Theo dõi số dư 5 ví (Operations, Investment, Savings, Boss, R&D) | 📋 Planned |
| P&L Report | Tính lợi nhuận/lỗ hàng ngày từ Trading dept | 📋 Planned |
| Budget Allocator | Phân bổ ngân sách theo OKRs từ BA | 📋 Planned |
| Expense Tracker | Ghi nhận chi phí: server, API, token usage | 📋 Planned |
| Monthly Report | Báo cáo tài chính tháng → BA/AO | 📋 Planned |

## Các Ví (từ MASTER_DNA_SEED.sql)
- `w-ops` 📌 **Vận hành** — Chi phí cố định, server, điện
- `w-inv` 📈 **Đầu tư** — Tái đầu tư vào Trade/AI
- `w-sav` 💰 **Tích lũy** — Quỹ dự phòng dài hạn
- `w-boss` 👑 **Ví Sếp Hưng** — Mục tiêu: 2,000$/tháng
- `w-rnd` 🧬 **Quỹ R&D** — Phát triển iZCore & model

## Node: `vps-win` (VPS Windows — Xeon Gold)
