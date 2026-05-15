# egghead_service Deployment Guide

## Current Status

- **Latest Code**: Commit `64f8a0e` 
- **Docker Image**: Building in GitHub Actions → ghcr.io/kukaraf/egghead_service:latest
- **Database**: Fresh start (per user request - deletes old data)
- **New Features**: File attachments, git versioning, OpenAPI docs, version endpoint

## Prerequisites

1. **Host**: rafa@bigboy (192.168.1.66)
2. **SSH Access**: Must be able to SSH to deploy
3. **Docker**: Docker and Docker Compose installed on bigboy
4. **Secrets**: `.env` file with production secrets (see `.env.prod.example`)

## Deployment Steps

### Step 1: Connect to Bigboy
```bash
ssh rafa@192.168.1.66
cd /app/egghead
```

### Step 2: Update Code and Configuration
```bash
# Pull latest docker-compose and Dockerfile changes
git pull origin main

# Ensure .env file has production secrets
cat .env  # Verify OIDC_CLIENT_ID, OIDC_CLIENT_SECRET, KV_API_KEY are set
```

### Step 3: Reset Database (Fresh Start)
```bash
# Remove existing data volume
docker volume rm egghead_data

# Remove any existing containers
docker-compose -f docker-compose.prod.yml down -v
```

### Step 4: Pull New Image and Start
```bash
docker-compose -f docker-compose.prod.yml pull
docker-compose -f docker-compose.prod.yml up -d
```

### Step 5: Verify Deployment
```bash
# Wait 10-15 seconds for service to start
sleep 15

# Check health
curl http://localhost:3000/health
# Expected: "ok"

curl http://localhost:3000/healthz | jq .
# Expected: {"status":"ok"}

# Check version
curl http://localhost:3000/version
# Expected: git commit SHA (e.g., "64f8a0e" or full SHA)

# View logs
docker-compose -f docker-compose.prod.yml logs -f egghead
```

### Step 6: Verify API Access
```bash
# From inside bigboy container (or from host if network allows)
curl http://192.168.1.66:3088/api/docs
# Should display Swagger UI

curl http://192.168.1.66:3088/api/openapi.json
# Should return OpenAPI 3.0 JSON spec
```

## What Gets Deployed

### Code Changes in 64f8a0e
- **File Attachments**: POST /api/tasks now accepts `files` array
  ```json
  {
    "tab_url": "...",
    "prompt": "...",
    "page_html": "...",
    "files": [
      {"name": "app.js", "content": "..."},
      {"name": "styles.css", "content": "..."}
    ]
  }
  ```

- **Git Versioning**: Generated scripts stored in `/app/scripts/{user_id}/` with git history
  - Task response includes `git_sha` after generation
  - View git log: `docker exec egghead_service git -C /app/scripts log --oneline`

- **Version Endpoint**: GET /version returns embedded git commit SHA
  - Build time: set by GitHub Actions via `VERSION=${{ github.sha }}` build arg

- **OpenAPI Documentation**:
  - GET /api/openapi.json: Full OpenAPI 3.0 spec
  - GET /api/docs: Swagger UI interface

- **Database Migrations**:
  - 0001_init.sql: Original schema
  - 0002_indexes.sql: Performance indexes
  - 0003_files_git.sql: NEW - files_json and git_sha columns

- **API Examples**: See API_EXAMPLES.md for complete workflow examples

## Debugging

### Database Issues
```bash
# Access SQLite directly
docker exec -it egghead_service sqlite3 ./data/egghead.db

# Check schema
.schema tasks

# Sample query
SELECT id, status, files_json, git_sha FROM tasks LIMIT 5;
```

### Git Operations Issues
```bash
# Check git repo state
docker exec egghead_service ls -la /app/scripts/

# View git log
docker exec egghead_service git -C /app/scripts log --oneline

# Check git config
docker exec egghead_service git -C /app/scripts config --list
```

### Log Inspection
```bash
# Real-time logs
docker-compose -f docker-compose.prod.yml logs -f egghead

# With timestamp and limited lines
docker-compose -f docker-compose.prod.yml logs --timestamps --tail 100 egghead

# JSON formatted logs (search-friendly)
docker-compose -f docker-compose.prod.yml logs egghead 2>&1 | grep worker
```

### Network/DNS Issues
```bash
# Test DNS resolution
docker exec egghead_service nslookup auth.osmosis.page 192.168.1.66

# Test OIDC connectivity
docker exec egghead_service curl -v https://auth.osmosis.page/application/o/userscripts/

# Check if service is listening
docker exec egghead_service netstat -tlnp | grep 3000
```

## Rollback Procedure

If deployment fails:
```bash
# Stop current container
docker-compose -f docker-compose.prod.yml down

# Check available images
docker images | grep egghead_service

# Restart with previous image
docker-compose -f docker-compose.prod.yml up -d

# Check logs
docker-compose -f docker-compose.prod.yml logs egghead
```

## Post-Deployment Testing

### Test Health Checks
```bash
curl http://192.168.1.66:3088/health
curl http://192.168.1.66:3088/healthz
curl http://192.168.1.66:3088/version
```

### Test API with Token
```bash
# Generate a test token (requires OIDC login first)
# Use web dashboard: https://userscripts.osmosis.page/settings

# Sample task submission
curl -X POST http://192.168.1.66:3088/api/tasks \
  -H "Authorization: Bearer egghead_YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "tab_url": "https://example.com",
    "prompt": "Add dark mode",
    "page_html": "<html><body>test</body></html>",
    "files": [
      {"name": "test.js", "content": "console.log(1)"}
    ]
  }'
```

### Monitor Task Lifecycle
```bash
# Watch task processing
docker-compose -f docker-compose.prod.yml logs -f egghead | grep -E "task|estimat|generat"

# Check git commits as scripts are generated
docker exec egghead_service git -C /app/scripts log --oneline --all
```

## Notes

- **Database Volume**: `egghead_data` - persists across restarts
- **Scripts Directory**: `./scripts:/app/scripts` - mounted volume with git repo
- **Port**: Exposed on 192.168.1.66:3088 (configured in docker-compose.prod.yml)
- **Restart Policy**: unless-stopped (auto-restarts after reboot)
- **Logging**: JSON format for structured log aggregation

## Support

See API_EXAMPLES.md for comprehensive API workflow documentation.

For issues, check:
1. Docker logs: `docker-compose logs egghead`
2. Database state: `docker exec egghead_service sqlite3 ./data/egghead.db`
3. Git repo: `docker exec egghead_service ls -la /app/scripts/`
