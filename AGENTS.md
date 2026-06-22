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
