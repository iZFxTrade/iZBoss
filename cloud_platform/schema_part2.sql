CREATE TABLE chat_history (id INTEGER PRIMARY KEY AUTOINCREMENT, session_id TEXT, role TEXT, content TEXT, timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP);
CREATE TABLE bots (id TEXT PRIMARY KEY, name TEXT NOT NULL, platform TEXT NOT NULL, token TEXT, webhook_url TEXT, status TEXT DEFAULT 'active');
CREATE TABLE skills_modules (id TEXT PRIMARY KEY, name TEXT NOT NULL, type TEXT, version TEXT, description TEXT, install_cmd TEXT);
CREATE TABLE webhooks_feeds (id TEXT PRIMARY KEY, name TEXT NOT NULL, type TEXT NOT NULL, url TEXT, provider TEXT, api_key TEXT, status TEXT DEFAULT 'active');
CREATE TABLE boss_auth_methods (id TEXT PRIMARY KEY, type TEXT NOT NULL, value TEXT, status TEXT DEFAULT 'active');
