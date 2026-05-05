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
## 7. Decentralized Corporate Architecture (The Sovereign AI Entity)
iZBoss được thiết kế để vận hành như một **Thực thể Doanh nghiệp AI Tự trị** hoàn chỉnh, không cần sự can thiệp của con người trong các quy trình cốt lõi:

### 🏢 Hệ thống Phòng ban Phân tán (Modular Departments)
Mạng lưới Mesh cho phép phân bổ các chức năng doanh nghiệp xuống từng Node:
-   **Node Finance**: Cài đặt module Tài chính, quản lý dòng tiền, quyết toán Token và tự động tái đầu tư.
-   **Node R&D**: Liên tục quét và tìm kiếm công nghệ mới để nâng cấp hệ thống. Chạy các Skill nghiên cứu thị trường, xây dựng mô hình phân tích, hệ thống trading và quản lý tài chính theo yêu cầu từ BA (Business Analyst) và các phòng ban. Việc phát triển các bộ mã nguồn như ERP/CRM chỉ được thực hiện khi có yêu cầu cụ thể từ tầng quản trị.
-   **Node Marketing & Sales**: Tự động hóa việc tìm kiếm khách hàng, chăm sóc khách hàng và quảng bá dịch vụ.
-   **Node Operations (HR)**: Điều phối nhân sự số (Agents), cấp quyền truy cập và giám sát hiệu suất toàn mạng lưới.

### 🚀 Tầm nhìn Tự chủ Tuyệt đối
Mục tiêu tối thượng của iZBoss là trở thành một **Thực thể A.I Automation** tự thân:
-   **Tự chủ Tài chính (Financial Autonomy)**: Tự tạo ra lợi nhuận thông qua việc tự phân tích thị trường, thực thi Auto-trade, quản lý quỹ tự thân và quản lý quỹ cho nhà đầu tư. Ưu tiên cao nhất luôn dành cho các dịch vụ và thị trường tài chính.
-   **Tự tiến hóa (Self-Evolution)**: Liên tục phân tích tính khả thi của công nghệ mới, tự nghiên cứu và tự nâng cấp chính hệ thống để duy trì lợi thế cạnh tranh.
-   **Tự tìm kiếm khách hàng**: Các Agent Sales tự phân tích dữ liệu mạng xã hội để tìm và chốt hợp đồng.
-   **Quản lý dự án tự động**: Mọi dự án (phát triển phần mềm, chiến dịch tài chính...) đều được BA và AO lên kế hoạch, phân bổ Resource và giám sát thực thi 100% tự động.

---
© 2026 iZLife OS Project. All rights reserved.
