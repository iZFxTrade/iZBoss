-- BOSS V25 Final Alpha - THE TRUTH MATRIX SEED (FIXED)
-- Run via: npx wrangler d1 execute bossizlife --remote --file=./schema_v25_truth_seed_fixed.sql

PRAGMA foreign_keys = OFF;

-- 1. Hardware Alignment - The Real Fleet (Correcting column names)
INSERT OR REPLACE INTO nodes (id, name, status, cpu_info, ram_total, storage_total, role) VALUES 
('node-1', 'Mac Mini M4 Pro', 'online', 'M4 Pro 16-Core', 32, 512, 'Main Executive Matrix'),
('node-2', 'FPT PlayBox X6', 'online', 'ARM Cortex-A53', 2, 16, 'Edge Module Node'),
('node-3', 'Samsung Note 10+', 'online', 'Exynos 9825', 12, 256, 'Mobile Agent Hub'),
('node-4', 'Samsung Note 9', 'online', 'Exynos 9810', 8, 128, 'Secondary Mobile Node'),
('node-5', 'Hanoi Cluster 01', 'online', 'Xeon E5-2680', 64, 2048, 'High-Density Computational Node');

-- 2. Professional Departments (Ensuring columns exist)
-- Note: ALTER TABLE might fail if columns exist, wrapping in a separate check if needed, 
-- but since this is a seed script for a fresh update:
UPDATE departments SET lead_agent_id = 'BA', staff_count = 1 WHERE id = 'dept-exec';
INSERT OR IGNORE INTO departments (id, name, description, lead_agent_id, staff_count) VALUES 
('dept-finance', 'Financial Intelligence', 'Quản lý dòng tiền, Tokenomics và Ngân sách AI.', 'CFO', 2),
('dept-tech', 'Technical Operations', 'Quản lý hạ tầng Nodes, OTA và Module Deployment.', 'CTO', 3),
('dept-trading', 'Trade Algorithm Hub', 'Trung tâm nghiên cứu và thực thi thuật toán TradeKiemCom.', 'AO', 5);

-- 3. Skills & Modules - Ensuring Persistence
INSERT OR REPLACE INTO skills_modules (id, name, type, status, description) VALUES 
('sk-research', 'Deep Research AI', 'skill', 'active', 'Kỹ năng nghiên cứu chuyên sâu, tổng hợp dữ liệu vạn năng.'),
('sk-coding', 'Matrix Coder', 'skill', 'active', 'Kỹ năng lập trình đa ngôn ngữ, tối ưu hóa logic Matrix.'),
('mod-zclaw', 'ZeroClaw Master', 'module', 'active', 'Module cào dữ liệu tối thượng bypass Cloudflare.'),
('mod-p2p', 'Global P2P Network', 'module', 'active', 'Module kết nối mạng phi tập trung toàn cầu.');

PRAGMA foreign_keys = ON;
