# iZ.Life BOSS Cloud Platform (V21)

Hệ thống điều hành trung tâm iZ.Life BOSS dựa trên nền tảng Cloudflare Workers và D1 Database.

## 🏗 Cấu trúc Hạ tầng
- **Cloudflare Worker**: Xử lý API, Giao diện Dashboard và Logic Agent.
- **Cloudflare D1 (SQLite)**: Cơ sở dữ liệu quan hệ cho Nodes, Agents, Departments và Chat History.
- **Cloudflare KV**: Lưu trữ dữ liệu News Feed và Cache tạm thời.

## 📂 Sơ đồ Thư mục
- `/src`: Mã nguồn chính của Worker (TypeScript).
- `/schema.sql`: Định nghĩa cấu trúc Database (V20+ Matrix).
- `/wrangler.toml`: Cấu hình deployment và bindings.

## 🚀 Lệnh vận hành
- **Deploy**: `npm run deploy`
- **Sync Schema (Remote)**: `npx wrangler d1 execute bossizlife --remote --file=./schema.sql`
- **Reset Database**: `npx wrangler d1 execute bossizlife --remote --command="PRAGMA foreign_keys = OFF; DROP TABLE ..."`

## 🛡 Bảo mật (Auth DNA)
Hệ thống sử dụng các phương thức xác thực:
1. **ED25519 Signature**: Xác thực lệnh từ Node tới Cloud.
2. **Cloudflare Access**: Bảo vệ bảng điều khiển Dashboard.
3. **API Keys (Secrets)**: Lưu giữ trong D1 (Encrypted) hoặc Wrangler Secrets.
