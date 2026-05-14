#!/bin/bash
# Initialize git repo and push to KUKARAF GitHub account
# Usage: bash scripts/init-and-push.sh [branch-name]

set -e

BRANCH=${1:-main}
REPO_NAME="egghead_service"
GITHUB_USER="kukaraf"

echo "🚀 Initializing egghead_service git repository..."
echo ""

# Check if git is initialized
if [ -d .git ]; then
    echo "⚠️  Git repository already initialized"
    echo "Checking remote..."
    if git remote get-url origin &>/dev/null; then
        REMOTE=$(git remote get-url origin)
        echo "Current origin: $REMOTE"
        read -p "Continue with existing repo? (y/n) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "Aborted."
            exit 1
        fi
    fi
else
    echo "Initializing new git repository..."
    git init
    echo "✅ Repository initialized"
fi

# Add all files
echo ""
echo "Staging files..."
git add -A
echo "✅ Files staged"

# Create initial commit if needed
if ! git diff-index --quiet HEAD --; then
    echo ""
    echo "Creating initial commit..."
    git commit -m "Initial commit: egghead_service backend

- OIDC authentication via auth.osmosis.page
- OpenRouter integration for Anthropic Claude via KV secrets
- Userscript generation task system
- Background workers for estimation and generation
- Web dashboard and API endpoints
- Docker build and compose configuration
- GitHub Actions CI/CD pipeline"
    echo "✅ Commit created"
else
    echo "No changes to commit"
fi

# Set branch
echo ""
echo "Ensuring branch is '$BRANCH'..."
if git symbolic-ref --short HEAD | grep -q "$BRANCH"; then
    echo "✅ Already on branch '$BRANCH'"
else
    git checkout -b "$BRANCH" 2>/dev/null || git checkout "$BRANCH"
    echo "✅ Switched to branch '$BRANCH'"
fi

# Add/update remote
echo ""
echo "Setting up remote..."
REMOTE_URL="https://github.com/$GITHUB_USER/$REPO_NAME.git"

if git remote get-url origin &>/dev/null; then
    CURRENT_REMOTE=$(git remote get-url origin)
    if [ "$CURRENT_REMOTE" != "$REMOTE_URL" ]; then
        echo "Updating remote from: $CURRENT_REMOTE"
        git remote set-url origin "$REMOTE_URL"
    fi
else
    git remote add origin "$REMOTE_URL"
fi

echo "✅ Remote set to: $REMOTE_URL"

# Summary before push
echo ""
echo "📋 Summary:"
echo "  Repository: $REPO_NAME"
echo "  Remote: $REMOTE_URL"
echo "  Branch: $BRANCH"
echo "  Commits: $(git rev-list --count HEAD)"
echo ""

# Confirm before push
read -p "Ready to push to origin/$BRANCH? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted. Commits are local only."
    exit 0
fi

# Push to remote
echo ""
echo "Pushing to GitHub..."
git push -u origin "$BRANCH"

echo ""
echo "✅ Successfully pushed to https://github.com/$GITHUB_USER/$REPO_NAME/tree/$BRANCH"
echo ""
echo "🎉 Next steps:"
echo "  1. Enable GitHub Actions in repository settings"
echo "  2. Ensure repository is public or enable packages for private repos"
echo "  3. Deploy with: docker-compose -f docker-compose.yml up -d"
echo ""
echo "For detailed deployment guide, see DEPLOY.md"
