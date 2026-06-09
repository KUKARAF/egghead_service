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
        .emoji-search { width: 100%; padding: 0.6rem 0.75rem; background: #0a0a0a; border: 1px solid #444; border-radius: 0.25rem; color: #fff; font-size: 1rem; box-sizing: border-box; margin-bottom: 0.75rem; outline: none; }
        .emoji-search:focus { border-color: #2563eb; }
        .emoji-selected { min-height: 3rem; background: #0a0a0a; border: 1px solid #333; border-radius: 0.25rem; display: flex; align-items: center; justify-content: center; gap: 0.25rem; padding: 0.5rem; margin-bottom: 0.75rem; font-size: 1.75rem; letter-spacing: 0.1rem; }
        .emoji-selected .placeholder { color: #555; font-size: 0.875rem; letter-spacing: normal; }
        .emoji-selected span { cursor: pointer; padding: 0.1rem 0.2rem; border-radius: 0.25rem; transition: background 0.1s; }
        .emoji-selected span:hover { background: #333; }
        .emoji-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(2.25rem, 1fr)); gap: 2px; max-height: 200px; overflow-y: auto; background: #0a0a0a; border: 1px solid #333; border-radius: 0.25rem; padding: 0.4rem; margin-bottom: 0.75rem; }
        .emoji-grid button { background: none; border: none; font-size: 1.4rem; cursor: pointer; padding: 0.2rem; border-radius: 0.25rem; line-height: 1; transition: background 0.1s; color: inherit; }
        .emoji-grid button:hover { background: #333; }
        .emoji-hint { color: #666; font-size: 0.8rem; margin-bottom: 0.5rem; text-align: center; }
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
            <div class="tab" onclick="switchTab('sessions')">Device Sessions</div>
            <div class="tab" onclick="switchTab('scripts')">Scripts</div>
            <div class="tab" onclick="switchTab('browsing-sessions')">Browsing Sessions</div>
            <div class="tab" onclick="switchTab('errors')">Errors</div>
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
        <div id="panel-scripts" class="panel">
            <div class="loading">Loading scripts...</div>
        </div>
        <div id="panel-browsing-sessions" class="panel">
            <div class="loading">Loading browsing sessions...</div>
        </div>
        <div id="panel-errors" class="panel">
            <div class="loading">Loading errors...</div>
        </div>
    </div>

    <div class="modal" id="scriptModal">
        <div class="modal-content" style="max-width: 600px;">
            <h3 id="scriptModalTitle">Create Script</h3>
            <div style="display: flex; flex-direction: column; gap: 0.75rem;">
                <div>
                    <label style="display: block; margin-bottom: 0.25rem; font-weight: 600;">Name *</label>
                    <input type="text" id="scriptName" style="width: 100%; padding: 0.5rem; background: #0a0a0a; border: 1px solid #444; border-radius: 0.25rem; color: #fff; box-sizing: border-box;" placeholder="e.g., Dark Mode Enabler" />
                </div>
                <div>
                    <label style="display: block; margin-bottom: 0.25rem; font-weight: 600;">URL *</label>
                    <input type="text" id="scriptUrl" style="width: 100%; padding: 0.5rem; background: #0a0a0a; border: 1px solid #444; border-radius: 0.25rem; color: #fff; box-sizing: border-box;" placeholder="e.g., https://example.com" />
                </div>
                <div>
                    <label style="display: block; margin-bottom: 0.25rem; font-weight: 600;">Prompt *</label>
                    <textarea id="scriptPrompt" style="width: 100%; padding: 0.5rem; background: #0a0a0a; border: 1px solid #444; border-radius: 0.25rem; color: #fff; box-sizing: border-box; min-height: 80px; font-family: monospace;" placeholder="Original prompt used to generate this script..."></textarea>
                </div>
                <div>
                    <label style="display: block; margin-bottom: 0.25rem; font-weight: 600;">Script Code</label>
                    <textarea id="scriptCode" style="width: 100%; padding: 0.5rem; background: #0a0a0a; border: 1px solid #444; border-radius: 0.25rem; color: #fff; box-sizing: border-box; min-height: 150px; font-family: monospace; font-size: 0.85rem;" placeholder="// ==UserScript==&#10;// @name        My Script&#10;// ==/UserScript=="></textarea>
                </div>

            </div>
            <div class="modal-actions">
                <button class="secondary" id="btnCopyHtml" onclick="copyScriptHtml()" style="display:none;" title="Copy the page HTML that was captured when this script was generated">Copy HTML</button>
                <button class="secondary" onclick="closeScriptModal()">Cancel</button>
                <button onclick="saveScript()">Save Script</button>
            </div>
        </div>
    </div>

    <div class="modal" id="assignModal">
        <div class="modal-content">
            <h3>Assign Script to Devices</h3>
            <div id="assignModalContent"></div>
            <div class="modal-actions">
                <button class="secondary" onclick="closeAssignModal()">Cancel</button>
                <button id="assignSaveBtn">Assign</button>
            </div>
        </div>
    </div>

    <div class="modal" id="sessionModal">
        <div class="modal-content" style="max-width: 420px;">
            <h3 id="sessionModalTitle">New Browsing Session</h3>
            <div>
                <label style="display: block; margin-bottom: 0.25rem; font-weight: 600;">Session Name *</label>
                <input type="text" id="sessionNameInput" style="width: 100%; padding: 0.5rem; background: #0a0a0a; border: 1px solid #444; border-radius: 0.25rem; color: #fff; box-sizing: border-box;" placeholder="e.g., Google Testing" onkeydown="if(event.key==='Enter') saveSession()" />
            </div>
            <div class="modal-actions">
                <button class="secondary" onclick="closeSessionModal()">Cancel</button>
                <button onclick="saveSession()">Create</button>
            </div>
        </div>
    </div>

    <div class="modal" id="pageModal">
        <div class="modal-content" style="max-width: 480px;">
            <h3 id="pageModalTitle">Add Page</h3>
            <div>
                <label style="display: block; margin-bottom: 0.25rem; font-weight: 600;">URL *</label>
                <input type="text" id="pageUrlInput" style="width: 100%; padding: 0.5rem; background: #0a0a0a; border: 1px solid #444; border-radius: 0.25rem; color: #fff; box-sizing: border-box;" placeholder="e.g., https://google.com" onkeydown="if(event.key==='Enter') savePage()" />
            </div>
            <div class="modal-actions">
                <button class="secondary" onclick="closePageModal()">Cancel</button>
                <button onclick="savePage()">Save</button>
            </div>
        </div>
    </div>

    <div class="modal" id="emojiModal">
        <div class="modal-content">
            <h3>Confirm Device</h3>
            <p style="margin:0 0 0.75rem;color:#aaa;font-size:0.9rem;">Ask the user which 3 emojis appear on their screen, then select the same ones below.</p>
            <div class="emoji-selected" id="emojiSelected"><span class="placeholder">Select 3 emojis</span></div>
            <input class="emoji-search" id="emojiSearch" type="text" placeholder="Search emojis..." autocomplete="off" />
            <div class="emoji-grid" id="emojiGrid"></div>
            <div class="emoji-hint">Click to add · Click selected emoji to remove</div>
            <div class="modal-actions">
                <button class="secondary" onclick="closeEmojiModal()">Cancel</button>
                <button id="approveBtn" onclick="submitApproval()" disabled>Approve</button>
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
            else if (tab === 'scripts') loadScripts();
            else if (tab === 'browsing-sessions') loadBrowsingSessions();
            else if (tab === 'errors') loadErrors();
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

        // ---- Emoji Picker ----
        let emojiPool = [];
        let selectedEmojis = [];

        async function loadEmojiPool() {
            if (emojiPool.length) return;
            const res = await fetch('/api/devices/emojis');
            emojiPool = await res.json(); // [{e, n}, ...]
        }

        function renderEmojiGrid(filter) {
            const grid = document.getElementById('emojiGrid');
            const term = (filter || '').toLowerCase();
            const matches = term
                ? emojiPool.filter(x => x.n.toLowerCase().includes(term))
                : emojiPool;
            grid.innerHTML = matches.slice(0, 200).map(x =>
                `<button type="button" title="${esc(x.n)}" onclick="pickEmoji('${x.e}')">${x.e}</button>`
            ).join('');
        }

        function renderSelected() {
            const el = document.getElementById('emojiSelected');
            if (selectedEmojis.length === 0) {
                el.innerHTML = '<span class="placeholder">Select 3 emojis</span>';
            } else {
                el.innerHTML = selectedEmojis.map((e, i) =>
                    `<span title="Click to remove" onclick="removeEmoji(${i})">${e}</span>`
                ).join('');
            }
            document.getElementById('approveBtn').disabled = selectedEmojis.length !== 3;
        }

        function pickEmoji(e) {
            if (selectedEmojis.length >= 3) return;
            selectedEmojis.push(e);
            renderSelected();
        }

        function removeEmoji(i) {
            selectedEmojis.splice(i, 1);
            renderSelected();
        }

        async function openEmojiModal(id) {
            pendingApprovalId = id;
            selectedEmojis = [];
            await loadEmojiPool();
            renderSelected();
            renderEmojiGrid('');
            document.getElementById('emojiSearch').value = '';
            document.getElementById('emojiModal').classList.add('active');
            setTimeout(() => document.getElementById('emojiSearch').focus(), 50);
        }

        document.addEventListener('DOMContentLoaded', () => {
            document.getElementById('emojiSearch').addEventListener('input', e => {
                renderEmojiGrid(e.target.value);
            });
            document.getElementById('emojiSearch').addEventListener('keydown', e => {
                if (e.key === 'Enter' && selectedEmojis.length === 3) submitApproval();
            });
        });

        function closeEmojiModal() {
            pendingApprovalId = null;
            document.getElementById('emojiModal').classList.remove('active');
        }

        async function submitApproval() {
            if (selectedEmojis.length !== 3) return;
            const confirm = selectedEmojis.join('');
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
                // Superusers get all sessions via /api/admin/users; others get their own.
                const adminResp = await fetch('/api/admin/users');
                if (adminResp.ok) {
                    const users = await adminResp.json();
                    const sessions = users.flatMap(u =>
                        u.device_sessions.map(s => ({ ...s, owner_email: u.email }))
                    );
                    if (sessions.length === 0) {
                        el.innerHTML = '<p class="empty">No active device sessions.</p>';
                        return;
                    }
                    let html = '<table><thead><tr><th>Device</th><th>Owner</th><th>Created</th><th>Expires</th><th>Last Used</th><th>Actions</th></tr></thead><tbody>';
                    for (const s of sessions) {
                        const created = new Date(s.created_at).toLocaleDateString();
                        const expires = new Date(s.expires_at).toLocaleDateString();
                        const lastUsed = s.last_used_at ? new Date(s.last_used_at).toLocaleDateString() : '—';
                        html += `<tr>
                            <td class="device-name">${esc(s.device_name)}</td>
                            <td class="meta">${esc(s.owner_email)}</td>
                            <td class="meta">${created}</td>
                            <td class="meta">${expires}</td>
                            <td class="meta">${lastUsed}</td>
                            <td><button class="danger" onclick="revokeSession('${s.id}', true)">Revoke</button></td>
                        </tr>`;
                    }
                    html += '</tbody></table>';
                    el.innerHTML = html;
                    return;
                }

                // Normal user fallback.
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

        async function revokeSession(id, isAdmin = false) {
            if (!confirm('Revoke this device session? The device will need to re-register.')) return;
            const url = isAdmin ? `/api/admin/sessions/${id}/revoke` : `/api/me/sessions/${id}/revoke`;
            const resp = await fetch(url, { method: 'POST' });
            if (resp.ok) loadSessions();
            else alert('Failed to revoke session');
        }

        function esc(s) {
            return s.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;').replace(/"/g,'&quot;');
        }

        // ---- Scripts ----
        async function loadScripts() {
            const el = document.getElementById('panel-scripts');
            try {
                const resp = await fetch('/api/me/scripts');
                if (!resp.ok) throw new Error('Failed to load scripts');
                const scripts = await resp.json();

                if (scripts.length === 0) {
                    el.innerHTML = '<p class="empty">No scripts yet. <a href="javascript:openScriptModal()">Create one</a></p>';
                    return;
                }

                let html = '<div style="margin-bottom: 1rem;"><button onclick="openScriptModal()">+ New Script</button></div>';
                html += '<table><thead><tr><th>Name</th><th>URL</th><th>Devices</th><th>Created</th><th>Actions</th></tr></thead><tbody>';
                for (const script of scripts) {
                    const date = new Date(script.created_at).toLocaleDateString();
                    const deviceCount = script.device_ids?.length || 0;
                    const id = script.id;
                    html += '<tr>';
                    html += '<td><strong>' + esc(script.name) + '</strong></td>';
                    html += '<td><small>' + esc(script.url) + '</small></td>';
                    html += '<td>' + deviceCount + ' device(s)</td>';
                    html += '<td class="meta">' + date + '</td>';
                    html += '<td class="actions">';
                    html += '<button onclick="editScript(\'' + id + '\')">Edit</button>';
                    html += '<button onclick="assignDevices(\'' + id + '\')">Assign</button>';
                    html += '<button class="danger" onclick="deleteScript(\'' + id + '\')">Delete</button>';
                    html += '</td>';
                    html += '</tr>';
                }
                html += '</tbody></table>';
                el.innerHTML = html;
            } catch (e) {
                el.innerHTML = '<p style="color:#f44">Error: ' + e.message + '</p>';
            }
        }

        let currentScriptId = null;

        function openScriptModal() {
            currentScriptId = null;
            document.getElementById('scriptName').value = '';
            document.getElementById('scriptPrompt').value = '';
            document.getElementById('scriptUrl').value = '';
            document.getElementById('scriptCode').value = '';
            document.getElementById('scriptModalTitle').textContent = 'Create Script';
            document.getElementById('btnCopyHtml').style.display = 'none';
            document.getElementById('scriptModal').classList.add('active');
        }

        async function editScript(id) {
            try {
                const resp = await fetch(`/api/me/scripts/${id}`);
                if (!resp.ok) throw new Error('Failed to load script');
                const script = await resp.json();
                currentScriptId = id;
                document.getElementById('scriptName').value = script.name;
                document.getElementById('scriptPrompt').value = script.prompt;
                document.getElementById('scriptUrl').value = script.url;
                document.getElementById('scriptCode').value = script.script_code;
                document.getElementById('scriptModalTitle').textContent = 'Edit Script';
                document.getElementById('btnCopyHtml').style.display = '';
                document.getElementById('scriptModal').classList.add('active');
            } catch (e) {
                alert('Error: ' + e.message);
            }
        }

        async function copyScriptHtml() {
            if (!currentScriptId) return;
            const btn = document.getElementById('btnCopyHtml');
            btn.textContent = 'Copying…';
            btn.disabled = true;
            try {
                const resp = await fetch(`/api/me/scripts/${currentScriptId}/html`);
                if (!resp.ok) throw new Error(resp.status === 404 ? 'No captured HTML for this script yet' : 'Failed to fetch HTML');
                const html = await resp.text();
                await navigator.clipboard.writeText(html);
                btn.textContent = 'Copied!';
                setTimeout(() => { btn.textContent = 'Copy HTML'; btn.disabled = false; }, 2000);
            } catch (e) {
                alert(e.message);
                btn.textContent = 'Copy HTML';
                btn.disabled = false;
            }
        }

        function closeScriptModal() {
            document.getElementById('scriptModal').classList.remove('active');
        }

        async function saveScript() {
            const name = document.getElementById('scriptName').value.trim();
            const prompt = document.getElementById('scriptPrompt').value.trim();
            const url = document.getElementById('scriptUrl').value.trim();
            const script_code = document.getElementById('scriptCode').value;

            if (!name || !prompt || !url) {
                alert('Please fill in all required fields');
                return;
            }

            const method = currentScriptId ? 'PUT' : 'POST';
            const endpoint = currentScriptId ? `/api/me/scripts/${currentScriptId}` : '/api/me/scripts';

            try {
                const resp = await fetch(endpoint, {
                    method,
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ name, prompt, url, script_code }),
                });
                if (!resp.ok) throw new Error('Failed to save script');
                closeScriptModal();
                loadScripts();
            } catch (e) {
                alert('Error: ' + e.message);
            }
        }

        async function deleteScript(id) {
            if (!confirm('Delete this script?')) return;
            try {
                const resp = await fetch(`/api/me/scripts/${id}`, { method: 'DELETE' });
                if (!resp.ok) throw new Error('Failed to delete script');
                loadScripts();
            } catch (e) {
                alert('Error: ' + e.message);
            }
        }

        async function assignDevices(scriptId) {
            try {
                let devices = [];
                const adminResp = await fetch('/api/admin/users');
                if (adminResp.ok) {
                    const users = await adminResp.json();
                    devices = users.flatMap(u =>
                        u.device_sessions.map(s => ({ ...s, owner_email: u.email }))
                    );
                } else {
                    const devResp = await fetch('/api/me/sessions');
                    if (!devResp.ok) throw new Error('Failed to load devices');
                    devices = await devResp.json();
                }

                const scriptResp = await fetch(`/api/me/scripts/${scriptId}`);
                if (!scriptResp.ok) throw new Error('Failed to load script');
                const script = await scriptResp.json();

                let html = '<label style="display: block; margin-bottom: 1rem;"><strong>Assign to devices:</strong></label>';
                for (const device of devices) {
                    const checked = script.device_ids.includes(device.id) ? 'checked' : '';
                    const label = device.owner_email
                        ? `${esc(device.device_name)} <small style="color:#888">(${esc(device.owner_email)})</small>`
                        : esc(device.device_name);
                    html += `<label style="display: block; margin-bottom: 0.5rem;">
                        <input type="checkbox" data-device-id="${device.id}" ${checked} />
                        ${label}
                    </label>`;
                }

                const modal = document.getElementById('assignModal');
                document.getElementById('assignModalContent').innerHTML = html;
                document.getElementById('assignSaveBtn').onclick = () => saveAssignments(scriptId);
                modal.classList.add('active');
            } catch (e) {
                alert('Error: ' + e.message);
            }
        }

        function closeAssignModal() {
            document.getElementById('assignModal').classList.remove('active');
        }

        async function saveAssignments(scriptId) {
            const checkboxes = document.querySelectorAll('#assignModalContent input[type="checkbox"]:checked');
            const device_ids = Array.from(checkboxes).map(cb => cb.dataset.deviceId);

            try {
                const resp = await fetch(`/api/me/scripts/${scriptId}/assign-devices`, {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ device_ids }),
                });
                if (!resp.ok) throw new Error('Failed to assign devices');
                closeAssignModal();
                loadScripts();
            } catch (e) {
                alert('Error: ' + e.message);
            }
        }

        // ---- Browsing Sessions ----
        async function loadBrowsingSessions() {
            const el = document.getElementById('panel-browsing-sessions');
            try {
                let sessions;
                const adminResp = await fetch('/api/admin/browsing-sessions');
                if (adminResp.ok) {
                    sessions = await adminResp.json();
                } else {
                    const resp = await fetch('/api/me/browsing-sessions');
                    if (!resp.ok) throw new Error('Failed to load browsing sessions');
                    sessions = await resp.json();
                }

                let html = '<div style="margin-bottom: 1rem; display: flex; gap: 0.5rem; align-items: center;">';
                html += '<button onclick="openNewSessionModal()">+ New Session</button>';
                html += '</div>';

                if (sessions.length === 0) {
                    html += '<p class="empty">No browsing sessions yet.</p>';
                } else {
                    for (const s of sessions) {
                        const date = new Date(s.created_at).toLocaleDateString();
                        html += `<div style="background:#1a1a1a;border-radius:0.5rem;padding:1rem;margin-bottom:1rem;">`;
                        html += `<div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:0.75rem;">`;
                        const ownerTag = s.owner_email ? ` <small style="color:#888;font-weight:normal;">(${esc(s.owner_email)})</small>` : '';
                        html += `<strong style="font-size:1.05rem;">${esc(s.name)}${ownerTag}</strong>`;
                        html += `<div class="actions">`;
                        html += `<span class="meta" style="margin-right:0.5rem;">${date}</span>`;
                        html += `<button class="danger" onclick="deleteSession('${s.id}')">Delete</button>`;
                        html += `</div></div>`;

                        html += `<div style="margin-bottom:0.5rem;">`;
                        if (s.pages.length === 0) {
                            html += `<p class="meta" style="margin:0 0 0.5rem;">No pages yet.</p>`;
                        } else {
                            for (const p of s.pages) {
                                html += `<div style="display:flex;align-items:center;gap:0.5rem;margin-bottom:0.4rem;padding:0.4rem;background:#0a0a0a;border-radius:0.25rem;">`;
                                html += `<span style="flex:1;font-family:monospace;font-size:0.875rem;word-break:break-all;">${esc(p.url)}</span>`;
                                html += `<button class="secondary" onclick="openEditPageModal('${s.id}','${p.id}','${esc(p.url).replace(/'/g,"\\'")}')" style="padding:0.2rem 0.5rem;font-size:0.8rem;">Edit</button>`;
                                html += `<button class="danger" onclick="removePage('${s.id}','${p.id}')" style="padding:0.2rem 0.5rem;font-size:0.8rem;">Remove</button>`;
                                html += `</div>`;
                            }
                        }
                        html += `<button class="secondary" onclick="openAddPageModal('${s.id}')" style="margin-top:0.25rem;">+ Add Page</button>`;
                        html += `</div></div>`;
                    }
                }

                el.innerHTML = html;
            } catch (e) {
                el.innerHTML = `<p style="color:#f44">Error: ${e.message}</p>`;
            }
        }

        async function deleteSession(id) {
            if (!confirm('Delete this browsing session and all its pages?')) return;
            const resp = await fetch(`/api/me/browsing-sessions/${id}`, { method: 'DELETE' });
            if (resp.ok) loadBrowsingSessions();
            else alert('Failed to delete session');
        }

        async function removePage(sessionId, pageId) {
            if (!confirm('Remove this page?')) return;
            const resp = await fetch(`/api/me/browsing-sessions/${sessionId}/pages/${pageId}`, { method: 'DELETE' });
            if (resp.ok) loadBrowsingSessions();
            else alert('Failed to remove page');
        }

        let pendingSessionId = null;
        let pendingPageId = null;

        function openNewSessionModal() {
            document.getElementById('sessionModalTitle').textContent = 'New Browsing Session';
            document.getElementById('sessionNameInput').value = '';
            document.getElementById('sessionModal').classList.add('active');
            setTimeout(() => document.getElementById('sessionNameInput').focus(), 50);
        }

        function closeSessionModal() {
            document.getElementById('sessionModal').classList.remove('active');
        }

        async function saveSession() {
            const name = document.getElementById('sessionNameInput').value.trim();
            if (!name) { alert('Please enter a session name'); return; }
            const resp = await fetch('/api/me/browsing-sessions', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ name }),
            });
            if (resp.ok) { closeSessionModal(); loadBrowsingSessions(); }
            else alert('Failed to create session');
        }

        function openAddPageModal(sessionId) {
            pendingSessionId = sessionId;
            pendingPageId = null;
            document.getElementById('pageModalTitle').textContent = 'Add Page';
            document.getElementById('pageUrlInput').value = '';
            document.getElementById('pageModal').classList.add('active');
            setTimeout(() => document.getElementById('pageUrlInput').focus(), 50);
        }

        function openEditPageModal(sessionId, pageId, url) {
            pendingSessionId = sessionId;
            pendingPageId = pageId;
            document.getElementById('pageModalTitle').textContent = 'Edit Page';
            document.getElementById('pageUrlInput').value = url;
            document.getElementById('pageModal').classList.add('active');
            setTimeout(() => document.getElementById('pageUrlInput').focus(), 50);
        }

        function closePageModal() {
            document.getElementById('pageModal').classList.remove('active');
            pendingSessionId = null;
            pendingPageId = null;
        }

        async function savePage() {
            const url = document.getElementById('pageUrlInput').value.trim();
            if (!url) { alert('Please enter a URL'); return; }
            let resp;
            if (pendingPageId) {
                resp = await fetch(`/api/me/browsing-sessions/${pendingSessionId}/pages/${pendingPageId}`, {
                    method: 'PUT',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ url }),
                });
            } else {
                resp = await fetch(`/api/me/browsing-sessions/${pendingSessionId}/pages`, {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ url }),
                });
            }
            if (resp.ok) { closePageModal(); loadBrowsingSessions(); }
            else alert('Failed to save page');
        }

        // ---- Errors ----
        let errorsAdminMode = false;

        async function loadErrors() {
            const el = document.getElementById('panel-errors');
            try {
                let errors;
                const adminResp = await fetch('/api/admin/errors');
                if (adminResp.ok) {
                    errors = await adminResp.json();
                    errorsAdminMode = true;
                } else {
                    const resp = await fetch('/api/me/errors');
                    if (!resp.ok) throw new Error('Failed to load errors');
                    errors = await resp.json();
                    errorsAdminMode = false;
                }

                if (errors.length === 0) {
                    el.innerHTML = '<p class="empty">No error reports yet.</p>';
                    return;
                }

                const showOwner = errorsAdminMode;
                let html = '<table><thead><tr><th>Session</th>' + (showOwner ? '<th>Owner</th>' : '') + '<th>URL</th><th>Reported</th><th>Actions</th></tr></thead><tbody>';
                for (const e of errors) {
                    const date = new Date(e.reported_at).toLocaleString();
                    const adminFlag = errorsAdminMode ? 'true' : 'false';
                    html += `<tr>
                        <td>${esc(e.session_name)}</td>
                        ${showOwner ? `<td class="meta">${esc(e.owner_email)}</td>` : ''}
                        <td style="font-family:monospace;font-size:0.875rem;word-break:break-all;">${esc(e.url)}</td>
                        <td class="meta">${date}</td>
                        <td class="actions">
                            <button class="secondary" onclick="viewErrorHtml('${e.id}', ${adminFlag})">View HTML</button>
                            <button class="danger" onclick="deleteError('${e.id}')">Delete</button>
                        </td>
                    </tr>`;
                }
                html += '</tbody></table>';
                el.innerHTML = html;
            } catch (e) {
                el.innerHTML = `<p style="color:#f44">Error: ${e.message}</p>`;
            }
        }

        async function deleteError(id) {
            if (!confirm('Delete this error report?')) return;
            const resp = await fetch(`/api/me/errors/${id}`, { method: 'DELETE' });
            if (resp.ok) loadErrors();
            else alert('Failed to delete error report');
        }

        async function viewErrorHtml(errorId, isAdmin) {
            const endpoint = isAdmin ? `/api/admin/errors/${errorId}` : `/api/me/errors/${errorId}`;
            try {
                const resp = await fetch(endpoint);
                if (!resp.ok) throw new Error('Failed to load HTML');
                const html = await resp.text();
                const blob = new Blob([html], { type: 'text/html' });
                const url = URL.createObjectURL(blob);
                window.open(url, '_blank');
            } catch (e) {
                alert('Could not load HTML: ' + e.message);
            }
        }

        // Initial load
        loadTasks();
        setInterval(() => {
            if (currentTab === 'tasks') loadTasks();
            else if (currentTab === 'devices') loadDevices();
            else if (currentTab === 'sessions') loadSessions();
            else if (currentTab === 'scripts') loadScripts();
            else if (currentTab === 'browsing-sessions') loadBrowsingSessions();
            else if (currentTab === 'errors') loadErrors();
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
