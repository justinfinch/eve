---
name: devbox-add
description: Add an infrastructure dependency (database, queue, cache, search index, etc.) or a new application dependency (language runtime, CLI tool, build tool) to the current repo's devbox.json — and, for infra deps, wire it as a `devbox services` entry so it starts/stops with the project. Use when the user says "add postgres", "add redis", "add a queue", "add elasticsearch", "we need <service> for this project", "install <tool> for this repo", or otherwise asks to extend the project-scoped dev environment without touching the host. Assumes `devbox-init` has already run (devbox.json + .envrc exist); if not, point the user at that skill first.
---

# devbox-add

Extend the repo's `devbox.json` with a new package — either an application dependency (CLI, runtime, build tool) or an infrastructure service (db, cache, queue, search). For infra, also wire `devbox services` so the daemon starts/stops with the project. Never `brew install` or `npm i -g` as a fallback — the whole point is that the dep belongs to the repo.

## Workflow

1. **Pre-flight.** Require `devbox.json` at repo root (`.envrc` too if user expects shell activation). If missing, stop and redirect to the `devbox-init` skill.
2. **Classify the request.**
   - **App dep** — a CLI, runtime, or build tool the developer invokes (`jq`, `ripgrep`, `just`, `terraform`, `python`). One-shot add; no service wiring.
   - **Infra dep** — a long-running daemon the app talks to (`postgresql`, `redis`, `mysql`, `mongodb`, `rabbitmq`, `elasticsearch`, …). Needs a `devbox services` entry and likely env vars.
   - If ambiguous, ask.
3. **Resolve the canonical package name.** Run `devbox search <name>` and pick the canonical nixpkgs name (e.g. user says "postgres" → package is `postgresql`; "rabbitmq" → `rabbitmq-server`). For common services, the catalog in [CATALOG.md](references/CATALOG.md) lists the canonical names plus default ports and plugin status — check it first.
4. **Pick a version pin.** Default to `@latest` unless the user named one or the project already constrains the version. Don't invent pins.
5. **Add it.** Run `devbox add <pkg>@<version>` from the repo root. This updates both `devbox.json` and `devbox.lock`. Prefer the CLI over hand-editing JSON.
6. **For infra deps, inspect the plugin.** Run `devbox info <pkg>`. Devbox ships built-in plugins for common databases/caches that auto-configure data dirs, env vars (`PGDATA`, `PGPORT`, `REDIS_PORT`, …), and `devbox services` entries. The `devbox info` output lists what the plugin provides.
   - **Plugin present:** the service is already wired. Move to step 7.
   - **No plugin:** add a `process-compose.yaml` entry yourself. See [CATALOG.md](references/CATALOG.md) for examples (`rabbitmq-server`, `elasticsearch`, `nats-server`, …). Set the working dir to `.devbox/virtenv/<name>` so state stays repo-local.
7. **Handle port collisions.** Most plugins use the upstream default port (postgres 5432, redis 6379, mysql 3306, mongo 27017). These commonly collide with the user's host install. Override in `devbox.json` `env` — see [CATALOG.md](references/CATALOG.md) for the right env var per service. Bump by +1 (5432→5433, etc.) unless the user has a reason.
8. **Document the source of truth in the repo's agent context file.** The "devbox.json is the source of truth for deps" policy needs to live somewhere coding agents will read. Pick a target file in this order — append the snippet from [assets/agents-md-snippet.md](assets/agents-md-snippet.md) to **all** that exist, and create `AGENTS.md` if none do:
   - `AGENTS.md` (cross-agent convention used by Codex CLI and others)
   - `CLAUDE.md` (Claude Code)
   - `.cursorrules` or `.cursor/rules/*.md` (Cursor)
   - `.windsurfrules` (Windsurf)
   - `.github/copilot-instructions.md` (GitHub Copilot)
   Idempotent: grep each target for the marker `<!-- devbox-source-of-truth -->` and skip if present. Don't edit `README.md` — that's user-facing, not agent-facing.
9. **Verify.**
   - App dep: `devbox run -- <pkg> --version` (or the binary's check command).
   - Infra dep: `devbox services up <name> -b` then a connection smoke test (`devbox run -- psql -c '\l'`, `devbox run -- redis-cli ping`, etc.). Stop the service afterwards (`devbox services stop <name>`) unless the user wants it running.
10. **Report.** Tell the user what changed in `devbox.json`, any new env vars, the start command for infra deps (`devbox services up <name> -b`), and any port override you applied.

## Notes

- **No host installs.** If `devbox add` fails (e.g. nixpkgs doesn't have the package under that name), search harder or ask the user — don't fall back to `brew`/`apt`/`npm -g`.
- **Port override convention.** Prefer +1 from the upstream default (e.g. `PGPORT=5433`) so the dev port is memorable but doesn't collide with a host install. Mention the override in the report so the user can update their `.env` / connection strings.
- **`devbox services` uses process-compose** under the hood. A repo-root `process-compose.yaml` overrides/extends what plugins provide — write one only when the plugin path doesn't cover the service.
- **The agent-context snippet is the policy hook.** Once it's in `AGENTS.md` / `CLAUDE.md` / etc., any coding agent that reads the repo's context file knows that adding deps means editing `devbox.json`, not running `brew install`.
- **Agent-agnostic on purpose.** This skill doesn't assume any particular coding agent. The snippet is written in agent-neutral language and lives in whichever context file(s) the repo already uses.

## Files

- [CATALOG.md](references/CATALOG.md) — common services: canonical package names, default ports, plugin status, override env vars.
- [assets/agents-md-snippet.md](assets/agents-md-snippet.md) — the "devbox.json is the source of truth" section to append to a repo's agent context file (`AGENTS.md`, `CLAUDE.md`, `.cursorrules`, etc.).
- [assets/process-compose.yaml](assets/process-compose.yaml) — starter `process-compose.yaml` for services without a built-in plugin.
