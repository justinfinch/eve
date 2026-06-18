#!/usr/bin/env bash
# Snapshot the devbox/direnv environment so Claude Code's (non-interactive)
# Bash tool sees packages declared in devbox.json. Fires on SessionStart and
# CwdChanged. Context: anthropics/claude-code#42229.
set -eu
cd "${CLAUDE_PROJECT_DIR:-$PWD}"
if command -v direnv >/dev/null 2>&1 && [ -f .envrc ]; then
  direnv export bash 2>/dev/null || true
fi
echo "true"
