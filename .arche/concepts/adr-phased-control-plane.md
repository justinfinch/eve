---
type: concept
title: ADR — Phased control plane (Web Serial v1 → Rust bridge v2)
created: 2026-06-22
updated: 2026-07-10
tags: [architecture, adr, poc, mini-molecule, control-plane, web-serial, can, bridge]
sources: [concepts/sad-mini-molecule-poc.md, concepts/ard-mini-molecule-poc.md]
status: accepted
---

# ADR — Phased control plane (Web Serial v1 → Rust bridge v2)

## Decision

We will build the browser↔device control plane in two phases — **v1: browser → Web Serial → Pico directly; v2: browser → WebSocket → Rust host bridge → serial/CAN → firmware** — and we will treat the **logical message contract as the invariant across both phases**. The topology is allowed to change; the protocol is not.

## Context

The [POC concept](poc-mini-molecule-cloud-workbench.md) names both shapes. v1 needs the shortest path to first light for a solo, weekend-scale build ([ARD](ard-mini-molecule-poc.md)); the full bridge is more upfront software. The forcing constraint: **Web Serial can reach the Pico's USB but cannot speak CAN**, so the *signature* multi-node CAN demo ([Adom Technical Architecture](adom-technical-architecture.md)) is impossible from the browser without a host bridge (or CAN-over-serial tunneling à la Adom's `slcanx`). Framed by [SAD — Mini-Molecule POC](sad-mini-molecule-poc.md).

## Alternatives considered

- **Rust host bridge from day 1** — full WebSocket↔serial/CAN bridge immediately. Max Adom fidelity and not Chrome-locked, but more upfront software and slower first light; pays the bridge cost before any of it is needed.
- **Web Serial only, never bridge** — simplest forever, but caps the system at single-node serial, Chrome-only, and makes the CAN multi-drop demo *permanently* impossible — it kills the signature-bus story.
- **Local native agent (Tauri / `hd-wsl2`-style)** — closest to Adom's client pattern and cross-browser, but the heaviest option to stand up for a POC; deferred, and the v2 bridge can grow into it.

## Consequences

- **Enables** — fastest first light (v1), while keeping the door open to CAN multi-node and cross-browser (v2) without touching firmware or browser protocol code.
- **Costs** — two transport adapters and two serializations over the system's life (see [ADR — Message contract](adr-message-contract-tsify.md)); Chrome-only until v2.
- **Locks in** — the *logical* protocol as a stable contract. That is the deliberate one-way door: every other component (sim, AI, workbench) is written against it, so getting the contract right now matters more than the topology.

## Update — 2026-07-10: v1 (Web Serial) skipped; built the bridge first

At Rung 1 of the hardware build (browser↔Pico loop; PR #4, branch `hardware/rung-1-browser-bridge`) we **consciously skipped v1 (Web Serial) and implemented v2 directly** — i.e. we chose the **"Rust host bridge from day 1"** alternative listed above after all. The topology built is `browser ⇄ WebSocket ⇄ Rust bridge (crates/bridge) ⇄ NDJSON-over-USB-serial ⇄ firmware`; JSON stays the wire encoding and `postcard-rpc` remains deferred (unchanged, per [ADR — Message contract](adr-message-contract-tsify.md)).

Why the phasing changed:

- **Adom fidelity.** A browser reaching straight into a serial port is the un-Adom shape; a browser calling an API that owns the hardware *is* Adom's pattern — [John Lauer](../entities/john-lauer.md)'s career-long move of putting physical resources behind a software API. The bridge is effectively the workbench's backend-for-frontend.
- **Unblocks Rung 2 (CAN) now, not later.** Web Serial can never reach CAN, so the bridge was mandatory regardless; building it at Rung 1 means Rung 2 adds *only* CAN, not CAN + bridge at once.
- **Lower cognitive friction for a software-dev learner.** WebSocket-to-an-API is home turf; the "shortest path to first light" that motivated v1 assumed the bridge was the expensive part, but for a software dev the hardware-serial handling (needed either way) is the real cost.
- **The sim and real device become symmetric peers** behind one WebSocket API, which is *more* faithful to [ADR — Device simulator](adr-device-simulator.md) than two browser transports would have been.

**The core decision held.** The ADR's thesis — *topology may change, the logical contract may not* — was validated on real RP2350 hardware: the same `SelfId`/`Command`/`Ack` contract flowed unchanged from simulator (WebSocket) to real firmware (serial via the bridge); only the courier changed. The Chrome-only limitation of v1 is also dropped as a bonus. See the Rung 1 write-up (`docs/lessons/hardware/writeups/rung-1-boundary.md`) for the bench story.

## Status

`accepted` — the two-phase framing stands; **v2 (Rust host bridge) is the implemented topology** as of Rung 1 (2026-07-10), reached by skipping the optional v1 stopgap.

## See also

- [SAD — Mini-Molecule POC](sad-mini-molecule-poc.md) — the solution this decision is part of.
- [ADR — Message contract via tsify](adr-message-contract-tsify.md) — the invariant this ADR depends on.
- [ADR — Device simulator](adr-device-simulator.md) — a third transport peer behind the same contract.
