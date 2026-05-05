# 🌌 iZ.Life B.O.S.S. — Business Operating System Services
### Decentralized AI Infrastructure Network (DePIN) | The Billion-Dollar Vision

🇺🇸 **English** | 🇻🇳 [Tiếng Việt](README_VN.md)

**iZ.Life B.O.S.S.** (Business Operating System Services) is not just software; it is a revolution in **Decentralized Physical Infrastructure Networks (DePIN)**. 

> **Core Philosophy: iZ.BOSS = "iZ" (Easy) to be the "BOSS".**
> We believe that everyone has the right and the capability to become an autonomous business owner with the power of Artificial Intelligence.

We are building a global Mesh network where every idle hardware device — from an old smartphone or PlayBox to a high-end server farm — can transform into an **Intelligence Node** to co-operate, co-earn, and co-profit.

---

## 🚀 The Billion-Dollar Vision: Comprehensive Intelligence Ecosystem
iZ.Life B.O.S.S. is designed to unlock human potential and optimize cash flow through two main pillars:

### 💎 1. For Individuals: The Immortal Financial Assistant
-   **iZThuChi (Free)**: A comprehensive personal financial planning tool (Income/expense tracking, fund management, financial health analysis, and cash flow forecasting).
-   **Signal Analysis Packages**: Real-time trading signal analysis systems for multiple markets.
-   **Investment & Fund Management Assistant**: A personal AI to support investment analysis, portfolio tracking, and profit optimization.
-   **Skill Marketplace**: A hub where Masters can lease automated bots or sell trading signals to the community.

### 🏢 2. For Enterprises: "One-Touch" Management
Transform iZ.Life B.O.S.S. into a fully autonomous business entity:
-   **Decentralized ERP & CRM**: Resource and customer management on the Mesh infrastructure, ensuring absolute security and zero data loss.
-   **Dedicated Business Assistant (BA)**: Every business owns a unique BA, automating roles for every position from CEO and Accountant to Sales and CS.
-   **Closed-Loop Automation**: The system automatically conducts market research, sources suppliers, creates business/marketing plans, implements, monitors, and optimizes.
-   **CEO Only Approves & Receives Results**: The CEO simply approves plans and costs; all operations, platform sales, customer support, and tax settlements are handled by AI Agents.
-   **Solutions for "One-Person Businesses"**: Empowering any individual to become a business owner with the support of a elite digital workforce.

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
