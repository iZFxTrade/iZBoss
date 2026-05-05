# iZ.Life BOSS (Business Operating System - Autonomous Entity)

🇺🇸 **English** | 🇻🇳 [Tiếng Việt](README_VN.md)

**iZ.Life BOSS** is an Autonomous Business Operating System—a complex ecosystem of distributed systems, intelligent AI agents serving as personnel and executives, and a hybrid architecture bridging P2P networks with Cloudflare's edge computing.

---

## 1. THE DNA LAYER: iZcore (The Kernel)
The ultra-lightweight core written in **Rust**, acting as the "genetic code" embedded in every device.
- **Hardware Fingerprinting**: Hashing algorithms for device identification based on CPU ID, MAC Address, and Disk Serial.
- **Self-Bootstrap**: Minimalist scripts connecting to `boss.iz.life` to determine environment and download installation packages.
- **P2P Handshake**: Powered by `libp2p` for automated peer discovery via mDNS / Gossip Protocol.
- **Auth Gate**: ED25519 digital signature verification—Only the Master (Admin) can awaken the Entity.
- **OTA Listener**: Background process receiving hot updates via Cloudflare R2 or neighboring nodes.

## 2. LAYER 1: Command Center (Cloudflare Workers)
- **Dashboard (boss.iz.life)**: Centralized management interface.
- **Node Manager**: Infrastructure management (health, resources, connectivity).
- **Agent & Process Monitor**: Real-time monitoring of agent processes, execution logs, and heartbeat status.
- **Skill & Module Registry**: Repository for logic/skills deployable across the network.
- **LLM & API Gateway**: Management of API keys (OpenAI, Gemini, etc.) and quota configurations.
- **Webhook & Data Feed Manager**: External data sourcing (Price feeds, News, Social webhooks).
- **OTA Warehouse**: Secure storage for executable binaries and installers on Cloudflare R2.
- **Task Queue (D1/KV)**: Distributed task scheduling for the entire mesh.

## 3. LAYER 2: Supreme Assistant System (The Executives)
A coordination of two specialized AIs:
1. **BOSS Assistant (BA)**: The **Strategic Brain**. Listens to visions, sets OKRs, and provides supreme oversight. Manages all high-level input from the Master.
2. **Administrative Officer (AO)**: The **Operating Director**. Allocates resources, manages Departmental Agents (HR), directly controls OTA deployments, and reports execution status back to the BA.

## 4. COMMUNICATION PROTOCOL (BA ↔ AO)
Utilizing **IACS (Inter-Assistant Communication Schema)**:
- `GOAL_ASSIGN` (BA -> AO): Objectives, budgets, and deadlines.
- `RESOURCE_QUERY` (BA -> AO): Inquiry regarding network and resource status.
- `STATUS_REPORT` (AO -> BA): Progress reports and risk assessment from departments.
- `ACTION_CONFIRM` (AO -> BA): Confirmation of initialized activities.

## 5. AUTONOMOUS DEPARTMENTS (Under AO)
- **Finance**: Wallet management and profit accounting.
- **Evolution (R&D)**: Automated resource scanning (HF, GitHub) and self-upgrading models.
- **iZFx (Trading)**: Proprietary trading, fund management, and EA optimization.
- **Marketing**: Automated media content and social presence.
- **Sales & CS**: Customer care and engagement via `izthuchi` funnels.
- **Agent Manager (HR)**: The AO's arm for digital workforce management.

## 6. ROADMAP
- **Phase 1: Q1 - The Launchpad**: Cloudflare Dashboard, Rust OTA.
- **Phase 2: Q2 - The Mesh**: CLI ED25519, P2P network.
- **Phase 3: Q3 - The Sovereignty**: Self-Hosting Dashboard, financial autonomy.
- **Phase 4: Singularity**: Self-Coding, Startup Machine.

---

## 7. PROJECT STRUCTURE
```text
/
├── /dna_izcore (Rust)             # Identity, P2P, OTA Kernel
├── /cloud_platform (TS)           # Cloudflare Deployment (Workers/D1)
├── /executive_layer (Rust)        # BA (Brain), AO (Action), HR Manager
├── /departments                   # Autonomous Departments (Rust/Wasm)
│   ├── /finance                   # Wallet & Accounting
│   ├── /rd                        # GitHub/HF Scanning & Self-Upgrade
│   ├── /trading                   # EA, AI Trading, Fund Management
│   ├── /marketing                 # Auto-Content & Social API
│   └── /sales_cs                  # Interaction Hub & Chatbots
├── /distributors (Binaries)       # OTA Binary Storage
└── /cli_gateway (Rust)            # Supreme CLI Interface
```

---

## 8. QUICK START

### ⚡ iZcore Quick Install (Every Device)
Use a single command to auto-detect your device, download the binary from **GitHub Releases**, and register into the matrix:

```bash
curl -fsSL https://boss.iz.life/install | sh
```

> **Note**: The current installer prioritizes fetching directly from the GitHub project. The `boss.iz.life` system is being developed into a decentralized storage network (similar to BitTorrent), ensuring iZcore distribution remains independent of any fixed domain in the future.

### 💻 Command Line Interface (CLI)
Once installed, use the `boss` command to manage the entire system:
- `boss status`: Check health of Nodes & Agents.
- `boss agent list`: View active digital workforce.
- `boss node fleet`: Manage devices across the network.
