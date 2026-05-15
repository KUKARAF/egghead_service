# egghead_service API Examples

Complete workflow examples for submitting userscript generation tasks via the API.

---

## Overview

The egghead_service API has two entry points:

- **Extension API** (`/api/tasks`) — used by the malpa Chrome extension or external clients with API tokens
- **Dashboard API** (`/api/me/*`) — used by the web dashboard with session cookies

All examples use `curl`. Substitute your actual values for `BASE_URL`, `TOKEN`, and `TASK_ID`.

---

## 1. Get Version & Health

### Check service version
```bash
curl http://192.168.1.66:3088/version
# → "v1.2.3" or "dev"
```

### Check health
```bash
curl http://192.168.1.66:3088/health
# → "ok"

curl http://192.168.1.66:3088/healthz | jq
# → {"status":"ok"}
```

---

## 2. Complete Flow: Submit Task → Approve → Get Script

### Step 1: Get an API Token

First, log in via the web dashboard to generate your API token:
```
1. Navigate to: https://userscripts.osmosis.page/
2. Click "Sign In" (OIDC)
3. Go to /settings
4. Copy your API token (format: `egghead_...`)
```

Or regenerate via the API (requires session):
```bash
curl -X POST https://userscripts.osmosis.page/api/me/token/regenerate \
  -H "Content-Type: application/json" \
  -b "session_token=YOUR_SESSION_COOKIE"
# → {"token": "egghead_abc123..."}
```

---

### Step 2: Submit a Task (with files)

Submit a userscript generation request with DOM HTML, raw source files, and a prompt:

```bash
TOKEN="egghead_your_token_here"
BASE_URL="https://userscripts.osmosis.page"

curl -X POST $BASE_URL/api/tasks \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "tab_url": "https://github.com/torvalds/linux",
    "prompt": "Add a dark mode toggle button in the top-right corner",
    "page_html": "<html><head>...</head><body>...</body></html>",
    "files": [
      {
        "name": "app.js",
        "content": "function init() { console.log(\"init\"); }"
      },
      {
        "name": "styles.css",
        "content": "body { color: #333; }"
      }
    ]
  }'
```

**Response (201 Created):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "pending"
}
```

Save the `id` for polling.

---

### Step 3: Poll Task Status (wait for estimate)

The **estimator worker** will process the task and return a cost estimate. Poll every 5 seconds:

```bash
TASK_ID="550e8400-e29b-41d4-a716-446655440000"

curl -X GET "$BASE_URL/api/tasks/$TASK_ID" \
  -H "Authorization: Bearer $TOKEN"
```

**Response while estimating:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "tab_url": "https://github.com/torvalds/linux",
  "prompt": "Add a dark mode toggle button in the top-right corner",
  "status": "estimating",
  "estimated_price_cents": null,
  "created_at": "2026-05-15T12:00:00Z",
  "updated_at": "2026-05-15T12:00:05Z"
}
```

**Response after estimate (status = "awaiting_approval"):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "tab_url": "https://github.com/torvalds/linux",
  "prompt": "Add a dark mode toggle button in the top-right corner",
  "status": "awaiting_approval",
  "estimated_price_cents": 15,
  "price_rationale": "Requires DOM manipulation and event listeners; moderate complexity.",
  "created_at": "2026-05-15T12:00:00Z",
  "updated_at": "2026-05-15T12:00:15Z"
}
```

---

### Step 4a: Approve the Estimate (Dashboard API)

Once you see `status: "awaiting_approval"`, approve via the dashboard API using your session:

```bash
curl -X POST "$BASE_URL/api/me/tasks/$TASK_ID/approve" \
  -H "Content-Type: application/json" \
  -b "session_token=YOUR_SESSION_COOKIE"
# → 204 No Content
```

This triggers the **generator worker** to start generating the script.

---

### Step 4b: Alternative - Approve via UI

Instead of the API, you can approve via the web dashboard:
```
1. Navigate to: https://userscripts.osmosis.page/dashboard
2. Find your task in the list
3. Click "Approve" button
```

---

### Step 5: Poll for Generated Script

Poll the task again. The **generator worker** will process and generate the script:

```bash
curl -X GET "$BASE_URL/api/tasks/$TASK_ID" \
  -H "Authorization: Bearer $TOKEN"
```

**Response while generating:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "processing",
  "estimated_price_cents": 15,
  "created_at": "2026-05-15T12:00:00Z",
  "updated_at": "2026-05-15T12:01:45Z"
}
```

**Response after generation (status = "done"):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "tab_url": "https://github.com/torvalds/linux",
  "prompt": "Add a dark mode toggle button in the top-right corner",
  "status": "done",
  "estimated_price_cents": 15,
  "price_rationale": "Requires DOM manipulation and event listeners; moderate complexity.",
  "script_name": "GitHub Dark Mode Toggle",
  "script_code": "(function() { 'use strict'; const btn = document.createElement('button'); btn.textContent = 'Dark Mode'; btn.style.position = 'fixed'; btn.style.top = '10px'; btn.style.right = '10px'; btn.onclick = () => document.body.style.filter = document.body.style.filter ? '' : 'invert(1)'; document.body.appendChild(btn); })();",
  "match_pattern": "*://github.com/*",
  "git_sha": "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0",
  "created_at": "2026-05-15T12:00:00Z",
  "updated_at": "2026-05-15T12:02:30Z"
}
```

The response includes the full userscript with the `==UserScript==` header prepended automatically.

---

## 3. Managing Your API Token

### Get current token info
```bash
curl -X GET "$BASE_URL/api/me/token" \
  -H "Content-Type: application/json" \
  -b "session_token=YOUR_SESSION_COOKIE"
```

**Response:**
```json
{
  "has_token": true,
  "label": "malpa",
  "last_used_at": "2026-05-15T12:05:00Z",
  "masked_token": "egghead_****abcd"
}
```

### Regenerate token (invalidates old one)
```bash
curl -X POST "$BASE_URL/api/me/token/regenerate" \
  -H "Content-Type: application/json" \
  -b "session_token=YOUR_SESSION_COOKIE"
```

**Response (only shown once):**
```json
{
  "token": "egghead_completely_new_token_here"
}
```

---

## 4. List Your Tasks

```bash
curl -X GET "$BASE_URL/api/me/tasks?page=1&per_page=20" \
  -H "Content-Type: application/json" \
  -b "session_token=YOUR_SESSION_COOKIE"
```

**Response:**
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "tab_url": "https://github.com/torvalds/linux",
    "prompt": "Add a dark mode toggle button in the top-right corner",
    "status": "done",
    "script_name": "GitHub Dark Mode Toggle",
    "estimated_price_cents": 15,
    "created_at": "2026-05-15T12:00:00Z",
    "updated_at": "2026-05-15T12:02:30Z"
  }
]
```

---

## 5. Reject a Price Estimate

If you don't like the quoted price, reject it:

```bash
curl -X POST "$BASE_URL/api/me/tasks/$TASK_ID/reject" \
  -H "Content-Type: application/json" \
  -b "session_token=YOUR_SESSION_COOKIE"
# → 204 No Content
```

Status becomes `"rejected"`. You can submit a new task with a modified prompt to try again.

---

## 6. View Generated Scripts via Filesystem

Generated scripts are stored in a git-managed directory. From the server:

```bash
# List all your generated scripts
ls /app/scripts/{your_user_id}/

# View git history
cd /app/scripts
git log --oneline

# Check out a previous version
git show a1b2c3d4e5f:your_user_id/script_name.user.js
```

---

## Error Responses

### 400 Bad Request
```json
{
  "error": "page_html exceeds 150000 bytes"
}
```

### 401 Unauthorized
```json
{
  "error": "unauthorized"
}
```
Reasons: Missing/invalid Bearer token, expired session.

### 404 Not Found
```json
{
  "error": "not found"
}
```
Reasons: Task ID doesn't exist or belongs to another user.

### 500 Internal Server Error
```json
{
  "error": "internal server error"
}
```
Check server logs. Likely an AI service issue or database error.

---

## State Machine Reference

```
POST /api/tasks
    ↓
status: "pending"
    ↓ (estimator worker claims and processes)
status: "estimating"
    ↓ (Claude returns price estimate)
status: "awaiting_approval"
    ├─→ POST /api/me/tasks/:id/approve
    │       ↓
    │   approved_at set
    │       ↓ (generator worker claims and processes)
    │   status: "processing"
    │       ↓ (Claude generates script)
    │   status: "done"
    │       ↓
    │   GET /api/tasks/:id → returns full script with header
    │
    └─→ POST /api/me/tasks/:id/reject
            ↓
        status: "rejected"
```

---

## Quick Integration Template (JavaScript)

```javascript
const TOKEN = 'egghead_your_token';
const BASE_URL = 'https://userscripts.osmosis.page';

async function submitAndWaitForScript(tabUrl, prompt, pageHtml, files = []) {
  // 1. Submit task
  const createResp = await fetch(`${BASE_URL}/api/tasks`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${TOKEN}`,
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ tab_url: tabUrl, prompt, page_html: pageHtml, files })
  });
  const { id } = await createResp.json();
  console.log('Task created:', id);

  // 2. Poll until status is "awaiting_approval"
  let task;
  while (true) {
    const statusResp = await fetch(`${BASE_URL}/api/tasks/${id}`, {
      headers: { 'Authorization': `Bearer ${TOKEN}` }
    });
    task = await statusResp.json();
    if (task.status === 'awaiting_approval') {
      console.log(`Estimated price: ${task.estimated_price_cents}¢`);
      console.log(`Reason: ${task.price_rationale}`);
      break;
    }
    if (task.status === 'failed') throw new Error(`Estimation failed: ${task.error_message}`);
    await new Promise(r => setTimeout(r, 2000)); // Wait 2s before retry
  }

  // 3. User approves via dashboard (or call approve API with session)
  // For now, we assume approval happens manually
  console.log('Waiting for user approval via dashboard...');
  
  // 4. Poll until status is "done"
  while (true) {
    const statusResp = await fetch(`${BASE_URL}/api/tasks/${id}`, {
      headers: { 'Authorization': `Bearer ${TOKEN}` }
    });
    task = await statusResp.json();
    if (task.status === 'done') {
      console.log(`Script generated: ${task.script_name}`);
      return task.script_code; // Full script with header
    }
    if (task.status === 'failed') throw new Error(`Generation failed: ${task.error_message}`);
    await new Promise(r => setTimeout(r, 2000));
  }
}
```

---

## Testing with cURL Aliases

Add these to your `.bashrc` or `.zshrc` for quick testing:

```bash
alias egghead-version='curl -s http://192.168.1.66:3088/version'
alias egghead-health='curl -s http://192.168.1.66:3088/healthz | jq'
alias egghead-create='curl -X POST https://userscripts.osmosis.page/api/tasks \
  -H "Authorization: Bearer $TOKEN" -H "Content-Type: application/json" -d'
alias egghead-get='curl -X GET "https://userscripts.osmosis.page/api/tasks/$TASK_ID" \
  -H "Authorization: Bearer $TOKEN"'
```

Then:
```bash
export TOKEN="egghead_xyz"
export TASK_ID="550e8400-..."
egghead-version
egghead-health
egghead-get | jq '.status'
```
