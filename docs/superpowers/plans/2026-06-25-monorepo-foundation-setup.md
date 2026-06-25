# Mini-Molecule Foundation Monorepo Setup — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the full Foundation seam — a monorepo where a software molecule (simulator) announces who it is over a WebSocket and a browser workbench displays it, with the message contract defined once in Rust and the TypeScript type generated from it, no hardware attached.

**Architecture:** Two Cargo workspaces. The *host* workspace holds `contract` (a `no_std + serde` crate that is the single source of truth) and `simulator` (a `std` tokio WebSocket binary). A *separate* `firmware` workspace holds an Embassy/RP2350 `no_std` binary that imports `contract` and compiles board-free. The `web` app (Vite + React + TS) consumes a TypeScript type **generated** from `contract` via `tsify` + `wasm-bindgen`. A single `just` command builds all four parts and a drift guard fails if the generated type is stale.

**Tech Stack:** Rust (no_std + std), `serde`, `tokio` + `tokio-tungstenite`, `tsify-next` + `wasm-bindgen`, `embassy-rp` (RP2350 / `thumbv8m.main-none-eabihf`), Vite + React + TypeScript (npm), `just`, devbox.

## Global Constraints

These are project-wide and apply to **every** task. Values copied verbatim from the design doc.

- **The contract is the single source of truth.** TS is *generated* from the Rust contract via tsify, with **no hand-maintained duplicate** (SC-6).
- **One command builds all four parts green** plus a **drift check** that fails if regenerating the web type would differ from what's committed (SC-5, SC-4).
- **Firmware compiles board-free; simulator is a first-class WebSocket peer; no hardware** (SC-2).
- **Two Cargo workspaces** (host + firmware): host `cargo build`/`test` never drags in the no_std embedded crate.
- **Task runner is `just`** (a justfile is the single entrypoint for all parts).
- **Web stack is Vite + React + TS, npm** (default per SAD).
- **Devbox is the source of truth for the dev environment.** Devbox already provides `rustup`, `nodejs`, `probe-rs-tools`, `gh`. New tools are added via `devbox add`, never a host install. Run commands in non-interactive contexts through `devbox run -- <cmd>`.
- **Clean checkout to green in ≤15 min** (SC-1): toolchain + targets are pinned so a fresh clone has what it needs.
- **byte-for-byte id/name** — the web shows exactly what the sim sent (SC-3).

**Requirement traceability** (from the design doc) is preserved per-task in the `Realizes` line of each task.

---

## File Structure

Created across the tasks below:

```
eve/
├── Cargo.toml                 # HOST workspace: contract + simulator (excludes firmware, web)
├── Cargo.lock
├── rust-toolchain.toml        # pins toolchain + wasm32 & thumbv8m targets
├── justfile                   # the single entrypoint for all parts
├── devbox.json                # + just, + wasm-bindgen-cli   (modified)
├── crates/
│   ├── contract/
│   │   ├── Cargo.toml
│   │   └── src/lib.rs         # SelfId — the SSOT hub
│   └── simulator/
│       ├── Cargo.toml
│       ├── src/lib.rs         # self_id() + serve()
│       ├── src/main.rs        # binds :8765, calls serve()
│       └── tests/handshake.rs # e2e: connect, read frame, parse SelfId
├── firmware/                  # SEPARATE workspace
│   ├── Cargo.toml
│   ├── rust-toolchain.toml -> (inherits repo root)
│   ├── .cargo/config.toml     # default target = thumbv8m.main-none-eabihf
│   ├── build.rs               # emits memory.x to OUT_DIR
│   ├── memory.x               # RP2350 memory layout
│   └── src/main.rs            # embassy-rp, no_std, imports contract, build-only
└── web/
    ├── package.json
    ├── vite.config.ts
    ├── src/
    │   ├── contract.gen.ts    # GENERATED from contract — committed, drift-checked
    │   ├── selfId.ts          # parseSelfId(raw): SelfId
    │   ├── selfId.test.ts     # vitest
    │   └── App.tsx            # connects to sim, shows id + name
    └── ...
```

---

## Task 1: Dev environment — toolchain pin, targets, devbox tools, justfile skeleton

**Realizes:** FR-8 (onboarding), SC-1 (≤15 min clean checkout). Foundation every later task depends on.

**Files:**
- Modify: `devbox.json`
- Create: `rust-toolchain.toml`
- Create: `justfile`

**Interfaces:**
- Produces: a `just` binary on PATH; `wasm-bindgen` CLI on PATH; the `wasm32-unknown-unknown` and `thumbv8m.main-none-eabihf` rustup targets installed; a `justfile` whose `just --list` runs. Later tasks add recipes to this same `justfile`.

- [ ] **Step 1: Add `just` and `wasm-bindgen-cli` to devbox**

Run:
```bash
devbox add just wasm-bindgen-cli
```
Expected: `devbox.json` now lists `just@latest` and `wasm-bindgen-cli@latest` in `packages`; devbox resolves and the lockfile updates without error.

- [ ] **Step 2: Pin the Rust toolchain and cross-compilation targets**

Create `rust-toolchain.toml`:
```toml
[toolchain]
channel = "1.85.0"
components = ["rustfmt", "clippy"]
targets = [
    "wasm32-unknown-unknown",
    "thumbv8m.main-none-eabihf",
]
```

- [ ] **Step 3: Verify the toolchain and targets install on a clean invocation**

Run:
```bash
devbox run -- rustup show
```
Expected: output shows `1.85.0` as the active toolchain and lists both `wasm32-unknown-unknown` and `thumbv8m.main-none-eabihf` under installed targets. (rustup auto-installs from `rust-toolchain.toml` on first use. If `1.85.0` is unavailable in your channel, bump to the current stable and note it — the pin only needs to be a recent stable that builds embassy-rp for RP2350.)

- [ ] **Step 4: Create the justfile skeleton**

Create `justfile`:
```just
# Mini-Molecule Foundation — single entrypoint for all four parts.
# Recipes are added by later foundation tasks: gen, check-gen, dev, build, check.

# List available recipes (default).
default:
    @just --list
```

- [ ] **Step 5: Verify `just` runs**

Run:
```bash
devbox run -- just --list
```
Expected: PASS — prints the `default` recipe. No error.

- [ ] **Step 6: Commit**

```bash
git add devbox.json devbox.lock rust-toolchain.toml justfile
git commit -m "chore: pin toolchain + targets, add just/wasm-bindgen, justfile skeleton"
```

---

## Task 2: `contract` crate — the SelfId single source of truth

**Realizes:** FR-1 (part of the four-part workspace), FR-2 (SelfId shape, empty capabilities).

**Files:**
- Create: `Cargo.toml` (host workspace root)
- Create: `crates/contract/Cargo.toml`
- Create: `crates/contract/src/lib.rs`

**Interfaces:**
- Produces: `contract::SelfId { id: String, name: String, fw_version: String, capabilities: Vec<String> }` — `#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]`. Serializes to JSON object with keys `id`, `name`, `fw_version`, `capabilities` in that order. The crate is `no_std + alloc` by default; feature `std` enables `serde/std`; feature `codegen` (added in Task 4) enables std + tsify. Consumed by `simulator` (Task 3, `std` feature), `firmware` (Task 7, default features), and the codegen flow (Task 4, `codegen` feature).

- [ ] **Step 1: Create the host workspace root manifest**

Create `Cargo.toml` (list only `contract` now — Task 3 adds `simulator` to `members` when it creates that crate; cargo errors if a listed member directory does not yet exist):
```toml
[workspace]
resolver = "2"
members = ["crates/contract"]
exclude = ["firmware", "web"]
```

Also append Rust build output to `.gitignore` (it is not yet ignored):
```
# Rust build output
target/
```

- [ ] **Step 2: Create the contract crate manifest**

Create `crates/contract/Cargo.toml`:
```toml
[package]
name = "contract"
version = "0.1.0"
edition = "2021"

[lib]
# rlib for firmware/simulator. The wasm cdylib needed for codegen is produced
# on demand in Task 4 via `cargo rustc --crate-type cdylib` (a no_std cdylib
# cannot link on the host, so cdylib must NOT live in this manifest).
crate-type = ["rlib"]

[dependencies]
serde = { version = "1", default-features = false, features = ["derive", "alloc"] }

[dev-dependencies]
serde_json = "1"

[features]
default = []
std = ["serde/std"]
```

- [ ] **Step 3: Write the failing test**

Create `crates/contract/src/lib.rs`:
```rust
#![cfg_attr(not(any(test, feature = "std", feature = "codegen")), no_std)]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

/// The molecule's self-description — the spine every later rung extends.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SelfId {
    pub id: String,
    pub name: String,
    pub fw_version: String,
    pub capabilities: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn self_id_serializes_to_expected_json() {
        let s = SelfId {
            id: "mol-001".into(),
            name: "Mini-Molecule".into(),
            fw_version: "0.1.0".into(),
            capabilities: Vec::new(),
        };
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(
            json,
            r#"{"id":"mol-001","name":"Mini-Molecule","fw_version":"0.1.0","capabilities":[]}"#
        );
    }

    #[test]
    fn self_id_roundtrips() {
        let s = SelfId {
            id: "mol-001".into(),
            name: "Mini-Molecule".into(),
            fw_version: "0.1.0".into(),
            capabilities: Vec::new(),
        };
        let json = serde_json::to_string(&s).unwrap();
        let back: SelfId = serde_json::from_str(&json).unwrap();
        assert_eq!(s, back);
    }
}
```

- [ ] **Step 4: Run the test to verify it passes**

Run:
```bash
devbox run -- cargo test -p contract
```
Expected: PASS — `self_id_serializes_to_expected_json` and `self_id_roundtrips` both green. (This is a write-then-pass rather than red-first because the type and test are introduced together; the assertion on exact JSON byte-order is the real check.)

- [ ] **Step 5: Verify the crate builds `no_std` (default features, no test/std)**

Run:
```bash
devbox run -- cargo build -p contract
```
Expected: PASS — compiles with the `#![no_std]` attribute active (no `test`/`std`/`codegen` features). Proves firmware can later depend on it board-free.

- [ ] **Step 6: Commit**

```bash
git add Cargo.toml Cargo.lock crates/contract
git commit -m "feat(contract): SelfId single-source-of-truth type (no_std + serde)"
```

---

## Task 3: `simulator` crate — the software molecule over WebSocket

**Realizes:** FR-4 (sim advertises SelfId, no hardware), SC-2 (hardware-free), SC-3 (byte-for-byte id/name on the wire).

**Files:**
- Create: `crates/simulator/Cargo.toml`
- Create: `crates/simulator/src/lib.rs`
- Create: `crates/simulator/src/main.rs`
- Test: `crates/simulator/tests/handshake.rs`

**Interfaces:**
- Consumes: `contract::SelfId` (Task 2), with the `std` feature.
- Produces: `simulator::self_id() -> contract::SelfId` (the canonical advertised value); `simulator::serve(listener: tokio::net::TcpListener) -> impl Future<Output = ()>` (accept loop that, per connection, sends exactly one `Message::Text` containing `serde_json::to_string(&self_id())`). The binary binds `127.0.0.1:8765`. Consumed by the web app (Task 6) and `just dev` (Task 8).

- [ ] **Step 1: Register the simulator crate in the workspace, then create its manifest**

First add `crates/simulator` to the host workspace `members` in the root `Cargo.toml` (Task 2 left it listing only `crates/contract`):
```toml
[workspace]
resolver = "2"
members = ["crates/contract", "crates/simulator"]
exclude = ["firmware", "web"]
```

Then create `crates/simulator/Cargo.toml`:
```toml
[package]
name = "simulator"
version = "0.1.0"
edition = "2021"

[dependencies]
contract = { path = "../contract", features = ["std"] }
serde_json = "1"
tokio = { version = "1", features = ["rt-multi-thread", "macros", "net"] }
tokio-tungstenite = "0.24"
futures-util = "0.3"

[dev-dependencies]
tokio = { version = "1", features = ["rt-multi-thread", "macros", "net", "time"] }
```

- [ ] **Step 2: Write the failing end-to-end test**

Create `crates/simulator/tests/handshake.rs`:
```rust
use contract::SelfId;
use futures_util::StreamExt;
use tokio::net::TcpListener;
use tokio_tungstenite::connect_async;

#[tokio::test]
async fn client_receives_one_selfid_frame_on_connect() {
    // Bind an ephemeral port so the test never collides with a running sim.
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(simulator::serve(listener));

    let (mut ws, _resp) = connect_async(format!("ws://{addr}")).await.unwrap();
    let msg = ws.next().await.expect("a frame").expect("ok frame");
    let text = msg.into_text().unwrap();

    // SC-3: the wire bytes parse as SelfId and match what the sim advertises.
    let parsed: SelfId = serde_json::from_str(text.as_str()).unwrap();
    assert_eq!(parsed, simulator::self_id());
    assert!(parsed.capabilities.is_empty());
}
```

- [ ] **Step 3: Run the test to verify it fails**

Run:
```bash
devbox run -- cargo test -p simulator --test handshake
```
Expected: FAIL — compile error, `simulator::serve` / `simulator::self_id` not found (lib not written yet).

- [ ] **Step 4: Write the simulator library**

Create `crates/simulator/src/lib.rs`:
```rust
use contract::SelfId;
use futures_util::SinkExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

/// The canonical self-description this software molecule advertises.
pub fn self_id() -> SelfId {
    SelfId {
        id: "mol-001".into(),
        name: "Mini-Molecule".into(),
        fw_version: "0.1.0".into(),
        capabilities: Vec::new(),
    }
}

/// Accept WebSocket connections forever; emit one SelfId frame per connection.
pub async fn serve(listener: TcpListener) {
    while let Ok((stream, _peer)) = listener.accept().await {
        tokio::spawn(handle(stream));
    }
}

async fn handle(stream: TcpStream) {
    let mut ws = match tokio_tungstenite::accept_async(stream).await {
        Ok(ws) => ws,
        Err(_) => return,
    };
    let json = serde_json::to_string(&self_id()).expect("SelfId serializes");
    // `.into()` adapts String to whatever the tungstenite Text payload type is.
    let _ = ws.send(Message::Text(json.into())).await;
}
```

- [ ] **Step 5: Write the binary entrypoint**

Create `crates/simulator/src/main.rs`:
```rust
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8765";
    let listener = TcpListener::bind(addr).await.expect("bind sim port");
    println!("simulator listening on ws://{addr}");
    simulator::serve(listener).await;
}
```

- [ ] **Step 6: Run the test to verify it passes**

Run:
```bash
devbox run -- cargo test -p simulator --test handshake
```
Expected: PASS — `client_receives_one_selfid_frame_on_connect` green.

- [ ] **Step 7: Verify the whole host workspace builds**

Run:
```bash
devbox run -- cargo build --workspace
```
Expected: PASS — `contract` + `simulator` build together.

- [ ] **Step 8: Commit**

```bash
git add crates/simulator Cargo.lock
git commit -m "feat(simulator): WebSocket software molecule emits SelfId on connect"
```

---

## Task 4: Codegen — generate `contract.gen.ts` from the contract (`just gen`)

**Realizes:** FR-3 (generated web type, no duplicate), SC-6 (zero duplicate definitions).

**Files:**
- Modify: `crates/contract/Cargo.toml`
- Modify: `crates/contract/src/lib.rs`
- Modify: `justfile`
- Create: `web/src/contract.gen.ts` (generated, committed)

**Interfaces:**
- Consumes: `contract::SelfId` (Task 2).
- Produces: `just gen` regenerates `web/src/contract.gen.ts`, a file exporting a TypeScript `interface SelfId { id: string; name: string; fw_version: string; capabilities: string[] }`, prefixed with a fixed `// @generated by \`just gen\` — do not edit` banner. Consumed by the web app (Task 6) and the drift guard (Task 5).

- [ ] **Step 1: Add the `codegen` feature and its dependencies to the contract**

In `crates/contract/Cargo.toml`, replace the `[dependencies]` and `[features]` sections with:
```toml
[dependencies]
serde = { version = "1", default-features = false, features = ["derive", "alloc"] }
tsify-next = { version = "0.5", optional = true }
wasm-bindgen = { version = "=0.2.100", optional = true }  # EXACT pin — must equal devbox wasm-bindgen-cli version (caret "0.2.100" drifts to a newer patch and breaks the CLI match)

[dev-dependencies]
serde_json = "1"

[features]
default = []
std = ["serde/std"]
codegen = ["std", "dep:tsify-next", "dep:wasm-bindgen"]
```

- [ ] **Step 2: Gate the tsify derive behind `codegen` in the contract**

In `crates/contract/src/lib.rs`, replace the `SelfId` definition (and add the imports above it) so the tsify derive is applied only under `codegen`:
```rust
#[cfg(feature = "codegen")]
use tsify_next::Tsify;
#[cfg(feature = "codegen")]
use wasm_bindgen::prelude::wasm_bindgen;

/// The molecule's self-description — the spine every later rung extends.
#[cfg_attr(feature = "codegen", derive(Tsify))]
#[cfg_attr(feature = "codegen", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SelfId {
    pub id: String,
    pub name: String,
    pub fw_version: String,
    pub capabilities: Vec<String>,
}
```
(Leave the `#![cfg_attr(...)]` header, `extern crate alloc;`, the `alloc`/`serde` `use` lines, and the `#[cfg(test)] mod tests` from Task 2 unchanged. The `wasm_bindgen` import is referenced by the macro expansion of `tsify(into_wasm_abi)`.)

- [ ] **Step 3: Verify the codegen build compiles to wasm**

Run (note `cargo rustc … --crate-type cdylib` — this builds a cdylib **only** for this wasm invocation, without putting cdylib in `Cargo.toml`, so host builds stay clean):
```bash
devbox run -- cargo rustc -p contract --no-default-features --features codegen --target wasm32-unknown-unknown --crate-type cdylib
```
Expected: PASS — produces `target/wasm32-unknown-unknown/debug/contract.wasm`. (Pin the `wasm-bindgen` dependency to exactly the devbox `wasm-bindgen-cli` version — `0.2.100` — so the CLI and crate match. If `tsify-next` `0.5` / `wasm-bindgen` `0.2.100` versions are incompatible with each other or with the devbox `wasm-bindgen-cli`, align all three: set the `wasm-bindgen` dependency to the exact version printed by `devbox run -- wasm-bindgen --version`, and pick the `tsify-next` release that depends on that `wasm-bindgen`. Re-run until green.)

- [ ] **Step 4: Add the `gen` recipe to the justfile**

In `justfile`, append:
```just
# Regenerate the web's TypeScript contract type from the Rust contract (SSOT).
gen:
    #!/usr/bin/env bash
    set -euo pipefail
    out="$(mktemp -d)"
    cargo rustc -p contract --no-default-features --features codegen \
        --target wasm32-unknown-unknown --crate-type cdylib
    wasm-bindgen --target bundler --out-dir "$out" --out-name contract \
        target/wasm32-unknown-unknown/debug/contract.wasm
    mkdir -p web/src
    {
        echo "// @generated by \`just gen\` — do not edit"
        cat "$out/contract.d.ts"
    } > web/src/contract.gen.ts
    rm -rf "$out"
    echo "wrote web/src/contract.gen.ts"
```

- [ ] **Step 5: Run codegen and verify the generated type**

Run:
```bash
devbox run -- just gen
```
Expected: PASS — writes `web/src/contract.gen.ts`. Then verify the type is present:
```bash
grep -A4 "interface SelfId" web/src/contract.gen.ts
```
Expected: shows an `export interface SelfId` block with `id: string`, `name: string`, `fw_version: string`, `capabilities: string[]`. (wasm-bindgen also emits init/boilerplate declarations in the same `.d.ts`; that is harmless — Task 6 imports `SelfId` as a type-only import and never touches the rest.)

- [ ] **Step 6: Commit**

```bash
git add crates/contract/Cargo.toml crates/contract/src/lib.rs justfile web/src/contract.gen.ts Cargo.lock
git commit -m "feat(contract): generate web SelfId type via tsify (just gen)"
```

---

## Task 5: Drift guard — fail if the generated type is stale (`just check-gen`)

**Realizes:** FR-7 (drift check, part of the single repeatable build), SC-4 (zero drift / fail on mismatch).

**Files:**
- Modify: `justfile`

**Interfaces:**
- Consumes: `just gen` (Task 4) and the committed `web/src/contract.gen.ts`.
- Produces: `just check-gen` — regenerates the type and fails (non-zero exit) if the result differs from what is committed. Consumed by `just check` (Task 8) as the CI/acceptance gate.

- [ ] **Step 1: Add the `check-gen` recipe**

In `justfile`, append:
```just
# Drift guard: regenerate and fail if the committed type is stale (SC-4).
check-gen:
    #!/usr/bin/env bash
    set -euo pipefail
    just gen
    if ! git diff --exit-code -- web/src/contract.gen.ts; then
        echo "ERROR: web/src/contract.gen.ts is stale — run \`just gen\` and commit." >&2
        exit 1
    fi
    echo "contract.gen.ts is up to date"
```

- [ ] **Step 2: Verify it passes against the committed file**

Run:
```bash
devbox run -- just check-gen
```
Expected: PASS — "contract.gen.ts is up to date", exit 0 (the file was just regenerated and committed in Task 4, so there is no diff).

- [ ] **Step 3: Verify it fails on drift**

Run:
```bash
# Simulate drift by hand-editing the generated file, then prove the guard catches it.
printf '\n// tampered\n' >> web/src/contract.gen.ts
devbox run -- just check-gen; echo "exit=$?"
```
Expected: FAIL — prints the "stale" error and `exit=1`. Then restore:
```bash
git checkout -- web/src/contract.gen.ts
devbox run -- just check-gen
```
Expected: PASS again.

- [ ] **Step 4: Commit**

```bash
git add justfile
git commit -m "feat: drift guard for generated contract type (just check-gen)"
```

---

## Task 6: Web workbench — connect to the sim and show id + name

**Realizes:** FR-5 (web shows id + name), SC-3 (byte-for-byte what the sim sent).

**Files:**
- Create: `web/` (Vite React-TS scaffold via command)
- Create: `web/src/selfId.ts`
- Test: `web/src/selfId.test.ts`
- Modify: `web/src/App.tsx`
- Modify: `web/package.json` (add vitest)

**Interfaces:**
- Consumes: `web/src/contract.gen.ts` (Task 4) — `import type { SelfId }`; the simulator WebSocket at `ws://127.0.0.1:8765` (Task 3).
- Produces: `parseSelfId(raw: string): SelfId` in `web/src/selfId.ts`; an `App` component that connects, parses one frame with `parseSelfId`, and renders `id` and `name`.

- [ ] **Step 1: Scaffold the Vite React-TS app**

Run:
```bash
devbox run -- npm create vite@latest web -- --template react-ts
devbox run -- npm --prefix web install
```
Expected: creates `web/` with `package.json`, `vite.config.ts`, `src/App.tsx`, etc.; installs dependencies into `web/node_modules`. (If the scaffold reports `web/` is not empty because `web/src/contract.gen.ts` already exists, choose to **ignore/continue** so the generated file is preserved; if the tool refuses, move `web/src/contract.gen.ts` aside, scaffold, then move it back.)

- [ ] **Step 2: Re-generate the contract type into the scaffolded app**

Run:
```bash
devbox run -- just gen
```
Expected: PASS — ensures `web/src/contract.gen.ts` exists alongside the scaffold (idempotent; restores it if the scaffold overwrote `src/`).

- [ ] **Step 3: Add vitest to the web app**

Run:
```bash
devbox run -- npm --prefix web install -D vitest
```
Then in `web/package.json`, add a `test` script to the `"scripts"` object:
```json
"test": "vitest run"
```

- [ ] **Step 4: Write the failing parser test**

Create `web/src/selfId.test.ts`:
```ts
import { describe, it, expect } from "vitest";
import { parseSelfId } from "./selfId";

describe("parseSelfId", () => {
  it("parses a SelfId frame into a typed object", () => {
    const raw =
      '{"id":"mol-001","name":"Mini-Molecule","fw_version":"0.1.0","capabilities":[]}';
    const parsed = parseSelfId(raw);
    expect(parsed.id).toBe("mol-001");
    expect(parsed.name).toBe("Mini-Molecule");
    expect(parsed.capabilities).toEqual([]);
  });
});
```

- [ ] **Step 5: Run the test to verify it fails**

Run:
```bash
devbox run -- npm --prefix web test
```
Expected: FAIL — cannot resolve `./selfId` (module not written yet).

- [ ] **Step 6: Write the parser using the generated type**

Create `web/src/selfId.ts`:
```ts
import type { SelfId } from "./contract.gen";

/** Parse a raw WebSocket frame into the generated SelfId type. */
export function parseSelfId(raw: string): SelfId {
  return JSON.parse(raw) as SelfId;
}
```

- [ ] **Step 7: Run the test to verify it passes**

Run:
```bash
devbox run -- npm --prefix web test
```
Expected: PASS — `parseSelfId` test green.

- [ ] **Step 8: Wire the App to the simulator**

Replace `web/src/App.tsx` with:
```tsx
import { useEffect, useState } from "react";
import type { SelfId } from "./contract.gen";
import { parseSelfId } from "./selfId";

const SIM_URL = "ws://127.0.0.1:8765";

export default function App() {
  const [selfId, setSelfId] = useState<SelfId | null>(null);
  const [status, setStatus] = useState("connecting…");

  useEffect(() => {
    const ws = new WebSocket(SIM_URL);
    ws.onmessage = (ev) => {
      setSelfId(parseSelfId(ev.data as string));
      setStatus("connected");
    };
    ws.onerror = () => setStatus("disconnected");
    ws.onclose = () => setStatus((s) => (s === "connected" ? s : "disconnected"));
    return () => ws.close();
  }, []);

  return (
    <main style={{ fontFamily: "system-ui", padding: "2rem" }}>
      <h1>Mini-Molecule Workbench</h1>
      <p>status: {status}</p>
      {selfId ? (
        <dl>
          <dt>id</dt>
          <dd data-testid="id">{selfId.id}</dd>
          <dt>name</dt>
          <dd data-testid="name">{selfId.name}</dd>
        </dl>
      ) : (
        <p>waiting for the molecule to announce itself…</p>
      )}
    </main>
  );
}
```

- [ ] **Step 9: Verify the web app type-checks and builds**

Run:
```bash
devbox run -- npm --prefix web run build
```
Expected: PASS — `tsc` + Vite build succeed (proves `App.tsx` and `selfId.ts` consume the generated `SelfId` type with no type errors).

- [ ] **Step 10: Commit**

```bash
git add web
git commit -m "feat(web): workbench connects to sim and shows SelfId id + name"
```

---

## Task 7: Firmware — Embassy/RP2350, imports contract, build-only

**Realizes:** FR-6 (firmware compiles board-free), SC-2 (hardware-free). Retires the Embassy-ramp risk at rung zero.

**Files:**
- Create: `firmware/Cargo.toml`
- Create: `firmware/.cargo/config.toml`
- Create: `firmware/build.rs`
- Create: `firmware/memory.x`
- Create: `firmware/src/main.rs`

**Interfaces:**
- Consumes: `contract::SelfId` (Task 2) with `default-features = false` (pure no_std).
- Produces: an Embassy binary for `thumbv8m.main-none-eabihf` that imports and references `contract::SelfId`, links board-free. Built (not run) by `just build` (Task 8).

- [ ] **Step 1: Create the firmware workspace manifest**

Create `firmware/Cargo.toml` (its own workspace — `[workspace]` with no members makes it standalone, and the host root already `exclude`s it):
```toml
[workspace]

[package]
name = "firmware"
version = "0.1.0"
edition = "2021"

[dependencies]
contract = { path = "../crates/contract", default-features = false }

embassy-executor = { version = "0.6", features = ["arch-cortex-m", "executor-thread"] }
embassy-rp = { version = "0.2", features = ["rp235xa", "time-driver"] }
cortex-m-rt = "0.7"
panic-halt = "1"
embedded-alloc = "0.6"
```
> Note: `embassy-rp` RP2350 (`rp235xa`) support and exact feature/version names move quickly. If the crates.io versions above don't resolve or don't expose `rp235xa`, pin Embassy to a git revision and copy the feature set from the upstream `examples/rp235x` (e.g. `embassy-rp = { git = "https://github.com/embassy-rs/embassy", rev = "<rev>", features = ["rp235xa", "time-driver"] }`). The verification build in Step 6 is the gate — adjust versions until it is green.

- [ ] **Step 2: Create the RP2350 memory layout**

Create `firmware/memory.x`:
```
MEMORY {
    FLASH : ORIGIN = 0x10000000, LENGTH = 2048K
    RAM   : ORIGIN = 0x20000000, LENGTH = 512K
}
```
> Note: RP2350 requires an embedded image header/boot block that the Embassy `rp235xa` support and its linker script provide. If linking fails for missing boot/image sections, replace this `memory.x` with the exact one from Embassy's current `examples/rp235x` for the pinned Embassy revision (it is kept in lockstep with the linker script Embassy ships).

- [ ] **Step 3: Emit the linker script from build.rs**

Create `firmware/build.rs`:
```rust
use std::fs;
use std::path::PathBuf;

fn main() {
    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    fs::write(out.join("memory.x"), include_bytes!("memory.x")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
}
```

- [ ] **Step 4: Configure the default target and link args**

Create `firmware/.cargo/config.toml`:
```toml
[build]
target = "thumbv8m.main-none-eabihf"

[target.thumbv8m.main-none-eabihf]
rustflags = [
    "-C", "link-arg=--nmagic",
    "-C", "link-arg=-Tlink.x",
]
```

- [ ] **Step 5: Write the firmware entrypoint that imports the contract**

Create `firmware/src/main.rs`:
```rust
#![no_std]
#![no_main]

use contract::SelfId;
use embassy_executor::Spawner;
use embedded_alloc::LlffHeap as Heap;
use panic_halt as _;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize the heap so the contract's alloc-backed types link/run.
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 4096;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(core::ptr::addr_of_mut!(HEAP_MEM) as usize, HEAP_SIZE) }
    }

    let _p = embassy_rp::init(Default::default());

    // Prove the firmware shares the single-source-of-truth contract (FR-6).
    let _id = SelfId {
        id: "mol-001".into(),
        name: "Mini-Molecule".into(),
        fw_version: "0.1.0".into(),
        capabilities: Default::default(),
    };

    loop {
        core::hint::spin_loop();
    }
}
```
(`.into()` for the `String` fields and `Default::default()` for the `Vec` field come from the `alloc` crate the contract already pulls in — no extra dependency, and no `cortex-m` import needed because the idle loop uses `core::hint::spin_loop()`.)

- [ ] **Step 6: Verify the firmware builds board-free**

Run:
```bash
devbox run -- cargo build --manifest-path firmware/Cargo.toml
```
Expected: PASS — links an ELF for `thumbv8m.main-none-eabihf` with no board attached. (This is the Embassy-ramp gate. If it fails on linker/boot sections or missing `rp235xa`, iterate per the Step 1/Step 2 notes — sync `Cargo.toml`, `memory.x`, and link args with Embassy's current `examples/rp235x` for your pinned revision — until green.)

- [ ] **Step 7: Verify the host workspace still ignores firmware**

Run:
```bash
devbox run -- cargo build --workspace
```
Expected: PASS — host build (`contract` + `simulator`) succeeds and does **not** compile `firmware` (proves the two-workspace split: host `cargo build` never drags in the embedded crate).

- [ ] **Step 8: Commit**

```bash
git add firmware
git commit -m "feat(firmware): Embassy/RP2350 board-free build importing the contract"
```

---

## Task 8: The one-command spine — `just build`, `just check`, `just dev`, README

**Realizes:** FR-7 (single repeatable build + drift check), FR-8 (onboarding from clean checkout), SC-1 (≤15 min), SC-5 (one command, all four green).

**Files:**
- Modify: `justfile`
- Modify: `README.md`

**Interfaces:**
- Consumes: every prior task — host workspace, web app, firmware build, `just gen`/`check-gen`.
- Produces: `just build` (builds all four parts, fails if any one breaks), `just check` (`build` + `check-gen` + clippy + tests — the CI/acceptance entrypoint), `just dev` (runs simulator + Vite dev server).

- [ ] **Step 1: Add the `build` recipe (all four parts)**

In `justfile`, append:
```just
# Build all four parts; fails if ANY one part breaks (SC-5).
build:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo build --workspace                                   # contract + simulator
    cargo build --manifest-path firmware/Cargo.toml           # firmware (build-only)
    npm --prefix web ci || npm --prefix web install
    npm --prefix web run build                                # web
    echo "all four parts built"
```

- [ ] **Step 2: Verify `build` builds all four**

Run:
```bash
devbox run -- just build
```
Expected: PASS — host workspace, firmware, and web all build; prints "all four parts built". (This is SC-5: one command proves the workspace binds all four.)

- [ ] **Step 3: Add the `check` recipe (the acceptance gate)**

In `justfile`, append:
```just
# CI / acceptance entrypoint: build everything, guard drift, lint, test.
check:
    #!/usr/bin/env bash
    set -euo pipefail
    just build
    just check-gen
    cargo clippy --workspace -- -D warnings
    cargo clippy --manifest-path firmware/Cargo.toml -- -D warnings
    cargo test --workspace
    npm --prefix web test
    echo "check passed"
```

- [ ] **Step 4: Verify `check` passes end to end**

Run:
```bash
devbox run -- just check
```
Expected: PASS — builds all four, drift guard clean, clippy clean, Rust tests + web tests green, prints "check passed". (If clippy flags warnings, fix them in the relevant crate before proceeding — `-D warnings` is intentional.)

- [ ] **Step 5: Add the `dev` recipe (primary scenario)**

In `justfile`, append:
```just
# Run the simulator and the web dev server together — the "see the id in the browser" scenario.
dev:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo run -p simulator &
    SIM_PID=$!
    trap "kill $SIM_PID 2>/dev/null || true" EXIT
    npm --prefix web run dev
```

- [ ] **Step 6: Manually verify the primary scenario**

Run:
```bash
devbox run -- just dev
```
Expected: simulator prints `simulator listening on ws://127.0.0.1:8765`; Vite prints a local URL. Open the URL in Chrome — the page shows `status: connected`, `id: mol-001`, `name: Mini-Molecule` (SC-3: byte-for-byte what the sim sent). `Ctrl-C` stops both.

- [ ] **Step 7: Write the README onboarding walkthrough**

In `README.md`, add a "Quickstart (Foundation)" section documenting the clean-checkout path (FR-8):
```markdown
## Quickstart (Foundation)

This repo's dev environment is managed by [devbox](https://www.jetify.com/devbox).
From a clean checkout:

1. Install devbox + direnv (see below), then `cd` into the repo (direnv auto-activates),
   or prefix commands with `devbox run --`.
2. First build (installs the pinned toolchain + targets automatically):

   ```bash
   devbox run -- just build
   ```

3. Run the full acceptance gate (build + drift guard + lint + tests):

   ```bash
   devbox run -- just check
   ```

4. See the molecule in the browser:

   ```bash
   devbox run -- just dev
   ```

   Open the printed Vite URL in Chrome — it shows the simulator's `id` and `name`.

### Recipes

| Recipe           | Does                                                            |
|------------------|----------------------------------------------------------------|
| `just gen`       | regenerate `web/src/contract.gen.ts` from the Rust contract    |
| `just check-gen` | drift guard: fail if the generated type is stale               |
| `just build`     | build all four parts (contract, simulator, web, firmware)      |
| `just check`     | `build` + drift guard + clippy + tests — the CI/acceptance gate |
| `just dev`       | run the simulator + web dev server (the primary scenario)      |
```

- [ ] **Step 8: Final full-gate verification**

Run:
```bash
devbox run -- just check
```
Expected: PASS — "check passed". This is the acceptance criterion for the whole Foundation slice.

- [ ] **Step 9: Commit**

```bash
git add justfile README.md
git commit -m "feat: one-command spine (just build/check/dev) + README quickstart"
```

---

## Self-Review

**Spec coverage** (design-doc requirements → task):

| Requirement | Task |
|-------------|------|
| FR-1 four parts, one workspace | Tasks 2, 3, 6, 7 (parts); Task 8 `just build` (binds them) |
| FR-2 SelfId shape, empty capabilities | Task 2 |
| FR-3 generated web type, no duplicate | Task 4 |
| FR-4 sim advertises, no hardware | Task 3 |
| FR-5 web shows id + name | Task 6 |
| FR-6 firmware compiles board-free | Task 7 |
| FR-7 single repeatable build + drift check | Tasks 5, 8 |
| FR-8 onboarding from clean checkout | Tasks 1, 8 (README) |
| SC-1 ≤15 min clean checkout | Task 1 (pin), Task 8 (one command) |
| SC-2 100% hardware-free success | Tasks 3, 7 |
| SC-3 byte-for-byte id/name | Tasks 3, 6 |
| SC-4 zero drift / fail on mismatch | Task 5 |
| SC-5 one command, all four green | Task 8 |
| SC-6 zero duplicate definitions | Task 4 |

**Type consistency:** `SelfId { id, name, fw_version, capabilities }` is used identically in Task 2 (definition), Task 3 (`self_id()`), Task 4 (generated TS), Task 6 (`parseSelfId` / `App`), Task 7 (firmware literal). `simulator::serve` / `simulator::self_id` signatures match between Task 3's lib, its test, and Task 8's `dev` recipe. `just gen` / `check-gen` / `build` / `check` / `dev` recipe names are consistent across Tasks 4–8 and the README.

**Known risk areas (flagged inline, gated by verification steps):**
- tsify-next / wasm-bindgen / `wasm-bindgen-cli` version alignment (Task 4, Step 3) — the build step is the gate.
- Embassy RP2350 (`rp235xa`) crate versions, `memory.x`, and linker args (Task 7) — sync with Embassy's current `examples/rp235x`; the board-free build (Step 6) is the gate.
