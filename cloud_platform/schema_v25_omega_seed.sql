-- BOSS V25.1 Omega - THE OMEGA SEED
-- Run via: npx wrangler d1 execute bossizlife --remote --file=./schema_v25_omega_seed.sql

PRAGMA foreign_keys = OFF;

-- 1. Ensure Agent-Department Leadership is Mapped
UPDATE departments SET lead_agent_id = 'BA' WHERE id = 'dept-exec';
UPDATE departments SET lead_agent_id = 'CFO' WHERE id = 'dept-finance';
UPDATE departments SET lead_agent_id = 'CTO' WHERE id = 'dept-tech';
UPDATE departments SET lead_agent_id = 'AO' WHERE id = 'dept-trading';

-- 2. Assign Agents to Departments and Nodes
-- (CTO Omega, CFO Alpha, BA, AO, etc.)
UPDATE agents SET dept_id = 'dept-exec' WHERE id = 'BA';
UPDATE agents SET dept_id = 'dept-finance' WHERE id = 'CFO';
UPDATE agents SET dept_id = 'dept-tech' WHERE id = 'CTO';
UPDATE agents SET dept_id = 'dept-trading' WHERE id = 'AO';

-- 3. Seed Library Modules - Force Persistence
-- Using IDs that won't conflict with current data
INSERT OR REPLACE INTO skills_modules (id, name, type, status, description) VALUES 
('mod-zclaw-01', 'ZeroClaw Master V2', 'module', 'active', 'Module cào dữ liệu tối thượng bypass Cloudflare.'),
('mod-p2p-01', 'Global P2P Network', 'module', 'active', 'Module kết nối mạng phi tập trung toàn cầu.'),
('mod-cli-01', 'CLI Anything Matrix', 'module', 'active', 'Module thực thi lệnh hệ thống vạn năng.'),
('mod-torrent-01', 'Torrent Matrix V3', 'module', 'active', 'Hệ thống quản lý tài nguyên P2P hiệu năng cao.');

INSERT OR REPLACE INTO skills_modules (id, name, type, status, description) VALUES 
('sk-research-01', 'Deep Research Omni', 'skill', 'active', 'Kỹ năng nghiên cứu đa tầng, tổng hợp tri thức vạn năng.'),
('sk-coding-01', 'Matrix Coder X', 'skill', 'active', 'Kỹ năng lập trình tối ưu hóa logic đa nền tảng.');

-- 4. Ensure Webhook data exists in Bots for the UI to see
INSERT OR REPLACE INTO bots (id, name, platform, token, status) VALUES 
('tg-boss', 'Boss.iz.life', 'telegram', '8438121452:AAFOJ-6ALOFOHAI-0bFB_HNRD1iwMaCl8x4', 'active');

PRAGMA foreign_keys = ON;
