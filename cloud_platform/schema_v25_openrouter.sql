-- V25 Hyper-Elite API Matrix Migration
-- Run via: npx wrangler d1 execute bossizlife --remote --file=./schema_v25_openrouter.sql

PRAGMA foreign_keys = OFF;

-- Insert OpenRouter API Key
INSERT OR IGNORE INTO matrix_api_keys (id, provider, key_label, key_value, status, usage_tpd, limit_tpd) 
VALUES (
    'ak-or-1', 
    'openrouter', 
    'Main OpenRouter', 
    'sk-or-v1-REPLACED_BY_AI_FOR_SECURITY', 
    'active', 
    0, 
    200000 -- Approximate free limit for small models
);

-- Ensure models have quota fields if not exist
-- (Assuming they were added or will be handled in code logic)

PRAGMA foreign_keys = ON;
