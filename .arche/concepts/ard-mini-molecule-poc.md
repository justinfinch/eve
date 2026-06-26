---
type: concept
title: ARD — Mini-Molecule POC
created: 2026-06-22
updated: 2026-06-22
tags: [architecture, ard, poc, mini-molecule, software]
sources: [concepts/poc-mini-molecule-cloud-workbench.md, concepts/adom-technical-architecture.md, concepts/learning-hardware-as-a-software-dev.md, queries/poc-hardware-bom.md, sources/adom-decoded-and-poc-plan.md]
status: accepted
---

# ARD — Mini-Molecule POC

Architecture Requirements Document for the **software** of the Mini-Molecule + Cloud Workbench POC. Captures what any software architecture for this system must satisfy, ahead of any specific design. Paired with [SAD — Mini-Molecule POC](sad-mini-molecule-poc.md).

**Scope note.** This ARD covers the *software* control plane only. The hardware-literacy deliverable — the physics, the per-part learning, the bring-up ritual — is owned by the [POC Hardware Manual](../stories/poc-hardware-manual-for-self.md) and [Learning Hardware as a Software Dev](learning-hardware-as-a-software-dev.md), and is explicitly out of scope here. This session's success metric is **a working software demo that is also faithful to Adom's stack** ([POC concept](poc-mini-molecule-cloud-workbench.md)).

## Stakeholders

- **Builder (Justin)** — needs a demo that actually runs *and* software legible enough to extend rung by rung. Judges it on: does it work, and does it read as Adom's stack.
- **Adom interviewer (implicit audience)** — judges whether the repo reads as "built a mini version of our product on our actual stack" rather than a generic Arduino demo ([source](../sources/adom-decoded-and-poc-plan.md)).
- **Future-self maintainer** — needs to add a capability (a learning-ladder rung) without re-architecting the control plane or hand-building new UI.

## Functional requirements

What the system must do. No design.

- **Self-identification** — on connect, the molecule announces `{id, name, fw_version, capabilities:[...]}` ([Molecules and Workcells](molecules-and-workcells.md)).
- **Remote actuation & measurement** — from the browser: read an ADC channel, toggle a GPIO, set a PWM duty.
- **Live streaming** — subscribe to a channel and watch a measured value update live.
- **AI test planning** — natural language → a validated test plan → execution → a plain-English readout.
- **Reproducibility** — a shareable permalink that reproduces a prior run.
- **Hardware-free development** — a simulator interchangeable with the real device for dev, CI, and demos.
- **Transport evolution** — serial single-node for v1; CAN multi-node for v2 *without changing the logical protocol*.

## Quality attributes

Each as a scenario (**stimulus → environment → response → measure**, Bass).

- **Stack fidelity** — when an Adom engineer reads the repo, in normal review, they recognize their own stack, measured by *reuse of named crates* (`postcard-rpc`, `mcp2515`, `ads1220`, `tsify`, Embassy) — target: all five present in the build.
- **Workability** — when a user runs an AI- or hand-authored plan against the simulator, in CI or on desk, it completes deterministically, measured by *100% repeatable result on the sim*.
- **Evolvability** — when a new capability is added in firmware, at any rung, a corresponding control appears in the browser, measured by *zero new bespoke UI screens required* (Ford).
- **Schema integrity** — when a firmware message type changes, in CI, the TS types must not silently drift, measured by *a failing `tsify` regen/diff check*.
- **Safety** — when the AI proposes a plan, before execution, no plan addressing an unadvertised capability or an out-of-bounds value reaches hardware, measured by *100% pre-execution validation against the capability registry* ([AI-Native EDA](ai-native-eda.md): LLMs can fry boards).
- **Testability** — when the workbench + AI path is exercised, in CI with no hardware attached, it runs green against the simulator.
- **Latency (soft)** — when a command is issued over serial, on desk, the round-trip stays responsive, target ~50–100 ms.

## Constraints

- **Reuse Adom's MIT/Apache crates** where they fit: `postcard-rpc`, `mcp2515`, `ads1220`, `tsify`, the Embassy fork ([Adom Technical Architecture](adom-technical-architecture.md), [source](../sources/adom-decoded-and-poc-plan.md)).
- **Chrome-only Web Serial is acceptable for v1** — the browser-to-serial path [John Lauer](../entities/john-lauer.md) pioneered ([POC concept](poc-mini-molecule-cloud-workbench.md)).
- **Desk-scale BOM** (~$75–95), RP2350 (Pico 2 / Waveshare RP2350-CAN) ([POC Hardware BOM](../queries/poc-hardware-bom.md)).
- **Solo developer, weekend-scale increments** — v1 first light in a couple of weekends; CAN + Rust + AI as v2.
- **Hardware literacy is out of scope** — owned by the build guide; this ARD does not re-specify it.

## Assumptions

- Web Serial is sufficient for v1 single-node; the *signature* CAN multi-node demo requires the v2 host bridge (Web Serial cannot speak CAN).
- The capability set is small and known up front (`gpio`, `adc`, `pwm`), so a self-describing model is tractable, not speculative.
- A current LLM (Claude, structured tool-use) can reliably emit a schema-valid test plan given the capability descriptor as context.

## Risks

- **Rust + Embassy ramp eats the schedule** — likelihood medium / impact high / mitigation: front-load the toolchain at rung 0/1, reuse Adom's drivers instead of writing them ([ADR — Firmware Rust + Embassy](adr-firmware-rust-embassy.md)).
- **postcard-rpc in the browser is friction** — likelihood high / impact medium / mitigation: JSON over Web Serial for v1, postcard-rpc only at the Rust↔embedded seam in v2 ([ADR — Message contract via tsify](adr-message-contract-tsify.md)).
- **Sim/device divergence gives false confidence** — likelihood medium / impact high / mitigation: a contract-conformance fitness function that runs the same suite against sim and device ([ADR — Device simulator](adr-device-simulator.md)).
- **AI hallucination drives bad hardware** — likelihood high / impact high (physical, permanent) / mitigation: capability-schema validation + sim-first execution ([ADR — AI constrained planner](adr-ai-constrained-planner.md)).

## See also

- [SAD — Mini-Molecule POC](sad-mini-molecule-poc.md) — the solution designed against this ARD.
- [POC: Mini-Molecule + Cloud Workbench](poc-mini-molecule-cloud-workbench.md) — the concept this ARD formalizes.
- [Learning Hardware as a Software Dev](learning-hardware-as-a-software-dev.md) — the hardware-literacy deliverable, out of scope here.
- [Adom Technical Architecture](adom-technical-architecture.md) — the stack this mirrors.
