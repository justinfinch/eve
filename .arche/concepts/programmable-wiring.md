---
type: concept
title: Programmable Wiring
created: 2026-06-17
updated: 2026-06-17
tags: [prior-art, switch-fabric, crosspoint, relay-matrix, jumperless, physics-limits]
sources: [sources/adom-decoded-and-poc-plan.md]
---

# Programmable Wiring

The problem of letting software connect any circuit node to any other — a **switch fabric** — and the physics limits that cap it, which Adom sidesteps by using robots to physically wire modules ([source](../sources/adom-decoded-and-poc-plan.md)).

## Explanation

"Programmable wiring" means a switch fabric: either **relay matrices** (instrument-grade — Pickering, up to 300V/2A, thousands of crosspoints, but bulky/expensive) or **analog crosspoint ICs** (ADI ADG2128 = 96 switches; the cheap **CH446Q** used by the open-source **Jumperless** programmable breadboard) ([source](../sources/adom-decoded-and-poc-plan.md)).

The hard limits are physics: every solid-state switch adds ~tens of ohms and parasitic capacitance, caps current to ~0.1A and voltage to ~±9–20V, and a full any-to-any crossbar grows as **N² switches**. Adom sidesteps the pure-crossbar ceiling by using **robots to physically wire modules** — which is why the robotics hire matters and ties back to [Molecules and Workcells](molecules-and-workcells.md) ([source](../sources/adom-decoded-and-poc-plan.md)).

## Examples

- Relay matrices (Pickering, 300V/2A, thousands of crosspoints) ([source](../sources/adom-decoded-and-poc-plan.md)).
- Analog crosspoint ICs: ADI ADG2128 (96 switches); CH446Q (~$2, the Jumperless chip) ([source](../sources/adom-decoded-and-poc-plan.md)).
- The CH446Q is the basis for [POC](poc-mini-molecule-cloud-workbench.md) Concept B (programmable patch matrix) ([source](../sources/adom-decoded-and-poc-plan.md)).

## See also

- [Molecules and Workcells](molecules-and-workcells.md)
- [Remote Labs (Prior Art)](remote-labs-prior-art.md)
- [POC: Mini-Molecule + Cloud Workbench](poc-mini-molecule-cloud-workbench.md)
