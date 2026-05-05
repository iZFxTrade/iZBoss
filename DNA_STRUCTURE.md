# iZ.Life BOSS V24 Master DNA Structure

Bản tài liệu này là "Hiến pháp" kỹ thuật cho BOSS Command Center. Mọi phiên bản nâng cấp (V25+) PHẢI tuân thủ các quy tắc cấu trúc dưới đây để đảm bảo không bị "nuốt menu" hoặc mất tính năng.

## 1. Cấu trúc Menu Sidebar (Bắt buộc 12 Module)
Sidebar KHÔNG được chứa Logo (Logo dời lên Header). Danh sách Menu phải giữ nguyên Icons và View IDs:
1.  **Dashboard**: Tổng quan Matrix (Vcards + Processes + Finance).
2.  **Nodes Fleet**: Quản lý hạ tầng Node (Mac Mini, VPS, Mobile...).
3.  **Workforce**: Quản lý Agent (BA, AO, Leader, Sub-agents).
4.  **LLM Matrix**: Danh mục Model (Cloudflare, Gemini, NVIDIA, OpenRouter).
5.  **Skills**: Các kỹ năng cốt lõi của iZCore.
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

## 6. Economic DNA (Token Economy)
Hệ thống tích hợp lớp kinh tế để khuyến khích sự phát triển bền vững:
- **Unique Node Wallets**: Mỗi thực thể iZCore được gán một địa chỉ ví định danh duy nhất.
- **Proof of Contribution (PoC)**: Cơ chế xác thực đóng góp dựa trên:
    - **Compute Power**: Năng lực xử lý CPU/GPU đóng góp cho mạng lưới.
    - **Task Execution**: Số lượng và độ chính xác của các tác vụ hoàn thành (Training, Trading, Bot).
    - **Storage Availability**: Độ tin cậy và dung lượng lưu trữ phân tán.
- **Reward Tiers**:
    - **Vàng (Gold)**: Training & Inference cho các Model LLM lớn.
    - **Bạc (Silver)**: Chạy Bot giao dịch (Trading) và điều hành phòng ban (Agent).
    - **Đồng (Bronze)**: Lưu trữ dữ liệu và duy trì kết nối mạng Mesh.

© 2026 iZLife OS Project. All rights reserved.
