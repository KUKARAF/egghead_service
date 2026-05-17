pub const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>egghead — AI Userscript Generation</title>
    <style>
        body { margin: 0; padding: 0; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; background: #0f0f0f; color: #fff; }
        .container { max-width: 1200px; margin: 0 auto; padding: 2rem; display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 100vh; text-align: center; }
        h1 { margin: 0; font-size: 3rem; font-weight: 700; margin-bottom: 1rem; }
        p { margin: 0; font-size: 1.25rem; color: #aaa; margin-bottom: 2rem; }
        a.btn { display: inline-block; padding: 0.75rem 2rem; background: #2563eb; color: #fff; text-decoration: none; border-radius: 0.5rem; font-size: 1.1rem; font-weight: 600; transition: background 0.2s; }
        a.btn:hover { background: #1d4ed8; }
    </style>
</head>
<body>
    <div class="container">
        <h1>egghead</h1>
        <p>AI-powered userscript generation for your browser</p>
        <a class="btn" href="/auth/login">Sign In</a>
    </div>
</body>
</html>"#;

pub const DASHBOARD_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Dashboard — egghead</title>
    <style>
        body { margin: 0; padding: 0; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; background: #0f0f0f; color: #fff; }
        .container { max-width: 1200px; margin: 0 auto; padding: 2rem; }
        .header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 2rem; }
        h1 { margin: 0; }
        .nav-links { display: flex; gap: 1rem; align-items: center; }
        a { color: #2563eb; text-decoration: none; }
        a:hover { text-decoration: underline; }
        button { padding: 0.5rem 1rem; background: #2563eb; color: #fff; border: none; border-radius: 0.25rem; cursor: pointer; }
        button:hover { background: #1d4ed8; }
        button.danger { background: #991b1b; }
        button.danger:hover { background: #7f1d1d; }
        button.secondary { background: #444; }
        button.secondary:hover { background: #555; }
        .tabs { display: flex; gap: 0; margin-bottom: 1.5rem; border-bottom: 2px solid #333; }
        .tab { padding: 0.75rem 1.5rem; cursor: pointer; border-bottom: 2px solid transparent; margin-bottom: -2px; color: #aaa; font-weight: 500; }
        .tab.active { color: #fff; border-bottom-color: #2563eb; }
        .tab:hover { color: #fff; }
        .tab-badge { background: #2563eb; color: #fff; font-size: 0.7rem; padding: 0.1rem 0.4rem; border-radius: 0.75rem; margin-left: 0.4rem; vertical-align: middle; }
        .panel { display: none; }
        .panel.active { display: block; }
        table { width: 100%; border-collapse: collapse; margin-top: 1rem; }
        th, td { padding: 0.75rem; text-align: left; border-bottom: 1px solid #333; }
        th { background: #1a1a1a; font-weight: 600; }
        .status { padding: 0.25rem 0.75rem; border-radius: 0.25rem; font-size: 0.875rem; display: inline-block; }
        .status.pending { background: #444; }
        .status.estimating { background: #664400; }
        .status.awaiting_approval { background: #664400; }
        .status.processing { background: #664400; }
        .status.done { background: #006622; }
        .status.failed { background: #662222; }
        .status.rejected { background: #444; }
        .actions { display: flex; gap: 0.5rem; flex-wrap: wrap; }
        .actions button { padding: 0.25rem 0.5rem; font-size: 0.875rem; }
        .price { color: #2563eb; font-weight: 600; }
        .loading { text-align: center; padding: 2rem; color: #aaa; }
        .empty { color: #666; padding: 2rem 0; }
        .emoji-display { font-size: 2rem; letter-spacing: 0.25rem; margin: 1rem 0; }
        .modal { display: none; position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.8); align-items: center; justify-content: center; z-index: 1000; }
        .modal.active { display: flex; }
        .modal-content { background: #1a1a1a; padding: 2rem; border-radius: 0.5rem; max-width: 420px; width: 100%; }
        .modal-content h3 { margin-top: 0; }
        .modal-content input { width: 100%; padding: 0.75rem; background: #0a0a0a; border: 1px solid #444; border-radius: 0.25rem; color: #fff; font-size: 1.5rem; letter-spacing: 0.25rem; text-align: center; box-sizing: border-box; margin-bottom: 1rem; }
        .modal-actions { display: flex; gap: 0.5rem; justify-content: flex-end; margin-top: 1rem; }
        .device-name { font-weight: 600; }
        .meta { color: #888; font-size: 0.875rem; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>Dashboard</h1>
            <div class="nav-links">
                <a href="/settings">Settings</a>
                <form method="POST" action="/auth/logout" style="margin: 0;">
                    <button type="submit">Logout</button>
                </form>
            </div>
        </div>

        <div class="tabs">
            <div class="tab active" onclick="switchTab('tasks')">Tasks</div>
            <div class="tab" onclick="switchTab('devices')" id="tab-devices">Pending Devices</div>
            <div class="tab" onclick="switchTab('sessions')">Sessions</div>
        </div>

        <div id="panel-tasks" class="panel active">
            <div class="loading">Loading tasks...</div>
        </div>
        <div id="panel-devices" class="panel">
            <div class="loading">Loading device requests...</div>
        </div>
        <div id="panel-sessions" class="panel">
            <div class="loading">Loading sessions...</div>
        </div>
    </div>

    <div class="modal" id="emojiModal">
        <div class="modal-content">
            <h3>Confirm Device</h3>
            <p>Ask the user to read the emoji shown on their screen, then enter it below to confirm.</p>
            <input id="emojiInput" type="text" placeholder="😀🎉🚀" autocomplete="off" />
            <div class="modal-actions">
                <button class="secondary" onclick="closeEmojiModal()">Cancel</button>
                <button onclick="submitApproval()">Approve</button>
            </div>
        </div>
    </div>

    <script>
        let currentTab = 'tasks';
        let pendingApprovalId = null;

        function switchTab(tab) {
            document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
            document.querySelectorAll('.panel').forEach(p => p.classList.remove('active'));
            event.target.classList.add('active');
            document.getElementById('panel-' + tab).classList.add('active');
            currentTab = tab;
            if (tab === 'tasks') loadTasks();
            else if (tab === 'devices') loadDevices();
            else if (tab === 'sessions') loadSessions();
        }

        // ---- Tasks ----
        async function loadTasks() {
            const el = document.getElementById('panel-tasks');
            try {
                const resp = await fetch('/api/me/tasks');
                if (!resp.ok) throw new Error('Failed to load tasks');
                const tasks = await resp.json();

                if (tasks.length === 0) {
                    el.innerHTML = '<p class="empty">No tasks yet.</p>';
                    return;
                }

                let html = '<table><thead><tr><th>Date</th><th>URL</th><th>Prompt</th><th>Status</th><th>Price</th><th>Actions</th></tr></thead><tbody>';
                for (const task of tasks) {
                    const date = new Date(task.created_at).toLocaleDateString();
                    const url = new URL(task.tab_url).hostname;
                    const status = `<span class="status ${task.status}">${task.status}</span>`;
                    let price = '—';
                    if (task.estimated_token_cost_eur != null || task.estimated_human_hours != null) {
                        const parts = [];
                        if (task.estimated_token_cost_eur != null) parts.push(`€${task.estimated_token_cost_eur.toFixed(2)} tokens`);
                        if (task.estimated_human_hours === 0) parts.push('(AI will handle)');
                        else if (task.estimated_human_hours != null) parts.push(`€${(task.estimated_human_hours * 20).toFixed(0)} labor (${task.estimated_human_hours}h)`);
                        price = parts.join('<br>');
                    }
                    const deleteBtn = `<button class="danger" onclick="deleteTask('${task.id}')" title="Delete">Delete</button>`;
                    let actions = deleteBtn;
                    if (task.status === 'awaiting_approval') {
                        actions = `<button onclick="approveTask('${task.id}')">Approve</button><button class="secondary" onclick="rejectTask('${task.id}')">Reject</button>${deleteBtn}`;
                    } else if (task.status === 'done') {
                        actions = `<button onclick="copyScript('${task.id}')">Copy Script</button>${deleteBtn}`;
                    }
                    html += `<tr><td>${date}</td><td>${url}</td><td>${task.prompt.slice(0, 40)}</td><td>${status}</td><td class="price">${price}</td><td class="actions">${actions}</td></tr>`;
                }
                html += '</tbody></table>';
                el.innerHTML = html;
            } catch (e) {
                el.innerHTML = `<p style="color:#f44">Error: ${e.message}</p>`;
            }
        }

        async function approveTask(id) { await fetch(`/api/me/tasks/${id}/approve`, { method: 'POST' }); loadTasks(); }
        async function rejectTask(id) { await fetch(`/api/me/tasks/${id}/reject`, { method: 'POST' }); loadTasks(); }
        async function deleteTask(id) {
            if (!confirm('Delete this task?')) return;
            await fetch(`/api/me/tasks/${id}`, { method: 'DELETE' });
            loadTasks();
        }
        async function copyScript(id) {
            const resp = await fetch(`/api/me/tasks/${id}`);
            const task = await resp.json();
            navigator.clipboard.writeText(task.script_code || '');
            alert('Script copied!');
        }

        // ---- Device Requests ----
        async function loadDevices() {
            const el = document.getElementById('panel-devices');
            try {
                const resp = await fetch('/api/me/devices');
                if (!resp.ok) throw new Error('Failed to load device requests');
                const reqs = await resp.json();

                const badge = document.getElementById('tab-devices');
                badge.innerHTML = 'Pending Devices' + (reqs.length > 0 ? ` <span class="tab-badge">${reqs.length}</span>` : '');

                if (reqs.length === 0) {
                    el.innerHTML = '<p class="empty">No pending device registrations.</p>';
                    return;
                }

                let html = '<table><thead><tr><th>Device Name</th><th>Email</th><th>Requested</th><th>Expires</th><th>Actions</th></tr></thead><tbody>';
                for (const r of reqs) {
                    const req = new Date(r.requested_at).toLocaleString();
                    const exp = new Date(r.expires_at).toLocaleTimeString();
                    html += `<tr>
                        <td class="device-name">${esc(r.name)}</td>
                        <td>${esc(r.email)}</td>
                        <td class="meta">${req}</td>
                        <td class="meta">${exp}</td>
                        <td class="actions">
                            <button onclick="openEmojiModal('${r.id}')">Approve</button>
                            <button class="danger" onclick="rejectDevice('${r.id}')">Reject</button>
                        </td>
                    </tr>`;
                }
                html += '</tbody></table>';
                el.innerHTML = html;
            } catch (e) {
                el.innerHTML = `<p style="color:#f44">Error: ${e.message}</p>`;
            }
        }

        function openEmojiModal(id) {
            pendingApprovalId = id;
            document.getElementById('emojiInput').value = '';
            document.getElementById('emojiModal').classList.add('active');
            setTimeout(() => document.getElementById('emojiInput').focus(), 50);
        }

        function closeEmojiModal() {
            pendingApprovalId = null;
            document.getElementById('emojiModal').classList.remove('active');
        }

        async function submitApproval() {
            const confirm = document.getElementById('emojiInput').value.trim();
            if (!confirm) return;
            const resp = await fetch(`/api/me/devices/${pendingApprovalId}/approve`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ confirm }),
            });
            if (resp.ok) {
                closeEmojiModal();
                loadDevices();
            } else {
                const data = await resp.json().catch(() => ({}));
                alert('Approval failed: ' + (data.error || resp.status));
            }
        }

        async function rejectDevice(id) {
            if (!confirm('Reject this device registration?')) return;
            await fetch(`/api/me/devices/${id}/reject`, { method: 'POST' });
            loadDevices();
        }

        // ---- Sessions ----
        async function loadSessions() {
            const el = document.getElementById('panel-sessions');
            try {
                const resp = await fetch('/api/me/sessions');
                if (!resp.ok) throw new Error('Failed to load sessions');
                const sessions = await resp.json();

                if (sessions.length === 0) {
                    el.innerHTML = '<p class="empty">No active device sessions.</p>';
                    return;
                }

                let html = '<table><thead><tr><th>Device</th><th>Created</th><th>Expires</th><th>Last Used</th><th>Actions</th></tr></thead><tbody>';
                for (const s of sessions) {
                    const created = new Date(s.created_at).toLocaleDateString();
                    const expires = new Date(s.expires_at).toLocaleDateString();
                    const lastUsed = s.last_used_at ? new Date(s.last_used_at).toLocaleDateString() : '—';
                    html += `<tr>
                        <td class="device-name">${esc(s.device_name)}</td>
                        <td class="meta">${created}</td>
                        <td class="meta">${expires}</td>
                        <td class="meta">${lastUsed}</td>
                        <td><button class="danger" onclick="revokeSession('${s.id}')">Revoke</button></td>
                    </tr>`;
                }
                html += '</tbody></table>';
                el.innerHTML = html;
            } catch (e) {
                el.innerHTML = `<p style="color:#f44">Error: ${e.message}</p>`;
            }
        }

        async function revokeSession(id) {
            if (!confirm('Revoke this device session? The device will need to re-register.')) return;
            const resp = await fetch(`/api/me/sessions/${id}/revoke`, { method: 'POST' });
            if (resp.ok) loadSessions();
            else alert('Failed to revoke session');
        }

        function esc(s) {
            return s.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;').replace(/"/g,'&quot;');
        }

        // Initial load
        loadTasks();
        setInterval(() => {
            if (currentTab === 'tasks') loadTasks();
            else if (currentTab === 'devices') loadDevices();
            else if (currentTab === 'sessions') loadSessions();
        }, 5000);

        // Poll for pending device badge even when not on that tab
        setInterval(async () => {
            if (currentTab === 'devices') return;
            const resp = await fetch('/api/me/devices').catch(() => null);
            if (!resp || !resp.ok) return;
            const reqs = await resp.json().catch(() => []);
            const badge = document.getElementById('tab-devices');
            badge.innerHTML = 'Pending Devices' + (reqs.length > 0 ? ` <span class="tab-badge">${reqs.length}</span>` : '');
        }, 10000);
    </script>
</body>
</html>"#;

pub const SETTINGS_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Settings — egghead</title>
    <style>
        body { margin: 0; padding: 0; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; background: #0f0f0f; color: #fff; }
        .container { max-width: 600px; margin: 0 auto; padding: 2rem; }
        h1 { margin: 0 0 2rem 0; }
        a { color: #2563eb; text-decoration: none; }
        .section { margin-bottom: 2rem; padding: 1rem; background: #1a1a1a; border-radius: 0.5rem; }
        .section h2 { margin-top: 0; }
        .token-display { padding: 1rem; background: #0a0a0a; border-radius: 0.25rem; font-family: monospace; word-break: break-all; margin-bottom: 1rem; }
        button { padding: 0.75rem 1.5rem; background: #2563eb; color: #fff; border: none; border-radius: 0.25rem; cursor: pointer; font-size: 1rem; }
        button:hover { background: #1d4ed8; }
        .modal { display: none; position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.8); align-items: center; justify-content: center; z-index: 1000; }
        .modal.active { display: flex; }
        .modal-content { background: #1a1a1a; padding: 2rem; border-radius: 0.5rem; max-width: 500px; }
        .modal-content h3 { margin-top: 0; }
        .close { cursor: pointer; float: right; font-size: 1.5rem; }
    </style>
</head>
<body>
    <div class="container">
        <h1><a href="/dashboard">← Dashboard</a></h1>
        <div class="section">
            <h2>API Token</h2>
            <p>Use this token in the malpa extension's settings.</p>
            <div id="token-info">Loading...</div>
            <button onclick="regenerateToken()">Regenerate Token</button>
        </div>
        <div class="section">
            <h2>Extension Setup</h2>
            <ol>
                <li>Open the malpa extension settings</li>
                <li>Check "Use egghead service (paid)"</li>
                <li>Paste your API token above into the "egghead API Token" field</li>
                <li>Save and you're ready to go!</li>
            </ol>
        </div>
    </div>

    <div class="modal" id="tokenModal">
        <div class="modal-content">
            <span class="close" onclick="closeModal()">&times;</span>
            <h3>New API Token</h3>
            <p><strong>Save this token now — you won't see it again!</strong></p>
            <div class="token-display" id="newTokenDisplay"></div>
            <button onclick="copyToken()">Copy Token</button>
            <button onclick="closeModal()">Done</button>
        </div>
    </div>

    <script>
        async function loadTokenInfo() {
            try {
                const resp = await fetch('/api/me/token');
                const data = await resp.json();
                let html = '';
                if (data.has_token) {
                    html = `<p><code>${data.masked_token}</code></p>`;
                    if (data.last_used_at) {
                        html += `<p style="color: #aaa; font-size: 0.875rem;">Last used: ${new Date(data.last_used_at).toLocaleDateString()}</p>`;
                    }
                } else {
                    html = '<p style="color: #aaa;">No token yet. Click "Regenerate" to create one.</p>';
                }
                document.getElementById('token-info').innerHTML = html;
            } catch (e) {
                document.getElementById('token-info').innerHTML = `<p style="color: #f44;">Error: ${e.message}</p>`;
            }
        }

        async function regenerateToken() {
            const resp = await fetch('/api/me/token/regenerate', { method: 'POST' });
            const data = await resp.json();
            document.getElementById('newTokenDisplay').textContent = data.token;
            document.getElementById('tokenModal').classList.add('active');
        }

        function closeModal() {
            document.getElementById('tokenModal').classList.remove('active');
            loadTokenInfo();
        }

        function copyToken() {
            const token = document.getElementById('newTokenDisplay').textContent;
            navigator.clipboard.writeText(token);
            alert('Token copied!');
        }

        loadTokenInfo();
    </script>
</body>
</html>"#;
