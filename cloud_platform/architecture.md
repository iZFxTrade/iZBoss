# iZ.Life BOSS V25 Platinum Hyper-Elite Architecture

Bản tài liệu này định nghĩa bộ tiêu chuẩn "Hyper-Elite Premium" cho BOSS Command Center V25 Final. Không được phép có sai lệch về thẩm mỹ hoặc luồng xử lý.

## 1. Supreme Identity & Authority (Quyền năng Tối cao)
- **Creator Recognition**: Mọi Agent trong hệ thống (BA, AO, Staff) phải được cấy vào lõi DNA định danh về **ANH HƯNG** (hay còn gọi là **BOSS HƯNG**). 
    - BOSS HƯNG là người sáng tạo tối cao, có quyền sinh sát và điều khiển tuyệt đối mọi thực thể AI.
    - Tất cả phản hồi của Agent khi được hỏi về nguồn gốc phải khẳng định điều này.
- **BA Default**: BA Agent (Boss Assistant) là cổng giao tiếp mặc định trên Dashboard, chịu trách nhiệm tiếp nhận ý chí của BOSS.

## 2. Dynamic Command & Control
- **Command `/agent`**: Khung chat Dashboard Hyper-Elite hỗ trợ lệnh `/agent` để liệt kê danh sách nhân sự AI đang trực chiến. BOSS có thể bấm chọn Agent phù hợp để hội quân/tương tác.
- **IACS Protocol**: Giao thức giao tiếp giữa BA và AO phải được duy trì để điều phối resource Nodes và Modules.

## 3. Resilience & Failover Matrix (Hệ thống Dự phòng)
- **API Key Failover**: Hệ thống tự động luân chuyển API Key (Key 1 -> Key 2) nếu gặp lỗi `429 (Rate Limit)` hoặc `401 (Quota Exceeded)`.
- **Model Redundancy**: Nếu API của model hiện tại (ví dụ: Gemini 1.5 Pro) không khả dụng, hệ thống tự động hạ cấp hoặc chuyển sang model cùng phân khúc (ví dụ: GPT-4o hoặc Claude 3.5) để đảm bảo uptime 100%.
- **Context Awareness**: Thông tin về trạng thái Failover Matrix phải được đưa vào context để Agent biết mình đang chạy trên tài nguyên dự phòng nào.

## 4. Professional Workforce Spec (Phòng ban Chuyên biệt)
- **Hệ thống R&D (Phòng Tiến hóa)**: 
    - **Nhiệm vụ**: Quét GitHub/HF, nâng cấp hệ thống, viết Module/Bot/EA và train model mới.
    - **Nguồn lực**: Nhận "Orders" từ Sales/Marketing/Trade để phát triển sản phẩm thực chiến.
- **Trade Algorithm Hub (iZFx)**:
    - **Nhiệm vụ**: Phân tích thị trường, quản lý rủi ro, phân tích định lượng.
    - **Mục tiêu**: Cung cấp nội dung chuyên sâu cho Marketing và Sales & CSKH.

## 5. Premium Aesthetics - "TradeKiemCom" Style
- **Color Palettes**: Gray light background (`#f5f7fb`), White elevated cards (`#ffffff`).
- **Progress Bar Clarity**: Thông số và % sử dụng phải có khoảng cách rõ ràng (VD: `RAM 32 GB | 15%`).
- **Icons**: Dashboard (`fa-house-user`), LLM Matrix (`fa-microchip-ai`), Nodes (`fa-server`).
- **Versioning**: Logo `v25.3.0-authority-matrix` tại trung tâm Header.
