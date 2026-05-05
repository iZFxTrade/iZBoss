# 📢 marketing — Phòng Marketing

## Chức năng
Tự động hóa toàn bộ hoạt động marketing — tạo nội dung bằng AI, lên lịch đăng bài, phân tích hiệu suất và mở rộng độ phủ thương hiệu iZ.Life trên các kênh mạng xã hội.

## Cấu trúc
```
departments/marketing/
├── src/
│   └── lib.rs      # Marketing automation logic (đang xây dựng)
└── Cargo.toml
```

## Nhiệm vụ chính
| Task | Mô tả | Trạng thái |
|---|---|---|
| Content Generator | Dùng LLM (Gemini Creative) tạo bài viết/caption | 📋 Planned |
| Social Scheduler | Lên lịch đăng bài tự động theo giờ vàng | 📋 Planned |
| Trend Monitor | Theo dõi xu hướng từ `webhooks_feeds.news-global` | 📋 Planned |
| Performance Report | Phân tích reach, engagement hàng tuần → BA | 📋 Planned |
| Campaign Manager | Quản lý chiến dịch quảng bá theo OKRs | 📋 Planned |

## Kênh mục tiêu
- **Telegram Channel**: iZ.Life Community
- **Facebook/Zalo**: TradeKiem & iZFx Group
- **Hub News**: `hub.iz.life/news` — feed tin tức nội bộ

## Agent phụ trách: `Growth Lead` (L-Marketing) — Node `vps-win`
## Model LLM: `Gemini 1.5 Pro` (model_group: creative)
