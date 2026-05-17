# Remote Deployment Guide: userscripts.osmosis.page

Deploy egghead_service to `rafa@bigboy:~/env/osmosis/userscripts` with external Caddy reverse proxy.

---

## 📋 Prerequisites

- SSH access to `rafa@bigboy`
- Docker & Docker Compose installed on remote server
- External Caddy instance configured at `userscripts.osmosis.page` (reverse proxy)
- OIDC client credentials from `auth.osmosis.page`
- KV API key from `kv.osmosis.page`

---

## 🚀 Deployment Steps

### 1. SSH to Remote Server

```bash
ssh rafa@bigboy
cd ~/env/osmosis/userscripts
```

### 2. Create Directory Structure

```bash
mkdir -p ~/env/osmosis/userscripts
mkdir -p ~/env/osmosis/userscripts/data
cd ~/env/osmosis/userscripts
```

### 3. Copy Files from GitHub

```bash
# Option A: Clone entire repo
git clone https://github.com/kukaraf/egghead_service.git .
cd ~/env/osmosis/userscripts

# Option B: Copy only deployment files
# wget https://raw.githubusercontent.com/kukaraf/egghead_service/main/docker-compose.prod.yml
# wget https://raw.githubusercontent.com/kukaraf/egghead_service/main/.env.prod.example
```

### 4. Configure Secrets

```bash
# Copy template
cp .env.prod.example .env

# Edit with production values
nano .env
```

Fill in:
```bash
OIDC_CLIENT_ID=your-production-id
OIDC_CLIENT_SECRET=your-production-secret
KV_API_KEY=your-kv-api-key
SESSION_SIGNING_KEY=your-32-byte-hex-key
```

Generate SESSION_SIGNING_KEY if needed:
```bash
openssl rand -hex 32
```

### 5. Start Service

```bash
docker-compose -f docker-compose.prod.yml up -d
```

Verify:
```bash
docker-compose -f docker-compose.prod.yml ps
docker-compose -f docker-compose.prod.yml logs -f egghead
```

### 6. Configure External Caddy Reverse Proxy

On your Caddy instance (external to this container), add:

```caddy
userscripts.osmosis.page {
  reverse_proxy 127.0.0.1:3000 {
    header_up Host {host}
    header_up X-Real-IP {remote}
    header_up X-Forwarded-For {remote}
    header_up X-Forwarded-Proto https
  }
}
```

Reload Caddy:
```bash
caddy reload  # or restart depending on your setup
```

### 7. Verify Deployment

```bash
# Local health check
curl http://localhost:3000/health

# Remote health check (through Caddy)
curl https://userscripts.osmosis.page/health

# Check logs
docker-compose -f docker-compose.prod.yml logs -f egghead
```

Expected response: `ok`

---

## 📁 Directory Structure

```
~/env/osmosis/userscripts/
├── docker-compose.prod.yml    # Public config (no secrets)
├── .env                        # Secrets (DO NOT COMMIT)
├── .env.prod.example           # Template for .env
├── data/                       # SQLite database volume
│   └── egghead.db
└── logs/                       # Optional: log rotation
```

---

## 🔄 Updates & Maintenance

### Update to Latest Image

```bash
docker-compose -f docker-compose.prod.yml pull
docker-compose -f docker-compose.prod.yml up -d
```

### Update to Specific Version

Edit `docker-compose.prod.yml`:
```yaml
image: ghcr.io/kukaraf/egghead_service:v1.2.3
```

Then:
```bash
docker-compose -f docker-compose.prod.yml up -d
```

### View Logs

```bash
docker-compose -f docker-compose.prod.yml logs -f egghead
```

### Stop Service

```bash
docker-compose -f docker-compose.prod.yml down
```

Keep data:
```bash
docker-compose -f docker-compose.prod.yml down  # data volume persists
```

### Backup Database

```bash
docker cp egghead_service:/app/data/egghead.db /backups/egghead.db.$(date +%Y%m%d)
```

### Restore Database

```bash
docker cp /backups/egghead.db.YYYYMMDD egghead_service:/app/data/egghead.db
docker-compose -f docker-compose.prod.yml restart egghead
```

---

## 🔐 Secrets Management

### What Goes in `.env` (NEVER commit)
```bash
OIDC_CLIENT_ID
OIDC_CLIENT_SECRET
KV_API_KEY
SESSION_SIGNING_KEY
```

### What Goes in `docker-compose.prod.yml` (OK to commit)
```yaml
environment:
  DATABASE_URL: sqlite:./data/egghead.db
  BASE_URL: https://userscripts.osmosis.page
  OIDC_ISSUER_URL: https://auth.osmosis.page/application/o/egghead/
  KV_URL: https://kv.osmosis.page
  LOG_FORMAT: json
```

### Protect `.env`

```bash
chmod 600 .env
# or
chmod 640 .env  # if group needs read access

# Verify
ls -la .env
```

---

## 🐛 Troubleshooting

### Service Won't Start

```bash
docker-compose -f docker-compose.prod.yml logs egghead

# Common issues:
# - Missing .env file: cp .env.prod.example .env && edit
# - Port 3000 in use: check 'docker ps'
# - Image not found: docker-compose pull
```

### OIDC Login Fails

- Verify OIDC_CLIENT_ID and OIDC_CLIENT_SECRET are correct
- Check OIDC_REDIRECT_URI is registered: `https://userscripts.osmosis.page/auth/callback`
- Verify OIDC_ISSUER_URL is reachable: `curl https://auth.osmosis.page`

### KV Secret Fetch Fails

- Verify KV_API_KEY is correct and has access to `EXTENSION_OPENROUTER_API`
- Check KV is reachable: `curl https://kv.osmosis.page/health`
- Review logs: `docker-compose logs egghead | grep -i kv`

### Caddy Not Proxying

- Verify Caddy is running: `systemctl status caddy` or `caddy version`
- Check Caddy config includes userscripts.osmosis.page section
- Verify service is listening on 127.0.0.1:3000: `curl http://127.0.0.1:3000/health`
- Reload Caddy: `caddy reload` (from Caddy config directory)
- Check Caddy logs: `caddy --version` and logs in `/var/log/caddy/` or journalctl

### Database Issues

```bash
# Inspect SQLite
docker-compose -f docker-compose.prod.yml exec egghead sqlite3 /app/data/egghead.db ".tables"

# Check database size
docker-compose -f docker-compose.prod.yml exec egghead du -h /app/data/egghead.db

# Backup before operations
docker cp egghead_service:/app/data/egghead.db /backups/egghead.db.$(date +%Y%m%d-%H%M%S)
```

---

## 📊 Monitoring

### Health Endpoint

```bash
# Local
curl http://localhost:3000/health

# Remote
curl https://userscripts.osmosis.page/health
```

### Container Status

```bash
docker-compose -f docker-compose.prod.yml ps
docker stats egghead_service
```

### Logs

```bash
# Real-time
docker-compose -f docker-compose.prod.yml logs -f egghead

# Last 100 lines
docker-compose -f docker-compose.prod.yml logs --tail=100 egghead

# With timestamps
docker-compose -f docker-compose.prod.yml logs -f --timestamps egghead
```

### Database Size

```bash
du -h ~/env/osmosis/userscripts/data/egghead.db
```

---

## 🔗 Related Services

- **malpa** (Chrome extension) → sends tasks to egghead_service
- **auth.osmosis.page** → OIDC provider
- **kv.osmosis.page** → secrets storage (OpenRouter API key)
- **Caddy** (external) → HTTPS reverse proxy
- **OpenRouter** → Claude 3.5 Haiku API

---

## ✅ Checklist

- [ ] SSH access verified to rafa@bigboy
- [ ] `~/env/osmosis/userscripts` directory created
- [ ] `docker-compose.prod.yml` copied
- [ ] `.env` configured with production secrets
- [ ] Service started: `docker-compose up -d`
- [ ] Local health check passes: `curl http://localhost:3000/health`
- [ ] Caddy configured and reloaded
- [ ] Remote health check passes: `curl https://userscripts.osmosis.page/health`
- [ ] Dashboard accessible: `https://userscripts.osmosis.page/dashboard`
- [ ] malpa extension configured with API token
- [ ] Test task created and approved

---

## 📞 Support

For issues beyond this guide, see:
- `DEPLOY.md` — General Docker Compose deployment
- `specification.md` — Technical architecture
- `README.md` — API reference
- GitHub Issues: https://github.com/kukaraf/egghead_service/issues
