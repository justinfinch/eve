# Design — Mini-Molecule Foundation Monorepo Setup

**Date:** 2026-06-25
**Status:** Approved (brainstorming) → ready for implementation plan
**Realizes:** [Spec — Project Foundation](../../../.arche/specs/spec-project-foundation.md) (the WHAT/WHY)
**Descends from:** [SAD — Mini-Molecule POC](../../../.arche/concepts/sad-mini-molecule-poc.md) + its ADRs (the settled HOW)

## Purpose

The concrete, technical realization of the Project Foundation spec: a single repository
skeleton that does one trivial thing **end to end** — a software molecule announces who it
is, and a browser workbench shows it — with no hardware attached. This document fixes the
scaffolding decisions the SAD leaves open (directory layout, task orchestration, codegen
wiring, web tooling) so the spec's functional requirements (FR-1..FR-8) and success criteria
(SC-1..SC-6) are met.

Scope this pass: **the full Foundation seam** — scaffold *and* implement the working
end-to-end self-ID slice, not just an empty skeleton.

## Settled upstream (not re-litigated here)

From the SAD Deployment view and the accepted ADRs:

- Monorepo, four parts: `contract` (Rust crate, the SSOT hub), `simulator` (Rust crate),
  `web` (TS/React app), `firmware` (Rust/Embassy crate, build-only this pass).
- The **contract is the single source of truth**; TS is *generated* from the Rust contract
  via **tsify** (the Adom-fidelity ADR — non-negotiable), with no hand-maintained duplicate.
- **One command builds all four parts green** plus a **drift check** that fails if
  regenerating the web type would differ from what's committed.
- Firmware compiles board-free; simulator is a first-class WebSocket peer; no hardware.
- Devbox already provides `rustup`, `nodejs`, `probe-rs-tools`, `gh`.

## Decisions made during brainstorming

| Decision | Choice | Why |
|----------|--------|-----|
| Scope | Full Foundation seam (working slice) | Brainstorming lands on a plan; the slice is the deliverable |
| Cross-language task runner | **`just`** (justfile) | Language-agnostic, tiny, devbox-friendly, Rust-primary fit |
| Workspace layout | **Two Cargo workspaces** (host + firmware) | Clean Embassy fit; host `cargo build`/`test` never drags in the no_std embedded crate |
| Web stack | Vite + React + TS, npm | Default per SAD ("TS/React") |
| Sim transport | tokio + tokio-tungstenite (WebSocket) | SAD: "simulator runs as a local process exposing a WebSocket" |
| Codegen extraction | `wasm-bindgen-cli` + `wasm32-unknown-unknown` | The mechanism tsify requires to emit the `.d.ts` |

## 1. Directory layout

```
eve/
├── Cargo.toml                 # HOST workspace: contract + simulator
├── Cargo.lock
├── justfile                   # the single entrypoint for all parts
├── devbox.json                # + just, + wasm-bindgen-cli
├── rust-toolchain.toml        # pins toolchain + wasm32 & thumbv8m targets
├── crates/
│   ├── contract/              # no_std serde types — the SSOT hub
│   │   └── src/lib.rs         # SelfId { id, name, fw_version, capabilities: [] }
│   └── simulator/             # std bin: WebSocket "software molecule"
│       └── src/main.rs
├── firmware/                  # SEPARATE workspace (own Cargo.toml + .cargo/config.toml)
│   ├── Cargo.toml
│   ├── .cargo/config.toml     # default target = thumbv8m.main-none-eabihf
│   └── src/main.rs            # embassy-rp, no_std, build-only this pass
└── web/                       # Vite + React + TS workbench
    ├── package.json
    └── src/
        ├── contract.gen.ts    # GENERATED from contract — committed, drift-checked
        └── App.tsx            # connects to sim, shows id + name
```

## 2. The single-source-of-truth seam (FR-3, SC-4, SC-6)

- `contract` is **`no_std` + serde**, with an optional `codegen` feature that pulls in
  `tsify` + `wasm-bindgen` *only* for type export. The same crate is therefore importable by
  `no_std` firmware **and** able to drive TS generation.
- `firmware` depends on `contract` with `default-features = false` (pure no_std).
  `simulator` depends on it with the `std` feature.
- **Codegen flow:** `just gen` builds `contract` to `wasm32-unknown-unknown` with the
  `codegen` feature, runs `wasm-bindgen` to emit the `.d.ts`, and writes the type into
  `web/src/contract.gen.ts` (committed). The web app uses these types at compile time only;
  runtime is plain JSON over the wire (v1), so no wasm ships.
- **Drift guard:** `just check-gen` regenerates into a temp path and `git diff --exit-code`s
  against the committed file — fails on any difference (SC-4). Zero hand-maintained
  duplicates (SC-6).

## 3. The runtime seam (FR-4, FR-5, SC-2, SC-3)

- `simulator` = a `std` binary (tokio + tokio-tungstenite) that, on WebSocket connect, emits
  one `SelfId { id, name, fw_version, capabilities: [] }` as JSON — no hardware.
- `web` = Vite + React + TS, connects to the sim's WebSocket, parses the frame as the
  *generated* `SelfId` type, and displays **id + name** — byte-for-byte what the sim sent
  (SC-3).

## 4. Firmware (FR-6) — build-only

- `embassy-rp` on RP2350, `no_std`, imports `contract`. Compiles for
  `thumbv8m.main-none-eabihf` with no board attached; never on the primary-scenario path
  this pass. Retires the Embassy-ramp risk at rung zero. (`probe-rs` is already in devbox for
  later flashing.)

## 5. `just` recipes — the one-command spine (FR-7, SC-5)

| Recipe | Does |
|--------|------|
| `just gen` | regenerate `contract.gen.ts` from the contract |
| `just check-gen` | regenerate to temp + `git diff --exit-code` (drift guard) |
| `just dev` | run simulator + Vite dev server → the primary "see the id in the browser" scenario |
| `just build` | build **all four**: host workspace, web, firmware (build-only) — green or fail |
| `just check` | `build` + `check-gen` + clippy/test → the CI/acceptance entrypoint |

`just build` failing if *any* one part breaks is exactly SC-5 (proves the workspace binds
them, not that they pass independently).

## 6. devbox additions

`devbox add just wasm-bindgen-cli`; `rust-toolchain.toml` pins `wasm32-unknown-unknown` +
`thumbv8m.main-none-eabihf` so a clean checkout has the targets needed (SC-1, ≤15 min target).

## Requirement traceability

| Spec requirement | Where met |
|------------------|-----------|
| FR-1 four parts, one workspace | §1 layout, §5 `just build` |
| FR-2 SelfId shape, empty capabilities | §2 contract `SelfId` |
| FR-3 generated web type, no duplicate | §2 codegen flow |
| FR-4 sim advertises, no hardware | §3 simulator |
| FR-5 web shows id + name | §3 web |
| FR-6 firmware compiles board-free | §4 firmware |
| FR-7 single repeatable build+drift check | §5 `just check` |
| FR-8 onboarding from clean checkout | §6 toolchain pin + README (plan step) |
| SC-1 ≤15 min clean checkout | §6 |
| SC-2 100% hardware-free success | §3 |
| SC-3 byte-for-byte id/name | §3 |
| SC-4 zero drift / fail on mismatch | §2 drift guard |
| SC-5 one command, all four green | §5 `just build` |
| SC-6 zero duplicate definitions | §2 |

## Open implementation details (for the plan, not decisions)

- Exact tsify-next vs tsify crate selection and `codegen` feature gating in `contract`.
- `wasm-bindgen` invocation details to emit only the `.d.ts` (discard the wasm).
- Simulator connection/heartbeat lifecycle (kept minimal this pass).
- README content for the primary-scenario walkthrough (FR-8).

## See also

- [Spec — Project Foundation](../../../.arche/specs/spec-project-foundation.md)
- [SAD — Mini-Molecule POC](../../../.arche/concepts/sad-mini-molecule-poc.md)
- [ADR — Message contract via tsify](../../../.arche/concepts/adr-message-contract-tsify.md)
- [ADR — Phased control plane](../../../.arche/concepts/adr-phased-control-plane.md)
- [ADR — Firmware Rust + Embassy](../../../.arche/concepts/adr-firmware-rust-embassy.md)
- [ADR — Device simulator](../../../.arche/concepts/adr-device-simulator.md)
