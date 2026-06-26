---
type: concept
title: ADR — Protocol-level device simulator
created: 2026-06-22
updated: 2026-06-22
tags: [architecture, adr, poc, mini-molecule, simulator, testability, ci]
sources: [concepts/sad-mini-molecule-poc.md, concepts/ard-mini-molecule-poc.md]
status: accepted
---

# ADR — Protocol-level device simulator

## Decision

We will build a **software "molecule" simulator** — a Rust binary implementing the **same capability registry and the same serde contract** as the firmware, exposed over a WebSocket so the **browser cannot tell it from real hardware**. The simulator is a first-class, interchangeable peer of the real device, not a test stub.

## Context

Without a simulator, all software progress couples to a flaky breadboard and a Chrome-only serial port, CI is impossible, and the AI layer would experiment on hardware that [can be fried](ai-native-eda.md). The ARD requires hardware-free development and a testability fitness function ([ARD](ard-mini-molecule-poc.md)). The contract being a single source of truth ([ADR — Message contract](adr-message-contract-tsify.md)) makes a *protocol-identical* simulator cheap to keep honest. Framed by [SAD — Mini-Molecule POC](sad-mini-molecule-poc.md).

## Alternatives considered

- **No simulator — always develop against real hardware** — simplest, but couples software to a flaky breadboard, makes CI impossible, and means AI experiments run on a board that can be damaged.
- **Record/replay fixtures** — capture real device traffic and replay it. Lighter, but cannot respond to commands interactively — useless for driving the workbench or the AI live.
- **Full physics simulation (SPICE-like)** — simulate analog behavior so ADC values reflect real circuits. Overkill: this is a control-plane POC; it needs protocol-level responses, not electrical accuracy.

## Consequences

- **Enables** — web + AI development with no hardware attached; CI green against the sim; a *safe sandbox* for the AI planner ([ADR — AI](adr-ai-constrained-planner.md)); the replay engine for shareable runs ([ADR — Reproducible run artifacts](adr-reproducible-run-artifacts.md)); a demo that survives a flaky bus.
- **Costs** — the simulator must be maintained in lockstep with the firmware's behavior; a contract-conformance fitness function (same suite vs sim and device) is required to bound sim/device drift.
- **Locks in** — Martin's dependency inversion: the workbench depends on the protocol contract (an abstraction), and real-device / simulator / (v2) bridge are interchangeable implementations behind that seam — the same seam [ADR — Phased control plane](adr-phased-control-plane.md) promised.

## Status

`accepted`

## See also

- [SAD — Mini-Molecule POC](sad-mini-molecule-poc.md) — the solution this decision is part of.
- [ADR — Message contract via tsify](adr-message-contract-tsify.md) — the shared contract the sim implements.
- [ADR — AI constrained planner](adr-ai-constrained-planner.md) — runs sim-first inside this sandbox.
- [ADR — Reproducible run artifacts](adr-reproducible-run-artifacts.md) — replays runs against the sim.
