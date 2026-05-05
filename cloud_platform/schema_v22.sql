-- V22 Health Telemetry & Disk Tracking Migration
-- Run via: npx wrangler d1 execute bossizlife --remote --file=./schema_v22.sql

PRAGMA foreign_keys = OFF;

-- Add storage telemetry to nodes
ALTER TABLE nodes ADD COLUMN storage_info TEXT;
ALTER TABLE nodes ADD COLUMN disk_usage INTEGER DEFAULT 0;

-- Seed enhanced data for existing 5-node fleet
UPDATE nodes SET storage_info = '1TB NVMe Gen4', disk_usage = 45 WHERE id = 'node-1'; -- Mac Mini
UPDATE nodes SET storage_info = '500GB SSD', disk_usage = 82 WHERE id = 'node-2'; -- VPS 1
UPDATE nodes SET storage_info = '500GB SSD', disk_usage = 64 WHERE id = 'node-3'; -- VPS 2
UPDATE nodes SET storage_info = '128GB UFS 2.1', disk_usage = 92 WHERE id = 'node-4'; -- Samsung 9
UPDATE nodes SET storage_info = '256GB UFS 3.0', disk_usage = 38 WHERE id = 'node-5'; -- Samsung 10+
UPDATE nodes SET storage_info = '16GB eMMC', disk_usage = 75 WHERE id = 'node-6'; -- FPT Playbox

PRAGMA foreign_keys = ON;
