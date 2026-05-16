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
        .nav-links { display: flex; gap: 1rem; }
        a { color: #2563eb; text-decoration: none; }
        a:hover { text-decoration: underline; }
        button { padding: 0.5rem 1rem; background: #2563eb; color: #fff; border: none; border-radius: 0.25rem; cursor: pointer; }
        button:hover { background: #1d4ed8; }
        table { width: 100%; border-collapse: collapse; margin-top: 1rem; }
        th, td { padding: 0.75rem; text-align: left; border-bottom: 1px solid #333; }
        th { background: #1a1a1a; font-weight: 600; }
        .status { padding: 0.25rem 0.75rem; border-radius: 0.25rem; font-size: 0.875rem; }
        .status.pending { background: #444; }
        .status.estimating { background: #664400; }
        .status.awaiting_approval { background: #664400; }
        .status.processing { background: #664400; }
        .status.done { background: #006622; }
        .status.failed { background: #662222; }
        .status.rejected { background: #444; }
        .actions { display: flex; gap: 0.5rem; }
        .actions button, .actions a { padding: 0.25rem 0.5rem; font-size: 0.875rem; }
        .price { color: #2563eb; font-weight: 600; }
        .loading { text-align: center; padding: 2rem; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>Dashboard</h1>
            <div class="nav-links">
                <a href="/settings">Settings</a>
                <form method="POST" action="/auth/logout" style="margin: 0; display: inline;">
                    <button type="submit">Logout</button>
                </form>
            </div>
        </div>
        <div id="content">
            <div class="loading">Loading tasks...</div>
        </div>
    </div>
    <script>
        async function loadTasks() {
            try {
                const resp = await fetch('/api/me/tasks');
                if (!resp.ok) throw new Error('Failed to load tasks');
                const tasks = await resp.json();

                if (tasks.length === 0) {
                    document.getElementById('content').innerHTML = '<p>No tasks yet. Use the malpa extension to request a userscript.</p>';
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
                        if (task.estimated_token_cost_eur != null) {
                            parts.push(`€${task.estimated_token_cost_eur.toFixed(2)} tokens`);
                        }
                        if (task.estimated_human_hours === 0) {
                            parts.push('(AI will handle)');
                        } else if (task.estimated_human_hours != null) {
                            parts.push(`€${(task.estimated_human_hours * 20).toFixed(0)} labor (${task.estimated_human_hours}h)`);
                        }
                        price = parts.join('<br>');
                    }

                    const deleteBtn = `<button class="btn-delete" onclick="deleteTask('${task.id}')" title="Delete">🗑</button>`;
                    let actions = deleteBtn;
                    if (task.status === 'awaiting_approval') {
                        actions = `<button onclick="approveTask('${task.id}')">Approve</button> <button onclick="rejectTask('${task.id}')">Reject</button> ${deleteBtn}`;
                    } else if (task.status === 'done') {
                        actions = `<button onclick="copyScript('${task.id}')">Copy Script</button> ${deleteBtn}`;
                    }

                    html += `<tr><td>${date}</td><td>${url}</td><td>${task.prompt.slice(0, 30)}...</td><td>${status}</td><td class="price">${price}</td><td class="actions">${actions}</td></tr>`;
                }
                html += '</tbody></table>';
                document.getElementById('content').innerHTML = html;
            } catch (e) {
                document.getElementById('content').innerHTML = `<p style="color: #f44;">Error: ${e.message}</p>`;
            }
        }

        async function approveTask(id) {
            await fetch(`/api/me/tasks/${id}/approve`, { method: 'POST' });
            loadTasks();
        }

        async function rejectTask(id) {
            await fetch(`/api/me/tasks/${id}/reject`, { method: 'POST' });
            loadTasks();
        }

        async function deleteTask(id) {
            if (!confirm('Delete this task?')) return;
            await fetch(`/api/me/tasks/${id}`, { method: 'DELETE' });
            loadTasks();
        }

        async function copyScript(id) {
            const resp = await fetch(`/api/me/tasks/${id}`);
            const task = await resp.json();
            navigator.clipboard.writeText(task.script_code || '');
            alert('Script copied to clipboard!');
        }

        loadTasks();
        setInterval(loadTasks, 5000);
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
