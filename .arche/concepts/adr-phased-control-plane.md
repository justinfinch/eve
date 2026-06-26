---
type: concept
title: ADR — Phased control plane (Web Serial v1 → Rust bridge v2)
created: 2026-06-22
updated: 2026-06-22
tags: [architecture, adr, poc, mini-molecule, control-plane, web-serial, can]
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

## Status

`accepted`

## See also

- [SAD — Mini-Molecule POC](sad-mini-molecule-poc.md) — the solution this decision is part of.
- [ADR — Message contract via tsify](adr-message-contract-tsify.md) — the invariant this ADR depends on.
- [ADR — Device simulator](adr-device-simulator.md) — a third transport peer behind the same contract.
