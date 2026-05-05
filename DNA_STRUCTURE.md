# iZ.Life BOSS V24 Master DNA Structure

Bản tài liệu này là "Hiến pháp" kỹ thuật cho BOSS Command Center. Mọi phiên bản nâng cấp (V25+) PHẢI tuân thủ các quy tắc cấu trúc dưới đây để đảm bảo không bị "nuốt menu" hoặc mất tính năng.

## 1. Cấu trúc Menu Sidebar (Bắt buộc 12 Module)
Sidebar KHÔNG được chứa Logo (Logo dời lên Header). Danh sách Menu phải giữ nguyên Icons và View IDs:
1.  **Dashboard**: Tổng quan Matrix (Vcards + Processes + Finance).
2.  **Nodes Fleet**: Quản lý hạ tầng Node (Mac Mini, VPS, Mobile...).
3.  **Workforce**: Quản lý Agent (BA, AO, Leader, Sub-agents).
4.  **LLM Matrix**: Danh mục Model (Cloudflare, Gemini, NVIDIA, OpenRouter).
5.  **Skills**: Các kỹ năng cốt lõi của iZcore.
6.  **Modules**: Thư viện DNA mở rộng.
7.  **Oanda Price**: Hook cập nhật giá từ Oanda.
8.  **Hub News**: Feed tin tức từ hub.iz.life/news.
9.  **API Registry**: Quản lý Cloud Auth & Keys.
10. **Bots Comms**: Quản lý Telegram/System Bots.
11. **Departments**: Cơ cấu tổ chức (Phòng ban).
12. **Finance Hub**: Theo dõi chi phí Token & Ngân sách.

## 2. Kiến trúc Header Platinum
- **Vị trí Trung tâm**: Logo `iZ.BOSS` (In nghiêng, Bold, Màu Accent).
- **Góc Phải (Right)**:
    - Nút chuyển Dark/Light Mode (Icon Only).
    - Nút Chat Assistant (Icon Only).

## 3. Cấu hình Agent (Agent Logic DNA)
Modal chỉnh sửa Agent PHẢI chứa đủ 6 trường thông tin:
- **Identity**: Tên Agent.
- **Role/Skill**: Chức năng nhiệm vụ (BA, AO, Lead...).
- **Department**: Phòng ban trực thuộc.
- **Model**: LLM được gán (Cloudflare cho BA/AO, NVIDIA cho R&D, Gemini cho Creative).
- **DNA Prompt**: System Prompt cốt lõi.
- **Status**: Trạng thái (Online/Offline/Standby).

## 4. Dashboard Intelligence
- **Vcards**: Theo dõi Total/Active/Offline cho Nodes, Agents và Brains.
- **Process Tracker**: Hiển thị Task, Agent, Node và Progress %.
- **Finance Feed**: Thống kê Token Total vs Used, phân loại Model Trả phí ($) và Miễn phí (Free).

## 5. Kết nối Dữ liệu
- Toàn bộ dữ liệu từ 11+ bảng D1 phải được load vào Dashboard để Trợ lý có thể đọc và phân tích (Context Injection).
- Trợ lý có quyền đề xuất Thêm/Sửa/Xóa dữ liệu dựa trên vai trò được gán.
