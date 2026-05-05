# 🌌 iZ.Life BOSS — Decentralized AI Infrastructure Network (DePIN)
### Redefining Infrastructure & Intelligence Ownership | The Billion-Dollar Vision

🇺🇸 **English** | 🇻🇳 [Tiếng Việt](README_VN.md)

**iZ.Life BOSS** is not just software; it is a revolution in **Decentralized Physical Infrastructure Networks (DePIN)**. We are building a global Mesh network where every idle hardware device — from an old smartphone or PlayBox to a high-end server farm — can transform into an **Intelligence Node** to co-operate, co-earn, and co-profit.

---

## 🚀 Vision: The Infinite Intelligence Ecosystem
We aim to build the world's largest distributed supercomputer, turning fragmented hardware resources into a unified entity capable of:
-   **AI Training & Inference**: Training and running massive LLMs in a decentralized manner.
-   **Autonomous Trading Bots**: Operating financial algorithms 24/7 without human intervention.
-   **Distributed Storage**: Secure, immortal data storage across the Mesh network.
-   **Task Execution Marketplace**: A marketplace for executing complex digital tasks.

## 🏢 Sovereign AI Financial Automation Entity
iZ.Life BOSS goes beyond infrastructure; we are building an **Autonomous Digital Financial Corporation**:
-   **Financial Autonomy (Highest Priority)**: Automated market analysis, trading, fund management, and **automated payroll/settlement** for LLM costs, agent leasing, and contributor rewards.
-   **Autonomous Operations & Scaling**: The Operations (HR) department autonomously "recruits" new nodes from the community or leases VPS/Cloud resources to expand capacity based on project demand.
-   **ERP/CRM & Financial Services**: Providing decentralized management solutions for external partners to maximize revenue.

## 💰 iZWallet: Sovereign Wallet & Digital Identity
iZCore now features a powerful, integrated non-custodial wallet system:
-   **BIP-39 Standard**: 24-word recovery phrase for absolute asset ownership.
-   **Node Identity**: Your wallet address serves as the unique identifier for your node on the P2P network.
-   **Founder Verification**: Automatic Root access if the wallet address matches the embedded `FOUNDER_WALLET_ADDRESS`.
-   **Digital Signing**: All sensitive system commands are signed using the wallet's private key.
-   **Automated Settlements**: Receive "Proof of Contribution" (PoC) rewards directly for computational contributions.

```bash
izcore wallet init
```

---

## 🤖 iZCore Agent: Natural Language Management
iZCore now supports activating a local AI Assistant (Tiny LLM) to help you run the system without remembering complex CLI commands:
```bash
izcore agent on
```
This feature automatically analyzes hardware resources and installs the most suitable language model (e.g., Phi-3 or Llama-3), turning your device into a communicative autonomous entity. When the network grows strong enough, a global **AO (Autonomous Operator)** will be activated as the ultimate network assistant.

## 🛡️ Immortal Mesh Network
iZ.Life BOSS is built for eternity. Even if the `boss.iz.life` domain disappears, the network continues to operate autonomously through:
-   **Direct IP Seeding**: Hardcoded seed node IP addresses in the binary.
-   **Peer Caching**: Nodes remember previously connected peers locally.
-   **Local mDNS**: Automatic local peer discovery without internet.

---

## 1. THE DNA LAYER: iZCore (The Immortal Kernel)
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
├── /dna_iZCore (Rust)             # Identity, P2P, OTA Kernel
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

### ⚡ iZCore Quick Install (Every Device)
Use a single URL for all platforms. The system automatically serves the correct script (Bash for Unix/Mac or PowerShell for Windows):

**Source 1: Via System Domain (Recommended)**
- **Linux / macOS / Android**:
  ```bash
  curl -fsSL https://boss.iz.life/install | sh
  ```
- **Windows (PowerShell)**:
  ```powershell
  irm https://boss.iz.life/install | iex
  ```

**Source 2: Direct from GitHub (Fallback/Absolute Trust)**
- **Unix-like**: `curl -fsSL https://raw.githubusercontent.com/iZFxTrade/iZBoss/main/dna_iZCore/install.sh | sh`
- **Windows**: `irm https://raw.githubusercontent.com/iZFxTrade/iZBoss/main/dna_iZCore/install.ps1 | iex`

> **Note**: The current installer prioritizes fetching directly from the GitHub project. The `boss.iz.life` system is being developed into a decentralized storage network (similar to BitTorrent).

### 💻 Command Line Interface (CLI)
Once installed, use the `boss` command to manage the entire system:
- `boss status`: Check health of Nodes & Agents.
- `boss agent list`: View active digital workforce.
- `boss node fleet`: Manage devices across the network.
