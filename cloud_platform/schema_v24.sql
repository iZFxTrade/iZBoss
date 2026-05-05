-- V24 Finance & Token Budgeting Migration
-- Run via: npx wrangler d1 execute bossizlife --remote --file=./schema_v24.sql

PRAGMA foreign_keys = OFF;

-- Expand llm_models for Cost Tracking
ALTER TABLE llm_models ADD COLUMN cost_per_1m_tokens DECIMAL(10, 4) DEFAULT 0;
ALTER TABLE llm_models ADD COLUMN is_paid BOOLEAN DEFAULT 0;

-- Expand departments for Budgeting
ALTER TABLE departments ADD COLUMN budget_limit DECIMAL(10, 2) DEFAULT 1000.00;
ALTER TABLE departments ADD COLUMN tokens_spent INTEGER DEFAULT 0;

-- Seed Finance Data
UPDATE llm_models SET cost_per_1m_tokens = 15.00, is_paid = 1 WHERE id IN ('gpt-4o', 'gemini-1.5-pro');
UPDATE llm_models SET cost_per_1m_tokens = 0, is_paid = 0 WHERE provider = 'cloudflare'; -- Free on CF
UPDATE llm_models SET cost_per_1m_tokens = 2.00, is_paid = 1 WHERE provider = 'nvidia';

-- Update Departments
UPDATE departments SET budget_limit = 5000.00, tokens_spent = 125000 WHERE id = 'FINANCE';
UPDATE departments SET budget_limit = 2000.00, tokens_spent = 85000 WHERE id = 'RD';

-- Ensure Feed/Hook sources are seeded
INSERT OR IGNORE INTO webhooks_feeds (id, name, type, endpoint, status) VALUES 
('hook-oanda', 'Oanda Price Feed', 'hook', 'https://api-fxtrade.oanda.com/v3/prices', 'online'),
('feed-hub', 'iZ Hub Intelligence', 'feed', 'https://hub.iz.life/news', 'online');

PRAGMA foreign_keys = ON;
