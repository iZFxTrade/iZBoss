/**
 * iZ.Life BOSS Command Center V25.1.0_OMEGA SOVEREIGN MATRIX
 * 
 * Restored Pre-ChatBoss Version
 * Compatible with Schema: nodes, agents, departments, llm_models, skills_modules, bots, webhooks_feeds
 */

export interface Env {
    DB: D1Database;
    AI: any;
}

export default {
    async fetch(request: Request, env: Env): Promise<Response> {
        const url = new URL(request.url);
        if (url.pathname === "/api/data") return handleDataApi(env);
        if (url.pathname === "/install") return new Response("#!/bin/sh\necho 'iZ.Life BOSS DNA Active'", { headers: { "Content-Type": "text/plain" } });

        return new Response(renderDashboard(), {
            headers: { "Content-Type": "text/html; charset=UTF-8" }
        });
    }
};

async function handleDataApi(env: Env): Promise<Response> {
    try {
        const res = await Promise.all([
            env.DB.prepare("SELECT * FROM nodes").all(),
            env.DB.prepare("SELECT agents.*, llm_models.name as model_name, departments.name as dept_name FROM agents LEFT JOIN llm_models ON agents.model_id = llm_models.id LEFT JOIN departments ON agents.dept_id = departments.id").all(),
            env.DB.prepare("SELECT * FROM departments").all(),
            env.DB.prepare("SELECT * FROM llm_models").all(),
            env.DB.prepare("SELECT * FROM skills_modules").all(),
            env.DB.prepare("SELECT * FROM webhooks_feeds").all(),
        ]);
        return Response.json({
            nodes: res[0].results,
            agents: res[1].results,
            depts: res[2].results,
            models: res[3].results,
            modules: res[4].results,
            feeds: res[5].results,
            core: { version: "v25.1.0", status: "Omega Seed", sync: new Date().toLocaleTimeString("vi-VN") }
        });
    } catch (e: any) {
        return Response.json({ error: e.message }, { status: 500 });
    }
}

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
        '.glass-card{background:var(--card-bg);border:1px solid var(--border);border-radius:1.5rem;transition:0.3s;box-shadow:0 10px 30px rgba(0,0,0,0.05);}.glass-card:hover{border-color:var(--accent); box-shadow:0 12px 40px rgba(25,123,254,0.1);}',
        '.tiny{font-size:0.65rem; text-transform:uppercase; letter-spacing:1px;} .prog-bar{height:6px; background:rgba(255,255,255,0.05); border-radius:10px; overflow:hidden;} .prog-val{height:100%; background:var(--accent); transition:1s;}',
        '</style></head><body>',
        '<div id="sidebar" class="sidebar d-flex flex-column py-4"><div class="px-4 mb-5 text-primary italic fw-black" style="font-size:1.4rem;">iZ.BOSS</div>',
        '<nav class="nav flex-column mt-2">',
        '<a href="#" class="nav-link active"><i class="fas fa-chart-pie me-3"></i><span>Dashboard</span></a>',
        '<a href="#" class="nav-link"><i class="fas fa-server me-3"></i><span>Nodes Fleet</span></a>',
        '<a href="#" class="nav-link"><i class="fas fa-robot me-3"></i><span>Workforce</span></a>',
        '<a href="#" class="nav-link"><i class="fas fa-brain me-3"></i><span>LLM Matrix</span></a>',
        '<a href="#" class="nav-link"><i class="fas fa-sitemap me-3"></i><span>Departments</span></a>',
        '</nav></div>',
        '<main id="main" style="margin-left:var(--side-w);"><header class="navbar-boss px-4 d-flex align-items-center"><div class="logo-center">iZ.BOSS<div style="font-size:0.5rem; font-weight:950; letter-spacing:3px; margin-top:-5px; opacity:0.8;">RESTORED OMEGA CORE</div></div></header>',
        '<div class="container-fluid p-5">',
        '<div class="row g-4 mb-5" id="dash-sum"></div>',
        '<div class="row g-4"><div class="col-lg-8"><div class="glass-card p-4"><h6 class="tiny fw-black mb-4 text-primary">Active Workforce Matrix (V25 Agents)</h6><div id="dash-agents" class="row g-3"></div></div></div>',
        '<div class="col-lg-4"><div class="glass-card p-4"><h6 class="tiny fw-black mb-4 text-primary">Fleet Distribution</h6><div id="dash-nodes" class="vstack gap-3"></div></div></div></div>',
        '</div></main>',
        '<script>',
        'let data = {}; async function sync(){try{const res=await fetch("/api/data");data=await res.json();render();}catch(e){console.error(e);}}',
        'function render(){',
        'const totA=data.agents.length; const actA=data.agents.filter(x=>x.status===' + "'" + 'running' + "'" + '||x.status===' + "'" + 'online' + "'" + ').length;',
        'const totN=data.nodes.length; const actN=data.nodes.filter(x=>x.status===' + "'" + 'online' + "'" + ').length;',
        'document.getElementById("dash-sum").innerHTML = [',
        '{t:"Workforce", v:totA, a:actA, i:"robot"}, {t:"Nodes", v:totN, a:actN, i:"server"}, {t:"Brains", v:data.models.length, a:data.models.length, i:"brain"}',
        '].map(x=>' + B + '<div class="col-md-4"><div class="glass-card p-4"><div class="d-flex justify-content-between align-items-center mb-3"><div class="tiny fw-bold opacity-50">' + D + '{x.t}</div><i class="fas fa-' + D + '{x.i} text-primary"></i></div><div class="row text-center"><div class="col-4"><b>' + D + '{x.v}</b><div class="tiny opacity-50">Total</div></div><div class="col-4 text-primary"><b>' + D + '{x.a}</b><div class="tiny">Active</div></div><div class="col-4 text-danger"><b>' + D + '{x.v-x.a}</b><div class="tiny">Off</div></div></div></div></div>' + B + ').join("");',
        'document.getElementById("dash-agents").innerHTML=(data.agents||[]).map(a=>' + B + '<div class="col-md-6"><div class="glass-card p-3 border-start border-primary border-4 shadow-sm"><div class="fw-black">' + D + '{a.name}</div><div class="tiny opacity-50 mb-2">' + D + '{a.dept_name || a.dept_id} | ' + D + '{a.skill}</div><div class="prog-bar"><div class="prog-val" style="width:' + D + '{a.progress}%"></div></div><div class="d-flex justify-content-between mt-2"><span class="tiny">' + D + '{a.status}</span><span class="tiny text-primary">' + D + '{a.progress}%</span></div></div></div>' + B + ').join("");',
        'document.getElementById("dash-nodes").innerHTML=(data.nodes||[]).map(n=>' + B + '<div class="d-flex justify-content-between align-items-center py-2 border-bottom border-light border-opacity-10"><div><div class="fw-bold">' + D + '{n.name}</div><div class="tiny opacity-50">' + D + '{n.role}</div></div><span class="badge ' + D + '{n.status===' + "'" + 'online' + "'" + '?' + "'" + 'bg-primary' + "'" + ':' + "'" + 'bg-secondary' + "'" + '} rounded-pill px-3">' + D + '{n.status}</span></div>' + B + ').join("");',
        '} sync(); setInterval(sync, 15000);',
        '</script></body></html>'
    ];
    return lines.join('');
}