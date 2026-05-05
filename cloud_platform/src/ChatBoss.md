# ChatBoss - Unified Communication Hub for BOSS HƯNG
## Mô Tả Hệ Thống Trò Chuyện Tích Hợp iZ.Life BOSS

### 1. Overview
**ChatBoss** là trung tâm giao tiếp thống nhất cho hệ thống iZ.Life BOSS, cho phép BOSS HƯNG (Founder) tương tác trực tiếp với:
- ✅ Agent BA (BOSS Assistant) - Trợ lý chiến lược
- ✅ Agent AO (Assistant Operating) - Điều hành vận hành  
- ✅ 6 Agent Leader (phòng ban)
- ✅ ~30 Agent con (chuyên biệt từng lĩnh vực)
- ✅ Telegram Bot (mobile integration)
- ✅ Dashboard Hyper-Elite (web UI)

### 2. Architecture Tích Hợp

```
┌─────────────────────────────────────────────────────────────────┐
│                    CHATBOSS - Command Center                     │
│                     (BOSS HƯNG Interface)                        │
└─────────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        ▼                     ▼                     ▼
   ┌─────────┐          ┌──────────┐        ┌─────────────┐
   │ Web UI  │          │ Telegram │        │ Mobile App  │
   │Dashboard│  ◄────►  │   Bot    │  ◄────► │ (iOS/And)   │
   │Hyper-Elt│          │ (izfxtrd)│        │   (future)  │
   └─────────┘          └──────────┘        └─────────────┘
        │                     │                     │
        └─────────────────────┼─────────────────────┘
                              │
                ┌─────────────────────────────┐
                │ Cloudflare Workers API      │
                │ /api/chat  |  /api/data     │
                │ /webhook/telegram            │
                └─────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        ▼                     ▼                     ▼
   ┌──────────────┐    ┌──────────────┐    ┌──────────────┐
   │   Agent BA   │    │   Agent AO   │    │ Agent Leaders│
   │(Strategic)   │    │(Operations)  │    │(6 Depts)     │
   └──────┬───────┘    └──────┬───────┘    └──────┬───────┘
          │                   │                   │
          └───────────────────┼───────────────────┘
                              │
                  ┌───────────────────────┐
                  │  Rooms Management     │
                  │ (Project/Dept/Task)   │
                  └───────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        ▼                     ▼                     ▼
   ┌──────────┐        ┌──────────┐        ┌──────────┐
   │ D1 DB    │        │ AI Models│        │   KV     │
   │(SQLite)  │        │(Workers  │        │(Cache)   │
   │Histories │        │AI)       │        │Feed      │
   └──────────┘        └──────────┘        └──────────┘
```

### 3. Resource Integration

#### 3.1 Database Resources (D1)
```sql
-- Chat History Storage
matrix_chat_history(id, agent_id, message, response, source, timestamp)

-- Agent Registry  
agents(id, name, dept_id, model_id, status, total_tokens_used)

-- Agents Interactions
matrix_bot_sessions(telegram_user_id, active_agent_id, auth_status)

-- Token Quota Tracking
matrix_quotas(provider, daily_usage, daily_limit, status)

-- Task Tracking
matrix_tasks(id, project_id, assigned_agent_id, status, name)

-- Department Structure
departments(id, name, role_type, lead_agent_id, function_desc)

-- LLM Models
llm_models(id, name, provider, model_path, api_key, daily_limit)
```

#### 3.2 API Endpoints
```typescript
// Chat API - Send message to Agent
POST /api/chat
{
  message: string,           // User message
  target_agent_id: string,   // "BA" | "AO" | "L-Trading" etc
  source: string,            // "telegram" | "web" | "internal"
  external_id: string        // Telegram user ID
}

// Data API - Fetch all resources
GET /api/data
Returns: {
  nodes, agents, depts, models, bots, processes,
  quotas, chat_history, wallets, projects, tasks,
  core: { version, ota, sync }
}

// Webhook - Telegram incoming messages
POST /webhook/telegram
// Secret token validated: X-Telegram-Bot-Api-Secret-Token

// Setup Webhook
POST /api/setup-webhook

// Universal Update/Add
POST /api/update  { table, data }
POST /api/add     { table, data }
```

#### 3.3 Authentication: Cloudflare Zero Trust
```
Web UI Routes:
  ├─ Protected by CF-Access-Authenticated-User-Email header
  ├─ Cf-Access policies @ Cloudflare dashboard
  └─ Email extracted: request.headers.get("Cf-Access-Authenticated-User-Email")

Telegram Routes:
  ├─ /webhook/telegram - Public
  ├─ Secret validation: X-Telegram-Bot-Api-Secret-Token
  ├─ Whitelist: ["izfxtrade", "fxbluenet"]
  └─ Direct authorization bypass for founder accounts

Public Routes:
  ├─ /install - iZcore bootstrap
  └─ / - Dynamic dashboard render
```

### 4. Chat Flow Architecture

```
User Input (Web/Telegram)
    │
    ▼
handleChatApi(request, env)
    │
    ├─ Parse: message, target_agent_id, source
    │
    ├─ Fetch Agent Config from D1
    │   (model_ids, system_prompt, dept_id)
    │
    ├─ Build Context:
    │   ├─ GLOBAL_SYSTEM_PROMPT (core mission)
    │   ├─ Agent system_prompt (role-specific)
    │   └─ TRUTH_MATRIX (current nodes, agents, projects)
    │
    ├─ Try Models (failover):
    │   ├─ Primary: agent.model_ids[0]
    │   ├─ Secondary: agent.model_ids[1]
    │   └─ Fallback: @cf/meta/llama-2-7b-chat-int8
    │
    ├─ Call env.AI.run(model_path, { messages })
    │
    ├─ Track Usage:
    │   ├─ Update matrix_quotas (daily_usage)
    │   └─ Update agents (total_tokens_used)
    │
    ├─ Store in D1:
    │   └─ INSERT matrix_chat_history(...)
    │
    └── Return: { response, model, timestamp, source }
```

### 5. Chat Capabilities by Agent

#### BA Agent (BOSS Assistant)
- **Role**: Strategic Planning & Overall System Direction
- **Access**: All departments, all resources
- **Responsibilities**:
  - Receive orders from BOSS HƯNG
  - Analyze system intelligence
  - Make strategic decisions
  - Coordinate with AO and all Agent Leaders
- **Models**: Llama 3 8B, Gemini 1.5 Pro (fallback)

#### AO Agent (Assistant Operating)
- **Role**: Operational Execution & Real-time Coordination
- **Access**: All agents, all tasks, operations
- **Responsibilities**:
  - Execute strategic plans from BA
  - Coordinate departments
  - Monitor task progress
  - Allocate resources
  - Real-time system monitoring
- **Models**: Llama 3 8B, Gemini 1.5 Pro

#### 6 Agent Leaders (L-Finance, L-Evolution, L-iZFx, L-Marketing, L-Sales, L-HR)
- **Role**: Department-specific management
- **Access**: Own department resources
- **Responsibilities**:
  - Manage department agents
  - Execute departmental tasks
  - Report to BA/AO
  - Allocate domain-specific resources
- **Models**: Gemini 1.5 Flash (optimized)

#### Agent Con (Specialized Workers)
- **Role**: Domain-specific execution
- **Responsibilities**:
  - Execute assigned tasks
  - Report results to Agent Leader
  - Collaborate with other agents
  - Handle domain expertise

### 6. Chat Integration Points

#### 6.1 Telegram Integration
```typescript
// Direct whitelist access for founders
const whitelist = ["izfxtrade", "fxbluenet"];

if (whitelist.includes(username)) {
  // Direct authorization bypass
  // No OTP, no guest auth - just pure access
  
  // Commands:
  // /start         - Initialize bot
  // /status        - Get fleet status
  // /finance       - Get treasury info
  // /agent         - List available agents
  // Any message    - Send to active_agent_id
}
```

#### 6.2 Session Management
```typescript
// Bot session tracking
matrix_bot_sessions {
  telegram_user_id,      // Unique Telegram ID
  active_agent_id,       // Current connected agent (BA/AO/etc)
  auth_status,           // "verified"
  last_interaction       // Timestamp
}

// User switches agent: sel_{agent_id} callback
// Message routed to active_agent_id
```

#### 6.3 Dynamic Dashboard (renderV25OmegaMaster)
```html
<!-- Key Features -->
- Real-time agent selector (dropdown)
- Message history with markdown parsing
- Agent list with status indicators
- Department matrix visualization
- Task execution pipeline
- Financial dashboard
- Node fleet monitoring
- LLM quota tracking

<!-- Commands in Dashboard -->
/agent          - Select target agent
/status         - System health
/finance        - Treasury info
Custom message  - Send to selected agent
```

### 7. Data Flow for Chat

```
1. Message Received
   ├─ Web: JavaScript fetch to /api/chat
   ├─ Telegram: handleTelegramWebhook()
   └─ Validated & parsed

2. Routing Decision
   ├─ Extract target_agent_id (default: "BA")
   ├─ Look up agent config in agents table
   └─ Get associated models and system prompt

3. Context Building
   ├─ Fetch TRUTH_MATRIX:
   │  ├─ All nodes from nodes table
   │  ├─ All agents from agents table
   │  └─ All projects from matrix_projects table
   ├─ Build system message with:
   │  ├─ GLOBAL_SYSTEM_PROMPT
   │  ├─ Agent-specific system_prompt
   │  └─ Current TRUTH_MATRIX state
   └─ Combine with user message

4. Model Selection (with Failover)
   ├─ Get agent.model_ids CSV
   ├─ Try each model sequentially:
   │  ├─ Call env.AI.run(model_path, { messages })
   │  └─ On success: return response
   ├─ If all fail:
   │  ├─ Use fallback: @cf/meta/llama-2-7b-chat-int8
   │  └─ Add warning: "[!WARNING] FAILOVER MATRIX ACTIVE"
   └─ Track which model was used

5. Usage Tracking
   ├─ Estimate tokens: (msg.length + response.length) / 4
   ├─ Update matrix_quotas by provider
   ├─ Update agents total_tokens_used
   └─ Check daily_limit for alerts

6. Response Handling
   ├─ Check for [ROUTE: AgentName] command in response
   │  └─ Auto-route follow-up message if found
   ├─ Parse markdown for rendering
   ├─ Store in matrix_chat_history
   ├─ Return to user:
   │  ├─ { response, model, timestamp, source }
   │  └─ By same channel (Web/Telegram)
   └─ Send Telegram message if source=telegram
```

### 8. Permission & Authorization

#### Web Dashboard
```
Authorization: Cloudflare Zero Trust (CF Access)
├─ /install           → Public (no auth)
├─ /webhook/telegram  → Secret token validation
└─ All other routes   → CF-Access-Authenticated-User-Email required

Email extracted: request.headers.get("Cf-Access-Authenticated-User-Email")
Display in badge: <span class="badge bg-primary">${userEmail}</span>
```

#### Telegram Bot
```
Authorization: Direct Whitelist
├─ Allowed users: ["izfxtrade", "fxbluenet"]
├─ Secret validation: X-Telegram-Bot-Api-Secret-Token
└─ Session tracking: matrix_bot_sessions table
```

### 9. Scalability & Monitoring

#### Token Quota Management
```typescript
// Track per-provider quotas
matrix_quotas {
  provider: "cloudflare" | "gemini" | "nvidia",
  daily_usage,       // Current day tokens used
  daily_limit,       // Hard cap per provider
  status            // "active" | "warning" | "exceeded"
}

// Display in dashboard:
Discord: CloudFlare: (total_usage / total_limit)
Example: "127,453 / ∞" (Cloudflare unlimited)
         "45,230 / 90,000" (Gemini quota)
```

#### Monitoring & Logging
```typescript
// Real-time tracking
- Chat history: matrix_chat_history table
- Agent status: agents table (status, progress)
- Task tracking: matrix_tasks table
- Quota alerts: matrix_quotas table

// Dashboard displays:
- Last 30 chat messages
- Agent activity status
- Daily token usage graph
- Task completion rate
- Fleet health indicators
```

### 10. Deployment & Configuration

#### Environment Variables (wrangler.toml)
```toml
[vars]
TELEGRAM_TOKEN = "8438121452:AAF0J-6ALOFOHAl-0bFB_HNRD1iwMaCl8x4"
TELEGRAM_SECRET = "boss_izlife_2026"
MY_TELEGRAM_ID = "0"  # BOSS HƯNG user ID

[[d1_databases]]
binding = "DB"
database_name = "bossizlife"
database_id = "83b83dbd-89b3-4ac3-ab5b-507a98681b9d"

[[kv_namespaces]]
binding = "NEWS"
id = "a483a061a1b94ca5b1b3edbcb8e468bc"

[ai]
binding = "AI"
```

#### Deployment Commands
```bash
# Deploy DB schema
npx wrangler d1 execute bossizlife --remote --file=schema.sql

# Deploy code
npx wrangler deploy

# View logs
npx wrangler tail
```

### 11. Integration Checklist

- [x] **D1 Database**: All tables created (agents, chat_history, quotas, etc.)
- [x] **Cloudflare AI**: Model access via env.AI.run()
- [x] **Telegram Bot**: Webhook + whitelist auth
- [x] **Workers**: Routing to agents via /api/chat
- [x] **KV Storage**: News feeds cached
- [x] **Zero Trust**: CF Access policy on dashboard
- [x] **Failover**: Multi-model strategy implemented
- [x] **Monitoring**: Usage tracking + quota alerts
- [x] **Dashboard**: Dynamic HTML with real-time updates

### 12. Durable Objects Implementation (Phase 2)

#### 12.1 Architecture with Durable Objects
```
Durable Objects Namespaces:

1. AGENT_COORDINATOR
   ├─ Class: AgentCoordinator
   ├─ State: Central orchestration logic
   ├─ Responsibilities:
   │  ├─ Manage BA & AO agent states
   │  ├─ Coordinate cross-department tasks
   │  └─ Handle agent lifecycle
   └─ WebSocket: Real-time BA/AO updates

2. ROOM_MANAGER
   ├─ Class: RoomManager
   ├─ Sharded by room_id
   ├─ Responsibilities:
   │  ├─ Real-time room state (Project/Dept/Task)
   │  ├─ Message broadcasting
   │  └─ Member presence tracking
   ├─ Per-room WebSocket connections
   └─ Room types: PROJECT, DEPARTMENT, TASK, MONITORING

3. AGENT_CHAT_SESSION
   ├─ Class: AgentChatSession
   ├─ Sharded by (agent_id + session_id)
   ├─ Responsibilities:
   │  ├─ persistent conversation state
   │  ├─ Token tracking per agent
   │  └─ Model failover orchestration
   └─ SQLite storage: 10GB per object

4. TELEGRAM_SESSION_MANAGER
   ├─ Class: TelegramSessionManager
   ├─ Sharded by telegram_user_id
   ├─ Responsibilities:
   │  ├─ Persistent session state
   │  ├─ Multi-agent switching
   │  └─ Message queue & retry logic
   └─ WebSocket: Real-time Telegram sync
```

#### 12.2 Durable Objects Data Flow
```
Request Flow with Durable Objects:

Web/Telegram Message
    │
    ▼
Worker: handleChatApi()
    │
    ├─ Get stub: env.AGENT_CHAT_SESSION.get(id)
    ├─ Call: socket.sendMessage({ message, target_agent_id })
    │
    ▼
Durable Object: AgentChatSession
    ├─ Persistent state: conversation history
    ├─ SQLite storage: chat_history table
    ├─ Call AI model via Worker
    ├─ Track quotas (websocket broadcast)
    ├─ Emit: "message_response" event
    │
    ▼
Worker receives response
    ├─ Return to user (Web/Telegram)
    └─ Broadcast to Room (via ROOM_MANAGER)
```

#### 12.3 Project Structure
```
cloud_platform/
├─ src/
│  ├─ index.ts                 (Main Worker - existing)
│  ├─ ChatBoss.md              (This documentation)
│  ├─ durable-objects/         (NEW)
│  │  ├─ AgentCoordinator.ts
│  │  ├─ RoomManager.ts
│  │  ├─ AgentChatSession.ts
│  │  ├─ TelegramSessionManager.ts
│  │  └─ types.ts
│  └─ middleware/
│     ├─ auth.ts
│     ├─ errorHandler.ts
│     └─ logger.ts
├─ schema.sql                  (Database schema - existing)
├─ wrangler.toml              (Updated with DO bindings)
├─ package.json
├─ tsconfig.json
└─ README.md
```

#### 12.4 Wrangler.toml Configuration
```toml
[env.production]
name = "boss"
main = "src/index.ts"
compatibility_date = "2024-03-20"

# Durable Objects Bindings
[[durable_objects.bindings]]
name = "AGENT_COORDINATOR"
class_name = "AgentCoordinator"
migration_tag = "v1"

[[durable_objects.bindings]]
name = "ROOM_MANAGER"
class_name = "RoomManager"
migration_tag = "v1"

[[durable_objects.bindings]]
name = "AGENT_CHAT_SESSION"
class_name = "AgentChatSession"
migration_tag = "v1"

[[durable_objects.bindings]]
name = "TELEGRAM_SESSION_MANAGER"
class_name = "TelegramSessionManager"
migration_tag = "v1"

[ai]
binding = "AI"

[[d1_databases]]
binding = "DB"
database_name = "bossizlife"
database_id = "83b83dbd-89b3-4ac3-ab5b-507a98681b9d"

[[kv_namespaces]]
binding = "NEWS"
id = "a483a061a1b94ca5b1b3edbcb8e468bc"

[vars]
TELEGRAM_TOKEN = "8438121452:AAF0J-6ALOFOHAl-0bFB_HNRD1iwMaCl8x4"
TELEGRAM_SECRET = "boss_izlife_2026"
```

#### 12.5 Implementation Phases

**Phase 2A: Real-time Communication**
- Implement AGENT_COORDINATOR DO
- WebSocket connections for BA/AO
- Real-time status updates
- Estimate: 2 weeks

**Phase 2B: Room Management**
- Implement ROOM_MANAGER DO
- Multi-room WebSocket support
- Broadcast messaging system
- Estimate: 2 weeks

**Phase 2C: Advanced Chat**
- Implement AGENT_CHAT_SESSION DO
- Persistent conversation state
- Token tracking & quotas
- Estimate: 2 weeks

**Phase 2D: Telegram Enhancement**
- Implement TELEGRAM_SESSION_MANAGER DO
- Session persistence
- Multi-agent session handling
- Estimate: 1 week

**Phase 3: Advanced Features**
- Agent-to-agent direct messaging
- Custom agent creation UI
- Plugin marketplace
- Estimate: 4 weeks

**Phase 4: Enterprise**
- Blockchain integration
- Decentralized deployment
- Multi-org support
- Estimate: 6 weeks
```

---

**Version**: 1.0  
**Last Updated**: 2026-03-25  
**Status**: Production Ready ✅
