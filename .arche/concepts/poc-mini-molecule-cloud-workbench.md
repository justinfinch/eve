---
type: concept
title: "POC: Mini-Molecule + Cloud Workbench"
created: 2026-06-17
updated: 2026-06-22
tags: [poc, interview-prep, mini-molecule, rust, embassy, can, web-serial, ai-layer, architecture]
sources: [sources/adom-decoded-and-poc-plan.md, discoveries/poc-unknown-unknowns.md, concepts/learning-hardware-as-a-software-dev.md]
---

# POC: Mini-Molecule + Cloud Workbench

A desk-scale proof-of-concept that mirrors Adom's real architecture: a small modular board (a "molecule") that self-identifies on a bus and is driven from a browser, with an AI layer that runs a test and explains the result — ideally built on Adom's own open-source Rust crates ([source](../sources/adom-decoded-and-poc-plan.md)).

## Explanation

**Design principle:** mirror Adom's real architecture at desk scale — every concept is a recognizable piece of the actual product, not a generic Arduino demo ([source](../sources/adom-decoded-and-poc-plan.md)).

**★ Concept A — "Mini-Molecule + Cloud Workbench" (recommended spine).** A small modular board that (1) **self-identifies** on a bus, announcing `{id, name, capabilities:[adc, gpio, pwm]}` on power-up; (2) is driven from a **browser workbench** over a serial/CAN→WebSocket bridge (echoing `slcanx` + `tsify` + a Hydrogen-Desktop-style local agent); (3) exposes **remote measurement + actuation** (read an ADC channel, toggle GPIO, set PWM, stream a value live); and (4) has an **AI layer** turning natural language into a test plan plus a plain-English readout, with a **shareable permalink** for reproducibility. It is the end-to-end vertical slice — molecule + bus + cloud control plane + AI + reproducibility. Stack signal maxes if firmware is **Rust + Embassy on an RP2040/RP2350 (Raspberry Pi Pico)** with CAN via an MCP2515 module. Scope: a couple of weekends for serial v1; add CAN + Rust + AI as v2 ([source](../sources/adom-decoded-and-poc-plan.md)).

**★ Concept B — "Programmable Patch Matrix."** Relays or an analog **crosspoint IC** (CH446Q, ~$2, the Jumperless chip) let the browser programmatically wire a component into a measurement path, then measure it — "wire it in software, measure it remotely." Bolt onto A to demonstrate programmable wiring + remote measurement together; closest to a workcell in miniature. See [Programmable Wiring](programmable-wiring.md) ([source](../sources/adom-decoded-and-poc-plan.md)).

**Concept C — "Remote Bring-Up Box."** An Arduino as a test fixture: power-cycle a board-under-test, poke an input, measure output, run an automated pass/fail suite, report to a dashboard — hobby-scale "CI/CD for hardware," aimed at the [Automated Remote Bring-Up](automated-remote-bring-up.md) white space ([source](../sources/adom-decoded-and-poc-plan.md)).

**Concept D — "Gcode Molecule Mover."** Drive a 2-axis motion rig from Gcode typed in the browser → serial → stepper pulses, echoing the Klipper 8-axis fork. Coolest robotics signal but most build time ([source](../sources/adom-decoded-and-poc-plan.md)).

**Recommendation:** build **A as the spine, add B if time allows**; keep C's pass/fail framing in the README. Together A+B demonstrate five Adom pillars — molecule + programmable wiring + remote measurement + AI + reproducibility. Total BOM for a strong A+B build: ~$20–30 ([source](../sources/adom-decoded-and-poc-plan.md)).

**The killer move:** build on Adom's own MIT/Apache-licensed crates — `postcard-rpc` (messaging), `mcp2515` (CAN on a $2 module), `tsify` (Rust→TS types), Embassy (firmware) — turning "I made a demo" into "I used your stack to build a mini version of your product" ([source](../sources/adom-decoded-and-poc-plan.md)).

## Build order — sequenced for *learning*, not feature-completeness

A [discovery session](../discoveries/poc-unknown-unknowns.md) reframed the deliverable: for a software developer, the POC's real output is **hardware literacy**, not a working rig — "get it working" is a trap. That changes the *order* you build Concept A's modules (same modules, same parts — not a divergence). Build order ≠ the product order above; sequence by hardware-concept difficulty and blast radius, with three disciplines: **build the browser↔device instrument early** (it's a software dev's home turf, and it becomes your readout for every later rung), **make self-ID the spine every rung extends** (`[gpio]` → `[gpio,adc]` → over-CAN — exactly how a real [molecule](molecules-and-workcells.md) behaves), and **do CAN early on purpose** ("bloody me on the signature tech"), de-risked by LOOPBACK self-test + a Golden Reference Node. Full ladder, the software-developer intuition traps, and the misattribution meta-skill live in [Learning Hardware as a Software Dev](learning-hardware-as-a-software-dev.md); the arrival/smoke-test ritual and learning-tooling additions live in the [POC Hardware BOM](../queries/poc-hardware-bom.md) ([source](../discoveries/poc-unknown-unknowns.md)).

## Examples

- Architecture sketch (Concept A): Browser Workbench (TS/React) ⇄ WebSocket ⇄ Host Bridge (Rust, serial/SocketCAN) ⇄ USB serial / CAN ⇄ Molecule firmware (Rust + Embassy on RP2040/RP2350) ⇄ ADC + component-under-test (+ optional CH446Q crosspoint) ([source](../sources/adom-decoded-and-poc-plan.md)).
- Pragmatic v1 skips the Rust bridge: browser → Web Serial API → Pico directly (Chrome-only — the browser-serial path [John Lauer](../entities/john-lauer.md) pioneered) ([source](../sources/adom-decoded-and-poc-plan.md)).

## See also

- [ARD — Mini-Molecule POC](ard-mini-molecule-poc.md) — software requirements formalized from this concept.
- [SAD — Mini-Molecule POC](sad-mini-molecule-poc.md) — the software solution architecture (+ 7 ADRs) designed against the ARD.
- [Learning Hardware as a Software Dev](learning-hardware-as-a-software-dev.md)
- [POC Unknown-Unknowns discovery](../discoveries/poc-unknown-unknowns.md)
- [Adom Technical Architecture](adom-technical-architecture.md)
- [Molecules and Workcells](molecules-and-workcells.md)
- [Programmable Wiring](programmable-wiring.md)
- [Automated Remote Bring-Up](automated-remote-bring-up.md)
- [Adom Industries](../entities/adom-industries.md)
- Story: [What the POC Builds — A Visual Explainer](../stories/poc-explainer-for-self.md)
