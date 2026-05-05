/**
 * iZ.Life BOSS Command Center — V25.5 Platinum
 * Cloudflare Worker entry point
 *
 * Routes:
 *   GET  /                      → Dashboard HTML
 *   GET  /install               → iZCore universal install script
 *   GET  /api/data              → Full system data (nodes, agents, depts...)
 *   POST /api/register          → Node self-registration
 *   POST /api/heartbeat         → Node heartbeat / keep-alive
 *   POST /api/p2p/announce      → P2P presence announcement
 *   GET  /api/p2p/peers         → Fetch active peer list
 *   GET  /api/ota/latest        → OTA manifest for latest binary
 */

export interface Env {
    DB: D1Database;
    AI: any;
    NEWS: KVNamespace;
    TELEGRAM_TOKEN: string;
    MY_TELEGRAM_ID: string;
}

export default {
    async fetch(request: Request, env: Env): Promise<Response> {
        const url = new URL(request.url);
        const method = request.method;
        const path = url.pathname;

        // ── Route Table ──────────────────────────────────────
        if (path === '/install' && method === 'GET')
            return handleInstallScript(request);

        if (path === '/api/data' && method === 'GET')
            return handleDataApi(env);

        if (path === '/api/register' && method === 'POST')
            return handleNodeRegister(request, env);

        if (path === '/api/heartbeat' && method === 'POST')
            return handleHeartbeat(request, env);

        if (path === '/api/users/sync' && method === 'GET')
            return handleUserSync(env);

        if (path === '/api/users/approve' && method === 'POST')
            return handleUserApprove(request, env);

        if (path === '/api/p2p/announce' && method === 'POST')
            return handleP2PAnnounce(request, env);

        if (path === '/api/p2p/peers' && method === 'GET')
            return handleP2PPeers(url, env);

        if (path === '/api/ota/latest' && method === 'GET')
            return handleOtaManifest(url);

        if (path === '/' || path === '')
            return new Response(renderDashboard(), {
                headers: { 'Content-Type': 'text/html; charset=UTF-8' }
            });

        return new Response('Not Found', { status: 404 });
    }
};

// ────────────────────────────────────────────────────────────
// /install — Serve universal iZCore installer script
// ────────────────────────────────────────────────────────────
// ────────────────────────────────────────────────────────────
// /install — Serve universal iZCore installer script
// ────────────────────────────────────────────────────────────
// ────────────────────────────────────────────────────────────
// /install — Serve universal iZCore installer (Bash or PowerShell)
// ────────────────────────────────────────────────────────────
function handleInstallScript(request: Request): Response {
    const ua = request.headers.get('user-agent') || '';
    const isWindows = ua.includes('Windows') || ua.includes('PowerShell') || ua.includes('MSIE');

    if (isWindows) {
        const psScript = `# iZ.Life BOSS — iZCore Windows Installer
$ErrorActionPreference = 'Stop'
$BOSS_API = "https://boss.iz.life"; $GITHUB_REPO = "iZFxTrade/iZBoss"
$INSTALL_DIR = "$env:USERPROFILE\\bin"; $BINARY_NAME = "iZCore.exe"
Write-Host "[iZCore] Detecting Windows device..." -ForegroundColor Cyan
try { $REL = Invoke-RestMethod -Uri "https://api.github.com/repos/$GITHUB_REPO/releases/latest"; $VER = $REL.tag_name } catch { $VER = "v0.1.1" }
$PLAT = "windows-x86_64"
$MAC = Get-CimInstance Win32_NetworkAdapterConfiguration | Where-Object { $_.IPEnabled -eq $true } | Select-Object -First 1 -ExpandProperty MACAddress
$DEVICE_ID = "iznode-" + ([System.BitConverter]::ToString([System.Security.Cryptography.SHA256]::Create().ComputeHash([System.Text.Encoding]::UTF8.GetBytes("$MAC-$env:COMPUTERNAME"))).Replace("-", "").ToLower().Substring(0, 16))
ok "Device ID: $DEVICE_ID"
$URL = "https://github.com/$GITHUB_REPO/releases/download/$VER/iZCore-$PLAT.exe"
if (-not (Test-Path $INSTALL_DIR)) { New-Item -Path $INSTALL_DIR -ItemType Directory | Out-Null }
log "Downloading binary from GitHub..."
Invoke-WebRequest -Uri $URL -OutFile "$INSTALL_DIR\\$BINARY_NAME"
if ($env:PATH -notlike "*$INSTALL_DIR*") { [Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$INSTALL_DIR", "User") }
log "Registering..."
$P = @{ device_id=$DEVICE_ID; platform=$PLAT; hostname=$env:COMPUTERNAME; version=$VER } | ConvertTo-Json
Invoke-RestMethod -Uri "$BOSS_API/api/register" -Method Post -Body $P -ContentType "application/json" | Out-Null
Start-Process -FilePath "$INSTALL_DIR\\$BINARY_NAME" -WindowStyle Hidden
Write-Host "[✓] iZCore installed! Run: iZCore --status" -ForegroundColor Green
`;
        return new Response(psScript, { headers: { 'Content-Type': 'text/plain; charset=utf-8' } });
    }

    const bashScript = `#!/bin/sh
# iZ.Life BOSS — iZCore Universal Installer (Bash)
set -e
BOSS_API="https://boss.iz.life"; GITHUB_REPO="iZFxTrade/iZBoss"
RED='\\033[0;31m'; GREEN='\\033[0;32m'; YELLOW='\\033[1;33m'; CYAN='\\033[0;36m'; BOLD='\\033[1m'; NC='\\033[0m'
log() { printf "\${CYAN}[iZCore]\${NC} %s\\n" "$1"; }
ok() { printf "\${GREEN}[✓]\${NC} %s\\n" "$1"; }
VERSION=$(curl -s https://api.github.com/repos/\${GITHUB_REPO}/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\\1/')
[ -z "\${VERSION}" ] && VERSION="v0.1.1"
OS=$(uname -s | tr '[:upper:]' '[:lower:]'); ARCH=$(uname -m)
case "$ARCH" in x86_64) A="x86_64" ;; aarch64|arm64) A="aarch64" ;; *) A="armv7" ;; esac
case "$OS" in linux) O="linux" ;; darwin) O="macos" ;; *) O="android" ;; esac
P="\${O}-\${A}"; ok "Device: \${P} (\${VERSION})"
URL="https://github.com/\${GITHUB_REPO}/releases/download/\${VERSION}/iZCore-\${P}"
curl -fsSL -o /tmp/iZCore "\${URL}" || curl -fsSL -o /tmp/iZCore "\${BOSS_API}/api/ota/download?platform=\${P}"
chmod +x /tmp/iZCore; sudo mv /tmp/iZCore /usr/local/bin/iZCore 2>/dev/null || mv /tmp/iZCore "\$HOME/bin/iZCore"
MAC=$(ip link 2>/dev/null | grep "link/ether" | head -1 | awk '{print $2}' || ifconfig 2>/dev/null | grep "ether" | head -1 | awk '{print $2}' || echo "00:00")
DEVICE_ID="iznode-$(printf "%s" "\$MAC" | sha256sum | cut -c1-16)"
curl -s -X POST "\${BOSS_API}/api/register" -H "Content-Type: application/json" -d "{\\"device_id\\":\\"\${DEVICE_ID}\\",\\"platform\\":\\"\${P}\\",\\"version\\":\\"\${VERSION}\\"}" >/dev/null 2>&1
iZCore &
ok "iZCore installed! Run: iZCore --status"
`;
    return new Response(bashScript, { headers: { 'Content-Type': 'text/plain; charset=utf-8' } });
}

// ────────────────────────────────────────────────────────────
// /api/register — Node self-registration
// ────────────────────────────────────────────────────────────
async function handleNodeRegister(request: Request, env: Env): Promise<Response> {
    try {
        const body: any = await request.json();
        const { device_id, platform, hostname, version, username } = body;

        if (!device_id) return Response.json({ error: 'device_id required' }, { status: 400 });

        // 1. Handle User Registration (Self-claim)
        if (username) {
            await env.DB.prepare(`
                INSERT INTO users (id, name, role, approved)
                VALUES (?, ?, 'user', 0)
                ON CONFLICT(id) DO NOTHING
            `).bind(username, username).run();
        }

        // 2. Upsert node with owner_id
        await env.DB.prepare(`
            INSERT INTO nodes (id, name, status, role, cpu_info, owner_id, last_heartbeat)
            VALUES (?, ?, 'online', 'node', ?, ?, CURRENT_TIMESTAMP)
            ON CONFLICT(id) DO UPDATE SET
                status = 'online',
                owner_id = COALESCE(owner_id, ?),
                last_heartbeat = CURRENT_TIMESTAMP
        `).bind(device_id, hostname || device_id, platform || 'unknown', username || null, username || null).run();

        console.log(`[Register] Node joined: ${device_id} (Claimed by: ${username || 'anonymous'})`);
        return Response.json({ ok: true, device_id, message: 'Node registered and ownership claimed.' });
    } catch (e: any) {
        return Response.json({ error: e.message }, { status: 500 });
    }
}

// ────────────────────────────────────────────────────────────
// /api/heartbeat — Node keep-alive ping
// ────────────────────────────────────────────────────────────
async function handleHeartbeat(request: Request, env: Env): Promise<Response> {
    try {
        const body: any = await request.json();
        const { device_id, status } = body;

        if (!device_id) return Response.json({ error: 'device_id required' }, { status: 400 });

        await env.DB.prepare(`
            UPDATE nodes SET status = ?, last_heartbeat = CURRENT_TIMESTAMP WHERE id = ?
        `).bind(status || 'online', device_id).run();

        return Response.json({ ok: true, device_id, timestamp: Date.now() });
    } catch (e: any) {
        return Response.json({ error: e.message }, { status: 500 });
    }
}

// ────────────────────────────────────────────────────────────
// /api/p2p/announce — P2P presence broadcast
// ────────────────────────────────────────────────────────────
async function handleP2PAnnounce(request: Request, env: Env): Promise<Response> {
    try {
        const body: any = await request.json();
        const { device_id, platform } = body;

        // Store peer announcement in KV with 5min TTL
        await env.NEWS.put(
            `p2p:peer:${device_id}`,
            JSON.stringify({ device_id, platform, seen_at: Date.now() }),
            { expirationTtl: 300 }
        );

        return Response.json({ ok: true, message: 'Presence announced' });
    } catch (e: any) {
        return Response.json({ error: e.message }, { status: 500 });
    }
}

// ────────────────────────────────────────────────────────────
// /api/p2p/peers — Get active peers list
// ────────────────────────────────────────────────────────────
async function handleP2PPeers(url: URL, env: Env): Promise<Response> {
    try {
        const { results } = await env.DB.prepare(
            `SELECT id, name, status, last_heartbeat FROM nodes WHERE status = 'online' LIMIT 50`
        ).all();
        return Response.json(results);
    } catch (e: any) {
        return Response.json([], { status: 200 });
    }
}

// ────────────────────────────────────────────────────────────
// /api/ota/latest — OTA manifest
// ────────────────────────────────────────────────────────────
function handleOtaManifest(url: URL): Response {
    const platform = url.searchParams.get('platform') || 'linux-x86_64';
    const manifest = {
        version: '0.1.0',
        platform,
        download_url: `https://boss.iz.life/api/ota/download?platform=${platform}&version=0.1.0`,
        sha256: 'pending',
        released_at: new Date().toISOString(),
        changelog: 'Initial release — DNA Kernel with P2P + OTA + Heartbeat'
    };
    return Response.json(manifest);
}

// ────────────────────────────────────────────────────────────
// /api/data — Full system data
// ────────────────────────────────────────────────────────────
async function handleDataApi(env: Env): Promise<Response> {
    try {
        const res = await Promise.all([
            env.DB.prepare('SELECT * FROM nodes').all(),
            env.DB.prepare('SELECT agents.*, departments.name as dept_name, llm_models.name as model_name FROM agents LEFT JOIN departments ON agents.dept_id = departments.id LEFT JOIN llm_models ON agents.model_id = llm_models.id').all(),
            env.DB.prepare('SELECT * FROM departments').all(),
            env.DB.prepare('SELECT * FROM llm_models').all(),
            env.DB.prepare('SELECT * FROM skills_modules').all(),
            env.DB.prepare('SELECT * FROM webhooks_feeds').all(),
        ]);
        return Response.json({
            nodes: res[0].results,
            agents: res[1].results,
            depts: res[2].results,
            models: res[3].results,
            modules: res[4].results,
            feeds: res[5].results,
            core: { version: 'v25.5.0', status: 'Platinum', sync: new Date().toLocaleTimeString('vi-VN') }
        });
    } catch (e: any) {
        return Response.json({ error: e.message }, { status: 500 });
    }
}

// ────────────────────────────────────────────────────────────
// /api/users/sync — Get all approved users for local auth
// ────────────────────────────────────────────────────────────
async function handleUserSync(env: Env): Promise<Response> {
    try {
        const { results } = await env.DB.prepare("SELECT id, name, role, secret_2fa, approved FROM users WHERE approved = 1").all();
        return Response.json({ users: results });
    } catch (e: any) {
        return Response.json({ error: e.message }, { status: 500 });
    }
}

// ────────────────────────────────────────────────────────────
// /api/users/approve — Approve a user and their claimed nodes
// ────────────────────────────────────────────────────────────
async function handleUserApprove(request: Request, env: Env): Promise<Response> {
    try {
        const body: any = await request.json();
        const { user_id, role, approved_by } = body;

        if (!user_id || !approved_by) return Response.json({ error: 'user_id and approved_by required' }, { status: 400 });

        await env.DB.prepare(`
            UPDATE users SET approved = 1, role = ? WHERE id = ?
        `).bind(role || 'user', user_id).run();

        console.log(`[Auth] User ${user_id} approved by ${approved_by}`);
        return Response.json({ ok: true, message: `User ${user_id} approved.` });
    } catch (e: any) {
        return Response.json({ error: e.message }, { status: 500 });
    }
}

// ────────────────────────────────────────────────────────────
// Dashboard HTML
// ────────────────────────────────────────────────────────────
function renderDashboard(): string {
    const B = String.fromCharCode(96);
    const D = String.fromCharCode(36);
    const lines = [
        '<!DOCTYPE html><html lang="vi" data-bs-theme="dark"><head><meta charset="UTF-8">',
        '<title>iZ.Life BOSS | Sovereign Matrix</title><link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css" rel="stylesheet">',
        '<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css">',
        '<link href="https://fonts.googleapis.com/css2?family=Outfit:wght@300;400;500;600;700;800;900&display=swap" rel="stylesheet">',
        '<style>:root{--accent:#197bfe;--bg:#070709;--side:#0c0c0e;--card-bg:#121214;--text:#f8f9fa;--border:rgba(255,255,255,0.08);--side-w:260px;}',
        'body{font-family:"Outfit",sans-serif;background:var(--bg);color:var(--text);font-size:0.9rem;overflow-x:hidden;} .sidebar{width:var(--side-w);height:100vh;position:fixed;left:0;top:0;background:var(--side);border-right:1px solid var(--border);z-index:1200;}',
        '.nav-link{border-radius:0.75rem;color:var(--text);opacity:0.6;font-weight:600;margin:0.2rem 0.75rem;padding:0.65rem 1rem;transition:0.2s;}.nav-link:hover,.nav-link.active{background:rgba(25,123,254,0.1);color:var(--accent);opacity:1;}',
        '.navbar-boss{height:85px;background:rgba(7,7,9,0.8);backdrop-filter:blur(30px);border-bottom:1px solid var(--border);position:sticky;top:0;z-index:1100;}',
        '.logo-center{position:absolute;left:50%;transform:translateX(-50%);font-weight:950;font-style:italic;font-size:2rem;color:var(--accent);letter-spacing:-3px;}',
        '.glass-card{background:var(--card-bg);border:1px solid var(--border);border-radius:1.5rem;transition:0.3s;}.glass-card:hover{border-color:var(--accent);}',
        '.tiny{font-size:0.65rem;text-transform:uppercase;letter-spacing:1px;}.prog-bar{height:6px;background:rgba(255,255,255,0.05);border-radius:10px;overflow:hidden;}.prog-val{height:100%;background:var(--accent);transition:1s;}',
        '.install-box{background:rgba(25,123,254,0.08);border:1px solid rgba(25,123,254,0.3);border-radius:1rem;padding:1rem 1.5rem;font-family:monospace;font-size:0.85rem;}',
        '</style></head><body>',
        '<div id="sidebar" class="sidebar d-flex flex-column py-4"><div class="px-4 mb-5 text-primary fw-black" style="font-size:1.4rem;font-style:italic;">iZ.BOSS</div>',
        '<nav class="nav flex-column mt-2">',
        '<a href="#" class="nav-link active"><i class="fas fa-chart-pie me-3"></i><span>Dashboard</span></a>',
        '<a href="#" class="nav-link"><i class="fas fa-server me-3"></i><span>Nodes Fleet</span></a>',
        '<a href="#" class="nav-link"><i class="fas fa-robot me-3"></i><span>Workforce</span></a>',
        '<a href="#" class="nav-link"><i class="fas fa-brain me-3"></i><span>LLM Matrix</span></a>',
        '<a href="#" class="nav-link"><i class="fas fa-sitemap me-3"></i><span>Departments</span></a>',
        '<a href="#" class="nav-link"><i class="fas fa-terminal me-3"></i><span>Install iZCore</span></a>',
        '</nav></div>',
        '<main id="main" style="margin-left:var(--side-w);"><header class="navbar-boss px-4 d-flex align-items-center"><div class="logo-center">iZ.BOSS<div style="font-size:0.5rem;font-weight:950;letter-spacing:3px;margin-top:-5px;opacity:0.8;">SOVEREIGN MATRIX V25.5</div></div></header>',
        '<div class="container-fluid p-5">',
        '<div class="row g-4 mb-5" id="dash-sum"></div>',
        '<div class="row g-4 mb-4"><div class="col-12"><div class="glass-card p-4"><h6 class="tiny fw-black mb-3 text-primary"><i class="fas fa-terminal me-2"></i>Install iZCore — One Command</h6>',
        '<div class="install-box text-primary">curl -fsSL https://boss.iz.life/install | sh</div>',
        '<div class="tiny opacity-50 mt-2">Tự động nhận diện thiết bị → tải binary → đăng ký → gia nhập mạng lưới</div></div></div></div>',
        '<div class="row g-4"><div class="col-lg-8"><div class="glass-card p-4"><h6 class="tiny fw-black mb-4 text-primary">Active Workforce Matrix</h6><div id="dash-agents" class="row g-3"></div></div></div>',
        '<div class="col-lg-4"><div class="glass-card p-4"><h6 class="tiny fw-black mb-4 text-primary">Node Fleet</h6><div id="dash-nodes" class="vstack gap-3"></div></div></div></div>',
        '</div></main>',
        '<script>',
        'let data={}; async function sync(){try{const r=await fetch("/api/data");data=await r.json();render();}catch(e){console.error(e);}}',
        'function render(){',
        'const totA=data.agents?.length||0; const actA=(data.agents||[]).filter(x=>x.status==="running"||x.status==="online").length;',
        'const totN=data.nodes?.length||0; const actN=(data.nodes||[]).filter(x=>x.status==="online").length;',
        'document.getElementById("dash-sum").innerHTML=[',
        '{t:"Workforce",v:totA,a:actA,i:"robot"},{t:"Nodes Fleet",v:totN,a:actN,i:"server"},{t:"LLM Brains",v:data.models?.length||0,a:data.models?.length||0,i:"brain"}',
        '].map(x=>' + B + '<div class="col-md-4"><div class="glass-card p-4"><div class="d-flex justify-content-between align-items-center mb-3"><div class="tiny fw-bold opacity-50">' + D + '{x.t}</div><i class="fas fa-' + D + '{x.i} text-primary"></i></div><div class="row text-center"><div class="col-4"><b>' + D + '{x.v}</b><div class="tiny opacity-50">Total</div></div><div class="col-4 text-primary"><b>' + D + '{x.a}</b><div class="tiny">Active</div></div><div class="col-4 text-danger"><b>' + D + '{x.v-x.a}</b><div class="tiny">Off</div></div></div></div></div>' + B + ').join("");',
        'document.getElementById("dash-agents").innerHTML=(data.agents||[]).map(a=>' + B + '<div class="col-md-6"><div class="glass-card p-3 border-start border-primary border-4"><div class="fw-black">' + D + '{a.name}</div><div class="tiny opacity-50 mb-2">' + D + '{a.dept_name||"-"} | ' + D + '{a.skill||"-"}</div><div class="prog-bar"><div class="prog-val" style="width:' + D + '{a.progress||0}%"></div></div><div class="d-flex justify-content-between mt-2"><span class="tiny">' + D + '{a.status}</span><span class="tiny text-primary">' + D + '{a.progress||0}%</span></div></div></div>' + B + ').join("");',
        'document.getElementById("dash-nodes").innerHTML=(data.nodes||[]).map(n=>' + B + '<div class="d-flex justify-content-between align-items-center py-2 border-bottom border-light border-opacity-10"><div><div class="fw-bold">' + D + '{n.name}</div><div class="tiny opacity-50">' + D + '{n.role}</div></div><span class="badge ' + D + '{n.status==="online"?"bg-primary":"bg-secondary"} rounded-pill px-3">' + D + '{n.status}</span></div>' + B + ').join("");',
        '} sync(); setInterval(sync,15000);',
        '</script></body></html>'
    ];
    return lines.join('');
}