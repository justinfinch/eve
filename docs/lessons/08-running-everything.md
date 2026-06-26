# Lesson 8 — Running Everything, End to End

## What you'll learn

A single, do-it-yourself session that takes you from a fresh terminal to the molecule's
name showing in your browser — and what to do when something goes wrong.

## Prerequisites

You need **devbox** installed once on your machine (see [Lesson 1](01-the-dev-environment.md)).
Optionally **direnv** for auto-activation, and **Chrome** for the browser step. Everything
else (Rust, Node, `just`, …) devbox provides.

## The full session

Open a terminal in the repo root (`/Users/justinfinch/Source/eve`).

### 1. Confirm the environment

```bash
devbox run -- rustc --version    # 1.85.0
devbox run -- node --version
devbox run -- just --version
```

The very first run may pause while `rustup` installs Rust 1.85.0 and the two compile
targets named in `rust-toolchain.toml`. That's expected and happens only once.

### 2. Build all four parts

```bash
devbox run -- just build
```

Watch it compile the host workspace (`contract` + `simulator`), the firmware (for the ARM
chip), and the web app, then print `all four parts built`. The first build is the slow
one — Rust compiles every dependency from scratch. Subsequent builds are fast.

### 3. Run the full acceptance gate

```bash
devbox run -- just check
```

This builds everything, runs the drift guard ([Lesson 4](04-codegen.md)), lints with
clippy, and runs the Rust and web test suites. It should end with `check passed`. If it
does, the whole Foundation is healthy.

### 4. See the molecule in the browser

```bash
devbox run -- just dev
```

You'll see two things start up:

- `simulator listening on ws://127.0.0.1:8765` — the software molecule ([Lesson 3](03-the-simulator.md)).
- A Vite line with a local URL, usually `http://localhost:5173/`.

Open that URL in **Chrome**. The page shows:

```
Mini-Molecule Workbench
status: connected
id    mol-001
name  Mini-Molecule
```

Press `Ctrl-C` in the terminal to stop both the dev server and the simulator.

### What just happened (the whole loop in one breath)

The Rust `contract` defines `SelfId` once. The simulator built a `SelfId`, serialized it
to JSON, and sent it over a WebSocket the moment your browser connected. The browser app
parsed that JSON into the `SelfId` type **generated** from the very same Rust contract,
and rendered the `id` and `name`. Every layer from Lessons 2–7, working together.

## Exploring on your own

A few experiments to cement understanding (all reversible with `git checkout`):

- **Change what the molecule says.** Edit `crates/simulator/src/lib.rs`, change
  `name: "Mini-Molecule".into()` to something else, re-run `just dev`, and watch the
  browser show your new name. (Byte-for-byte: the browser shows exactly what the sim
  sent.)
- **Watch the drift guard fire.** Follow the "Try it yourself" in [Lesson 4](04-codegen.md).
- **Break the contract on purpose.** Add a field to `SelfId` in
  `crates/contract/src/lib.rs`, run `just check`, and read the failures — the Rust JSON
  test, the drift guard, and (after `just gen`) the TypeScript all react. This is the SSOT
  protecting you.

## Troubleshooting

| Symptom | Likely cause | Fix |
|---------|--------------|-----|
| `command not found: just` (or `cargo`, `node`) | You're not in the devbox environment | Prefix with `devbox run --`, or run `devbox shell`, or install direnv ([Lesson 1](01-the-dev-environment.md)) |
| First build hangs for a long time | `rustup` is downloading Rust 1.85.0 + targets | Normal on first run; let it finish |
| Browser shows `status: disconnected` | The simulator isn't running | Use `just dev` (runs both), or start `cargo run -p simulator` in another terminal |
| Browser stuck on "waiting…" | Connected, but no frame yet, or wrong URL | Confirm the simulator printed `listening on ws://127.0.0.1:8765`; the app connects there |
| WebSocket won't connect in the browser | Some browsers block `ws://` from `https://` pages | Use the plain `http://localhost:5173` URL Vite prints, in Chrome |
| `just check-gen` fails with "stale" | `web/src/contract.gen.ts` is out of date | Run `devbox run -- just gen` and commit the result |
| Firmware build fails from repo root | Cargo didn't read `firmware/.cargo/config.toml` | Build from inside the dir: `bash -c 'cd firmware && cargo build'` ([Lesson 6](06-the-firmware.md)) |
| `cargo build --workspace` tries to build firmware | — | It shouldn't; the root `Cargo.toml` `exclude`s it. Confirm you didn't add `firmware` to `members` |
| clippy fails `just check` with warnings | `-D warnings` makes warnings fatal | Read the clippy message and fix it in the named crate — that's intended |

## Where to go next

You've now run and understood the entire Foundation. The capabilities to add next — real
ADC/GPIO/PWM, the AI layer, the permalink — all hang off this skeleton:

- The `capabilities` list in `SelfId` (empty today) is where new abilities get advertised.
- The simulator's single-frame `handle` becomes a richer, streaming conversation.
- The firmware moves from "compiles" to "flashed and running" (that's where
  `probe-rs-tools` from `devbox.json` comes in).

For the bigger vision and the design rationale, see the [main README](../../README.md) and
the institutional context under [`.arche/`](../../.arche/index.md).

## Recap

- One session: `just build` → `just check` → `just dev`, then open the URL in Chrome.
- Seeing `id: mol-001` / `name: Mini-Molecule` in the browser means every layer from the
  Rust contract to the React UI is connected and agreeing.
- When something breaks, the table above maps the symptom to the layer — and you now know
  enough about each layer to fix it.

**You've finished the lessons.** Head back to the [index](README.md) anytime.
</content>
