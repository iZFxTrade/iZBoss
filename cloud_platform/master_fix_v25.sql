-- Master Sovereign Schema V25.5 - Fixes All 500 Errors
PRAGMA foreign_keys = OFF;

-- 1. Departments
CREATE TABLE IF NOT EXISTS departments (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    parent_id TEXT, 
    role_type TEXT,
    function_desc TEXT,
    lead_agent_id TEXT
);

-- 2. Nodes
CREATE TABLE IF NOT EXISTS nodes (
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

-- 3. LLM Models
CREATE TABLE IF NOT EXISTS llm_models (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    provider TEXT NOT NULL, 
    type TEXT, 
    model_path TEXT,
    api_key TEXT,
    base_url TEXT,
    status TEXT DEFAULT 'active',
    model_group TEXT DEFAULT 'reasoning',
    tpm_limit INTEGER DEFAULT 0,
    tpd_limit INTEGER DEFAULT 0
);

-- 4. Agents
CREATE TABLE IF NOT EXISTS agents (
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
    total_tokens_used INTEGER DEFAULT 0,
    daily_token_limit INTEGER DEFAULT 100000,
    FOREIGN KEY(dept_id) REFERENCES departments(id),
    FOREIGN KEY(node_id) REFERENCES nodes(id),
    FOREIGN KEY(model_id) REFERENCES llm_models(id)
);

-- 5. Bots
CREATE TABLE IF NOT EXISTS bots (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    platform TEXT NOT NULL, 
    token TEXT,
    webhook_url TEXT,
    status TEXT DEFAULT 'active'
);

-- 6. Skills & Modules
CREATE TABLE IF NOT EXISTS skills_modules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    type TEXT,
    version TEXT,
    description TEXT,
    install_cmd TEXT,
    status TEXT DEFAULT 'active'
);

-- 7. Webhooks & Feeds
CREATE TABLE IF NOT EXISTS webhooks_feeds (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    url TEXT,
    provider TEXT, 
    api_key TEXT,
    status TEXT DEFAULT 'active'
);

-- 8. Matrix Processes
CREATE TABLE IF NOT EXISTS matrix_processes (
    id TEXT PRIMARY KEY,
    description TEXT,
    agent_id TEXT,
    node_id TEXT,
    status TEXT,
    progress_pct INTEGER DEFAULT 0,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 9. Matrix API Keys
CREATE TABLE IF NOT EXISTS matrix_api_keys (
    id TEXT PRIMARY KEY,
    provider TEXT,
    key_label TEXT,
    key_value TEXT,
    status TEXT DEFAULT 'active',
    usage_tpm INTEGER DEFAULT 0,
    usage_tpd INTEGER DEFAULT 0,
    limit_tpd INTEGER DEFAULT 10000,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 10. Matrix Chat History
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

-- 11. Wallets
CREATE TABLE IF NOT EXISTS wallets (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    address TEXT,
    purpose TEXT,
    status TEXT DEFAULT 'active',
    balance_mock TEXT DEFAULT '0.00'
);

-- SEED DATA
INSERT OR IGNORE INTO nodes (id, name, cpu_info, ram_total, storage_total, status, role) VALUES 
('mac-m4', 'MAC Mini M4', 'Apple M4', 32, 1024, 'online', 'master'),
('playbox-2020', 'FPT Play Box', 'Cortex A53', 2, 16, 'online', 'edge');

INSERT OR IGNORE INTO departments (id, name, function_desc, lead_agent_id) VALUES 
('dept-exec', 'Executive Board', 'Sovereign Command Center', 'BA'),
('dept-finance', 'Financial Ops', 'Wallet & Asset Management', 'CFO');

INSERT OR IGNORE INTO llm_models (id, name, provider, type, model_path, status, model_group, tpd_limit) VALUES 
('cf-llama3', 'Llama 3 (CF)', 'cloudflare', 'general', '@cf/meta/llama-3-8b-instruct', 'active', 'planning', 5000000);

INSERT OR IGNORE INTO agents (id, name, dept_id, node_id, status, role_desc, model_id, is_leader) VALUES 
('BA', 'BOSS Assistant', 'dept-exec', 'mac-m4', 'online', 'Strategic Brain', 'cf-llama3', 1),
('AO', 'Admin Officer', 'dept-exec', 'mac-m4', 'online', 'System Ops', 'cf-llama3', 1);

INSERT OR IGNORE INTO wallets (id, name, address, purpose, balance_mock) VALUES 
('w-ops', '📌 Vận hành', '0x...ops', 'Duy trì hệ thống.', '1,250.00'),
('w-boss', '👑 Ví Sếp Hưng', '0x...boss', 'Mục tiêu: 2,000$/tháng.', '500.00');

INSERT OR IGNORE INTO matrix_api_keys (id, provider, key_label, status, usage_tpd, limit_tpd) VALUES 
('key-cf', 'cloudflare', 'Default API', 'active', 980000, 5000000);

PRAGMA foreign_keys = ON;
