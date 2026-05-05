# BOSS Worker Architecture (src/README.md)

Tài liệu chi tiết về logic vận hành của Dashboard Worker.

## 🧠 Logic xử lý (index.ts)
Mọi yêu cầu được phân luồng qua `fetch` handler:

### 1. Endpoint `/api/data`
- Truy vấn toàn bộ hạm đội Node, Agent, và Cấu hình từ D1.
- Trả về JSON cho Front-end Dashboard.

### 2. Endpoint `/api/chat`
- **Identity Awareness**: Xác định Agent gửi lệnh dựa trên `target_agent_id`.
- **Context Injection**: Tự động chèn dữ liệu hệ thống (Node status, history) vào System Prompt.
- **Provider Routing**: Chuyển hướng tới Cloudflare AI (Llama), Gemini API, hoặc NVIDIA NIM.
- **Latency Tracking**: (V21) Đo lường thời gian phản hồi của AI.

### 3. Dashboard UI (renderV20Ecosystem)
- Sử dụng **Bootstrap v5** (V21+) cho layout.
- SPA (Single Page Application) điều hướng qua `switchView`.
- **Chat Interface**: Hỗ trợ Markdown rendering qua `marked.js`.

## 🛠 Quản lý Agents (Matrix CRUD)
- (V21) Cung cấp giao diện Modal để Admin trực tiếp sửa đổi `system_prompt`, `role_desc`, và gán `model_id` chuyên dụng.
- Đồng bộ hóa trực tiếp xuống bảng `agents` trong D1.
