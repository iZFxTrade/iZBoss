-- V25 Hyper-Elite Bot Session Migration
-- Run via: npx wrangler d1 execute bossizlife --remote --file=./schema_v25_bot.sql

PRAGMA foreign_keys = OFF;

-- Table to track Telegram user sessions and their active agent
CREATE TABLE IF NOT EXISTS matrix_bot_sessions (
    telegram_user_id TEXT PRIMARY KEY,
    active_agent_id TEXT DEFAULT 'BA',
    auth_status TEXT DEFAULT 'pending', -- pending, verified
    last_interaction DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Seed whitelist directly into sessions if needed or handled in code
-- INSERT OR IGNORE INTO matrix_bot_sessions (telegram_user_id, active_agent_id, auth_status) VALUES ('iZFxTrade_ID', 'BA', 'verified');

PRAGMA foreign_keys = ON;
