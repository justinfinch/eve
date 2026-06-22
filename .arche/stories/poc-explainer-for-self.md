---
type: story
title: "What the POC Builds — A Visual Explainer"
created: 2026-06-17
updated: 2026-06-22
tags: [poc, explainer, mini-molecule, interview-prep, visual]
companion: stories/poc-hardware-manual-for-self.md
sources: [concepts/poc-mini-molecule-cloud-workbench.md, concepts/adom-technical-architecture.md, concepts/programmable-wiring.md, concepts/automated-remote-bring-up.md, concepts/molecules-and-workcells.md, sources/adom-decoded-and-poc-plan.md, entities/adom-industries.md, entities/john-lauer.md]
audience: Justin (self) — understanding the POC before building / interviewing
audience_depth: internal
action_ask: "After reading, be able to explain what each POC concept builds and what the full POC demonstrates — i.e. that it replicates Adom's whole product loop at desk scale."
framework: pyramid
format: narrative
time_budget: ~1500 words, scroll-through
html: assets/stories/poc-explainer-for-self.html
---

# What the POC Builds — A Visual Explainer

> **Audience.** Justin (self) — building intuition for the POC, not pitching it.
> **The ask.** Be able to explain, from memory, what each concept builds and what the full POC demonstrates.
> **Framework.** Pyramid (answer-first).  **Format.** Narrative (scrollable).  **Budget.** ~1500 words.

## Outline

1. The one-liner — the POC is a desk-scale working replica of Adom's whole product loop, browser-driven. ([POC](../concepts/poc-mini-molecule-cloud-workbench.md))
2. The full system diagram — end-to-end data path from browser to component-under-test. ([POC](../concepts/poc-mini-molecule-cloud-workbench.md))
3. What each Concept builds — A/B/C/D, each mapped to the real Adom piece it mirrors. ([POC](../concepts/poc-mini-molecule-cloud-workbench.md), [Adom Technical Architecture](../concepts/adom-technical-architecture.md))
4. What the full POC delivers — the 5 pillars + the v1→v2 build path. ([POC](../concepts/poc-mini-molecule-cloud-workbench.md))
5. The "so what" — why it's "I built a mini version of your product on your stack," not a generic demo. ([POC](../concepts/poc-mini-molecule-cloud-workbench.md), [Adom, Decoded](../sources/adom-decoded-and-poc-plan.md))

## Style

- **Narrative shape** — long-form explainer (single scrollable page), because the goal is study/reference, not live presentation.
- **Diagram tools** — (a) **inline SVG** for the hero end-to-end architecture topology (custom shapes, the spatial flow *is* the claim); (b) **CSS + HTML** cards for the per-concept breakdown and the 5-pillar grid; (c) **CSS + HTML** for the v1→v2 progression. No Mermaid — the architecture is a bespoke linear topology, not a stock flowchart.
- **Accent color** — teal/cyan (novelty + technical), dark-mode-first, respects `prefers-color-scheme`.

## Sections

### 1. The one-liner

Adom's thesis is "the AWS of electronics prototyping" — a cloud-connected, robot-run factory where engineers design, wire, and test real electronics from a browser instead of owning a lab, per [Adom Industries](../entities/adom-industries.md) citing [Adom, Decoded](../sources/adom-decoded-and-poc-plan.md). The POC's design principle is to **mirror that real architecture at desk scale** — every piece is a recognizable slice of the actual product, not a generic Arduino demo, per [POC: Mini-Molecule + Cloud Workbench](../concepts/poc-mini-molecule-cloud-workbench.md). So the one-liner: **the POC builds a small modular board ("molecule") that self-identifies on a bus and is driven from a browser, with an AI layer that runs a test and explains the result** ([POC](../concepts/poc-mini-molecule-cloud-workbench.md)).

### 2. The full system diagram

The end-to-end path (Concept A spine + optional B), copied from the POC's own architecture sketch: Browser Workbench (TS/React) ⇄ WebSocket ⇄ Host Bridge (Rust, serial/SocketCAN) ⇄ USB serial / CAN ⇄ Molecule firmware (Rust + Embassy on RP2040/RP2350) ⇄ ADC + component-under-test (+ optional CH446Q crosspoint), per [POC](../concepts/poc-mini-molecule-cloud-workbench.md). The pragmatic v1 collapses the middle: browser → Web Serial API → Pico directly (Chrome-only) — the browser-to-serial pattern [John Lauer](../entities/john-lauer.md) pioneered with SPJS/ChiliPeppr, per [POC](../concepts/poc-mini-molecule-cloud-workbench.md). Rendered as inline SVG in the HTML.

**What it physically does (callout under the diagram).** The board's declared capabilities are `adc, gpio, pwm`, so the v1 demo is concrete: **flash/toggle an LED** (GPIO), **dim that LED or spin a small DC fan/motor** (PWM), and **read a sensor — pot or thermistor — and stream the live value to the browser** (ADC), per [POC](../concepts/poc-mini-molecule-cloud-workbench.md). Optional add-ons go further: Concept B software-wires a component into the measurement path then measures it ([Programmable Wiring](../concepts/programmable-wiring.md)); Concept D actually turns stepper motors via Gcode ([POC](../concepts/poc-mini-molecule-cloud-workbench.md)). The point isn't the blinking LED — it's that the whole loop (browser → board → real circuit → live readout → AI explanation → permalink) runs end to end. Exact demo parts in [POC hardware query](../queries/poc-hardware-bom.md).

### 3. What each Concept builds

Each concept maps to a real Adom subsystem decoded in [Adom Technical Architecture](../concepts/adom-technical-architecture.md):

- **Concept A — Mini-Molecule + Cloud Workbench (the spine).** Builds the full vertical slice: a board that (1) **self-identifies** on a bus (`{id, name, capabilities:[adc, gpio, pwm]}` on power-up), (2) is **driven from a browser** over a serial/CAN→WebSocket bridge (echoing `slcanx` + `tsify` + a Hydrogen-Desktop-style local agent), (3) does **remote measurement + actuation** (read an ADC channel, toggle GPIO, set PWM, stream a value live), and (4) has an **AI layer** turning natural language into a test plan plus a plain-English readout, with a **shareable permalink** for reproducibility. Firmware is Rust + Embassy on an RP2040/RP2350, CAN via MCP2515 — mirrors Adom's Rust/Embassy firmware + CAN-FD bus + Rust→TS control plane, per [POC](../concepts/poc-mini-molecule-cloud-workbench.md) and [Adom Technical Architecture](../concepts/adom-technical-architecture.md).
- **Concept B — Programmable Patch Matrix.** Builds a CH446Q analog crosspoint (the ~$2 Jumperless chip) so the browser can programmatically wire a component into the measurement path, then measure it — "wire it in software, measure it remotely." It's the closest thing to a **workcell in miniature**, mirroring Adom's robot-wiring, per [POC](../concepts/poc-mini-molecule-cloud-workbench.md) and [Programmable Wiring](../concepts/programmable-wiring.md).
- **Concept C — Remote Bring-Up Box.** Builds an Arduino test fixture that power-cycles a board-under-test, pokes an input, measures output, runs an automated pass/fail suite, and reports to a dashboard — hobby-scale "CI/CD for hardware," aimed squarely at the [Automated Remote Bring-Up](../concepts/automated-remote-bring-up.md) white space, the genuinely unsolved problem, per [POC](../concepts/poc-mini-molecule-cloud-workbench.md).
- **Concept D — Gcode Molecule Mover.** Builds a 2-axis motion rig driven by Gcode typed in the browser → serial → stepper pulses, echoing Adom's Klipper 8-axis fork. Coolest robotics signal, most build time, per [POC](../concepts/poc-mini-molecule-cloud-workbench.md) and [Adom Technical Architecture](../concepts/adom-technical-architecture.md).

CSS card grid in the HTML, one card per concept, each tagged with the Adom piece it mirrors and a build-effort marker.

### 4. What the full POC delivers

The recommendation is **build A as the spine, add B if time allows, keep C's pass/fail framing in the README**, per [POC](../concepts/poc-mini-molecule-cloud-workbench.md). Together **A + B demonstrate five Adom pillars** — molecule + programmable wiring + remote measurement + AI + reproducibility — for a total BOM of ~$20–30, per [POC](../concepts/poc-mini-molecule-cloud-workbench.md). Build path: **v1** = browser → Web Serial → Pico, serial only (a couple of weekends); **v2** = add CAN + Rust bridge + AI layer, per [POC](../concepts/poc-mini-molecule-cloud-workbench.md). Rendered as a 5-pillar CSS grid plus a v1→v2 progression strip.

### 5. The "so what"

The killer move is building on Adom's **own** MIT/Apache-licensed crates — `postcard-rpc` (messaging), `mcp2515` (CAN), `tsify` (Rust→TS types), Embassy (firmware) — which turns "I made a demo" into "I used your stack to build a mini version of your product," per [POC](../concepts/poc-mini-molecule-cloud-workbench.md) citing [Adom, Decoded](../sources/adom-decoded-and-poc-plan.md). That framing is the whole point of the exercise.

## The ask (closing)

After reading: be able to sketch the end-to-end path from memory and say, for each concept, what it builds and which Adom subsystem it mirrors — and why A+B together prove the five pillars on Adom's own stack.

## See also

- **Companion (hardware build):** [POC Hardware Manual — Build It, Understand It](./poc-hardware-manual-for-self.md) — this story covers *what* the POC builds; the manual is the *how-to-build-and-understand-the-hardware* counterpart (per-part reference, arrival ritual, rungs 0–6, diagnosis playbook). Keep the facts in sync when either is re-rendered.
- **Companion:** `/README.md` at the repo root shares this story's structure but is written in an **external resume voice** (first-person, addressed to an Adom reviewer), whereas this story is internal/self-facing. Keep the *facts* in sync when re-rendering; do **not** flatten the README's voice back to this one.
- [POC: Mini-Molecule + Cloud Workbench](../concepts/poc-mini-molecule-cloud-workbench.md)
- [Adom Technical Architecture](../concepts/adom-technical-architecture.md)
- [Programmable Wiring](../concepts/programmable-wiring.md)
- [Automated Remote Bring-Up](../concepts/automated-remote-bring-up.md)
- [Molecules and Workcells](../concepts/molecules-and-workcells.md)
- [Adom, Decoded — Problem Space + a POC of Their Actual App](../sources/adom-decoded-and-poc-plan.md)
- [Adom Industries](../entities/adom-industries.md)
- [John Lauer](../entities/john-lauer.md)
