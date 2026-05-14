# egghead_service — Specification & Implementation Checklist

## Context
**What:** A paid userscript-generation backend. Users sign in with OIDC, the malpa Chrome extension submits a page's HTML + prompt (+ optional action recording), the backend uses Claude to estimate a price, the user approves on a web dashboard, then Claude generates a ViolentMonkey-compatible userscript. Extension polls for the result and saves it locally.

**Why:** Move AI calls off the browser (where users must manage API keys) onto a managed service with proper auth and billing.

**Stack:** Rust + Axum 0.7 + SQLite (SQLx 0.7) — mirrors kv_manager exactly.  
**Auth:** OIDC via auth.osmosis.page (copied from kv_manager's oidc.rs / session.rs / middleware.rs).  
**AI:** OpenRouter (anthropic/claude-3.5-haiku) with API key fetched from kv.osmosis.page KV store.

## Decisions
- **Approval UX:** Dashboard-only. Extension broadcasts "open dashboard to approve" message while polling. No in-popup approval UI.
- **Action recording:** JSON event log (clicks, inputs, scrolls) captured by a "Record" button added to the extension popup.
- **Billing:** Estimates only — no payment gate. `balance_cents` column exists but is never checked or decremented. Approval just sets `approved_at`.

---

## Project Setup

- [x] `Cargo.toml` — axum 0.7, axum-extra 0.9, tower-http 0.5, tokio full, sqlx 0.7 (sqlite+chrono+uuid), openidconnect 3, reqwest 0.12, serde, sha2, hex, rand, base64, hmac, dotenvy, chrono, uuid (v4+serde), time, url, thiserror, anyhow, tracing, tracing-subscriber
- [x] `.env.example` — template with ANTHROPIC_API_KEY, DATABASE_URL, OIDC config, etc.
- [x] `migrations/0001_init.sql` — users, session_tokens, api_tokens, tasks tables
- [x] `migrations/0002_indexes.sql` — indexes on tasks.user_id, tasks.status, api_tokens.token_hash, session_tokens.token_hash

---

## Core Modules (All Implemented ✓)

### Config & Database
- [x] `src/config.rs` — Config struct + `from_env()`; includes kv_url and kv_api_key for secrets retrieval
- [x] `src/db.rs` — `create_pool()` with WAL + foreign_keys pragmas; run sqlx migrations
- [x] `src/state.rs` — AppState (pool, config, oidc_client Arc<RwLock<Option<CoreClient>>>, http_client)
- [x] `src/error.rs` — AppError enum + IntoResponse

### Auth (OIDC + Sessions + API Tokens)
- [x] `src/auth/mod.rs` — `auth::router()` mounting login + callback + logout
- [x] `src/auth/oidc.rs` — PKCE OIDC flow (copied from kv_manager); callback upserts user into `users` table; redirects to `/dashboard`
- [x] `src/auth/session.rs` — `create_session()`, `validate_session()`, `revoke_session()`
- [x] `src/auth/token.rs` — `generate_api_token()` (egghead_* prefix), `hash_key()`, `generate_session_token()`
- [x] `src/auth/extractors.rs` — `SessionAuth` extractor (cookie or Bearer) + `ApiTokenAuth` extractor (Bearer egghead_*); updates last_used_at on use

### Models
- [x] `src/models/task.rs` — Task struct (sqlx::FromRow), TaskView (lightweight, for API)
- [x] `src/models/user.rs` — User struct

### AI Integration
- [x] `src/ai/client.rs` — OpenRouterClient (reqwest wrapper); `complete(system, user_message, max_tokens) -> String`; posts to openrouter.ai/api/v1/chat/completions with Bearer auth
- [x] `src/ai/secrets.rs` — `fetch_openrouter_key()` fetches `EXTENSION_OPENROUTER_API` from kv.osmosis.page via KV API
- [x] `src/ai/estimate.rs` — `call_estimate()` → EstimateResponse {price_cents, rationale}
- [x] `src/ai/generate.rs` — `call_generate()` → GenerateResponse {name, match_pattern, script_code}; `with_userscript_header()` prepends ==UserScript==

### Background Workers
- [x] `src/worker/mod.rs` — `spawn_all()` launches estimator, generator, cleanup, oidc_retry
- [x] `src/worker/estimator.rs` — polls `status='pending'` every N secs; claims with optimistic UPDATE; calls AI for price; transitions to `awaiting_approval`
- [x] `src/worker/generator.rs` — polls `status='awaiting_approval' AND approved_at IS NOT NULL`; calls AI for script; transitions to `done`
- [x] `src/worker/cleanup.rs` — reset stuck tasks (> 5min in estimating/processing); delete expired sessions
- [x] `src/worker/oidc_retry.rs` — retry OIDC discovery if startup failed

### API Routes
- [x] `src/api/mod.rs` — `api::router()` — all /api/* routes
- [x] `src/api/tasks.rs` — `POST /api/tasks` (ApiTokenAuth), `GET /api/tasks/:id` (ApiTokenAuth, prepends ViolentMonkey header when done)
- [x] `src/api/me.rs` — `GET /api/me/tasks`, `GET /api/me/tasks/:id`, `POST /api/me/tasks/:id/approve`, `POST /api/me/tasks/:id/reject`, `GET /api/me/token`, `POST /api/me/token/regenerate` (all SessionAuth)

### Frontend
- [x] `src/frontend/mod.rs` — `frontend::router()` for HTML pages
- [x] `src/frontend/templates.rs` — inline HTML: landing page, dashboard, settings
- [x] `src/frontend/pages.rs` — `serve_index()`, `serve_dashboard()`, `serve_settings()` handlers; redirect to /auth/login if session invalid

### Security & Middleware
- [x] `src/middleware/mod.rs` + `src/middleware/security_headers.rs` — copy kv_manager security headers

### Main
- [x] `src/main.rs` — router assembly, AppState init, `worker::spawn_all()`, tokio::main, health endpoints

---

## Database Schema

```sql
-- Users (discovered via OIDC)
CREATE TABLE users (
    id            TEXT PRIMARY KEY,
    oidc_subject  TEXT UNIQUE,
    email         TEXT,
    created_at    TEXT DEFAULT datetime('now'),
    balance_cents INTEGER DEFAULT 0
);

-- Browser session tokens (10 hours)
CREATE TABLE session_tokens (
    id           TEXT PRIMARY KEY,
    token_hash   TEXT UNIQUE,
    oidc_subject TEXT,
    email        TEXT,
    expires_at   TEXT,
    created_at   TEXT DEFAULT datetime('now')
);

-- API tokens for malpa extension
CREATE TABLE api_tokens (
    id           TEXT PRIMARY KEY,
    token_hash   TEXT UNIQUE,
    user_id      TEXT REFERENCES users(id) ON DELETE CASCADE,
    label        TEXT DEFAULT 'malpa',
    created_at   TEXT DEFAULT datetime('now'),
    last_used_at TEXT,
    revoked_at   TEXT  -- NULL = active
);

-- Userscript generation tasks
CREATE TABLE tasks (
    id                    TEXT PRIMARY KEY,
    user_id               TEXT REFERENCES users(id) ON DELETE CASCADE,
    tab_url               TEXT,
    prompt                TEXT,
    page_html             TEXT,
    action_recording      TEXT,
    status                TEXT DEFAULT 'pending'
                              CHECK(status IN ('pending', 'estimating', 'awaiting_approval',
                                                'processing', 'done', 'failed', 'rejected')),
    estimated_price_cents INTEGER,
    price_rationale       TEXT,
    approved_at           TEXT,
    rejected_at           TEXT,
    script_name           TEXT,
    script_code           TEXT,
    match_pattern         TEXT,
    error_message         TEXT,
    worker_started_at     TEXT,
    created_at            TEXT DEFAULT datetime('now'),
    updated_at            TEXT DEFAULT datetime('now')
);
```

---

## Task State Machine

```
[POST /api/tasks] → pending
  → (estimator worker claims) → estimating
      → (Claude returns price) → awaiting_approval
          → (user approves) → approved_at set (status stays awaiting_approval)
              → (generator worker claims) → processing
                  → (Claude returns script) → done
          → (user rejects) → rejected
      → (AI/parse error) → failed
  → (network error) → failed
  
Cleanup: stuck in estimating/processing with worker_started_at > 5min → reset to pending
```

---

## REST API

### Auth (no auth required)
- `GET /auth/login` — PKCE OIDC redirect
- `GET /auth/callback` — exchange code, upsert user, set session cookie, redirect to `/dashboard`
- `POST /auth/logout` — delete session, clear cookie (SessionAuth)

### Extension-facing (ApiTokenAuth: `Authorization: Bearer egghead_*`)
- `POST /api/tasks` — body: `{tab_url, prompt, page_html, action_recording?}` → `{id, status}`
- `GET /api/tasks/:id` — returns task; when done includes `script_code` with ==UserScript== header prepended

### Dashboard API (SessionAuth: session cookie or Bearer)
- `GET /api/me/tasks` — list user tasks newest first
- `GET /api/me/tasks/:id` — single task
- `POST /api/me/tasks/:id/approve` — 204 No Content
- `POST /api/me/tasks/:id/reject` — 204 No Content
- `GET /api/me/token` — `{has_token, label, last_used_at, masked_token}`
- `POST /api/me/token/regenerate` — `{token: "egghead_..."}` once

### Frontend (HTML pages; redirect to /auth/login if unauthed)
- `GET /` — landing page with Sign In button
- `GET /dashboard` — task list table; approve/reject buttons; copy-script button
- `GET /settings` — API token display (masked) + Regenerate button

### Health
- `GET /health` → `ok`
- `GET /healthz` → JSON DB health check

---

## AI Prompts

### Estimation (src/ai/estimate.rs)

**Model:** anthropic/claude-3.5-haiku via OpenRouter  
**System:**
> You are a pricing assistant for a userscript generation service. Estimate the cost in US cents.
> Pricing: simple DOM/CSS (5–10¢), moderate automation (10–25¢), complex logic/APIs (25–50¢).
> Respond with ONLY valid JSON: `{"price_cents": <int>, "rationale": "<one sentence>"}`

**User message:** `Page URL: {tab_url}\nUser request: {prompt}\nPage HTML:\n---\n{html}\n---`

### Generation (src/ai/generate.rs)

**Model:** anthropic/claude-3.5-haiku via OpenRouter  
**System:**
> You are an expert JS developer writing ViolentMonkey-compatible userscripts.
> Rules: vanilla JS only; self-contained IIFE; no eval/document.write; no CDN deps; idempotent.
> Do NOT include the ==UserScript== header.
> Respond with ONLY valid JSON: `{"name": "...", "match_pattern": "...", "script_code": "..."}`

**User message:** `Page URL: {tab_url}\nUser request: {prompt}\nPage HTML:\n---\n{html}\n---\nAction recording:\n---\n{recording or "None"}\n---`

**Header prepended at read time (GET /api/tasks/:id when done):**
```
// ==UserScript==
// @name         {script_name}
// @namespace    https://egghead.osmosis.page
// @version      1.0
// @match        {match_pattern}
// @grant        none
// ==/UserScript==
```

---

## Chrome Extension Changes

See `chrome_extension_changes.todo.md` for detailed extension integration checklist.

**TL;DR:**
- Add `handleGenerateViaEgghead()` function to background.js
- Add egghead settings (enabled toggle, API token) to settings page
- Add "Record" button to popup (optional action logging)
- Extend getDOM message handler to return full HTML (not just structural summary)

---

## Verification Checklist

- [ ] `cargo build` succeeds with no errors
- [ ] `DATABASE_URL=sqlite:./test.db sqlx migrate run` applies both migrations cleanly
- [ ] `GET /health` returns `ok`
- [ ] `GET /healthz` returns `{"status":"ok"}`
- [ ] `GET /auth/login` redirects to auth.osmosis.page authorization URL
- [ ] Full OIDC round-trip: user logs in, callback sets session cookie, redirects to /dashboard
- [ ] `/dashboard` loads and shows "No tasks yet" (no session: redirects to login)
- [ ] `POST /api/me/token/regenerate` (with session auth) returns token starting with `egghead_`
- [ ] `POST /api/tasks` (with Bearer token) creates task with status=pending, returns id
- [ ] Worker transitions task to `awaiting_approval` within poll interval (5s by default)
- [ ] `/api/me/tasks` (session auth) lists the created task
- [ ] `POST /api/me/tasks/:id/approve` transitions task (status still awaiting_approval, but approved_at set)
- [ ] Worker transitions approved task to `processing` then `done` (check logs)
- [ ] `GET /api/tasks/:id` when done returns script_code with ==UserScript== header prepended
- [ ] Stuck task cleanup resets estimating/processing tasks after 5 min
- [ ] Extension successfully calls POST /api/tasks, polls GET /api/tasks/:id
- [ ] Task shows in dashboard after approval
- [ ] Reject button transitions task to `rejected`

---

## Key Reference Files (from kv_manager)

- `/var/home/rafa/dev/kv_manager/src/auth/oidc.rs` — OIDC flow (copied)
- `/var/home/rafa/dev/kv_manager/src/auth/middleware.rs` — extractor pattern (adapted)
- `/var/home/rafa/dev/kv_manager/src/auth/session.rs` — session management (copied)
- `/var/home/rafa/dev/kv_manager/src/main.rs` — router assembly pattern
- `/var/home/rafa/dev/malpa/background.js` — extension message protocol
- `/var/home/rafa/dev/malpa/content.js` — getDOM handler (to extend)

---

## Token Formats

- **Session token:** URL_SAFE_NO_PAD base64 (32 bytes). Stored as SHA256 hash in DB. HttpOnly, Secure cookie.
- **API token:** `egghead_` + URL_SAFE_NO_PAD base64 (32 bytes). Stored as SHA256 hash. Bearer header.

---

## Notes

- No bcrypt needed (all tokens use SHA256 hashing).
- CORS: extension service workers can call any URL without CORS if host_permissions allow.
- **API Key Fetch:** On each task (estimate/generate), the worker fetches the latest OpenRouter API key from `kv.osmosis.page/kv/EXTENSION_OPENROUTER_API` using KV_API_KEY (secret). This allows rotating/revoking the key without redeploying.
- OpenRouter calls: non-streaming, single-shot request/response (max_tokens: 256 for estimate, 4096 for generate).
- Database: SQLite with WAL mode, foreign keys enabled, auto-cleanup of expired sessions.
- Workers run as `tokio::spawn` loops inside the same binary (no separate queue/worker process).
- Approval is web-dashboard-only (extension doesn't approve, just polls and notifies user).

## Environment Variables

**Required:**
- `DATABASE_URL` — SQLite connection string
- `KV_API_KEY` — API key for reading secrets from kv.osmosis.page
- `OIDC_ISSUER_URL`, `OIDC_CLIENT_ID`, `OIDC_CLIENT_SECRET`, `OIDC_REDIRECT_URI` — OIDC configuration

**Optional:**
- `LISTEN_ADDR` — bind address (default: 0.0.0.0:3000)
- `KV_URL` — kv.osmosis.page base URL (default: https://kv.osmosis.page)
- `BASE_URL` — egghead service public URL (for redirects/links)
- `SESSION_SIGNING_KEY` — 32-byte hex key for OIDC state cookie; auto-generated if empty
- `WORKER_POLL_INTERVAL_SECS` — background worker poll interval (default: 5)
- `MAX_HTML_BYTES` — max HTML size accepted (default: 150000)
- `LOG_FORMAT` — set to "json" for structured logging
