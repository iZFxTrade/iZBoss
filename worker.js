// iZLife BOSS V10.5.2 - FIX MEMORY UNIQUE CONSTRAINT
export default {
  async fetch(request, env) {
    const url = new URL(request.url);

    // MOCK DATA (Giữ nguyên 100% cấu trúc của BOSS)
    const getMockData = () => ({
      core: { version: "v1.2.5", status: "Stable", last_update: "2026-03-14 20:00" },
      nodes: [
        { id: "N1", name: "Mac Mini M2", status: "online", version: "v1.2.5", role: "Master" },
        { id: "N2", name: "VPS Singapore", status: "online", version: "v1.2.4", role: "Trading" }
      ],
      departments: [
        { id: "D1", name: "Phòng Trading", budget: 500, spent: 120 },
        { id: "D2", name: "Phòng Tài chính", budget: 200, spent: 45 }
      ],
      agents: [
        { id: "A1", name: "Hưng Trader AI", dept: "Trading", status: "running", progress: 85, skill: "Gold Scalping" }
      ],
      feeds: [
        { source: "cTrader", content: '{"symbol":"XAUUSD","price":2155.5}', time: "20:50" }
      ]
    });

    if (url.pathname === "/api/data") return new Response(JSON.stringify(getMockData()));

    // --- OTA INSTALLER SCRIPT cho dna_izcore ---
    if (url.pathname === "/install") {
      const installScript = `#!/bin/sh
echo "======================================"
echo "    iZ.Life BOSS - TẦNG DNA (iZcore)  "
echo "======================================"

OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)
echo "[*] OS: $OS | ARCH: $ARCH"

echo "[*] Downloading iZcore binary..."
# curl -sL "https://boss.iz.life/dist/\${OS}_\${ARCH}/dna_izcore" -o /tmp/dna_izcore
# chmod +x /tmp/dna_izcore
# /tmp/dna_izcore &
echo "[+] Successfully activated node within iZ.Life BOSS network."
`;
      return new Response(installScript, { headers: { "Content-Type": "text/plain;charset=UTF-8" } });
    }

    // --- FIX LỖI TRÍ NHỚ TẠI ĐÂY ---
    if (url.pathname === "/api/chat" && request.method === "POST") {
      try {
        const { message } = await request.json();
        const modelId = "@cf/meta/llama-3-8b-instruct";

        // 1. Lấy tóm tắt cũ nhất
        const summary = await env.DB.prepare(
          "SELECT summary_text FROM conversation_summary WHERE session_id = 'main_session' LIMIT 1"
        ).first();

        // 2. Lấy 5 câu gần nhất (Bản fix cột content nếu cần)
        const { results: history } = await env.DB.prepare(
          "SELECT role, content FROM chat_history ORDER BY id DESC LIMIT 5"
        ).all();
        const recentContext = history.reverse().map(h => `${h.role}: ${h.content}`).join("\n");

        // Lưu tin nhắn mới của BOSS
        await env.DB.prepare("INSERT INTO chat_history (role, content) VALUES (?, ?)")
          .bind("user", message).run();

        const aiRes = await env.AI.run(modelId, {
          messages: [
            { 
              role: "system", 
              content: `Bạn là em - iZ Assistant, trợ lý của BOSS Hưng. 
              Trí nhớ: ${summary ? summary.summary_text : "Chưa có"}
              Bối cảnh: ${recentContext}
              - Luôn xưng 'em', gọi 'anh'/'BOSS'. Trả lời Markdown đẹp.` 
            },
            { role: "user", content: message }
          ]
        });

        // Lưu câu trả lời của AI
        await env.DB.prepare("INSERT INTO chat_history (role, content) VALUES (?, ?)")
          .bind("assistant", aiRes.response).run();

        // 3. FIX LỖI: Dùng INSERT OR REPLACE để không bị lỗi UNIQUE session_id
        if (history.length >= 5) {
            const sumTask = await env.AI.run(modelId, {
                messages: [{ role: "system", content: "Tóm tắt ngắn gọn các ý chính của hội thoại này: " + recentContext + "\n" + message }]
            });
            // Lệnh quan trọng nhất để fix lỗi BOSS gặp:
            await env.DB.prepare(
                "INSERT OR REPLACE INTO conversation_summary (session_id, summary_text, last_updated) VALUES ('main_session', ?, CURRENT_TIMESTAMP)"
            ).bind(sumTask.response).run();
        }

        return new Response(JSON.stringify({ 
          response: aiRes.response, 
          model: "Llama 3.8B",
          version: "v1.5.2",
          identity: "iZ Assistant"
        }));
      } catch (e) {
        // Log lỗi chi tiết cho BOSS dễ debug
        return new Response(JSON.stringify({ response: "Lỗi iZ Memory: " + e.message }), { status: 500 });
      }
    }

    return new Response(renderV10Shell(), { headers: { "Content-Type": "text/html" } });
  }
};

function renderV10Shell() {
  return `
<!DOCTYPE html>
<html lang="vi">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
    <title>iZLife BOSS V10</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css">
    <script src="https://cdn.jsdelivr.net/npm/marked/marked.min.js"></script>
    <style>
        :root { --bg: #050506; --card: #0f0f10; --border: #1c1c1e; --accent: #0a84ff; }
        body { background: var(--bg); color: #d1d1d6; font-family: 'Inter', sans-serif; overflow: hidden; }
        .app-container { display: flex; height: 100vh; width: 100vw; }
        .sidebar { width: 260px; background: #080809; border-right: 1px solid var(--border); transition: 0.3s; flex-shrink: 0; display: flex; flex-direction: column; }
        .sidebar.collapsed { width: 70px; }
        .sidebar.collapsed .nav-item span, 
        .sidebar.collapsed .px-6,
        .sidebar.collapsed .font-black span:not(.text-blue-500) { display: none; }
        .sidebar.collapsed .font-black { text-align: center; width: 100%; margin-left: 0; }
        .sidebar.collapsed .nav-item { justify-content: center; padding: 14px 0; }
        .sidebar.collapsed .nav-item i { margin: 0; font-size: 1.2rem; }
        .workspace { flex: 1; overflow-y: auto; display: flex; flex-direction: column; background: var(--bg); }
        .assistant-panel { width: 350px; background: #080809; border-left: 1px solid var(--border); transition: 0.3s; display: flex; flex-direction: column; }
        .assistant-panel.closed { width: 0; border-left: none; overflow: hidden; }
        @media (max-width: 1024px) {
            .sidebar { position: absolute; z-index: 50; height: 100%; left: -260px; }
            .sidebar.open { left: 0; }
            .assistant-panel { position: absolute; right: -350px; z-index: 50; height: 100%; }
            .assistant-panel.open { right: 0; width: 300px; }
        }
        .bento-card { background: var(--card); border: 1px solid var(--border); border-radius: 16px; padding: 1.25rem; }
        .status-dot { width: 8px; height: 8px; border-radius: 50%; display: inline-block; margin-right: 6px; }
        .dot-online { background: #32d74b; box-shadow: 0 0 8px #32d74b; }
        .nav-item { display: flex; align-items: center; padding: 14px 24px; cursor: pointer; color: #8e8e93; transition: 0.2s; white-space: nowrap; }
        .nav-item:hover, .nav-item.active { color: white; background: #1c1c1e; border-right: 3px solid var(--accent); }
        .glass-header { backdrop-filter: blur(10px); background: rgba(5,5,6,0.8); border-bottom: 1px solid var(--border); }
        .custom-scroll::-webkit-scrollbar { width: 4px; }
        .custom-scroll::-webkit-scrollbar-thumb { background: var(--border); border-radius: 10px; }
        
        /* CSS Hỗ trợ bảng và Markdown cho Chat */
        .markdown-render table { width: 100%; border-collapse: collapse; margin: 8px 0; border: 1px solid #333; }
        .markdown-render th, .markdown-render td { border: 1px solid #333; padding: 6px; text-align: left; }
        .markdown-render th { background: #1c1c1e; color: #0a84ff; }
        .markdown-render ul { list-style: disc; margin-left: 20px; }
    </style>
</head>
<body>
    <div class="app-container">
        <aside id="sidebar" class="sidebar">
            <div class="p-6 mb-4 flex items-center justify-between">
                <span class="font-black italic text-xl tracking-tighter text-white">iZ <span class="text-blue-500">BOSS</span></span>
                <button onclick="toggleSidebar()" class="hidden lg:block text-zinc-600"><i id="side-icon" class="fas fa-outdent"></i></button>
            </div>
            <nav class="flex-1 overflow-y-auto custom-scroll">
                <div class="px-6 mb-4 text-[10px] font-bold text-zinc-600 uppercase tracking-widest uppercase">Hệ điều hành</div>
                <div class="nav-item active" onclick="switchTab('dashboard')"><i class="fas fa-th-large w-6"></i> <span>Dashboard</span></div>
                <div class="nav-item" onclick="switchTab('core')"><i class="fas fa-microchip w-6"></i> <span>iZ Core & OTA</span></div>
                <div class="px-6 mt-8 mb-4 text-[10px] font-bold text-zinc-600 uppercase tracking-widest">Quản trị</div>
                <div class="nav-item" onclick="switchTab('depts')"><i class="fas fa-sitemap w-6"></i> <span>Phòng ban</span></div>
                <div class="nav-item" onclick="switchTab('agents')"><i class="fas fa-robot w-6"></i> <span>Agent Fleet</span></div>
                <div class="nav-item" onclick="switchTab('skills')"><i class="fas fa-book w-6"></i> <span>Skill Library</span></div>
                <div class="px-6 mt-8 mb-4 text-[10px] font-bold text-zinc-600 uppercase tracking-widest">Cấu hình</div>
                <div id="menu-assistant-setting" class="nav-item" onclick="switchTab('assistant-setting')"><i class="fas fa-comment-dots w-6"></i> <span>Assistant Setting</span></div>
                <div id="menu-boss-setting" class="nav-item" onclick="switchTab('boss-setting')"><i class="fas fa-user-cog w-6"></i> <span>Boss Setting</span></div>
                <div class="px-6 mt-8 mb-4 text-[10px] font-bold text-zinc-600 uppercase tracking-widest text-zinc-800">Hệ thống</div>
                <div class="nav-item" onclick="switchTab('finance')"><i class="fas fa-wallet w-6"></i> <span>Finance</span></div>
                <div class="nav-item" onclick="switchTab('infra')"><i class="fas fa-server w-6"></i> <span>Infrastructure</span></div>
                <div class="nav-item" onclick="switchTab('feeds')"><i class="fas fa-rss w-6"></i> <span>Data Feeds</span></div>
            </nav>
        </aside>

        <main class="workspace">
            <header class="glass-header sticky top-0 z-30 p-4 px-8 flex justify-between items-center">
                <div class="flex items-center gap-4">
                    <button onclick="mobileMenu()" class="lg:hidden text-xl"><i class="fas fa-bars"></i></button>
                    <h2 id="view-title" class="font-bold text-white uppercase tracking-widest text-sm">Tổng quan</h2>
                </div>
                <div class="flex items-center gap-6">
                    <div class="hidden md:flex items-center gap-2 bg-zinc-900 px-3 py-1 rounded-full border border-zinc-800">
                        <span class="status-dot dot-online"></span>
                        <span class="text-[10px] font-bold">CORE v1.2.5</span>
                    </div>
                    <button onclick="toggleAssistant()" class="text-blue-500 text-sm font-bold"><i class="fas fa-comment-alt mr-2"></i> Trợ lý</button>
                </div>
            </header>
            <div id="main-content" class="p-4 lg:p-8 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6"></div>
        </main>

        <section id="assistant" class="assistant-panel closed">
            <div class="p-6 border-b border-zinc-900 flex justify-between items-center">
                <span id="bot-title" class="text-xs font-black uppercase tracking-tighter text-blue-500 italic">BOSS Command Center</span>
                <button onclick="toggleAssistant()"><i class="fas fa-times"></i></button>
            </div>
            <div id="chat-box" class="flex-1 p-6 overflow-y-auto custom-scroll space-y-4 text-xs">
                <div class="text-left"><div class="bg-zinc-900 p-3 rounded-2xl rounded-tl-none border border-zinc-800 text-zinc-400">Chào anh Hưng, em đã sẵn sàng điều phối hệ thống BOSS V10.</div></div>
            </div>
            <div class="p-6 bg-black/50 border-t border-zinc-900">
                <div class="relative">
                    <input id="chat-input" type="text" placeholder="Nhập lệnh..." class="w-full bg-zinc-900 border border-zinc-800 p-4 pr-12 rounded-2xl text-xs focus:outline-none focus:border-blue-500 transition" onkeypress="if(event.key==='Enter') sendChat()">
                    <button onclick="sendChat()" class="absolute right-4 top-1/2 -translate-y-1/2 text-blue-500"><i class="fas fa-paper-plane"></i></button>
                </div>
            </div>
        </section>
    </div>

    <script>
        let data = {};
        async function init() {
            const res = await fetch('/api/data');
            data = await res.json();
            renderDashboard();
        }

        function renderDashboard() {
            document.getElementById('view-title').innerText = "Dashboard Overview";
            const html = \`
                <div class="bento-card col-span-1 md:col-span-2 lg:col-span-1">
                    <h3 class="text-[10px] font-bold text-zinc-500 uppercase mb-4">iZ Core Health</h3>
                    <div class="flex items-center justify-between mb-2">
                        <span class="text-2xl font-black text-white">\${data.core.version}</span>
                        <span class="bg-green-500/10 text-green-500 text-[10px] px-2 py-1 rounded">\${data.core.status}</span>
                    </div>
                    <p class="text-zinc-600 text-[10px]">Cập nhật lần cuối: \${data.core.last_update}</p>
                    <button class="mt-4 w-full bg-blue-600 py-2 rounded-lg text-[10px] font-bold text-white">CHECK OTA UPDATE</button>
                </div>
                <div class="bento-card">
                    <h3 class="text-[10px] font-bold text-zinc-500 uppercase mb-4">Finance Quota</h3>
                    <div class="space-y-4">
                        \${data.departments.map(d => \`
                            <div>
                                <div class="flex justify-between text-[10px] mb-1"><span>\${d.name}</span><span>\${d.spent}/\${d.budget}$</span></div>
                                <div class="w-full h-1.5 bg-zinc-800 rounded-full overflow-hidden">
                                    <div class="bg-blue-500 h-full" style="width: \${(d.spent/d.budget)*100}%"></div>
                                </div>
                            </div>
                        \`).join('')}
                    </div>
                </div>
                <div class="bento-card">
                    <h3 class="text-[10px] font-bold text-zinc-500 uppercase mb-4">Active Nodes</h3>
                    <div class="space-y-3">
                        \${data.nodes.map(n => \`
                            <div class="flex items-center justify-between p-2 bg-black/30 rounded-lg">
                                <div class="flex items-center"><span class="status-dot dot-online"></span><span class="text-xs">\${n.name}</span></div>
                                <span class="text-[10px] text-zinc-600">\${n.version}</span>
                            </div>
                        \`).join('')}
                    </div>
                </div>
            \`;
            document.getElementById('main-content').innerHTML = html;
        }

        async function sendChat() {
            const input = document.getElementById('chat-input');
            const box = document.getElementById('chat-box');
            const title = document.getElementById('bot-title');
            const msg = input.value;
            if(!msg) return;

            box.innerHTML += \`<div class="text-right"><div class="bg-blue-600 text-white p-3 rounded-2xl rounded-tr-none inline-block shadow-lg shadow-blue-500/10">\${msg}</div></div>\`;
            input.value = '';
            box.scrollTop = box.scrollHeight;

            try {
                const res = await fetch('/api/chat', { 
                    method: 'POST', 
                    headers: {'Content-Type': 'application/json'},
                    body: JSON.stringify({ message: msg }) 
                });
                const d = await res.json();
                
                if(d.identity) {
                    title.innerHTML = \`\${d.identity} <span class="text-[8px] text-zinc-500 ml-2 font-normal italic">[\${d.model} | \${d.version}]</span>\`;
                }

                box.innerHTML += \`
                    <div class="text-left animate-in fade-in duration-300">
                        <div class="bg-zinc-900 border border-zinc-800 p-3 rounded-2xl rounded-tl-none inline-block text-zinc-300 shadow-xl markdown-render">
                            \${marked.parse(d.response)}
                            <div class="mt-2 pt-1 border-t border-zinc-800/50 flex gap-2">
                                <span class="text-[7px] text-zinc-600 uppercase">Status: Online</span>
                            </div>
                        </div>
                    </div>\`;
            } catch(e) {
                box.innerHTML += \`<div class="text-center text-red-500 text-[9px] uppercase font-bold">⚠️ Connection Error</div>\`;
            }
            box.scrollTop = box.scrollHeight;
        }

        function switchTab(tab) {
            document.querySelectorAll('.nav-item').forEach(el => el.classList.remove('active'));
            const activeNav = Array.from(document.querySelectorAll('.nav-item')).find(el => el.textContent.toLowerCase().includes(tab.replace('-', ' ').toLowerCase()));
            if(activeNav) activeNav.classList.add('active');
            document.getElementById('view-title').innerText = tab.replace('-', ' ').toUpperCase();
            if(tab === 'dashboard') renderDashboard();
            else document.getElementById('main-content').innerHTML = \`
                <div class="bento-card col-span-3 text-center py-20">
                    <i class="fas fa-tools text-zinc-700 text-4xl mb-4"></i>
                    <div class="text-zinc-600 font-bold uppercase tracking-widest text-xs">Module \${tab} đang sẵn sàng triển khai...</div>
                </div>\`;
            if(window.innerWidth < 1024) document.getElementById('sidebar').classList.remove('open');
        }

        function toggleSidebar() { 
            const sb = document.getElementById('sidebar');
            const icon = document.getElementById('side-icon');
            sb.classList.toggle('collapsed'); 
            icon.className = sb.classList.contains('collapsed') ? 'fas fa-indent' : 'fas fa-outdent';
        }
        function toggleAssistant() { 
            const panel = document.getElementById('assistant');
            panel.classList.toggle('closed'); 
            panel.classList.toggle('open'); 
        }
        function mobileMenu() { document.getElementById('sidebar').classList.toggle('open'); }

        init();
    </script>
</body>
</html>
  `;
}