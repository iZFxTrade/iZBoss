-- V25 Hyper-Elite Omni-Channel Chat Migration
-- Run via: npx wrangler d1 execute bossizlife --remote --file=./schema_v25_hyper.sql

PRAGMA foreign_keys = OFF;

-- Table for unified chat history across all sources
CREATE TABLE IF NOT EXISTS matrix_chat_history (
    id TEXT PRIMARY KEY,
    agent_id TEXT,
    message TEXT,
    response TEXT,
    source TEXT DEFAULT 'internal', -- internal, telegram, zalo, etc.
    source_bot_id TEXT, -- which bot instance
    external_user_id TEXT, -- e.g. telegram user id
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Whitelist for bot access
CREATE TABLE IF NOT EXISTS bot_whitelist (
    id TEXT PRIMARY KEY,
    bot_id TEXT,
    username TEXT,
    status TEXT DEFAULT 'active'
);

-- Seed whitelist
INSERT OR IGNORE INTO bot_whitelist (id, bot_id, username) VALUES 
('wl-1', 'telegram_main', 'iZFxTrade'),
('wl-2', 'telegram_main', 'FxBlueNet');

PRAGMA foreign_keys = ON;
