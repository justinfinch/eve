---
type: concept
title: Learning Hardware as a Software Dev
created: 2026-06-22
updated: 2026-06-22
tags: [poc, hardware-literacy, software-dev, learning-ladder, misattribution, interview-prep]
sources: [discoveries/poc-unknown-unknowns.md, concepts/poc-mini-molecule-cloud-workbench.md, queries/poc-hardware-bom.md, concepts/adom-technical-architecture.md, concepts/molecules-and-workcells.md]
---

# Learning Hardware as a Software Dev

The Mini-Molecule POC's real deliverable is **hardware literacy and how to interface with hardware via code** — *not* a working rig. For a software developer, "get it working" is a trap: a live bus you can't explain is worse than a humbler one you deeply understand. The success metric is being able to *speak to* the hardware (e.g. in an Adom interview), so the build is structured as a documented learning trail, not a demo to make pass ([source](../discoveries/poc-unknown-unknowns.md)).

## Explanation

**The reframe — the POC is a textbook you write by building.** Each module ships a short "what's actually happening in the hardware" write-up aimed at future-you: the physics, why this chip, the failure modes, and the code↔silicon mapping. The repo's value is that trail, which is also the interview currency. This extends the existing self-directed [Visual Explainer story](../stories/poc-explainer-for-self.md) from "what it builds" toward "what I learned building it" ([source](../discoveries/poc-unknown-unknowns.md)).

**The software-developer intuition traps.** Three mental models a software dev carries that are silently false in hardware ([source](../discoveries/poc-unknown-unknowns.md)):

- **Hardware has no `try/catch`.** A wrong value isn't a catchable exception — it's current going where it shouldn't, and the failure is physical and permanent. There's no `git reset`. Measure before you connect.
- **"It compiled, so it's right" — the bug is off-screen.** Clean firmware can still produce garbage from a cold breadboard contact, the wrong SPI mode (CPOL/CPHA), a byte-order flip, or an unconnected MISO row. "The code is correct" should send you to the *physical* layer, not narrow the search.
- **State lives in the physical world, not in variables.** A floating GPIO reads noise (there is no default `0`); a chip's config registers (e.g. the MCP2515's) persist *inside the chip* across re-flashes; capacitors and power rails carry state (brown-out on motor start, a missing decoupling cap → mystery resets). "Initialize your variables" has a hardware twin: pull resistors, register config, decoupling.

Not every software instinct betrays you: **"fail fast on the riskiest assumption" transfers correctly** — it justifies tackling CAN early (below).

**The cross-cutting meta-skill — suspect the physical layer first.** The deepest pattern is **misattribution**: physical failures masquerade as software bugs. A crystal mismatch reads as a "CAN bug"; latent ESD damage reads as "flaky firmware"; a wiring fault reads as a "code bug." For a software dev the meta-skill is learning to suspect the physical layer first — and the tooling and arrival ritual in [POC Hardware BOM](../queries/poc-hardware-bom.md) exist to make the physical layer *observable* so you stop blaming code ([source](../discoveries/poc-unknown-unknowns.md)).

**The re-sequenced learning ladder.** Learning order ≠ feature order. The POC's product sequence (self-ID → bridge → measure → AI) demos well but doesn't teach safely or cumulatively. Re-ordered by hardware-concept difficulty and blast radius — same modules, same parts, *not* a divergence from the concept ([source](../discoveries/poc-unknown-unknowns.md)):

| Rung | Build | New hardware concept | Why here |
|---|---|---|---|
| **0** | Power Pico, blink *external* LED, **measure with a multimeter** | GPIO, current-limit resistor, 3.3V logic, the meter as debugger | safest first contact |
| **1** | Browser↔device loop (Web Serial) + **self-ID spine** `[gpio]` | serialization, host↔device boundary, capability announcement | a software dev's home turf — build the *instrument* first |
| **2** | **2-node CAN** (1 Waveshare reference + 1 hand-built) | bus arbitration, differential signaling, bit-timing | "bloody me early" on the Adom-signature, highest-gotcha tech |
| **3** | Pot via Pico's **built-in ADC** | analog↔digital, resolution, ref voltage | validates the whole software loop at ~zero hardware risk |
| **4** | Same pot via **ADS1220 over SPI** | SPI (CPOL/CPHA), register config, 24-bit delta-sigma, PGA | only the SPI chip is new → failures are unambiguous |
| **5** | **PWM a motor/fan** via transistor + flyback | PWM, current, transistors, power rails, back-EMF | actuation + power discipline |
| **6** | **AI layer** + shareable permalink | NL→test-plan, reproducibility | pure software, layered last |

Three disciplines make it a *learning* ladder, not just a build order ([source](../discoveries/poc-unknown-unknowns.md)):

1. **Build the instrument before the experiments.** The browser↔device loop comes early so every later rung has a self-built, software-native readout — the control plane is learning instrumentation, not a feature to demo last. It also signals you grasp [John Lauer](../entities/john-lauer.md)'s browser-to-serial throughline.
2. **Self-ID is the spine, not a rung.** `{id, name, capabilities:[]}` is the through-line every rung extends (`[gpio]` → `[gpio,adc]` → `[...pwm]` → over-CAN). Each lesson becomes a capability the molecule advertises — literally how an Adom [molecule](molecules-and-workcells.md) behaves.
3. **Bloody-on-CAN / fail fast on the signature tech.** CAN moves to rung 2 because it's the highest-gotcha *and* the bus Adom is built on ([Adom Technical Architecture](adom-technical-architecture.md)); a stall there is the most informative stall, and it's de-risked by the LOOPBACK self-test and Golden Reference Node in the [BOM](../queries/poc-hardware-bom.md).

## Examples

- **Misattribution in action:** a single CAN node looks alive in LOOPBACK, but two nodes never ACK — because their MCP2515 crystals differ (8 vs 16 MHz). A software dev hunts a "CAN bug" for hours; the fix was a 30-second crystal-can read at unboxing ([source](../discoveries/poc-unknown-unknowns.md)).
- **A software instinct that transfers:** "de-risk the riskiest assumption first" → do CAN early, on purpose, because the struggle is the curriculum for the protocol you most need to defend ([source](../discoveries/poc-unknown-unknowns.md)).

## See also

- [POC Unknown-Unknowns — Learning Hardware as a Software Dev](../discoveries/poc-unknown-unknowns.md) (origin discovery)
- [POC: Mini-Molecule + Cloud Workbench](poc-mini-molecule-cloud-workbench.md)
- [POC Hardware BOM](../queries/poc-hardware-bom.md)
- [Adom Technical Architecture](adom-technical-architecture.md)
- [Molecules and Workcells](molecules-and-workcells.md)
- [What the POC Builds — A Visual Explainer](../stories/poc-explainer-for-self.md)
</content>
