-- ======================================================
-- iZ.Life BOSS - MASTER DNA SEED (V25.5 Omega)
-- ======================================================
-- Essential seed data to populate the Sovereign Matrix.

PRAGMA foreign_keys = OFF;

-- 1. LLM Models & Groups
INSERT INTO llm_models (id, name, provider, type, model_path, model_group) VALUES 
('cf-llama3-8b', 'Llama 3 8B (CF)', 'cloudflare', 'general', '@cf/meta/llama-3-8b-instruct', 'planning'),
('gemini-1.5-pro', 'Gemini 1.5 Pro', 'gemini', 'high-intelligence', 'gemini-1.5-pro', 'research'),
('gemini-1.5-flash', 'Gemini 1.5 Flash', 'gemini', 'general', 'gemini-1.5-flash', 'creative'),
('nvidia-codellama-70b', 'CodeLlama 70B (NVIDIA)', 'nvidia', 'code', 'meta/codellama-70b-instruct', 'code'),
('gpt-4o', 'GPT-4o (Reasoning)', 'openai', 'high-intelligence', 'gpt-4o', 'reasoning');

-- 2. API Keys Registry (Masked for Security)
INSERT INTO matrix_api_keys (id, provider, key_label, key_value) VALUES 
('key-nv-1', 'nvidia', 'Main NVIDIA NIM', 'nvapi-REPLACED_BY_AI_FOR_SECURITY'),
('key-gm-1', 'gemini', 'Main Gemini Pro', 'AIzaSy-REPLACED_BY_AI_FOR_SECURITY'),
('key-or-1', 'openrouter', 'Main OpenRouter', 'sk-or-v1-REPLACED_BY_AI_FOR_SECURITY');

-- 3. Departments Hierarchy
INSERT INTO departments (id, name, role_type, function_desc, lead_agent_id) VALUES 
('dept-exec', 'Hội đồng Quản trị', 'executive', 'Đơn vị điều hành tối cao BOSS.', 'BA'),
('dept-finance', 'Phòng Tài chính', 'department', 'Quản lý ví, hạch toán lợi nhuận.', 'CFO'),
('dept-tech', 'Phòng Tiến hóa (R&D)', 'department', 'Nghiên cứu DNA core, tự nâng cấp.', 'CTO'),
('dept-trading', 'Phòng iZFx (Trading)', 'department', 'Vận hành EA TradeKiem.', 'AO'),
('dept-marketing', 'Phòng Marketing', 'department', 'Chiến dịch nội dung tự động.', 'L-Marketing'),
('dept-sales', 'Phòng Sales & CSKH', 'department', 'Phễu izthuchi & Hub CS.', 'L-Sales');

-- 4. Infrastructure Fleet (5 Core Nodes)
INSERT INTO nodes (id, name, cpu_info, ram_total, storage_total, status, role) VALUES 
('mac-m4', 'MAC Mini M4', 'Apple M4', 32768, 1024, 'online', 'master'),
('vps-win', 'VPS Windows', 'Xeon Gold', 16384, 512, 'online', 'vps'),
('note-9', 'Samsung Note 9', 'Exynos 9810', 6144, 128, 'online', 'mobile'),
('note-10-p', 'Samsung Note 10+', 'Exynos 9825', 12288, 256, 'online', 'mobile'),
('playbox-2020', 'FPT Play Box 2020', 'Cortex A53', 2048, 16, 'online', 'edge');

-- 5. Unified Workforce Matrix (Executive + Leaders)
INSERT INTO agents (id, name, dept_id, node_id, status, skill, role_desc, model_id, is_leader, progress) VALUES 
('BA', 'Trợ lý BOSS (BA)', 'dept-exec', 'mac-m4', 'idle', 'Strategic Planning', 'Bộ não chiến lược của BOSS.', 'gpt-4o', 1, 0),
('AO', 'Trợ lý Điều hành (AO)', 'dept-trading', 'mac-m4', 'idle', 'Global Ops', 'Giám đốc vận hành Matrix.', 'cf-llama3-8b', 1, 0),
('CFO', 'CFO Alpha', 'dept-finance', 'vps-win', 'idle', 'Treasury', 'Trưởng phòng Tài chính.', 'gemini-1.5-flash', 1, 0),
('CTO', 'CTO Omega', 'dept-tech', 'mac-m4', 'running', 'DNA R&D', 'Trưởng phòng R&D.', 'nvidia-codellama-70b', 1, 75),
('L-Marketing', 'Growth Lead', 'dept-marketing', 'vps-win', 'idle', 'Automation', 'Trưởng phòng Marketing.', 'gemini-1.5-pro', 1, 0),
('L-Sales', 'Sales Director', 'dept-sales', 'note-10-p', 'idle', 'Sales Flow', 'Giám đốc Kinh doanh.', 'gemini-1.5-flash', 1, 0);

-- 6. Financial Sovereignty (Wallets)
INSERT INTO wallets (id, name, address, purpose, balance_mock) VALUES 
('w-ops', '📌 Vận hành (Operations)', '0x...ops', 'Duy trì hệ thống, server, chi phí cố định.', 'Active'),
('w-inv', '📈 Đầu tư (Investment)', '0x...invest', 'Tái đầu tư vào các dự án Trade/AI mới.', 'Active'),
('w-sav', '💰 Tích lũy (Accumulation)', '0x...sav', 'Quỹ dự phòng an toàn dài hạn.', 'Active'),
('w-boss', '👑 Ví Sếp Hưng', '0x...boss', 'Mục tiêu: 2,000$/tháng cho BOSS HƯNG.', 'Active'),
('w-rnd', '🧬 Quỹ R&D', '0x...rnd', 'Phát triển iZcore, mua sắm linh kiện.', 'Active');

-- 7. Skills & DNA Modules
INSERT INTO skills_modules (id, name, type, status, description) VALUES 
('mod-zclaw-01', 'ZeroClaw Master V2', 'module', 'active', 'Module cào dữ liệu tối thượng bypass Cloudflare.'),
('mod-p2p-01', 'Global P2P Network', 'module', 'active', 'Module kết nối mạng phi tập trung toàn cầu.'),
('mod-cli-01', 'CLI Anything Matrix', 'module', 'active', 'Module thực thi lệnh hệ thống vạn năng.'),
('sk-research-01', 'Deep Research Omni', 'skill', 'active', 'Kỹ năng nghiên cứu đa tầng vạn năng.'),
('sk-coding-01', 'Matrix Coder X', 'skill', 'active', 'Kỹ năng lập trình tối ưu hóa đa nền tảng.');

-- 8. Bot Integration & Whitelist
INSERT INTO bots (id, name, platform, token, status) VALUES 
('tg-boss', 'iZ BOSS Main Bot', 'telegram', '8438121452:AAFOJ-REPLACED', 'active');

INSERT INTO bot_whitelist (id, bot_id, username) VALUES 
('wl-1', 'tg-boss', 'iZFxTrade'),
('wl-2', 'tg-boss', 'FxBlueNet');

-- 9. External Feeds
INSERT INTO webhooks_feeds (id, name, type, url, provider) VALUES 
('gold-price', 'XAUUSD Feed', 'data', 'https://api.izfx.com/gold', 'iZFX Data'),
('news-global', 'Global News', 'feed', 'https://news.iz.life/feed', 'Reuters World');

PRAGMA foreign_keys = ON;
