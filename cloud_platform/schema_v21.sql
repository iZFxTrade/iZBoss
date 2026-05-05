-- V21 Specialized Neural Registry
-- Run via: npx wrangler d1 execute bossizlife --remote --file=./schema_v21.sql

PRAGMA foreign_keys = OFF;

-- Ensure specialized models are seeded
INSERT INTO llm_models (id, name, provider, type, api_key, model_path) VALUES 
('gemini-1.5-pro', 'Gemini 1.5 Pro', 'gemini', 'high-intelligence', 'AIzaSyAdP9OQ7SpKP9w63vQJLdXJWvDQyIxZLEU', 'gemini-1.5-pro'),
('gemini-1.5-flash', 'Gemini 1.5 Flash', 'gemini', 'general', 'AIzaSyAdP9OQ7SpKP9w63vQJLdXJWvDQyIxZLEU', 'gemini-1.5-flash'),
('imagen-3', 'Google Imagen 3', 'google', 'image', 'AIzaSyAdP9OQ7SpKP9w63vQJLdXJWvDQyIxZLEU', 'imagen-3'),
('veo-3-1', 'Google Veo 3.1', 'google', 'video', 'AIzaSyAdP9OQ7SpKP9w63vQJLdXJWvDQyIxZLEU', 'veo-3.1'),
('nano-banana', 'Nano Banana Video', 'banana', 'video', 'AIzaSyAdP9OQ7SpKP9w63vQJLdXJWvDQyIxZLEU', 'nano-banana-v1'),
('nvidia-nemotron-4', 'NVIDIA Nemotron 4', 'nvidia', 'general', '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', 'nvidia/nemotron-4-340b-instruct'),
('nvidia-codellama-70b', 'NVIDIA CodeLlama', 'nvidia', 'code', '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', 'meta/codellama-70b-instruct')
ON CONFLICT(id) DO UPDATE SET api_key=excluded.api_key, model_path=excluded.model_path;

-- Cleanup legacy placeholders
DELETE FROM llm_models WHERE id IN ('sora', 'dalle-3');

PRAGMA foreign_keys = ON;
