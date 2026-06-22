---
type: concept
title: SAD — Mini-Molecule POC
created: 2026-06-22
updated: 2026-06-22
tags: [architecture, sad, poc, mini-molecule, software, rust, embassy, tsify, ai]
sources: [concepts/ard-mini-molecule-poc.md, concepts/poc-mini-molecule-cloud-workbench.md, concepts/adom-technical-architecture.md, sources/adom-decoded-and-poc-plan.md]
status: accepted
---

# SAD — Mini-Molecule POC

Solution Architecture Document for the **software** of the Mini-Molecule + Cloud Workbench POC. Holistic description of the chosen architecture. Frames against [ARD — Mini-Molecule POC](ard-mini-molecule-poc.md) and links to every ADR carrying a load-bearing decision.

## Context

A desk-scale board (a "molecule") self-identifies on a bus, is driven from a browser workbench, exposes remote measure/actuate, and carries an AI layer that turns natural language into a test plan and a plain-English readout — deliberately mirroring [Adom's real architecture](adom-technical-architecture.md) at desk scale ([POC concept](poc-mini-molecule-cloud-workbench.md)). This SAD covers the software control plane; the hardware build and literacy trail are owned elsewhere ([Learning Hardware as a Software Dev](learning-hardware-as-a-software-dev.md)).

## Drivers

The forces that shaped the architecture, from the ARD's quality attributes and constraints.

- **Stack fidelity** — the demo's value is that it reads as Adom's own stack; reuse `tsify`, `postcard-rpc`, `mcp2515`, `ads1220`, Embassy ([ARD](ard-mini-molecule-poc.md)).
- **It must actually work** — co-equal with fidelity; a working sim-backed demo is non-negotiable ([ARD](ard-mini-molecule-poc.md)).
- **Evolvability rung-by-rung** — adding a capability must not force new UI or a re-architecture ([ARD](ard-mini-molecule-poc.md)).
- **AI safety** — an LLM in the loop must never reach hardware with an invalid or out-of-bounds command ([ARD](ard-mini-molecule-poc.md)).
- **Solo, weekend-scale** — favor the shortest path to first light, with later capability deferred behind stable seams ([ARD](ard-mini-molecule-poc.md)).

## Logical view

- **Molecule firmware** — Rust + Embassy on RP2350. Owns the capability registry, executes commands, emits self-ID and sample frames ([ADR — Firmware](adr-firmware-rust-embassy.md)).
- **Capability registry** — the self-describing model `{id, name, fw_version, capabilities:[...]}`; the single domain spine every rung extends and the AI's tool schema ([ADR — Capability registry](adr-capability-registry.md)).
- **Shared contract types** — serde types defined once in Rust, TS generated via `tsify`; the single source of truth for firmware, simulator, bridge, and browser ([ADR — Message contract](adr-message-contract-tsify.md)).
- **Device simulator** — a Rust "software molecule" implementing the same registry + contract; an interchangeable peer of the real device ([ADR — Device simulator](adr-device-simulator.md)).
- **Browser workbench** — TS/React. Renders its controls *from* the capability descriptor; issues commands and subscriptions; hosts the run/permalink UI.
- **AI planner** — turns NL + capability descriptor into a validated, typed test-plan DSL; a second call narrates results ([ADR — AI constrained planner](adr-ai-constrained-planner.md)).
- **Host bridge (v2)** — Rust process bridging WebSocket↔serial/CAN; enters only when CAN multi-node is needed ([ADR — Phased control plane](adr-phased-control-plane.md)).

## Process view

How the components interact at runtime. Integration style named per Hohpe.

- **Connect & self-ID** — on connect the molecule (or sim) emits its capability descriptor; the browser builds its UI from it. The descriptor is also handed to the AI planner as context.
- **Command (request/response)** — browser → `command{request_id, capability, channel, op, args}` → device → `result{request_id, ...}`. Writes are idempotent where physics allows. Maps to **postcard-rpc *endpoints*** in v2.
- **Live read (subscription)** — browser → `subscribe{channel, rate}` → device pushes `sample` frames until `unsubscribe`. **Backpressure = device-side rate cap**; the device never emits faster than its cap. Maps to **postcard-rpc *topics*** in v2.
- **Lifecycle** — heartbeat/timeout detects unplug; the UI degrades to "disconnected" rather than hanging (Nygard).
- **AI run** — NL → planner emits typed plan → validate against registry + bounds → execute **sim-first**, then optionally on hardware with the *same* plan → second AI call writes the readout ([ADR — AI](adr-ai-constrained-planner.md)).
- **Transport** — v1: browser ⇄ Web Serial ⇄ firmware (JSON). v2: browser ⇄ WebSocket ⇄ Rust bridge ⇄ serial/CAN ⇄ firmware (postcard-rpc at the embedded seam). The *logical* protocol is identical across both ([ADR — Phased control plane](adr-phased-control-plane.md)).

## Data view

What data exists, who owns it, what crosses trust boundaries (Helland, Vernon).

- **Capability descriptor** — owned by the molecule (the aggregate); an immutable snapshot crossing to the browser/AI as a *value*, not a live reference.
- **Command / result / sample frames** — typed by the shared contract; identity via `request_id`; samples are values in flight, not shared state.
- **Run artifact** — `{validated plan + capability snapshot + result samples}`: immutable, content-addressed, the unit of reproducibility; replayable against the simulator ([ADR — Reproducible run artifacts](adr-reproducible-run-artifacts.md)).
- **Invariants** — "address only advertised capabilities," "channel in range," "PWM within limits" live *inside* the molecule aggregate and are echoed as TS-type guards in the browser and as pre-execution checks in the AI path.

## Deployment view

- **Repo** — a monorepo: a `contract` Rust crate (serde types + `tsify`), a `firmware` crate (Embassy), a `simulator` crate, and a `web` app (TS/React). The `contract` crate is the hub; `tsify` output is generated into the web app.
- **v1** — no server. Browser (static) + Web Serial + Pico. The simulator runs as a local process exposing a WebSocket.
- **v2** — add the Rust host bridge (local process / `hd-wsl2`-style agent) for CAN multi-node; the AI planner call runs in a thin backend/serverless function holding the API key, returning a validated plan executed client-side.
- **Blast radius** — desk-scale, single user; "on-call" is the developer. A device fault degrades to "disconnected"; a sim is always available as a fallback target.

## Cross-cutting

- **Observability** — structured logs of every command/result/sample; the logic analyzer is the *physical*-layer observability tool (owned by the build guide). A run artifact is the audit record of what was executed.
- **Security** — desk-local; the only secret is the LLM API key, kept server-side in the v2 AI function (never in the browser). The AI cannot exceed the capability schema, so prompt-injection cannot manufacture an out-of-schema hardware command.
- **Build/release** — `tsify` regen wired into the build so TS types are never hand-maintained; CI runs the workbench + AI path against the simulator with no hardware.

## Fitness functions

- **Contract conformance** — the same protocol test suite runs against simulator *and* device; any divergence fails. Runs in CI (sim) and on-bench (device). Tripping it = a contract bug or a sim/firmware drift.
- **Schema-drift guard** — CI regenerates `tsify` types and fails on any uncommitted diff. Guarantees firmware and browser types never silently diverge.
- **AI safety gate** — a test corpus of adversarial NL prompts must yield *zero* plans that pass validation while addressing an unadvertised capability or out-of-bounds value. Tripping it = a validation hole.
- **Capability-to-UI** — adding a capability in firmware must surface a control with no new UI screen; an automated check asserts the workbench renders an unknown-but-well-typed capability.

## Decision summary

Every load-bearing decision has its own ADR.

- [ADR — Phased control plane](adr-phased-control-plane.md) — Web Serial v1 → Rust bridge v2; the message contract is the invariant, not the topology.
- [ADR — Message contract via tsify](adr-message-contract-tsify.md) — Rust serde types → TS via `tsify` (single source of truth); JSON v1 / postcard-rpc v2 at the embedded seam.
- [ADR — Capability registry](adr-capability-registry.md) — self-describing capability model; browser UI renders from the capability list.
- [ADR — Firmware Rust + Embassy](adr-firmware-rust-embassy.md) — Rust + Embassy on RP2350; fidelity over ramp-cost, consciously accepted.
- [ADR — Device simulator](adr-device-simulator.md) — protocol-level Rust simulator sharing the contract; CI, hardware-free dev, AI sandbox.
- [ADR — AI constrained planner](adr-ai-constrained-planner.md) — AI emits a capability-schema'd, validated, sim-first test plan; never a raw actuator.
- [ADR — Reproducible run artifacts](adr-reproducible-run-artifacts.md) — immutable run artifacts, replayable against the simulator.

**How they interlock:** the capability registry (003) is the AI's guardrail (006); the simulator (005) is the AI's safety sandbox (006) and the replay engine (007); the shared contract (002) is what makes the phased topology (001) reversible.

## Risks and trade-offs

- **Chrome-only for v1** — accepted; cross-browser arrives with the v2 bridge.
- **Rust + Embassy ramp** — accepted in exchange for fidelity; de-risked by crate reuse and front-loading the toolchain ([ADR — Firmware](adr-firmware-rust-embassy.md)).
- **Two serializations (JSON v1, postcard v2)** — accepted; the *logical* contract is stable, so the cost is a translation layer at one seam, not a redesign.
- **Sim is not the hardware** — accepted; the contract-conformance fitness function bounds the false-confidence risk, but analog/physical faults remain the build guide's domain.

## See also

- [Spec — Project Foundation](../specs/spec-project-foundation.md) — the first increment realizing this SAD's Deployment & Build views as a trivial end-to-end seam.
- [ARD — Mini-Molecule POC](ard-mini-molecule-poc.md) — requirements this solution satisfies.
- [POC: Mini-Molecule + Cloud Workbench](poc-mini-molecule-cloud-workbench.md) — the originating concept.
- [Adom Technical Architecture](adom-technical-architecture.md) — the stack mirrored.
- [What the POC Builds — A Visual Explainer](../stories/poc-explainer-for-self.md) — companion narrative.
