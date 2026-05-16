# TODO

## AI-via-PR (future)

When estimation returns `human_hours = 0`, the AI should autonomously handle the task without requiring a human developer.

**Trigger:** User approves a task where `estimated_human_hours = 0`.

**Implementation notes:**
- Generator worker detects `human_hours = 0` from the task row and enters AI mode
- Instead of a direct PUT commit, open a GitHub Pull Request:
  - Create branch `ai/{task_id}` from main
  - Commit files: `{user}/{site}/{script_name}.user.js` + `.toml` (status = "pr_open")
  - Open PR with title = script_name and body = original prompt + rationale
- Requires `GITHUB_TOKEN` with `pull_requests: write` scope (see `GH_token.md`)
- Task status transitions: `approved` → `processing` → `pr_open` (new status value needed)
- Add `pr_open` to the status CHECK constraint in a new migration
- TOML file updated to `status = "merged"` once PR is merged (via webhook or periodic polling)
- The service should expose a webhook endpoint for GitHub merge events to auto-close the task
