-- Surgical Fix for matrix_api_keys and Master V25.5 Setup
PRAGMA foreign_keys = OFF;

-- Ensure matrix_api_keys has limit_tpd
-- SQLite doesn't support IF NOT EXISTS in ALTER TABLE directly, so we try multiple steps or just use the master schema if we can.
-- But since we are using 'npx wrangler d1 execute', we can just use surgical ALTERs.

ALTER TABLE matrix_api_keys ADD COLUMN limit_tpd INTEGER DEFAULT 10000;

-- Ensure matrix_chat_history exists
CREATE TABLE IF NOT EXISTS matrix_chat_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT,
    role TEXT, 
    content TEXT,
    model TEXT,
    source TEXT,
    target_agent_id TEXT,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Ensure matrix_processes exists
CREATE TABLE IF NOT EXISTS matrix_processes (
    id TEXT PRIMARY KEY,
    description TEXT,
    agent_id TEXT,
    node_id TEXT,
    status TEXT,
    progress_pct INTEGER DEFAULT 0,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Ensure wallets exists and updated balance
CREATE TABLE IF NOT EXISTS wallets (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    address TEXT,
    purpose TEXT,
    status TEXT DEFAULT 'active',
    balance_mock TEXT DEFAULT '0.00'
);

-- Seed/Update essential data for UI
INSERT OR REPLACE INTO matrix_api_keys (id, provider, key_label, status, usage_tpd, limit_tpd) VALUES 
('key-cf', 'cloudflare', 'Default API', 'active', 980000, 5000000);

INSERT OR REPLACE INTO wallets (id, name, address, purpose, balance_mock) VALUES 
('w-ops', '📌 Vận hành', '0x...ops', 'Duy trì hệ thống, server, điện nước.', '1,250.00'),
('w-inv', '📈 Đầu tư', '0x...invest', 'Tái đầu tư vào các dự án Trade.', '5,400.00'),
('w-boss', '👑 Ví Sếp Hưng', '0x...boss', 'Mục tiêu: 2,000$/tháng.', '500.00'),
('w-rnd', '🧬 Quỹ R&D', '0x...rnd', 'Sản xuất linh kiện, CPU, RAM.', '2,000.00');

INSERT OR IGNORE INTO nodes (id, name, cpu_info, ram_total, storage_total, status, role) VALUES 
('mac-m4', 'MAC Mini M4', 'Apple M4', 32, 1024, 'online', 'master'),
('playbox-2020', 'FPT Play Box', 'Cortex A53', 2, 16, 'online', 'edge');

INSERT OR IGNORE INTO agents (id, name, dept_id, node_id, status, role_desc, model_id, is_leader) VALUES 
('BA', 'BOSS Assistant', 'dept-exec', 'mac-m4', 'online', 'Strategic Brain', 'gemini-pro', 1);

PRAGMA foreign_keys = ON;
