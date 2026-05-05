# 📦 cloud_platform — Cloudflare Edge Layer

## Chức năng
Đây là **bệ phóng Cloudflare** — trung tâm điều phối của toàn bộ hệ thống iZ.Life BOSS. Chạy hoàn toàn trên Cloudflare Workers (serverless), không cần server riêng, phủ sóng toàn cầu.

## Công nghệ
- **Runtime**: Cloudflare Workers (TypeScript)
- **Database**: Cloudflare D1 (SQLite-compatible, binding: `DB`)
- **KV Storage**: Cloudflare KV (binding: `NEWS`)
- **AI**: Cloudflare Workers AI (binding: `AI`)
- **Domain**: `boss.iz.life`

## Cấu trúc thư mục
```
cloud_platform/
├── src/
│   ├── index.ts                    # Entry point — Router chính của Worker
│   ├── routes/                     # [TODO] Tách routes riêng biệt
│   │   ├── api.ts                  # /api/data, /api/agents, /api/nodes
│   │   ├── telegram.ts             # /webhook/telegram
│   │   └── dashboard.ts            # / — Render HTML Dashboard
│   ├── services/                   # [TODO] Business logic layer
│   │   ├── agentService.ts
│   │   └── nodeService.ts
│   ├── durable-objects/            # Cloudflare Durable Objects (stateful sessions)
│   │   ├── AgentChatSession.ts     # Quản lý phiên chat với Agent
│   │   ├── AgentCoordinator.ts     # Điều phối Agent tasks
│   │   ├── RoomManager.ts          # Quản lý phòng chat
│   │   ├── TelegramSessionManager.ts # Quản lý session Telegram
│   │   └── types.ts                # TypeScript type definitions
│   └── middleware/
│       ├── auth.ts                 # Xác thực request (Cloudflare Zero Trust)
│       ├── errorHandler.ts         # Xử lý lỗi toàn cục
│       └── logger.ts               # Request logging
├── MASTER_DNA_SCHEMA.sql           # ⭐ Cấu trúc bảng D1 — Phiên bản chuẩn V25.5
├── MASTER_DNA_SEED.sql             # ⭐ Dữ liệu mẫu — Nodes, Agents, Departments
├── wrangler.toml                   # Cấu hình deploy Cloudflare Workers
├── package.json                    # NPM dependencies
└── tsconfig.json                   # TypeScript config
```

## API Endpoints
| Endpoint | Method | Mô tả |
|---|---|---|
| `/` | GET | Dashboard HTML (Sovereign Matrix UI) |
| `/api/data` | GET | Trả về toàn bộ data: nodes, agents, depts, models |
| `/install` | GET | Bootstrap script cho iZCore Kernel |
| `/webhook/telegram` | POST | Nhận cập nhật từ Telegram Bot |

## Database (D1 Tables)
Xem chi tiết tại `MASTER_DNA_SCHEMA.sql`. Các bảng chính:
- `nodes` — Fleet thiết bị
- `agents` — Workforce AI
- `departments` — Cơ cấu tổ chức
- `llm_models` — Danh mục LLM
- `matrix_api_keys` — API Key Registry
- `wallets` — Tài chính
- `matrix_chat_history` — Lịch sử chat
- `bots` / `bot_whitelist` — Bot Comms
- `skills_modules` — Skills & Modules
- `webhooks_feeds` — Data Feeds

## Lệnh triển khai
```bash
# Dev local
npx wrangler dev

# Deploy production
npx wrangler deploy

# Apply schema lên D1
npx wrangler d1 execute bossizlife --remote --file=./MASTER_DNA_SCHEMA.sql
npx wrangler d1 execute bossizlife --remote --file=./MASTER_DNA_SEED.sql
```
