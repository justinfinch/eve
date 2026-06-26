# Lesson 7 — The One-Command Spine (justfile)

## What you'll learn

How all four parts are built, checked, and run through a single file of named commands —
the `justfile` — so you never have to remember the long underlying invocations.

## The concept

You've now seen each part has its own build/test commands: `cargo test -p contract`,
`cargo run -p simulator`, `npm --prefix web run build`, `cd firmware && cargo build`, and
the codegen pipeline. Memorizing and chaining those is error-prone.

**[`just`](https://github.com/casey/just)** is a command runner — like `make`, but
designed for exactly this. You write **recipes** (named groups of commands) in a
`justfile`, and run them with `just <recipe>`. The `justfile` becomes the **single
entrypoint** for the whole project. Everything you learned in Lessons 2–6 is wrapped into
five recipes.

## Walk the code

The whole `justfile` lives at the repo root. Here are its recipes.

### `default` — list everything

```bash
default:
    @just --list
```

Running `just` with no arguments lists the available recipes. A friendly front door.

### `gen` and `check-gen` — codegen + drift guard

These are [Lesson 4](04-codegen.md) — `gen` regenerates `web/src/contract.gen.ts` from
the Rust contract; `check-gen` fails if the committed version is stale. They're recipes
like any other.

### `build` — build all four parts

```bash
build:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo build --workspace                  # contract + simulator (the host workspace)
    (cd firmware && cargo build)             # firmware (note: cd into firmware — Lesson 6)
    npm --prefix web ci || npm --prefix web install
    npm --prefix web run build               # web
    echo "all four parts built"
```

One command builds **everything**: the host Rust workspace, the firmware (from inside its
own directory, per [Lesson 6](06-the-firmware.md)), and the web app. `set -euo pipefail`
means "stop at the first failure" — so if *any* part breaks, `just build` fails. That's
the guarantee: green means all four genuinely build.

### `check` — the acceptance gate

```bash
check:
    #!/usr/bin/env bash
    set -euo pipefail
    just build                                   # 1. everything builds
    just check-gen                               # 2. generated TS isn't stale
    cargo clippy --workspace -- -D warnings      # 3. lint the host code (warnings = failure)
    (cd firmware && cargo clippy -- -D warnings) # 4. lint the firmware
    cargo test --workspace                       # 5. run Rust tests
    npm --prefix web test                        # 6. run web tests
    echo "check passed"
```

`check` is the **whole quality bar in one command** — what you'd run before committing or
in CI. It builds all four parts, runs the drift guard, lints with **clippy** (`-D
warnings` turns every warning into a hard error), and runs both the Rust and web test
suites. If `check` passes, the Foundation is healthy.

### `dev` — run the live experience

```bash
dev:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo run -p simulator &       # start the simulator in the background
    SIM_PID=$!                     # remember its process id
    trap "kill $SIM_PID 2>/dev/null || true" EXIT   # kill it when this recipe exits
    npm --prefix web run dev       # start the Vite dev server in the foreground
```

`dev` is the "see it work" recipe. It launches the simulator ([Lesson 3](03-the-simulator.md))
in the background, then starts the Vite dev server ([Lesson 5](05-the-web-workbench.md))
in the foreground. The `trap ... EXIT` line ensures that when you `Ctrl-C` the dev server,
the simulator is shut down too — no orphaned process left listening on port 8765.

## How the lessons map to recipes

| Recipe | What it ties together | Lessons |
|--------|-----------------------|---------|
| `just gen` | regenerate the TS type from Rust | [4](04-codegen.md) |
| `just check-gen` | fail if the generated type is stale | [4](04-codegen.md) |
| `just build` | build contract + simulator + web + firmware | [2](02-the-contract.md)–[6](06-the-firmware.md) |
| `just check` | build + drift guard + clippy + all tests | all |
| `just dev` | run simulator + web together | [3](03-the-simulator.md), [5](05-the-web-workbench.md) |

## Run it yourself

```bash
devbox run -- just                # list all recipes
devbox run -- just build          # build all four parts
devbox run -- just check          # the full acceptance gate
devbox run -- just dev            # run the live experience (Ctrl-C to stop)
```

If you've installed direnv ([Lesson 1](01-the-dev-environment.md)), you can drop the
`devbox run --` prefix and just type `just build`, etc.

## Recap

- **`just`** turns the many underlying commands into a handful of named **recipes** in one
  `justfile` — the single entrypoint.
- **`just build`** builds all four parts and fails if any breaks; **`just check`** adds the
  drift guard, clippy, and all tests — the acceptance gate; **`just dev`** runs the live
  simulator + browser experience.
- The recipes are just thin wrappers over the commands you learned part-by-part in the
  earlier lessons.

**Next:** [Lesson 8 — Running everything](08-running-everything.md): a full hands-on
session, plus troubleshooting.
</content>
