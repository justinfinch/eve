<!-- devbox-source-of-truth -->
## Dev environment: devbox is the source of truth

This repo uses [devbox](https://www.jetify.com/devbox) for its dev environment. Runtime versions, CLI tools, and infra services (databases, caches, queues) are declared in `devbox.json` and isolated from the host.

**When adding a dependency:**

- **App deps** (CLI, runtime, build tool): `devbox add <pkg>@<version>`. Never `brew install`, `apt install`, or `npm i -g` as a substitute — those modify the host and won't reproduce on a teammate's machine or in CI.
- **Infra deps** (db, queue, cache, search): `devbox add <pkg>`, then start with `devbox services up <pkg> -b`. Most common services have a built-in devbox plugin that wires env vars (`PGPORT`, `REDIS_PORT`, …) and a `process-compose` entry automatically — check `devbox info <pkg>` after adding.
- **Port overrides** go in `devbox.json` under `env` (e.g. `"PGPORT": "5433"`) to avoid collisions with anything already running on the host.

**Activation:**

- Interactive shells: `direnv` auto-activates on `cd` (if the direnv shell hook is installed).
- Non-interactive contexts (CI, scripts, coding-agent tool calls): run commands through `devbox run -- <command>` so the declared environment is applied. `direnv` does not activate in non-interactive shells.

**Never** install runtimes, CLIs, or services on the host as a workaround. If something can't be expressed in `devbox.json`, raise it — don't paper over it with a host install.

<!-- arche-context-source -->
## Institutional context: consult the Arche first

This repo has an **Arche** at `./.arche/` — a curated knowledge base of *institutional context the code does not carry*: business domain, SME knowledge, architectural decisions (ARDs / SADs / ADRs), and research. It follows Karpathy's LLM-wiki pattern and is maintained by the `arche-*` skills. Treat it as a first-class context source, not an optional lookup.

**Before** planning, designing, scoping, specifying a feature, or making a setup decision (dev environment, tooling, dependencies), orient in the Arche first so the work descends from what the team already knows:

1. Read `./.arche/index.md` — the catalog and entry point.
2. Walk to the relevant `entities/` and `concepts/` (including ARDs / SADs / ADRs) pages.
3. Surface the relevant decisions, constraints, and prior research *before* proposing an approach, and cite the pages you used.

If the `arche-query` skill is available (e.g. `/arche-query` in Claude Code), invoke it to do this rather than reading pages ad hoc — it walks the index, cites provenance, and flags gaps. The other `arche-*` skills cover the rest of the institutional-context workflow: `arche-ingest` (add a source), `arche-architect` (design / ADRs), `arche-discover` (ideation), `arche-tell` (communicate), `arche-lint` (health check).

**This grounding applies to your existing dev-methodology skills too.** When a spec-writing, planning, or implementation skill from another library (spec-kit, superpowers, your own) runs in this repo, it should consult the Arche first by the same steps above. The Arche does not own spec or plan artifacts — it supplies the grounded *why/what/decisions* those skills build on, then gets out of the way.

**The Arche is institutional context, not code documentation.** If a question is answered by *reading the code*, the Arche is the wrong place. Do not dump in-flight TODOs or debugging notes into it — those stay in PRs and commit messages.
