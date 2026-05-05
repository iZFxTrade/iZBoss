iZWallet: Native Autonomous Wallet System for iZLife OS

1. Introduction

iZWallet là module ví điện tử bản địa (native) được tích hợp trực tiếp vào lõi iZcore. Đây là giải pháp ví không lưu ký (non-custodial) được thiết kế để đảm bảo quyền tự chủ tuyệt đối cho Founder, Admin và người đóng góp thiết bị (Node Contributors) trong hệ sinh thái iZLife.

iZWallet không chỉ là nơi lưu trữ tài sản mà còn là công cụ xác thực định danh (Identity Provider) để ký số các lệnh thực thi trong mạng lưới P2P.

2. Core Philosophy

Zero-Dependency: Không phụ thuộc vào bất kỳ công ty ví hay cơ sở hạ tầng của bên thứ ba (như MetaMask, Trust Wallet).

Sovereign Control: Người dùng toàn quyền sở hữu Private Key thông qua cụm từ khôi phục (Mnemonic Seed).

Integrated Identity: Ví chính là định danh của Node. Mọi hành động trên CLI đều được ký số bằng khóa của ví để đảm bảo tính toàn vẹn.

3. Technical Architecture

3.1 Cryptographic Standards

Key Generation: Sử dụng thuật toán Ed25519 (hoặc secp256k1 cho khả năng tương thích EVM) để tạo cặp khóa công khai và bí mật.

Mnemonic: Tuân thủ tiêu chuẩn BIP-39 (24 từ khóa tiếng Anh) giúp người dùng có thể khôi phục ví trên bất kỳ nền tảng chuẩn Web3 nào khác nếu cần.

Derivation Path: Sử dụng cấu trúc cây phân cấp để quản lý nhiều tài khoản (ví dụ: ví nhận lợi nhuận, ví vận hành node, ví dự phòng).

3.2 Security & Storage

Cold Storage Logic: Private key được mã hóa bằng chuẩn AES-256-GCM. Khóa giải mã chỉ được tạo ra tạm thời trong RAM khi người dùng nhập đúng mật khẩu và mã 2FA (TOTP).

Physical Isolation: Trên các thiết bị nhúng có hỗ trợ, khóa sẽ được lưu trữ trong vùng nhớ an toàn (Secure Element hoặc TEE).

Obfuscation: Mã nguồn ví được đóng gói (build) vào file nhị phân izcore với các kỹ thuật chống dịch ngược để bảo vệ thuật toán sinh key.

4. Feature Set

4.1 Native Asset Management

Revenue Distribution: Nhận và lưu trữ lợi nhuận hàng tháng tự động từ dự án iZFx.Trade.

Internal Payments: Thanh toán phí thuê Skill, mua Module hoặc trả thưởng cho các user cấp dưới (Mod/Admin) ngay trong hệ thống.

P2P Transfer: Chuyển khoản nội bộ giữa các User ID trong mạng lưới iZLife OS với phí giao dịch gần như bằng không.

Transaction History: Truy xuất lịch sử biến động số dư chi tiết (gửi, nhận, thanh toán) theo thời gian thực.

Multi-chain Support: Hỗ trợ iZLife Internal Ledger và các mạng EVM (BNB Chain, Polygon).

4.2 Network Command Signing

Mọi lệnh điều khiển hệ thống nhạy cảm (như cập nhật firmware, thay đổi quyền user) đều yêu cầu iZWallet ký số (Digitally Signed) trước khi phát tán ra mạng lưới.

5. CLI Implementation (Workflows)

5.1 Wallet Initialization (Auto-Onboarding)

Khi một node mới được thiết lập bởi người dùng:

izcore wallet init


Hệ thống sinh 24 từ khóa bí mật.

Hệ thống yêu cầu người dùng thiết lập mật khẩu cấp 2.

Hiển thị địa chỉ ví công khai (FOUNDER_WALLET_ADDRESS).

5.2 Internal Transfer (P2P)

Chuyển tiền nhanh giữa các thành viên trong mạng lưới thông qua User ID:

izcore wallet pay --to mod_alpha --amount 500 --note "Monthly Reward"


5.3 Transaction History (Audit Trail)

Xem lịch sử giao dịch gần nhất:

izcore wallet history


Kết quả hiển thị:

[TRANSACTION HISTORY]
------------------------------------------------------------
ID: #7782 | Date: 2026-05-01 | Type: RECEIVE
From: iZFx-Revenue-Pool | Amount: +2,500.00 USDT
Note: April 2026 Profit Sharing

ID: #7785 | Date: 2026-05-03 | Type: PAY
To: mod_alpha | Amount: -500.00 USDT
Note: Monthly Reward

ID: #7790 | Date: 2026-05-04 | Type: FEE
Service: Skill-Market-Scanner | Amount: -50.00 USDT
------------------------------------------------------------
Current Balance: 1,950.00 USDT


5.4 External Transfer

Rút tiền ra các ví sàn hoặc ví cá nhân bên ngoài:

izcore wallet send --amount 1000 --to 0x... --memo "Withdraw to Binance"


Yêu cầu xác nhận mã 2FA trước khi ký giao dịch.

6. Founder Address Injection (GitHub Integration)

Đây là cơ chế quan trọng nhất để thiết lập quyền sở hữu tối cao ngay từ khi cài đặt:

Chuẩn bị (Pre-requisite): Founder cần tạo một địa chỉ ví ngoài (ví dụ: MetaMask) và lưu lại địa chỉ công khai (0x...).

GitHub Secrets: Anh khai báo địa chỉ này vào biến FOUNDER_WALLET_ADDRESS trên GitHub.

Build Injection: Lúc build, trình biên dịch nhúng địa chỉ này vào mục ROOT_ADDRESS của hệ thống.

Verification: Khi anh chạy lệnh izcore wallet init, hệ thống sẽ kiểm tra: Nếu địa chỉ vừa tạo (hoặc địa chỉ khôi phục từ Seed Phrase) trùng với FOUNDER_WALLET_ADDRESS, nó sẽ tự động cấp quyền Founder (Full Access) cho node đó.

7. Future Roadmap

Hardware Integration: Hỗ trợ kết nối trực tiếp với ví lạnh Ledger/Trezor qua cổng USB/Bluetooth trên CLI.

Smart Contract Multi-Sig: Hỗ trợ ví đa chữ ký cho các quyết định quản trị mạng lưới cấp cao.

ZK-Privacy: Tích hợp Zero-Knowledge Proofs để ẩn danh hóa các giao dịch nội bộ giữa các node.

© 2026 iZLife OS - Secure, Autonomous, Decentralized.