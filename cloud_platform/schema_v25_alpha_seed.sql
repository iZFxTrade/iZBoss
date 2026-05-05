-- BOSS V25 Final Alpha - Library & Model Convergence
-- Run via: npx wrangler d1 execute bossizlife --remote --file=./schema_v25_alpha_seed.sql

PRAGMA foreign_keys = OFF;

-- 1. Seed Missing Library Modules
INSERT OR IGNORE INTO skills_modules (id, name, type, status, description) VALUES 
('mod-zeroclaw', 'ZeroClaw Alpha', 'module', 'active', 'Hệ thống cào dữ liệu tủy chỉnh đa kênh, bypass Cloudflare.'),
('mod-cli-any', 'CLI Anything', 'module', 'active', 'Giao diện dòng lệnh vạn năng, thực thi script hệ thống đa nền tảng.'),
('mod-p2p-net', 'P2P Global Net', 'module', 'active', 'Quản lý phân phối tài nguyên P2P và kết nối phi tập trung.'),
('mod-torrent-hub', 'Torrent Matrix', 'module', 'active', 'Hệ thống quản lý download/upload Torrent hiệu năng cao.');

-- 2. Seed OpenRouter Free Models & Pro Models
INSERT OR REPLACE INTO llm_models (id, name, provider, model_path, status, daily_token_limit) VALUES 
('or-seedance-1.5', 'Seedance 1.5 Pro', 'openrouter', 'bytedance/seedance-1-5-pro', 'active', 50000),
('or-seedream-4.5', 'Seedream 4.5', 'openrouter', 'bytedance-seed/seedream-4.5', 'active', 50000),
('or-veo-3.1', 'Google Veo 3.1', 'openrouter', 'google/veo-3.1', 'active', 50000),
('or-gemma-27b', 'Gemma 3 27b IT', 'openrouter', 'google/gemma-3-27b-it:free', 'active', 1000000),
('or-minimax-m2.5', 'Minimax M2.5 Free', 'openrouter', 'minimax/minimax-m2.5:free', 'active', 1000000),
('or-qwen3-coder', 'Qwen3 Coder Free', 'openrouter', 'qwen/qwen3-coder:free', 'active', 1000000),
('or-gpt-oss-120b', 'GPT OSS 120b Free', 'openrouter', 'openai/gpt-oss-120b:free', 'active', 1000000);

-- 3. Ensure some mock processes exist for Node-Agent mapping visualization
INSERT OR IGNORE INTO matrix_processes (id, node_id, agent_id, status, progress_pct, description) VALUES 
('proc-ba-master', 'node-1', 'BA', 'running', 45, 'Core Executive Oversight'),
('proc-ao-master', 'node-1', 'AO', 'running', 20, 'Internal Matrix Optimization'),
('proc-cfo-finance', 'node-2', 'CFO', 'running', 15, 'Tokenomics & Budgeting');

PRAGMA foreign_keys = ON;
