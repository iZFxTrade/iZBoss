# 📈 trading — Phòng iZFx (Trading)

## Chức năng
Vận hành hệ thống giao dịch tự động — kết nối với EA TradeKiem, theo dõi vị thế mở, quản lý quỹ khách hàng và tối ưu hóa hiệu suất giao dịch.

## Cấu trúc
```
departments/trading/
├── src/
│   └── lib.rs      # Trading logic (đang xây dựng)
└── Cargo.toml
```

## Nhiệm vụ chính
| Task | Mô tả | Trạng thái |
|---|---|---|
| EA Monitor | Theo dõi trạng thái EA TradeKiem | 📋 Planned |
| Position Tracker | Đọc open positions từ cTrader/MT5 | 📋 Planned |
| PnL Calculator | Tính P&L real-time theo symbol | 📋 Planned |
| Risk Manager | SL/TP alert khi vượt ngưỡng | 📋 Planned |
| XAUUSD Feed | Poll giá từ `webhooks_feeds.gold-price` | 📋 Planned |
| Fund Report | Báo cáo quỹ khách → Finance dept | 📋 Planned |

## Tích hợp
- **cTrader**: Protobuf TCP/TLS API (từ conversation ea188ba6)
- **XAUUSD Feed**: `https://api.izfx.com/gold`
- **Node**: `vps-win` (Xeon Gold — latency thấp)

## Agent phụ trách: `Quants Master` (L-iZFx)
