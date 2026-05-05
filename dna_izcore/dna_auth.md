iZCore Auth: Decentralized Multi-User Authentication System

1. Overview

iZCore Auth is the core security layer of the iZLife B.O.S.S. (Business Operating System Services). It provides a robust, decentralized, and lightweight authentication mechanism designed specifically for edge computing environments and high-performance distributed networks.

Built with Rust, iZCore Auth utilizes industry-standard TOTP (Time-based One-Time Password) algorithms to verify identities across a network of autonomous nodes without relying on a central authority.

2. Operating Principles

2.1 Silent Connection & Background Sync

Upon installation, iZcore connects to the global network silently as a background service. It establishes a secure channel for data synchronization but remains "headless" and locked until an authorized user interacts via the CLI.

2.2 Intelligent Ownership Discovery

When a user executes the izcore command on a terminal:

Pre-owned Node: If the node is already linked to a User ID, the system immediately challenges for a 6-digit 2FA code.

Unclaimed Node: If the node is new or unowned, iZcore initiates an "Auto-Onboarding" sequence, allowing the first user to claim ownership, set a Username, and generate a 2FA Secret.

3. Core Architecture

3.1 Technology Stack

Language: Rust (Zero-cost abstractions, memory safety).

Encryption: HMAC-SHA1 for TOTP, SHA-256 for secret seeding.

Data Format: Encrypted JSON-based local ledger (config.json).

3.2 Multi-Tier User & Device Ownership

Founder (Root): Global network control and Skill/Module repository management.

Admin: Manages business logic and approves permissions for new contributors.

Node Contributor (Owner): Users who contribute hardware. They have full control over their devices and can view their cluster status.

4. Security Design

4.1 Founder Key Protection (DNA Security)

The system utilizes GitHub Secrets during the CI/CD pipeline to inject the FOUNDER_SECRET_KEY into the binary. This ensures no plain-text keys exist in the public repository, while allowing the binary to recognize the Founder's identity globally.

4.2 Two-Factor Authentication (2FA)

First-Run Sync: Generates a QR Code for Google/Microsoft Authenticator.

Identity Verification: Users must provide both a User ID and a valid 2FA Code.

5. Command Line Interface (CLI) Workflow

Onboarding a New Node

# First interaction on an unclaimed node
izcore

> No owner detected for this node.
> Starting Auto-Onboarding...
> Please enter your desired Username: my_username
> [System displays QR Code for 2FA setup]
> Please enter 2FA Code to verify: 123456
> Node successfully linked to 'my_username'.


Accessing the Management Dashboard

Once authenticated, the CLI displays a comprehensive status report of the user's ecosystem:

------------------------------------------------------------
Welcome, [my_username] | Role: Node Owner (Verified)
------------------------------------------------------------
[NETWORK STATUS]
Connected Nodes: 5
Online: 4 | Offline: 1

[RESOURCE INVENTORY]
1. Node: FPT-PlayBox-T550 (Living Room)
   Status: Active | Skill: Market-Scanner-V2
   CPU: 12% | RAM: 450MB/2GB

2. Node: Raspberry-Pi-4 (Lab)
   Status: Idle | Skill: None (Ready for Deployment)

3. Node: Windows-Workstation (Office)
   Status: Active | Skill: Trading-Bot-X
   Running: MT5 Terminal Interface
------------------------------------------------------------
Available Actions: [list-nodes] [install-skill] [sys-log]


6. Roadmap & Future Enhancements

Zero-Knowledge Proofs (ZKP): Anonymous identity sync.

Community Skill Marketplace: Sharing verified modules across the network.

Hardware Security: YubiKey support for Founder-level actions.

7. Immortal Discovery & Sovereignty

The iZCore kernel is designed for survival in hostile or fragmented network environments. It implements a multi-stage discovery protocol that eliminates single points of failure:

Peer Caching: Every successfully established P2P connection is stored in peer_cache.json. Upon reboot, the node attempts to reconnect to these known peers before querying external trackers.

Direct IP Seeding: The binary contains hardcoded IP addresses of iZBoss Seed Nodes. This bypasses DNS-level censorship or domain seizure.

Autonomous Mesh: Once a single connection is made, the Kademlia DHT takes over, allowing the node to explore the entire global mesh without ever contacting boss.iz.life.

© 2026 iZLife OS Project. All rights reserved.
8. Sovereign Economic Layer & Wallet Identity

iZCore v0.2.4+ formalizes the link between financial identity and system authority:

iZWallet Integration: Every node uses a native, non-custodial wallet (BIP-39) as its root identity. The public address is used for network discovery and permission checks.

Cryptographic Command Signing: Sensitive actions (updates, user management, financial transfers) must be digitally signed using the node's private key before being broadcast to the mesh.

Founder Verification: If the node's wallet matches the FOUNDER_WALLET_ADDRESS embedded during the secure build process, the node is granted absolute "Founder" privileges automatically.

Reward Distribution: Tokens are distributed directly to the node's wallet based on PoC (Proof of Contribution).

© 2026 iZLife OS Project. All rights reserved.

