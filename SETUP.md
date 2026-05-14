# egghead_service — Setup & Deployment Guide

Complete guide to initialize, build, and deploy egghead_service.

---

## 📋 Prerequisites

1. **GitHub Account** with kukaraf credentials
2. **Docker & Docker Compose** installed (for deployment)
3. **Git** installed (for repository management)
4. **Rust** (if building locally)

---

## 🚀 Step 1: Initialize Git Repository & Push to GitHub

### Using the Wrapper Script (Easiest)

```bash
cd /var/home/rafa/dev/egghead_service
bash scripts/init-and-push.sh main
```

The script will:
1. Initialize git if needed
2. Stage all files
3. Create initial commit
4. Add remote: `https://github.com/kukaraf/egghead_service.git`
5. Push to `main` branch
6. Print confirmation with next steps

### Manual Setup (Alternative)

```bash
cd /var/home/rafa/dev/egghead_service

# Initialize repo
git init
git config user.email "you@example.com"
git config user.name "Your Name"

# Add all files
git add -A

# Create commit
git commit -m "Initial commit: egghead_service backend"

# Add remote
git remote add origin https://github.com/kukaraf/egghead_service.git

# Push
git branch -M main
git push -u origin main
```

---

## 🐳 Step 2: Configure GitHub Actions

### Enable Container Registry Access

1. Go to: https://github.com/kukaraf/egghead_service
2. Settings → Packages → Grant access to Actions
3. Ensure `GITHUB_TOKEN` has `packages: write` permission (automatic)

### Verify Workflow

1. Go to: https://github.com/kukaraf/egghead_service/actions
2. Click **"Build and Push to GHCR"**
3. Confirm workflow runs on next push

---

## 🔐 Step 3: Configure Environment Secrets

### For Local Development

```bash
cd /var/home/rafa/dev/egghead_service
cp .env.example .env
```

Edit `.env`:
```bash
# OIDC (from auth.osmosis.page)
OIDC_CLIENT_ID=your-client-id
OIDC_CLIENT_SECRET=your-client-secret

# KV (for OpenRouter secret)
KV_API_KEY=your-kv-api-key
KV_URL=https://kv.osmosis.page

# Optional (auto-generated if empty)
SESSION_SIGNING_KEY=
```

### For Docker Deployment

```bash
cp .env.docker .env
# Edit with same values as above
docker-compose up -d
```

---

## 🏗️ Step 4: Verify Build

### Check GitHub Actions Build

```bash
# After push, workflow runs automatically
# Monitor at: https://github.com/kukaraf/egghead_service/actions
```

### Verify Docker Image

Once build completes:

```bash
# Check image exists in GHCR
docker pull ghcr.io/kukaraf/egghead_service:latest

# Or inspect via GitHub
# go to: https://github.com/kukaraf/egghead_service/pkgs/container/egghead_service
```

---

## 🚢 Step 5: Deploy with Docker Compose

### Quick Deploy

```bash
cd /var/home/rafa/dev/egghead_service

# Configure environment
cp .env.docker .env
# Edit .env with your secrets

# Start service
docker-compose up -d

# Check health
curl http://localhost:3000/health
# Should return: ok

# Check logs
docker-compose logs -f egghead
```

### Access Services

- **Web Dashboard:** http://localhost:3000/dashboard
- **API Health:** http://localhost:3000/health
- **API Docs:** See README.md for endpoint list

### Stop Service

```bash
docker-compose down
```

---

## 📦 Step 6: Deploy to Production

### Using Docker Compose (Recommended)

```bash
# On production server
git clone https://github.com/kukaraf/egghead_service.git
cd egghead_service

# Configure
cp .env.docker .env
# Edit .env with production secrets
export $(cat .env | grep -v '#' | xargs)

# Deploy
docker-compose up -d

# Monitor
docker-compose logs -f egghead
```

### With a Reverse Proxy (Caddy)

```caddy
egghead.osmosis.page {
  reverse_proxy localhost:3000 {
    header_up Host {host}
    header_up X-Real-IP {remote}
    header_up X-Forwarded-For {remote}
  }
}
```

Then configure in `.env`:
```bash
BASE_URL=https://egghead.osmosis.page
OIDC_REDIRECT_URI=https://egghead.osmosis.page/auth/callback
```

---

## 🔄 Step 7: Update to New Version

### Pull Latest Image

```bash
docker-compose pull
docker-compose up -d
```

### Update to Specific Version

Edit `docker-compose.yml`:
```yaml
image: ghcr.io/kukaraf/egghead_service:v1.2.3
```

Then:
```bash
docker-compose up -d
```

---

## 🧪 Step 8: Test Integration with malpa

### Setup Extension

1. Open malpa extension settings
2. Enable "Use egghead service (paid)"
3. Paste your API token:
   ```bash
   curl http://localhost:3000/auth/login
   # Login, go to /settings, copy token
   ```

### Test Generation

1. Open any webpage
2. Click malpa icon → describe desired script
3. Submit → see task created
4. Go to http://localhost:3000/dashboard
5. Approve price estimate
6. Wait for generation → copy script to extension

---

## 📋 Checklist

- [ ] Git repo initialized and pushed to origin
- [ ] GitHub Actions workflow enabled
- [ ] Environment secrets configured (.env file)
- [ ] Docker image built successfully
- [ ] Service running: `curl http://localhost:3000/health`
- [ ] Dashboard accessible: http://localhost:3000/dashboard
- [ ] API token generated and saved
- [ ] malpa extension configured with token
- [ ] Test task created and approved
- [ ] Script generated and saved in extension

---

## 🐛 Troubleshooting

### Service Won't Start

```bash
# Check logs
docker-compose logs egghead

# Common issues:
# - Missing env vars: check .env is populated
# - Port 3000 in use: change in docker-compose.yml
# - Database error: check /app/data volume permissions
```

### OIDC Login Fails

- Verify OIDC_CLIENT_ID and OIDC_CLIENT_SECRET are correct
- Verify OIDC_REDIRECT_URI is registered with auth.osmosis.page
- Check OIDC_ISSUER_URL is reachable

### KV Secret Fetch Fails

- Verify KV_API_KEY has read access to `EXTENSION_OPENROUTER_API` key
- Verify KV_URL is reachable (default: https://kv.osmosis.page)
- Check logs: `docker-compose logs egghead`

### Docker Image Not Building

- Check GitHub Actions logs: https://github.com/kukaraf/egghead_service/actions
- Verify Dockerfile is present: `ls Dockerfile`
- Check build cache: `docker builder prune`

---

## 📚 Documentation

- **README.md** — Project overview, quick start, API reference
- **specification.md** — Complete technical architecture
- **DEPLOY.md** — Detailed deployment guide
- **chrome_extension_changes.todo.md** — malpa extension integration
- **.env.example** — All environment variables
- **.env.docker** — Docker-specific template

---

## 🔗 Related Services

- **malpa**: Chrome extension (sends tasks)
- **auth.osmosis.page**: OIDC provider
- **kv.osmosis.page**: Secrets store (OpenRouter API key)
- **OpenRouter**: AI API proxy (Claude via openrouter.ai)

---

## ✅ You're Ready!

After completing all steps:

1. **Service is running** on Docker
2. **Images auto-build** on push via GitHub Actions
3. **Dashboard accessible** for user management
4. **API ready** for malpa extension integration
5. **Scripts generated** via Claude AI

Start by deploying locally, then scale to production with a reverse proxy.

For issues, check **DEPLOY.md** troubleshooting section or logs:
```bash
docker-compose logs -f egghead
```
