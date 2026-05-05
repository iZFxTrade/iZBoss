# 🤝 sales_cs — Phòng Sales & Chăm Sóc Khách Hàng

## Chức năng
Hub tương tác với khách hàng — vận hành chatbot đa kênh (Telegram/Zalo), quản lý phễu bán hàng `izthuchi`, chăm sóc khách hàng tự động 24/7 và theo dõi conversion rate.

## Cấu trúc
```
departments/sales_cs/
├── src/
│   └── lib.rs      # Sales & CS logic (đang xây dựng)
└── Cargo.toml
```

## Nhiệm vụ chính
| Task | Mô tả | Trạng thái |
|---|---|---|
| Telegram Bot | Chatbot tự động trả lời khách qua `tg-boss` | 📋 Planned |
| Whitelist Manager | Quản lý `bot_whitelist` (approve/revoke user) | 📋 Planned |
| Funnel Tracker | Theo dõi phễu `izthuchi` — lead → close | 📋 Planned |
| Auto Reply | Trả lời câu hỏi thường gặp bằng LLM | 📋 Planned |
| Sales Report | Báo cáo conversion, doanh thu → Finance | 📋 Planned |

## Tích hợp
- **Telegram Bot**: `tg-boss` — token quản lý trong `bots` table (D1)
- **Whitelist**: `iZFxTrade`, `FxBlueNet` (từ `bot_whitelist`)
- **Phễu izthuchi**: Landing page chuyển đổi khách hàng
- **Node**: `note-10-p` (Samsung Note 10+ — Mobile-first)

## Agent phụ trách: `Sales Director` (L-Sales)
## Model LLM: `Gemini 1.5 Flash` (nhanh, phù hợp realtime chat)
