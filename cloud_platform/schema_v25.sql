-- V25 Multi-Key API Registry & Model Groups Migration
-- Run via: npx wrangler d1 execute bossizlife --remote --file=./schema_v25.sql

PRAGMA foreign_keys = OFF;

-- New table for API Keys Registry
CREATE TABLE IF NOT EXISTS matrix_api_keys (
    id TEXT PRIMARY KEY,
    provider TEXT, -- openai, gemini, nvidia, openrouter, etc.
    key_label TEXT, -- e.g. "Key Alpha", "Key Beta"
    key_value TEXT,
    status TEXT DEFAULT 'active', -- active, limit_reached, revoked
    usage_tpm INTEGER DEFAULT 0,
    usage_tpd INTEGER DEFAULT 0,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Expand llm_models for Groups and Quotas
ALTER TABLE llm_models ADD COLUMN model_group TEXT DEFAULT 'reasoning'; -- code, research, creative, reasoning, planning
ALTER TABLE llm_models ADD COLUMN tpm_limit INTEGER DEFAULT 0; -- Tokens Per Minute
ALTER TABLE llm_models ADD COLUMN tpd_limit INTEGER DEFAULT 0; -- Tokens Per Day

-- Seed Model Groups
UPDATE llm_models SET model_group = 'research' WHERE id = 'gemini-1.5-pro';
UPDATE llm_models SET model_group = 'code' WHERE provider = 'nvidia';
UPDATE llm_models SET model_group = 'reasoning' WHERE id = 'gpt-4o';
UPDATE llm_models SET model_group = 'planning' WHERE provider = 'cloudflare';

-- Seed initial API Key record
INSERT OR IGNORE INTO matrix_api_keys (id, provider, key_label, key_value) VALUES 
('key-nv-1', 'nvidia', 'Main NVIDIA NIM', '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'),
('key-gm-1', 'gemini', 'Main Gemini Pro', 'AIzaSyAdP9OQ7SpKP9w63vQJLdXJWvDQyIxZLEU');

PRAGMA foreign_keys = ON;
