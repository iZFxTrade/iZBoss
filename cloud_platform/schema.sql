-- iZ.Life BOSS - Database Schema V20 (Emergency Recovery & Expansion)

PRAGMA foreign_keys = OFF;
DROP TABLE IF EXISTS agents;
DROP TABLE IF EXISTS nodes;
DROP TABLE IF EXISTS departments;
DROP TABLE IF EXISTS chat_history;
DROP TABLE IF EXISTS llm_models;
DROP TABLE IF EXISTS webhooks_feeds;
DROP TABLE IF EXISTS assistant_configs;
DROP TABLE IF EXISTS bots;
DROP TABLE IF EXISTS skills_modules;
DROP TABLE IF EXISTS boss_auth_methods;
PRAGMA foreign_keys = ON;

-- 1. Departments
CREATE TABLE departments (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    parent_id TEXT, 
    role_type TEXT, -- 'executive', 'department'
    function_desc TEXT,
    leader_id TEXT, 
    FOREIGN KEY(parent_id) REFERENCES departments(id)
);

-- 2. Nodes (Fixed 5-Node Fleet V20)
CREATE TABLE nodes (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    status TEXT DEFAULT 'offline',
    role TEXT DEFAULT 'node',
    cpu_info TEXT,
    ram_total INTEGER,
    ram_usage INTEGER DEFAULT 0,
    storage_total INTEGER,
    storage_usage INTEGER DEFAULT 0,
    last_heartbeat TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 3. Agents (Unified Workforce Matrix)
CREATE TABLE agents (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    dept_id TEXT,
    node_id TEXT,
    status TEXT DEFAULT 'idle',
    skill TEXT,
    role_desc TEXT,
    system_prompt TEXT,
    model_id TEXT,
    progress INTEGER DEFAULT 0,
    current_task TEXT,
    is_leader BOOLEAN DEFAULT 0,
    FOREIGN KEY(dept_id) REFERENCES departments(id),
    FOREIGN KEY(node_id) REFERENCES nodes(id),
    FOREIGN KEY(model_id) REFERENCES llm_models(id)
);

-- 4. LLM Models (V20 Gemini Sync)
CREATE TABLE llm_models (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    provider TEXT NOT NULL, 
    type TEXT, 
    model_path TEXT,
    api_key TEXT,
    base_url TEXT,
    status TEXT DEFAULT 'active'
);

-- 5. Chat History (Memory Core)
CREATE TABLE chat_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT,
    role TEXT, 
    content TEXT,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 6. Integrated Bots & Comms
CREATE TABLE bots (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    platform TEXT NOT NULL, 
    token TEXT,
    webhook_url TEXT,
    status TEXT DEFAULT 'active'
);

-- 7. Skills & DNA Modules
CREATE TABLE skills_modules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    type TEXT,
    version TEXT,
    description TEXT,
    install_cmd TEXT
);

-- 8. Webhooks & Feeds
CREATE TABLE webhooks_feeds (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    url TEXT,
    provider TEXT, 
    api_key TEXT,
    status TEXT DEFAULT 'active'
);

-- 9. Boss Auth (Deprecated - using Cloudflare Zero Trust)
-- Previous auth_methods table removed in favor of Cloudflare Access policies

-- SEED DATA V20
INSERT INTO llm_models (id, name, provider, type, model_path, api_key) VALUES 
('cf-llama3-8b', 'Llama 3 8B (CF)', 'cloudflare', 'general', '@cf/meta/llama-3-8b-instruct', NULL),
('gemini-1.5-pro', 'Gemini 1.5 Pro', 'gemini', 'high-intelligence', 'gemini-1.5-pro', 'AIzaSyAdP9OQ7SpKP9w63vQJLdXJWvDQyIxZLEU'),
('gemini-1.5-flash', 'Gemini 1.5 Flash', 'gemini', 'general', 'gemini-1.5-flash', 'AIzaSyAdP9OQ7SpKP9w63vQJLdXJWvDQyIxZLEU'),
('nvidia-codellama-70b', 'CodeLlama 70B (NVIDIA)', 'nvidia', 'code', 'meta/codellama-70b-instruct', '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'),
('nvidia-nemotron-4', 'Nemotron 4 (NVIDIA)', 'nvidia', 'strategic', 'nvidia/nemotron-4-340b-instruct', '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');

INSERT INTO departments (id, name, role_type, function_desc) VALUES 
('executive', 'Hội đồng Quản trị', 'executive', 'Đơn vị điều hành tối cao BOSS.'),
('finance', 'Phòng Tài chính', 'department', 'Quản lý ví, hạch toán lợi nhuận.'),
('evolution', 'Phòng Tiến hóa (R&D)', 'department', 'Nghiên cứu DNA core, tự nâng cấp.'),
('izfx', 'Phòng iZFx (Trading)', 'department', 'Vận hành EA TradeKiem.'),
('marketing', 'Phòng Marketing', 'department', 'Chiến dịch nội dung tự động.'),
('sales_cs', 'Phòng Sales & CSKH', 'department', 'Phễu izthuchi & Hub CS.'),
('hr_manager', 'Phòng Nhân sự (HR)', 'department', 'Giám sát Workforce Agent.');

-- V20 Fleet (5 Specific Nodes)
INSERT INTO nodes (id, name, cpu_info, ram_total, storage_total, status, role) VALUES 
('mac-m4', 'MAC Mini M4', 'Apple M4', 32768, 1024, 'online', 'master'),
('vps-win', 'VPS Windows', 'Xeon Gold', 16384, 512, 'online', 'vps'),
('note-9', 'Samsung Note 9', 'Exynos 9810', 6144, 128, 'online', 'mobile'),
('note-10-p', 'Samsung Note 10+', 'Exynos 9825', 12288, 256, 'online', 'mobile'),
('playbox-2020', 'FPT Play Box 2020', 'Cortex A53', 2048, 16, 'online', 'edge');

-- V20 Agents (Executive + Leaders)
INSERT INTO agents (id, name, dept_id, node_id, status, skill, role_desc, system_prompt, model_id, is_leader, progress, current_task) VALUES 
('BA', 'Trợ lý BOSS (BA)', 'executive', 'mac-m4', 'idle', 'Strategic Planning', 'Bộ não chiến lược của hệ thống BOSS.', 'Bạn là BOSS Assistant (BA). Phân tích dữ liệu hệ thống và báo cáo chiến lược. Trả lời tiếng Việt.', 'cf-llama3-8b', 1, 0, 'Analyzing Collective Intelligence'),
('AO', 'Trợ lý Điều hành (AO)', 'executive', 'mac-m4', 'idle', 'Global Ops', 'Giám đốc vận hành hệ thống Matrix.', 'Bạn là Administrative Officer (AO). Điều phối Agent phòng ban. Trả lời tiếng Việt.', 'cf-llama3-8b', 1, 0, 'Allocating Node Resources'),
('L-Finance', 'CFO Alpha', 'finance', 'vps-win', 'idle', 'Treasury', 'Trưởng phòng Tài chính', 'Bạn là CFO Alpha. Trả lời tiếng Việt.', 'gemini-1.5-flash', 1, 0, 'Audit Pending'),
('L-Evolution', 'CTO Omega', 'evolution', 'mac-m4', 'running', 'DNA R&D', 'Trưởng phòng R&D', 'Bạn là CTO Omega. Sử dụng NVIDIA Model. Trả lời tiếng Việt.', 'nvidia-codellama-70b', 1, 75, 'Refining Wasm P2P Kernel'),
('L-iZFx', 'Quants Master', 'izfx', 'vps-win', 'running', 'Trade Engines', 'Trưởng phòng Trading', 'Bạn là Quants Master. Trả lời tiếng Việt.', 'gemini-1.5-pro', 1, 30, 'Monitoring Gold Grid'),
('L-Marketing', 'Growth Lead', 'marketing', 'vps-win', 'idle', 'Automation', 'Trưởng phòng Marketing', 'Bạn là Growth Lead. Trả lời tiếng Việt.', 'gemini-1.5-pro', 1, 0, 'Planning Viral Wave'),
('L-Sales', 'Sales Director', 'sales_cs', 'note-10-p', 'idle', 'Sales Flow', 'Giám đốc Kinh doanh', 'Bạn là Sales Director. Trả lời tiếng Việt.', 'gemini-1.5-flash', 1, 0, 'Client Onboarding'),
('L-HR', 'HR Director', 'hr_manager', 'mac-m4', 'idle', 'Workforce', 'Trưởng phòng Nhân sự', 'Bạn là HR Director. Trả lời tiếng Việt.', 'cf-llama3-8b', 1, 0, 'Verifying Agent Heartbeats');

INSERT INTO bots (id, name, platform, token) VALUES 
('tg-boss', 'iZ BOSS Main Bot', 'telegram', '8438121452:AAF0J-6ALOFOHAl-0bFB_HNRD1iwMaCl8x4');

INSERT INTO webhooks_feeds (id, name, type, url, provider) VALUES 
('gold-price', 'XAUUSD Feed', 'data', 'https://api.izfx.com/gold', 'iZFX Data'),
('news-global', 'Global News', 'feed', 'https://news.iz.life/feed', 'Reuters World');

INSERT INTO skills_modules (id, name, type, version, description, install_cmd) VALUES 
('zeroclaw', 'Zeroclaw Engine', 'core', 'v2.1', 'Lõi thực thi song song.', './install zeroclaw'),
('cli-anything', 'CLI-Anything', 'util', 'v1.4', 'Điều khiển từ xa.', './install cli-anything');
