# egghead_service

AI-powered userscript generation backend service. Pairs with the **malpa** Chrome extension to provide paid script generation via Claude AI (using OpenRouter and KV secret storage).

## Features

✅ OIDC authentication (auth.osmosis.page)  
✅ OpenRouter Gemma AI integration (secrets from kv.osmosis.page)  
✅ Userscript generation task pipeline  
✅ Background worker system (estimation → approval → generation)  
✅ Web dashboard for price approval / reject / delete  
✅ API token management  
✅ GitHub script storage (generated scripts pushed to a GitHub repo)  
✅ Docker + Docker Compose ready  
✅ GitHub Actions CI/CD (auto-build & push to GHCR)  

## Quick Start

### Using Docker Compose (Recommended)

```bash
# Clone repo
git clone https://github.com/kukaraf/egghead_service.git
cd egghead_service

# Copy and configure environment
cp .env.docker .env
# Edit .env with your secrets:
#   OIDC_CLIENT_ID, OIDC_CLIENT_SECRET
#   KV_API_KEY (for OpenRouter secret)

# Run
docker-compose up -d

# Check health
curl http://localhost:3000/health
```

Visit `http://localhost:3000/dashboard` to access the web interface.

### Local Development (Rust)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Setup
cargo build
cargo sqlx migrate run

# Run (set env vars first)
export DATABASE_URL=sqlite:./egghead.db
export OIDC_ISSUER_URL=https://auth.osmosis.page/application/o/egghead/
export OIDC_CLIENT_ID=your-id
export OIDC_CLIENT_SECRET=your-secret
export KV_API_KEY=your-kv-key

cargo run
```

## Architecture

```
malpa (Chrome ext)
    ↓ POST /api/tasks
egghead_service (REST API)
    ↓
worker::estimator (polls for pending)
    ↓ calls OpenRouter Claude
kv.osmosis.page (fetches API key)
    ↓ returns task with estimated_price_cents
user approves on web dashboard
    ↓
worker::generator (polls for approved)
    ↓ calls OpenRouter Gemma
    ↓ generates full userscript
    ↓ pushes script to GitHub repo (tasks.git_sha stored)
malpa (polls GET /api/tasks/:id)
    ↓ receives script with ViolentMonkey header
    ↓ saves locally + registers in browser
```

## Setup OIDC

Before deploying, register egghead_service with your OIDC provider (auth.osmosis.page):

1. Create a new OIDC client
2. Set the **Redirect URI** to: `https://<your-domain>/auth/callback`
   - For local dev: `http://localhost:3000/auth/callback`
   - For production (userscripts.osmosis.page): `https://userscripts.osmosis.page/auth/callback`
3. Copy the client ID and secret to your `.env` file

## Configuration

**Required Environment Variables:**
```bash
DATABASE_URL                 # sqlite:./egghead.db
OIDC_CLIENT_ID              # from auth.osmosis.page
OIDC_CLIENT_SECRET          # from auth.osmosis.page
OIDC_REDIRECT_URI           # https://<your-domain>/auth/callback
KV_API_KEY                  # API key for kv.osmosis.page
```

**Optional:**
```bash
OIDC_ISSUER_URL             # default: https://auth.osmosis.page/application/o/egghead/
KV_URL                      # default: https://kv.osmosis.page
SESSION_SIGNING_KEY         # auto-generated if empty
WORKER_POLL_INTERVAL_SECS   # default: 5
MAX_HTML_BYTES              # default: 150000
LOG_FORMAT                  # set to "json" for structured logs
```

See `.env.example` and `.env.docker` for full examples.

## API Endpoints

### Auth (OIDC)
- `GET /auth/login` — Start OIDC flow
- `GET /auth/callback` — OIDC callback (redirects to /dashboard)
- `POST /auth/logout` — Logout

### Extension API (Bearer token required: `egghead_*`)
- `POST /api/tasks` — Submit task: `{tab_url, prompt, page_html, action_recording?}`
- `GET /api/tasks/:id` — Poll task status (returns script_code with header when done)

### Dashboard API (Session required)
- `GET /api/me/tasks` — List user tasks
- `GET /api/me/tasks/:id` — Task detail
- `POST /api/me/tasks/:id/approve` — Approve price estimate
- `POST /api/me/tasks/:id/reject` — Reject price
- `DELETE /api/me/tasks/:id` — Delete task
- `GET /api/me/token` — Show API token (masked)
- `POST /api/me/token/regenerate` — Get new token

### Frontend
- `GET /` — Landing page
- `GET /dashboard` — Task management dashboard
- `GET /settings` — API token management

### Health
- `GET /health` → `ok`
- `GET /healthz` → `{"status":"ok"}`

## Deployment

### Quick Start (Local)

```bash
docker-compose up -d
curl http://localhost:3000/health
```

### Production (Remote Server)

For detailed setup with external Caddy reverse proxy, see **DEPLOY_REMOTE.md**.

Quick summary:
```bash
# On remote server
mkdir -p ~/env/osmosis/userscripts/data
cd ~/env/osmosis/userscripts

# Copy files (via scp or git)
git clone https://github.com/KUKARAF/egghead_service.git .
cp .env.prod.example .env

# Fill in secrets (.env)
nano .env
# Required:
#   OIDC_CLIENT_ID
#   OIDC_CLIENT_SECRET
#   KV_API_KEY (for EXTENSION_OPENROUTER_API)
#   SESSION_SIGNING_KEY (or auto-generate: openssl rand -hex 32)

# Start service
docker-compose -f docker-compose.prod.yml up -d

# Verify
curl http://localhost:3000/health
```

### With Caddy Reverse Proxy

On your Caddy instance:
```
userscripts.osmosis.page {
  reverse_proxy 127.0.0.1:3000 {
    header_up X-Real-IP {remote}
    header_up X-Forwarded-For {remote}
    header_up X-Forwarded-Proto https
  }
}
```

Then reload: `caddy reload`

See **DEPLOY.md** for detailed troubleshooting and backup procedures.

### GitHub Actions

Automated builds on:
- Push to `main`
- Tag push (`v*.*.*`)

Images pushed to: `ghcr.io/kukaraf/egghead_service`

Tags:
- `latest` (main branch)
- `main`, `develop` (branch tips)
- `v1.0.0` (semantic versions)
- `sha-abc123def456` (commit SHA)

## Development

### Project Structure

```
src/
  ├── main.rs                # Entry point, router, health checks
  ├── config.rs              # Config from env
  ├── db.rs                  # SQLite pool setup
  ├── error.rs               # Error handling
  ├── state.rs               # AppState
  ├── auth/                  # OIDC, sessions, API tokens, extractors
  ├── models/                # Task, User structs
  ├── api/                   # REST endpoints (tasks, me)
  ├── ai/                    # OpenRouter client, estimate, generate, secrets
  ├── worker/                # Background tasks (estimator, generator, cleanup)
  ├── frontend/              # Landing, dashboard, settings pages
  └── middleware/            # Security headers

migrations/
  ├── 0001_init.sql          # Tables: users, session_tokens, api_tokens, tasks
  └── 0002_indexes.sql       # Indexes
```

### Build & Test

```bash
# Check
cargo check

# Build
cargo build --release

# Run tests
cargo test

# Format
cargo fmt

# Lint
cargo clippy
```

### Database Migrations

```bash
# Run migrations
cargo sqlx migrate run

# Create new migration
cargo sqlx migrate add -r migration_name
# Edit migrations/TIMESTAMP_migration_name.sql
# Run: cargo sqlx migrate run
```

## Specifications

- **Architecture:** See `specification.md` for complete design docs
- **Extension Integration:** See `chrome_extension_changes.todo.md`
- **Deployment:** See `DEPLOY.md`

## Stack

- **Backend:** Rust + Axum 0.7
- **Database:** SQLite with sqlx
- **Auth:** OIDC (auth.osmosis.page) + sessions
- **AI:** OpenRouter (google/gemma-4-31b-it) — key fetched from kv.osmosis.page at runtime
  - KV Key: `EXTENSION_OPENROUTER_API`
- **Script storage:** GitHub Contents API — generated scripts pushed to a GitHub repo
  - KV Key: `GITHUB_TOKEN` (PAT with `repo` write scope)
  - KV Key: `GITHUB_REPO` (e.g. `kukaraf/userscripts`)
  - Commit SHA stored in `tasks.git_sha`; push is non-fatal
- **Secrets:** kv.osmosis.page (runtime secret fetching, no restart required on key rotation)
- **Container:** Docker + Docker Compose
- **CI/CD:** GitHub Actions → ghcr.io

## Related Projects

- **malpa** (Chrome extension): Sends tasks, retrieves scripts
- **kv_manager**: Reference for OIDC & KV patterns
- **auth.osmosis.page**: OIDC provider
- **kv.osmosis.page**: KV store for secrets

## License

MIT

## Support

Issues & questions: GitHub Issues

For detailed troubleshooting, see `DEPLOY.md` and `specification.md`.

---

**Ready to deploy?** Run `bash scripts/init-and-push.sh main` to initialize Git and push to origin.
