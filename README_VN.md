# 🌌 iZ.Life BOSS — Mạng Lưới Hạ Tầng Trí Tuệ Nhân Tạo Phi Tập Trung (DePIN)
### Định Nghĩa Lại Quyền Sở Hữu Hạ Tầng & Trí Tuệ | Tầm Nhìn Tỷ Đô

🇻🇳 **Tiếng Việt** | 🇺🇸 [English](README.md)

**iZ.Life BOSS** không chỉ là một phần mềm, mà là một cuộc cách mạng về **Hạ tầng Vật lý Phi tập trung (DePIN)**. Chúng tôi đang xây dựng một mạng lưới Mesh toàn cầu, nơi mọi thiết bị phần cứng nhàn rỗi — từ chiếc điện thoại cũ, PlayBox, đến những dàn Server khủng — đều có thể hóa thân thành một **Node Trí tuệ** để cùng vận hành, cùng kiếm tiền và cùng chia sẻ lợi nhuận.

---

## 🚀 Tầm Nhìn: Hệ Sinh Thái Trí Tuệ Vô Hạn
Chúng tôi hướng tới việc xây dựng một siêu máy tính phân tán lớn nhất thế giới, biến mọi tài nguyên phần cứng rời rạc thành một thực thể thống nhất có khả năng:
-   **AI Training & Inference**: Huấn luyện và vận hành các mô hình LLM khổng lồ một cách phi tập trung.
-   **Autonomous Trading Bot**: Vận hành các thuật toán giao dịch tài chính tự trị 24/7.
-   **Distributed Storage**: Lưu trữ dữ liệu bảo mật, bất tử trên mạng lưới Mesh.
-   **Task Execution Marketplace**: Thị trường thực thi các tác vụ kỹ thuật số phức tạp.

## 💰 Mô Hình Kinh Tế Token (Proof of Contribution)
Mỗi User tham gia iZBoss sẽ được cấp một **Địa chỉ Ví duy nhất**. Hệ thống sẽ tự động trả thưởng Token dựa trên năng lực tính toán và giá trị thực tế mà Node đó đóng góp:
-   **Compute Rewards**: Trả thưởng cho việc cho mượn CPU/GPU để chạy Model AI.
-   **Trading Rewards**: Chia sẻ lợi nhuận khi Node tham gia vận hành các Bot tài chính thành công.
-   **Storage Rewards**: Thu nhập thụ động từ việc lưu trữ dữ liệu cho mạng lưới.
-   **Skill Rewards**: Thưởng cho việc phát triển và đóng góp các Kỹ năng (Skills) mới vào DNA.

---

## 🤖 iZCore Agent: Quản Trị Bằng Ngôn Ngữ Tự Nhiên
iZCore hiện đã hỗ trợ kích hoạt Trợ lý AI cục bộ (Tiny LLM) để giúp bạn điều hành hệ thống mà không cần nhớ các câu lệnh CLI phức tạp:
```bash
izcore agent on
```
Tính năng này sẽ tự động phân tích phần cứng và cài đặt mô hình ngôn ngữ (như Phi-3 hoặc Llama-3) để biến thiết bị của bạn thành một thực thể có khả năng giao tiếp và tự vận hành. Khi mạng lưới đủ mạnh, một **AO (Autonomous Operator)** toàn cầu sẽ được kích hoạt để làm trợ lý cho toàn bộ mạng lưới.

## 🛡️ Mạng Lưới Bất Tử (Immortal Mesh Network)
iZ.Life BOSS được thiết kế để tồn tại vĩnh cửu. Ngay cả khi tên miền `boss.iz.life` biến mất, mạng lưới vẫn tự vận hành nhờ:
-   **Direct IP Seeding**: Các Node hạt giống được nhúng trực tiếp bằng địa chỉ IP trong mã nguồn.
-   **Peer Caching**: Mỗi Node tự ghi nhớ danh sách bạn bè đã từng kết nối.
-   **Local mDNS**: Tự nhận diện anh em trong mạng LAN không cần internet.

---

## 1. TẦNG DNA: iZCore (The Kernel - Bất tử)
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
├── /dna_iZCore (Rust)             # Lõi định danh, P2P, OTA Kernel
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

### ⚡ Cài đặt iZCore (Mọi thiết bị)
Sử dụng một URL duy nhất cho mọi nền tảng. Hệ thống sẽ tự động trả về script phù hợp (Bash cho Unix/Mac hoặc PowerShell cho Windows):

**Nguồn 1: Qua tên miền hệ thống (Khuyên dùng)**
- **Linux / macOS / Android**:
  ```bash
  curl -fsSL https://boss.iz.life/install | sh
  ```
- **Windows (PowerShell)**:
  ```powershell
  irm https://boss.iz.life/install | iex
  ```

**Nguồn 2: Trực tiếp từ GitHub (Dự phòng/Tin cậy tuyệt đối)**
- **Unix-like**: `curl -fsSL https://raw.githubusercontent.com/iZFxTrade/iZBoss/main/dna_iZCore/install.sh | sh`
- **Windows**: `irm https://raw.githubusercontent.com/iZFxTrade/iZBoss/main/dna_iZCore/install.ps1 | iex`

> **Lưu ý**: Nguồn cài đặt hiện tại ưu tiên lấy trực tiếp từ GitHub dự án. Hệ thống `boss.iz.life` đang được hoàn thiện để trở thành một mạng lưới lưu trữ phi tập trung (tương tự BitTorrent).

### 💻 Giao diện dòng lệnh (CLI)
Sau khi cài đặt, sử dụng lệnh `boss` để quản trị toàn hệ thống:
- `boss status`: Kiểm tra trạng thái các Nodes & Agents.
- `boss agent list`: Xem danh sách nhân sự số đang hoạt động.
- `boss node fleet`: Quản lý các thiết bị trong mạng lưới.
