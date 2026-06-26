# Lesson 1 — The Dev Environment (devbox)

## What you'll learn

How this project gets the right versions of every tool (Rust, Node, `just`, and more)
onto your machine **without polluting your machine**, and why you'll see `devbox run --`
in front of commands.

## The concept

A project like this needs several tools: the Rust compiler, Node.js, a WebAssembly
tool, the `just` task runner, and so on. If everyone installs these globally with
`brew` / `apt` / `npm i -g`, two bad things happen:

1. **Drift** — your Rust is 1.85, a teammate's is 1.90, and the project behaves
   differently on each machine.
2. **Mess** — your laptop accumulates global tools that fight with other projects.

**devbox** solves this. It reads one file, `devbox.json`, that lists exactly which
tools and versions this project needs, and provides them in an **isolated environment**
scoped to this repo. Nothing is installed globally. A teammate (or CI, or a fresh
clone) gets byte-for-byte the same tools.

This is the project's rule, written down in [`AGENTS.md`](../../AGENTS.md):

> **devbox is the source of truth** for the dev environment. Never `brew install` as a
> substitute — that modifies your host and won't reproduce on a teammate's machine.

## Walk the code

### `devbox.json` — the tool manifest

```json
{
  "packages": [
    "rustup@latest",          // the Rust toolchain installer
    "nodejs@latest",          // Node.js, for the web app + npm
    "probe-rs-tools@latest",  // flashes/debugs real microcontrollers (for later)
    "gh@latest",              // GitHub CLI
    "just@latest",            // the task runner (Lesson 7)
    "wasm-bindgen-cli@latest" // the Rust→wasm→TS bridge tool (Lesson 4)
  ],
  ...
}
```

Each line is a tool devbox guarantees is available. When you add a new tool, you don't
`brew install` — you run `devbox add <pkg>`, which appends it here and locks the exact
version in `devbox.lock`.

### `rust-toolchain.toml` — pinning Rust itself

`rustup` (installed by devbox) is just the *installer*. This file says exactly which
Rust to use:

```toml
[toolchain]
channel = "1.85.0"                 # the exact Rust version
components = ["rustfmt", "clippy"] # formatter + linter
targets = [
    "wasm32-unknown-unknown",      # so we can compile Rust to WebAssembly (Lesson 4)
    "thumbv8m.main-none-eabihf",   # so we can compile for the ARM microcontroller (Lesson 6)
]
```

A **target** is "what kind of CPU/platform am I compiling *for*." By default Rust
compiles for your laptop. Here we also declare two extra targets: WebAssembly (used to
generate the TypeScript type) and a Cortex-M microcontroller (used by the firmware).
The first time you build, `rustup` reads this file and installs all of it automatically.

### `.envrc` — automatic activation (optional convenience)

```bash
eval "$(devbox generate direnv --print-envrc)"
```

If you install a companion tool called [**direnv**](https://direnv.net), this file makes
the devbox environment **activate automatically** whenever you `cd` into the repo — so
you can just type `just build` instead of `devbox run -- just build`. It's optional;
`devbox run --` always works without it.

## Run it yourself

You only need devbox installed once. Then:

```bash
# Confirm devbox is available and reading this project's manifest:
devbox version

# Drop into a shell that has all the project's tools on PATH:
devbox shell
# ...now `rustc --version`, `node --version`, `just --version` all work.
# Type `exit` to leave.
```

Or, without entering a shell, prefix any single command with `devbox run --`:

```bash
devbox run -- rustc --version     # should print 1.85.0
devbox run -- node --version
devbox run -- just --version
```

### The first build installs the toolchain

The very first time you build, `rustup` quietly downloads Rust 1.85.0 and the two extra
targets named in `rust-toolchain.toml`. This can take a few minutes — that's normal, and
it only happens once.

```bash
devbox run -- rustup show
# Look for: active toolchain 1.85.0, and both targets listed as installed.
```

## Why `devbox run --` everywhere?

In an **interactive** terminal with direnv installed, the environment is already active,
so you can drop the prefix. But in **non-interactive** contexts — scripts, CI, or an AI
coding agent running commands for you — direnv doesn't kick in. `devbox run -- <cmd>`
guarantees the right environment no matter the context. These lessons use the prefix so
every command works for everyone.

## Recap

- **devbox** gives this repo its own isolated set of tools, pinned to exact versions, with
  zero global installs.
- **`devbox.json`** lists the tools; **`rust-toolchain.toml`** pins Rust + its compile targets.
- Use **`devbox run -- <cmd>`** to run anything with the project's tools (or `devbox shell`
  to enter an interactive session; or install **direnv** for automatic activation).

**Next:** [Lesson 2 — The contract](02-the-contract.md), the single Rust type everything
else is built around.
</content>
