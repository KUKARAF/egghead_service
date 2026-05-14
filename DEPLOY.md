# egghead_service Deployment Guide

## Quick Start (Docker Compose)

### 1. Clone and Setup

```bash
git clone https://github.com/kukaraf/egghead_service.git
cd egghead_service
cp .env.docker .env
```

### 2. Configure Environment

Edit `.env` with your secrets:

```bash
# Required
OIDC_CLIENT_ID=your-client-id
OIDC_CLIENT_SECRET=your-client-secret
KV_API_KEY=your-kv-api-key

# Optional (will be auto-generated if empty)
SESSION_SIGNING_KEY=
```

### 3. Run with Docker Compose

```bash
docker-compose up -d
```

The service will be available at `http://localhost:3000`

- Health check: `http://localhost:3000/health`
- Dashboard: `http://localhost:3000/dashboard` (after login)
- Settings: `http://localhost:3000/settings`

### 4. Check Logs

```bash
docker-compose logs -f egghead
```

### 5. Stop Service

```bash
docker-compose down
```

---

## Docker Image Details

**Registry:** ghcr.io/kukaraf/egghead_service  
**Tags:**
- `latest` — points to main branch
- `v1.0.0` — semantic version tags
- `main` — branch tip
- `sha-abc123` — commit SHA

**Build:** Automated on push to main or tag creation via GitHub Actions (`.github/workflows/build-and-push.yml`)

---

## Database

SQLite database is stored in `egghead_data` Docker volume. To inspect:

```bash
docker-compose exec egghead sqlite3 /app/data/egghead.db
```

To backup:

```bash
docker-compose exec egghead cp /app/data/egghead.db /app/data/egghead.db.backup
docker cp egghead_service:/app/data/egghead.db.backup ./
```

---

## Troubleshooting

### Service won't start

Check logs:
```bash
docker-compose logs egghead
```

Common issues:
- Missing env vars: check `.env` is populated
- Port 3000 in use: change ports in docker-compose.yml
- Database permission error: verify volume permissions

### OIDC login fails

- Verify OIDC_CLIENT_ID and OIDC_CLIENT_SECRET are correct
- Check OIDC_REDIRECT_URI matches your registered redirect (must be https in production)
- Check auth.osmosis.page is accessible

### KV key fetch fails

- Verify KV_API_KEY is valid and has access to `EXTENSION_OPENROUTER_API` key
- Check KV_URL is reachable (default: https://kv.osmosis.page)

---

## Production Deployment

### Using a reverse proxy

Example with Caddy (recommended):

```caddy
egghead.osmosis.page {
  reverse_proxy localhost:3000 {
    header_up Host {host}
    header_up X-Real-IP {remote}
    header_up X-Forwarded-For {remote}
  }
}
```

### Using a custom domain

Update in `.env`:
```bash
BASE_URL=https://your-domain.com
OIDC_REDIRECT_URI=https://your-domain.com/auth/callback
```

Re-register OIDC redirect URI with auth.osmosis.page.

### SSL/TLS

Use a reverse proxy (Caddy, nginx, Traefik) in front of Docker Compose to handle SSL.

---

## Updating

### Pull latest image

```bash
docker-compose pull
docker-compose up -d
```

### Update to specific version

Edit `docker-compose.yml`:
```yaml
image: ghcr.io/kukaraf/egghead_service:v1.2.3
```

Then:
```bash
docker-compose up -d
```

---

## GitHub Actions

The `.github/workflows/build-and-push.yml` workflow:

1. Triggers on:
   - Push to main branch
   - Pull requests to main
   - Tag push (v*.*)

2. Builds Docker image with Buildx (multi-platform support)

3. Pushes to ghcr.io if:
   - Not a pull request
   - Has GITHUB_TOKEN (automatic)

4. Tags with:
   - Branch name (e.g., `main`)
   - Semantic version (e.g., `v1.0.0`)
   - Short commit SHA
   - `latest` for default branch

---

## Related Services

- **malpa** (Chrome extension): Sends tasks to egghead_service
- **kv.osmosis.page** (KV store): Stores secrets (OpenRouter API key)
- **auth.osmosis.page** (OIDC provider): User authentication
- **OpenRouter** (AI API): Provides Claude via openrouter.ai

---

## Support

For issues, see specification.md for detailed architecture and troubleshooting guide.
