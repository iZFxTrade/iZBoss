-- V23 Multi-State Tracking & Token Analytics Migration
-- Run via: npx wrangler d1 execute bossizlife --remote --file=./schema_v23.sql

PRAGMA foreign_keys = OFF;

-- Expand llm_models for Token tracking
ALTER TABLE llm_models ADD COLUMN total_tokens_used INTEGER DEFAULT 0;

-- New table for Active Processes (Agent/Node context)
CREATE TABLE IF NOT EXISTS matrix_processes (
    id TEXT PRIMARY KEY,
    agent_id TEXT,
    node_id TEXT,
    description TEXT,
    progress_pct INTEGER DEFAULT 0,
    status TEXT DEFAULT 'running',
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(agent_id) REFERENCES agents(id),
    FOREIGN KEY(node_id) REFERENCES nodes(id)
);

-- Seed initial analytics data
UPDATE llm_models SET total_tokens_used = 125000 WHERE id = 'gemini-1.5-pro';
UPDATE llm_models SET total_tokens_used = 450000 WHERE id = 'nvidia-nemotron-4';

INSERT INTO matrix_processes (id, agent_id, node_id, description, progress_pct, status) VALUES 
('proc-1', 'BA', 'node-1', 'Analyzing Market Trends (iZfx)', 65, 'running'),
('proc-2', 'AO', 'node-2', 'Optimizing Node 2 Latency', 42, 'running'),
('proc-3', 'RD_LEAD', 'node-1', 'Benchmarking Nemotron Performance', 10, 'running');

PRAGMA foreign_keys = ON;
