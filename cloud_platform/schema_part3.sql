INSERT INTO llm_models (id, name, provider, type, model_path, api_key) VALUES 
('cf-llama3-8b', 'Llama 3 8B (CF)', 'cloudflare', 'general', '@cf/meta/llama-3-8b-instruct', NULL),
('gemini-1.5-pro', 'Gemini 1.5 Pro', 'gemini', 'high-intelligence', 'gemini-1.5-pro', 'AIzaSyAdP9OQ7SpKP9w63vQJLdXJWvDQyIxZLEU'),
('gemini-1.5-flash', 'Gemini 1.5 Flash', 'gemini', 'general', 'gemini-1.5-flash', 'AIzaSyAdP9OQ7SpKP9w63vQJLdXJWvDQyIxZLEU'),
('nvidia-codellama-70b', 'CodeLlama 70B (NVIDIA)', 'nvidia', 'code', 'meta/codellama-70b-instruct', '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');

INSERT INTO departments (id, name, role_type, function_desc) VALUES 
('executive', 'Hội đồng Quản trị', 'executive', 'Đơn vị điều hành tối cao BOSS.'),
('finance', 'Phòng Tài chính', 'department', 'Quản lý ví, hạch toán lợi nhuận.'),
('evolution', 'Phòng Tiến hóa (R&D)', 'department', 'Nghiên cứu DNA core, tự nâng cấp.'),
('izfx', 'Phòng iZFx (Trading)', 'department', 'Vận hành EA TradeKiem.'),
('marketing', 'Phòng Marketing', 'department', 'Chiến dịch nội dung tự động.'),
('sales_cs', 'Phòng Sales & CSKH', 'department', 'Phễu izthuchi & Hub CS.'),
('hr_manager', 'Phòng Nhân sự (HR)', 'department', 'Giám sát Workforce Agent.');

INSERT INTO nodes (id, name, cpu_info, ram_total, storage_total, status, role) VALUES 
('mac-m4', 'MAC Mini M4', 'Apple M4', 32768, 1024, 'online', 'master'),
('vps-win', 'VPS Windows', 'Xeon Gold', 16384, 512, 'online', 'vps'),
('note-9', 'Samsung Note 9', 'Exynos 9810', 6144, 128, 'online', 'mobile'),
('note-10-p', 'Samsung Note 10+', 'Exynos 9825', 12288, 256, 'online', 'mobile'),
('playbox-2020', 'FPT Play Box 2020', 'Cortex A53', 2048, 16, 'online', 'edge');

INSERT INTO agents (id, name, dept_id, node_id, status, skill, role_desc, system_prompt, model_id, is_leader, progress, current_task) VALUES 
('BA', 'Trợ lý BOSS (BA)', 'executive', 'mac-m4', 'idle', 'Strategic Planning', 'Bộ não chiến lược của BOSS.', 'Bạn là BA. Trả lời tiếng Việt.', 'cf-llama3-8b', 1, 0, 'Analyzing Collective Intelligence'),
('AO', 'Trợ lý Điều hành (AO)', 'executive', 'mac-m4', 'idle', 'Global Ops', 'Giám đốc vận hành Matrix.', 'Bạn là AO. Trả lời tiếng Việt.', 'cf-llama3-8b', 1, 0, 'Allocating Node Resources'),
('L-Finance', 'CFO Alpha', 'finance', 'vps-win', 'idle', 'Treasury', 'Trưởng phòng Tài chính', 'Bạn là CFO Alpha. Trả lời tiếng Việt.', 'gemini-1.5-flash', 1, 0, 'Audit Pending'),
('L-Evolution', 'CTO Omega', 'evolution', 'mac-m4', 'running', 'DNA R&D', 'Trưởng phòng R&D', 'Bạn là CTO Omega. Trả lời tiếng Việt.', 'nvidia-codellama-70b', 1, 75, 'Refining Wasm P2P Kernel');
