---
type: concept
title: ADR — Firmware on Rust + Embassy (RP2350)
created: 2026-06-22
updated: 2026-06-22
tags: [architecture, adr, poc, mini-molecule, firmware, rust, embassy, rp2350]
sources: [concepts/sad-mini-molecule-poc.md, concepts/ard-mini-molecule-poc.md]
status: accepted
---

# ADR — Firmware on Rust + Embassy (RP2350)

## Decision

We will write the molecule firmware in **Rust with the Embassy async framework on the RP2350**, reusing Adom's own driver crates (`mcp2515`, `ads1220`) and messaging (`postcard-rpc`) rather than writing equivalents. We consciously accept Rust + Embassy's steeper ramp and slower first light in exchange for stack fidelity.

## Context

The Arche is emphatic that Rust + Embassy "maxes stack signal" and is *the* Adom match — Embassy is the firmware foundation of Adom's stack ([Adom Technical Architecture](adom-technical-architecture.md)). The user flagged stack-fidelity as a co-equal success metric alongside a working demo ([ARD](ard-mini-molecule-poc.md)). The counter-pressure is the ramp cost, now that the *learning* deliverable has been moved out to the build guide and the demo must work. Framed by [SAD — Mini-Molecule POC](sad-mini-molecule-poc.md).

## Alternatives considered

- **MicroPython** — fastest to working, excellent REPL for iteration. Rejected: near-zero Adom signal — Adom is a Rust shop — so it undercuts the stack-fidelity co-goal that is half the point of the POC.
- **Arduino / C++** — familiar, huge library ecosystem, quick first light. Rejected: still not Adom's stack; C++ is not the signal, and you'd reimplement what Adom's Rust crates already provide.
- **Rust but RTIC / bare-metal** — keeps Rust fidelity but swaps Embassy's async model for interrupt-driven RTIC or raw registers. Rejected: loses the specific Embassy match, and async fits the rate-capped streaming-subscription design (SAD process view) more naturally.

## Consequences

- **Enables** — the headline pitch ("I wrote Embassy firmware on your stack, self-IDing over CAN with your `mcp2515`/`ads1220` drivers"); a clean async home for the subscription/streaming timers; direct crate reuse instead of driver authorship.
- **Costs** — the steepest toolchain ramp of the options; slowest path to first blink; embedded-Rust debugging is less forgiving than a REPL.
- **Locks in** — the Rust embedded toolchain and Embassy's async executor model as the firmware foundation.

**De-risking (accepted as part of this decision):** front-load the toolchain at rung 0/1 (blink + serial) so the pain is paid before anything subtle; reuse Adom's drivers rather than writing them; lean on the simulator ([ADR — Device simulator](adr-device-simulator.md)) so web/AI work never blocks on firmware.

## Status

`accepted`

## See also

- [SAD — Mini-Molecule POC](sad-mini-molecule-poc.md) — the solution this decision is part of.
- [Adom Technical Architecture](adom-technical-architecture.md) — the Embassy/crate provenance.
- [ADR — Device simulator](adr-device-simulator.md) — decouples software progress from the firmware ramp.
