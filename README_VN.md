# iZ.Life BOSS (Hệ điều hành Doanh nghiệp - Thực thể Tự trị)

🇻🇳 **Tiếng Việt** | 🇺🇸 [English](README.md)

Dự án **iZ.Life BOSS** là một hệ điều hành doanh nghiệp thực thể tự trị, bao gồm các hệ thống phân tán, các AI thông minh hoạt động như nhân sự và điều hành, cùng với kiến trúc kết nối P2P và Cloudflare.

---

## 1. TẦNG DNA: iZcore (The Kernel)
Lõi mã nguồn siêu nhẹ viết bằng **Rust**, đóng vai trò là "mã gen" nằm trong mọi thiết bị.
- **Hardware Fingerprinting**: Thuật toán băm (Hashing) định danh thiết bị dựa trên CPU ID, MAC Address, Disk Serial.
- **Self-Bootstrap**: Script tối giản kết nối về `boss.iz.life` để xác định môi trường và tải bộ cài.
- **P2P Handshake**: Thư viện `libp2p` tự tìm đồng đội qua mDNS / Gossip Protocol.
- **Auth Gate**: Xác thực chữ ký số (ED25519) - Chỉ admin mới có quyền đánh thức thực thể.
- **OTA Listener**: Tiến trình ngầm nhận bản cập nhật nóng qua Cloudflare R2 / Node lân cận.

## 2. LỚP 1: Command Center (Cloudflare Workers)
- **Dashboard (boss.iz.life)**: Giao diện quản lý tập trung.
- **Node Manager**: Quản lý thiết bị tham gia mạng, sức khỏe, tài nguyên.
- **Agent & Process Monitor**: Giám sát tiến trình Agent, log thực thi, trạng thái Heartbeat.
- **Skill & Module Registry**: Kho lưu trữ các logic/skills có thể điều phối xuống Node.
- **LLM & API Gateway**: Quản lý API Key (OpenAI, Gemini...), cấu hình hạn mức.
- **Webhook & Data Feed Manager**: Nguồn cung cấp dữ liệu (Giá cả, News, Social webhook).
- **OTA Warehouse**: Kho chứa file thực thi, cài đặt trên Cloudflare R2.
- **Task Queue (D1/KV)**: Hàng đợi công việc cho toàn mạng lưới.

## 3. LỚP 2: Hệ thống Trợ lý Tối cao (The Executives)
Phối hợp 2 AI chuyên biệt:
1. **Trợ lý BOSS (BA - Boss Assistant)**: Bộ não chiến lược (Strategic Brain), lắng nghe ý tưởng, thiết lập OKRs, giám sát tối cao.
2. **Trợ lý Điều hành (AO - Administrative Officer)**: Giám đốc vận hành (Operating Director), phân bổ resource, quản trị Agent phòng ban (HR), trực tiếp điểu hành OTA và báo cáo thực thi trở lại cho BA.

## 4. CƠ CHẾ GIAO TIẾP (BA ↔ AO PROTOCOL)
Sử dụng **IACS (Inter-Assistant Communication Schema)**:
- `GOAL_ASSIGN` (BA -> AO): Giao mục tiêu, ngân sách, thời hạn.
- `RESOURCE_QUERY` (BA -> AO): Hỏi về mạng lưới, tài nguyên.
- `STATUS_REPORT` (AO -> BA): Báo cáo tiến độ và rủi ro từ phòng ban.
- `ACTION_CONFIRM` (AO -> BA): Xác nhận hoạt động đã được khởi tạo.

## 5. CÁC PHÒNG BAN TỰ TRỊ (UNDER AO)
- **Finance**: Quản lý ví, hạch toán lợi nhuận.
- **Evolution (R&D)**: Quét tài nguyên free (HF, GitHub), tự nâng cấp mô hình.
- **iZFx (Trading)**: Cày quỹ, quản lý quỹ khách, tối ưu EA.
- **Marketing**: Tự động hóa nội dung truyền thông.
- **Sales & CSKH**: Chăm sóc khách hàng, phễu `izthuchi`.
- **Agent Manager (HR)**: Cánh tay của AO quản lý "nhân sự số".

## 6. ROADMAP PHÁT TRIỂN
- **Giai đoạn 1: Q1 - The Launchpad**: Cloudflare Dashboard, Rust OTA.
- **Giai đoạn 2: Q2 - The Mesh**: CLI ED25519, P2P network.
- **Giai đoạn 3: Q3 - The Sovereignty**: Self-Hosting Dashboard, tự quản tài chính dự án.
- **Giai đoạn 4: Singularity**: Self-Coding, Startup Machine.

---

## 7. CẤU TRÚC THƯ MỤC
```text
/
├── /dna_izcore (Rust)             # Lõi định danh, P2P, OTA Kernel
├── /cloud_platform (TS)           # Bệ phóng Cloudflare (Workers/D1)
├── /executive_layer (Rust)        # BA (Brain), AO (Action), HR Manager
├── /departments                   # Các phòng ban tự trị (Rust/Wasm)
│   ├── /finance                   # Quản lý ví & Kế toán
│   ├── /rd                        # Quét GitHub, HF, Tự nâng cấp
│   ├── /trading                   # EA, AI Trading, Quản lý quỹ
│   ├── /marketing                 # Auto-Content & Social API
│   └── /sales_cs                  # Hub & Chatbot tương tác
├── /distributors (Binaries)       # OTA Binary Storage
└── /cli_gateway (Rust)            # Cổng giao diện dòng lệnh tối cao
```

---

## 8. TRIỂN KHAI NHANH

### ⚡ Cài đặt iZcore (Mọi thiết bị)
Bạn có thể chọn một trong hai nguồn cài đặt sau (cả hai đều tự động nhận diện thiết bị và tải binary):

**Cách 1: Qua tên miền hệ thống (Khuyên dùng)**
```bash
curl -fsSL https://boss.iz.life/install | sh
```

**Cách 2: Trực tiếp từ GitHub (Dự phòng/Tin cậy tuyệt đối)**
```bash
curl -fsSL https://raw.githubusercontent.com/iZFxTrade/izboss/main/dna_izcore/install.sh | sh
```

> **Lưu ý**: Nguồn cài đặt hiện tại ưu tiên lấy trực tiếp từ GitHub dự án. Hệ thống `boss.iz.life` đang được hoàn thiện để trở thành một mạng lưới lưu trữ phi tập trung (tương tự BitTorrent), giúp việc phân phối iZcore không phụ thuộc vào bất kỳ tên miền cố định nào trong tương lai.

### 💻 Giao diện dòng lệnh (CLI)
Sau khi cài đặt, sử dụng lệnh `boss` để quản trị toàn hệ thống:
- `boss status`: Kiểm tra trạng thái các Nodes & Agents.
- `boss agent list`: Xem danh sách nhân sự số đang hoạt động.
- `boss node fleet`: Quản lý các thiết bị trong mạng lưới.
