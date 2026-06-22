---
name: devbox-init
description: Scaffold an isolated, declarative dev environment for the current repo using Devbox + direnv — lighter-weight than containers, agent-agnostic. Detects language/runtime from manifest files and seeds devbox.json, writes .envrc, walks the user through installing devbox/direnv if missing (including the shell hook line they must add manually), and runs `direnv allow`. As an optional final step (only if the user is on Claude Code), can drop in a SessionStart/CwdChanged hook so devbox-declared packages are visible to Claude Code's non-interactive Bash tool — skipped otherwise, since `devbox run -- <cmd>` is the portable fallback. Use when the user says "set up devbox", "init devbox", "isolate this repo's dev env", "add a devbox", "scaffold direnv", or asks for a project-scoped runtime without Docker.
---

# devbox-init

Scaffold `devbox.json` + `.envrc` so this repo's runtime, CLI tools, and env vars are declared in-repo and isolated from the host. Optionally wire a Claude Code hook so the same env is visible to Claude Code's Bash tool.

## Workflow

1. **Pre-flight.** Run from the repo root. If `devbox.json` or `.envrc` already exist, list them and ask before overwriting any file.
   - **Consult the Arche.** If `./.arche/` exists, invoke `/arche-query` *before* choosing runtimes, versions, or tools — runtime/dependency/tooling decisions are setup decisions that may already be recorded as ADRs or domain constraints. Surface them via the skill rather than reading Arche pages ad hoc, and let what it returns ground the choices in the steps below. If there's no `./.arche/`, skip.
2. **Check `devbox` is installed** (`command -v devbox`). If missing:
   - Attempt: `curl -fsSL https://get.jetify.com/devbox | bash` (run as a non-root user; the installer handles Nix if needed).
   - If the installer fails or needs interaction, stop and tell the user to run the command themselves, then continue.
3. **Check `direnv` is installed** (`command -v direnv`). If missing:
   - macOS: `brew install direnv`.
   - Linux (universal): `curl -sfL https://direnv.net/install.sh | bash`.
   - Distro packages also exist (`apt install direnv`, `dnf install direnv`, `pacman -S direnv`).
4. **Check direnv shell hook.** Grep the user's shell rc for `direnv hook`. If absent, **do not edit their rc silently** — print the exact line they need to add and pause:
   - bash: `eval "$(direnv hook bash)"` → `~/.bashrc`
   - zsh:  `eval "$(direnv hook zsh)"`  → `~/.zshrc`
   - fish: `direnv hook fish | source`  → `~/.config/fish/config.fish`
5. **Detect runtimes** by inspecting manifest files at the repo root (see table below). For each match, *also* try to detect a version pin from the project itself (see the version-detection column). If nothing pins the version, ask the user — and if they don't care, fall through to `@latest`. If no manifests are detected at all, scaffold with an empty `packages` array and tell the user they can `devbox add <pkg>@<version>` later (or use the companion `devbox-add` skill); the isolation is still set up.
6. **Write `devbox.json`** from [assets/devbox.json](assets/devbox.json), substituting the detected packages. Leave `env` and `shell.scripts` empty unless the user has requested specifics.
7. **Write `.envrc`** from [assets/envrc](assets/envrc) (verbatim — just the `eval "$(devbox generate direnv --print-envrc)"` line plus an optional `dotenv` fallback for `.env` files).
8. **Run `direnv allow`** in the repo root to trust the new `.envrc`. If the user's shell hook isn't installed yet, note that the env will activate on next `cd` after they finish step 4.
9. **Optional, Claude-Code-specific: offer the env-snapshot hook.** Skip this step unless the user is using Claude Code in this repo — other coding agents either don't have the problem (they inherit direnv) or solve it differently, and `devbox run -- <cmd>` is the portable fallback for anyone. If they are on Claude Code, phrase the offer as: *"Want me to drop in a `.claude/hooks/load-devbox-env.sh` that snapshots the devbox env on SessionStart and CwdChanged? This makes devbox-declared packages visible to Claude Code's Bash tool, which runs non-interactively and bypasses direnv's shell hook. Skip if you don't use Claude Code in this repo."*
   - If yes: copy [assets/load-devbox-env.sh](assets/load-devbox-env.sh) to `.claude/hooks/load-devbox-env.sh` (`chmod +x`), then merge the `SessionStart` + `CwdChanged` entries from [assets/claude-settings-hooks.json](assets/claude-settings-hooks.json) into `.claude/settings.json` (create the file if missing; preserve any other top-level keys and existing hook entries).
   - If no / not applicable: skip — `devbox run -- <cmd>` works from any non-interactive context.
10. **Verify** by running `devbox shell -- echo ok` in the repo root. Report success or the first error.

## Runtime detection table

For each matched manifest, look for a version signal in the listed places. Use the first one found; if none, ask the user; if they pass, use `@latest`.

| Manifest at repo root        | Seed package(s)            | Version signals to check (in order)                                  |
|------------------------------|----------------------------|----------------------------------------------------------------------|
| `package.json`               | `nodejs`                   | `.nvmrc`, `.node-version`, `engines.node` in `package.json`          |
| `pyproject.toml` / `requirements.txt` / `Pipfile` | `python` (+ `uv` if `pyproject.toml` uses it) | `.python-version`, `requires-python` in `pyproject.toml`             |
| `go.mod`                     | `go`                       | `go` directive in `go.mod`                                           |
| `Cargo.toml`                 | `rustup`                   | `rust-toolchain` / `rust-toolchain.toml`, `rust-version` in `Cargo.toml` |
| `Gemfile`                    | `ruby`                     | `.ruby-version`, `ruby` directive in `Gemfile`                       |
| `pom.xml` / `build.gradle*`  | `jdk` (+ `maven` or `gradle`) | `.java-version`, `maven.compiler.release` in `pom.xml`               |
| `composer.json`              | `php`                      | `require.php` constraint in `composer.json`                          |
| `mix.exs`                    | `elixir`                   | `.tool-versions` (`elixir` line)                                     |
| `Justfile`                   | also add `just`            | —                                                                    |

Multiple matches stack (a polyglot repo gets multiple packages). Always offer to add `just` if a `Justfile` is present, regardless of language. Don't invent version numbers — only emit a pin if you detected one or the user gave you one.

## Notes

- **No CLAUDE.md edits.** Codifying "devbox.json is the source of truth for deps" is the job of a companion skill, not this one. This skill only scaffolds files.
- **Don't `brew install` runtimes** as a fallback. The whole point of this skill is that runtimes belong in `devbox.json`. If devbox cannot be installed at all, stop and tell the user — do not silently install Node/Python on the host.
- **Don't edit the user's shell rc.** Adding `eval "$(direnv hook ...)"` is a persistent change to their environment; always have them do it.
- Reference implementation: `~/Source/tankful-superpowers` — the templates in this skill are derived from its working setup.
