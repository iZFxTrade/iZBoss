-- ======================================================
-- iZ.Life BOSS - MASTER DNA SCHEMA (V25.5 Platinum)
-- ======================================================
-- Unified database structure for the Sovereign Matrix.

PRAGMA foreign_keys = OFF;

-- 1. Departments Hierarchy
DROP TABLE IF EXISTS departments;
CREATE TABLE departments (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    parent_id TEXT, 
    role_type TEXT, -- 'executive', 'department'
    function_desc TEXT,
    leader_id TEXT, 
    lead_agent_id TEXT, -- V25 Extension
    FOREIGN KEY(parent_id) REFERENCES departments(id)
);

-- 2. Infrastructure Fleet (Nodes)
DROP TABLE IF EXISTS nodes;
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
    owner_id TEXT, -- User ID who claims this node
    last_heartbeat TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(owner_id) REFERENCES users(id)
);

-- 2.1 Multi-User Authentication (RBAC)
DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id TEXT PRIMARY KEY, -- telegram_id or chosen username
    name TEXT NOT NULL,
    role TEXT DEFAULT 'user', -- root, admin, mod, user
    secret_2fa TEXT,
    approved BOOLEAN DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 3. Unified Workforce Matrix (Agents)
DROP TABLE IF EXISTS agents;
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

-- 4. LLM Models Registry (V25 Hyper-Sync)
DROP TABLE IF EXISTS llm_models;
CREATE TABLE llm_models (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    provider TEXT NOT NULL, 
    type TEXT, 
    model_path TEXT,
    api_key TEXT,
    base_url TEXT,
    status TEXT DEFAULT 'active',
    model_group TEXT DEFAULT 'reasoning', -- V25 Extension
    tpm_limit INTEGER DEFAULT 0,
    tpd_limit INTEGER DEFAULT 0
);

-- 5. Multi-Key API Registry (V25 Security)
DROP TABLE IF EXISTS matrix_api_keys;
CREATE TABLE matrix_api_keys (
    id TEXT PRIMARY KEY,
    provider TEXT, -- openai, gemini, nvidia, openrouter, etc.
    key_label TEXT,
    key_value TEXT,
    status TEXT DEFAULT 'active',
    usage_tpm INTEGER DEFAULT 0,
    usage_tpd INTEGER DEFAULT 0,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 6. Financial Sovereignty (Wallets)
DROP TABLE IF EXISTS wallets;
CREATE TABLE wallets (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    address TEXT,
    purpose TEXT,
    status TEXT DEFAULT 'active',
    balance_mock TEXT DEFAULT '0.00'
);

-- 7. Unified Communication Layer (Omni-Channel)
DROP TABLE IF EXISTS matrix_chat_history;
CREATE TABLE matrix_chat_history (
    id TEXT PRIMARY KEY,
    agent_id TEXT,
    message TEXT,
    response TEXT,
    source TEXT DEFAULT 'internal', -- internal, telegram, zalo, etc.
    source_bot_id TEXT,
    external_user_id TEXT,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 8. Bot Integration & Whitelist
DROP TABLE IF EXISTS bots;
CREATE TABLE bots (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    platform TEXT NOT NULL, 
    token TEXT,
    webhook_url TEXT,
    status TEXT DEFAULT 'active'
);

DROP TABLE IF EXISTS bot_whitelist;
CREATE TABLE bot_whitelist (
    id TEXT PRIMARY KEY,
    bot_id TEXT,
    username TEXT,
    status TEXT DEFAULT 'active'
);

-- 9. Skills & DNA Modules
DROP TABLE IF EXISTS skills_modules;
CREATE TABLE skills_modules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    type TEXT,
    version TEXT,
    description TEXT,
    install_cmd TEXT,
    status TEXT DEFAULT 'active'
);

-- 10. Webhooks & External Feeds
DROP TABLE IF EXISTS webhooks_feeds;
CREATE TABLE webhooks_feeds (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    url TEXT,
    provider TEXT, 
    api_key TEXT,
    status TEXT DEFAULT 'active'
);

PRAGMA foreign_keys = ON;
