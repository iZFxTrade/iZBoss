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

CREATE TABLE departments (id TEXT PRIMARY KEY, name TEXT NOT NULL, parent_id TEXT, role_type TEXT, function_desc TEXT, leader_id TEXT, FOREIGN KEY(parent_id) REFERENCES departments(id));
CREATE TABLE nodes (id TEXT PRIMARY KEY, name TEXT NOT NULL, status TEXT DEFAULT 'offline', role TEXT DEFAULT 'node', cpu_info TEXT, ram_total INTEGER, ram_usage INTEGER DEFAULT 0, storage_total INTEGER, storage_usage INTEGER DEFAULT 0, last_heartbeat TIMESTAMP DEFAULT CURRENT_TIMESTAMP);
CREATE TABLE llm_models (id TEXT PRIMARY KEY, name TEXT NOT NULL, provider TEXT NOT NULL, type TEXT, model_path TEXT, api_key TEXT, base_url TEXT, status TEXT DEFAULT 'active');
CREATE TABLE agents (id TEXT PRIMARY KEY, name TEXT NOT NULL, dept_id TEXT, node_id TEXT, status TEXT DEFAULT 'idle', skill TEXT, role_desc TEXT, system_prompt TEXT, model_id TEXT, progress INTEGER DEFAULT 0, current_task TEXT, is_leader BOOLEAN DEFAULT 0, FOREIGN KEY(dept_id) REFERENCES departments(id), FOREIGN KEY(node_id) REFERENCES nodes(id), FOREIGN KEY(model_id) REFERENCES llm_models(id));
