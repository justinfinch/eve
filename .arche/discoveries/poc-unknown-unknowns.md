---
type: discovery
title: "POC Unknown-Unknowns — Learning Hardware as a Software Dev"
created: 2026-06-22
updated: 2026-06-22
tags: [poc, unknown-unknowns, hardware-literacy, software-dev, pre-mortem, interview-prep]
sources: [concepts/poc-mini-molecule-cloud-workbench.md, queries/poc-hardware-bom.md, concepts/adom-technical-architecture.md, concepts/molecules-and-workcells.md, concepts/automated-remote-bring-up.md, entities/john-lauer.md, stories/poc-explainer-for-self.md, concepts/learning-hardware-as-a-software-dev.md]
topic: Potential unknown-unknowns in the Mini-Molecule POC concept
techniques: [Pre-Mortem / Failure Analysis, Assumption Reversal, First Principles Thinking, Yes-And Building, Cross-domain anti-bias pivots]
idea_count: 24
context_pages: [concepts/poc-mini-molecule-cloud-workbench.md, queries/poc-hardware-bom.md, concepts/adom-technical-architecture.md, concepts/molecules-and-workcells.md, concepts/automated-remote-bring-up.md, sources/adom-decoded-and-poc-plan.md]
---

# POC Unknown-Unknowns — Learning Hardware as a Software Dev

**Topic:** surface potential unknown-unknowns in the Mini-Molecule POC without diverging from the basic concept. **Goal that emerged in the first exchanges and reframed the whole session:** the user is a software developer; the POC's real deliverable is **hardware literacy and how to interface with hardware via code**, documented module-by-module — *not* a working rig. "Get it working" is explicitly a trap; the success metric is being able to *speak to* the hardware (e.g. in an Adom interview), not demo it. Constraint: stay on the recommended Concept A spine ([POC: Mini-Molecule + Cloud Workbench](../concepts/poc-mini-molecule-cloud-workbench.md)) with hardware already on order ([POC Hardware BOM](../queries/poc-hardware-bom.md)).

## Themes

### Theme 1 — The reframe: the deliverable is *literacy*, not a working rig

- **#2 Works ≠ Understood** — a live bus you can't explain is worse than a humbler one you deeply understand. The artifact isn't the deliverable; *the ability to speak to it* is. ([Adom Technical Architecture](../concepts/adom-technical-architecture.md))
- **#3 The all-in-one escape hatch is a trap** — the Waveshare shortcut removes exactly the gotchas (5V/3.3V MISO seam, crystal match) that are the hardware lessons. Optimizing for "working" strips out "understood." (Reversed later by #14.) ([POC Hardware BOM](../queries/poc-hardware-bom.md))
- **#4 The POC is a textbook you write by building** — each module ships a "what's actually happening in the hardware" write-up for future-you: physics, why-this-chip, failure modes, code↔silicon mapping. The repo's value is the documented learning trail. ([Visual Explainer story](../stories/poc-explainer-for-self.md))

### Theme 2 — Software-developer intuition traps (cognitive blind spots)

- **#5 Hardware has no try/catch** — a wrong value isn't a catchable exception; it's current going where it shouldn't, and the failure is physical and permanent. No `git reset`. Measure before you connect. ([POC Hardware BOM](../queries/poc-hardware-bom.md))
- **#6 "It compiled, so it's right" → the bug is off-screen** — firmware builds clean but the chip returns garbage: cold breadboard contact, wrong SPI mode (CPOL/CPHA), byte-order flip, unconnected MISO row. "Code is correct" should send you to the *physical* layer, not narrow the search. ([Adom Technical Architecture](../concepts/adom-technical-architecture.md))
- **#8 State lives in the physical world** — floating GPIO reads noise (no default 0); the MCP2515's config registers persist *inside the chip* across re-flashes; capacitors/power rails carry state (brown-out, missing decoupling cap → mystery resets). "Initialize your variables" has a hardware twin: pull resistors, register config, decoupling. ([POC Hardware BOM](../queries/poc-hardware-bom.md))
- *Inverse observation (#13):* not every software instinct betrays you — "fail fast on the riskiest assumption" **transfers correctly** to hardware.

### Theme 3 — The re-sequenced learning ladder

- **#9 Learning order ≠ feature order** — the POC's product order (self-ID → bridge → measure → AI) demos well but doesn't teach safely/cumulatively. Re-order by *hardware-concept difficulty and blast radius*; same modules, same parts — not a divergence. ([POC: Mini-Molecule + Cloud Workbench](../concepts/poc-mini-molecule-cloud-workbench.md))
- **#10 Build the instrument before the experiments** — pull the browser↔device loop forward; it's a software dev's home turf. Then every later hardware rung has a self-built, software-native readout. The control plane is *learning instrumentation*, not a feature to demo last. ([Adom Technical Architecture](../concepts/adom-technical-architecture.md) — [John Lauer](../entities/john-lauer.md)'s browser-to-serial throughline)
- **#11 Built-in ADC as variable-isolator** — read a pot via the Pico's built-in ADC first to validate the *entire* software loop with ~zero hardware risk, so swapping to the ADS1220/SPI leaves the SPI chip as the *only* new variable. Sequencing as binary-search debugger.
- **#12 Self-ID is the spine, not a rung** — make `{id, name, capabilities:[]}` the through-line every rung extends (`[gpio]` → `[gpio,adc]` → `[...pwm]` → over-CAN). Each lesson becomes a capability the molecule advertises — literally how an Adom "molecule" behaves. ([Molecules and Workcells](../concepts/molecules-and-workcells.md))
- **#13 Bloody-on-CAN / fail fast on the signature tech** — move CAN early (rung 2). It's the highest-gotcha *and* the Adom-signature bus; a stall there is the most informative stall, and the struggle is the curriculum for the protocol you most need to speak to. ([Adom Technical Architecture](../concepts/adom-technical-architecture.md), [Automated Remote Bring-Up](../concepts/automated-remote-bring-up.md))

**Re-sequenced ladder:** `0` blink+measure → `1` browser loop + self-ID spine (`[gpio]`) → `2` CAN (bloody early) → `3` built-in ADC (validate loop) → `4` ADS1220/SPI (isolate the hard bus) → `5` PWM/actuation → `6` AI layer. Discipline: validate each rung in isolation before stacking; each rung answers *"what hardware concept did I learn, and how does my code map to the silicon?"* in writing.

### Theme 4 — Tooling & observability (instruments to buy/use)

- **#7 Your debugger is a multimeter** — and ideally a ~$10 USB logic analyzer + PulseView. When the bus is silent you can't see *why* without physical instruments. The most important "part" for the *learning* goal isn't in the parts list. ([POC Hardware BOM](../queries/poc-hardware-bom.md))
- **#14 ⭐ The Golden Reference Node** — buy *one* Waveshare RP2350-CAN (known-good) and build the *other* node from separate parts. When the bus won't sync, swap to bisect "my hand-wired node + firmware" vs "the bus itself." Directly reverses #3: same part, opposite role — escape hatch if it's your only node, diagnostic oracle if it's your second.
- **#15 CAN-early makes the logic analyzer non-negotiable** — CAN failures are invisible (differential pair, ACK bits, bit-timing); the browser readout can't see them. The sequencing decision forces a BOM decision, time-sensitive while the order is open.
- **#23 Current draw & heat are vital signs** — a ~$10 USB power meter gives a *pre-code* health signal: near-zero current = nothing running; too-high + hot = backwards/shorted chip, cut power. The "smoke" in smoke test, quantified.

### Theme 5 — The arrival → bring-up ritual (procurement / receiving)

- **#1 The crystal lottery** — mismatched MCP2515 crystals (8 vs 16 MHz, by batch) make a single node look alive but never ACK across two; manifests as a *plausible software bug*. The real risk is misattribution, not the crystal. ([POC Hardware BOM build note](../queries/poc-hardware-bom.md))
- **#16 ⭐ No package-lock.json for atoms** — no hash guarantees you got the right bytes. Every part is guilty until proven innocent (clones, mislabels, wrong crystals, DOA, missing SMD). Adopt manufacturing's *receiving inspection*: the box arriving is "ready to audit," not "ready to build."
- **#17 Listing lies, silkscreen half-lies, chip tells the truth** — a reliability hierarchy of evidence. Decode chip top-markings with a loupe/macro and cross-check the datasheet (confirm ADS1220 is SPI, Pico is genuine RP2350 not RP2040, transceiver is TJA1050). ([POC Hardware BOM](../queries/poc-hardware-bom.md))
- **#18 Read the crystal cans first** — 30-second flip-and-read of both MCP2515 crystals, label with tape. Pre-empts #1's misattribution spiral by front-loading the physical fact before any code exists to blame.
- **#19 ⭐ ESD: invisible, delayed, self-misattributing** — software devs have zero ESD reflexes; static causes *latent* damage (works today, flaky next week, dead next month) you'll blame on firmware. Nearly-free mitigation: touch grounded metal, avoid sliding boards in bags, $5 wrist strap.
- **#20 Unit-test before integration — for atoms** — every part gets a standalone "is it alive?" gate before joining a circuit; a failing part never contaminates a multi-part debug. Order: power alone → no heat/smell → confirm a vital sign → integrate.
- **#21 ⭐ Chips ship with self-test modes** — the MCP2515 LOOPBACK mode routes its own TX→RX internally: prove the CAN node (chip + SPI + firmware + crystal) on *one* node, no bus, no second node. Plus write-register-then-read-back. The datasheet's "modes" section is a bring-up toolbox. Dissolves the CAN-early risk (#13). ([Adom Technical Architecture](../concepts/adom-technical-architecture.md))
- **#22 Verify against math, not an instrument you also don't trust** — short ADS1220 inputs → read ≈0; known divider → code must equal `(Vin/Vref)×2²³` by hand. A wrong number means wrong wiring/config; a right number means you understand the conversion well enough to explain it.
- **#24 Passive parts test with a meter only** — the BSS138 level shifter needs no code: apply 3.3V one side, measure the other. Knowing *which* parts are meter-only vs code-required is itself hardware literacy.

### 🧵 Cross-cutting thread — Misattribution

The session's deepest insight: in hardware, **physical failures masquerade as software bugs** (#1 crystal → "CAN bug", #19 ESD → "flaky firmware", #6 wiring → "code bug"). For a software dev the meta-skill is *learning to suspect the physical layer first* — and every tool and ritual above exists to make the physical layer observable so you stop blaming code.

## Top ideas (promoted)

- **The whole literacy approach (Themes 1–3 + misattribution thread)** — new concept page created: [Learning Hardware as a Software Dev](../concepts/learning-hardware-as-a-software-dev.md).
- **Arrival ritual + tooling (Themes 5 & 4)** — promoted to [POC Hardware BOM](../queries/poc-hardware-bom.md) (arrival/smoke-test ritual + logic analyzer, USB power meter, 2nd Waveshare reference node, ESD strap).
- **Re-sequenced ladder + literacy reframe** — promoted to [POC: Mini-Molecule + Cloud Workbench](../concepts/poc-mini-molecule-cloud-workbench.md) (learning-order ladder, CAN-early, self-ID-as-spine, pointer to the new learning page).

## Technique narrative

**Pre-Mortem / Failure Analysis** opened the session ("it's interview day and the demo fails — rewind the tape") and immediately triggered the session's pivotal reframe: the user redefined "failure" from *non-working* to *not-understood*, relocating the whole topic from electrical risk to **competence risk**. **Yes-And Building** then grew that into the "POC as textbook" and "all-in-one is a trap" ideas. **Assumption Reversal** surfaced the software-intuition traps (Theme 2) by flipping premises a software dev treats as obviously true (try/catch, compiles-means-correct, state-in-variables). **First Principles** on the module dependency graph produced the re-sequenced ladder (Theme 3) and the realization that the host↔device boundary is *easy mode* for a software dev and belongs early.

**Anti-bias domain pivots** did real work: moving from cognitive traps → *structure/sequencing* → *procurement/logistics* unlocked Theme 5, the most concrete and time-sensitive vein (hardware literally on order). Two creative breakthroughs both *reversed earlier ideas*: **#14 Golden Reference Node** turned the "Waveshare = learning-killer" (#3) into the best CAN debugging tool, and **#21 LOOPBACK self-test** dissolved the CAN-early risk (#13) the structure phase had introduced — the session arguing with itself productively.

## See also

Cited inline during the session:
- [POC: Mini-Molecule + Cloud Workbench](../concepts/poc-mini-molecule-cloud-workbench.md)
- [POC Hardware BOM](../queries/poc-hardware-bom.md)
- [Adom Technical Architecture](../concepts/adom-technical-architecture.md)
- [Molecules and Workcells](../concepts/molecules-and-workcells.md)
- [Automated Remote Bring-Up](../concepts/automated-remote-bring-up.md)
- [John Lauer](../entities/john-lauer.md)
- [What the POC Builds — A Visual Explainer](../stories/poc-explainer-for-self.md)

Ideas promoted to:
- [Learning Hardware as a Software Dev](../concepts/learning-hardware-as-a-software-dev.md) (new)
- [POC Hardware BOM](../queries/poc-hardware-bom.md)
- [POC: Mini-Molecule + Cloud Workbench](../concepts/poc-mini-molecule-cloud-workbench.md)
</content>
</invoke>
