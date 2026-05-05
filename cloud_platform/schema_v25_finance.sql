-- iZ.Life BOSS V25.5 Sovereign Matrix - Financial Seed
CREATE TABLE IF NOT EXISTS wallets (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    address TEXT,
    purpose TEXT,
    status TEXT DEFAULT 'active',
    balance_mock TEXT DEFAULT '0.00'
);

INSERT OR REPLACE INTO wallets (id, name, address, purpose, balance_mock) VALUES 
('w-ops', '📌 Vận hành (Operations)', '0x...ops', 'Duy trì hệ thống, server, điện nước và chi phí cố định.', 'Active'),
('w-inv', '📈 Đầu tư (Investment)', '0x...invest', 'Tái đầu tư vào các dự án Trade, AI và mảng kinh doanh mới.', 'Active'),
('w-sav', '💰 Tích lũy (Accumulation)', '0x...sav', 'Quỹ dự phòng an toàn và tích lũy tài sản dài hạn.', 'Active'),
('w-boss', '👑 Ví Sếp Hưng', '0x...boss', 'Mục tiêu: Chuyển 2,000$ mỗi tháng cho BOSS HƯNG.', 'Active'),
('w-rnd', '🧬 Quỹ R&D', '0x...rnd', 'Phát triển iZcore, mua sắm linh kiện, train model mới.', 'Active');
