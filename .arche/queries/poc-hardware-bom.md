---
type: query
title: "What Hardware Do I Need to Buy (POC BOM)"
created: 2026-06-17
updated: 2026-06-17
tags: [poc, bom, hardware, parts-list, interview-prep, mini-molecule]
sources: [concepts/poc-mini-molecule-cloud-workbench.md, concepts/adom-technical-architecture.md, concepts/programmable-wiring.md, sources/adom-decoded-and-poc-plan.md]
---

# What Hardware Do I Need to Buy (POC BOM)

> what hardware do I need to buy

## Answer

Hardware is scoped entirely around the recommended **"Mini-Molecule + Cloud Workbench"** POC — build **Concept A** as the spine, add **Concept B** only if time allows — per [POC: Mini-Molecule + Cloud Workbench](../concepts/poc-mini-molecule-cloud-workbench.md) citing [Adom, Decoded](../sources/adom-decoded-and-poc-plan.md). Part numbers below mirror the Adom chip families named in [Adom Technical Architecture](../concepts/adom-technical-architecture.md) (RP2040/RP2350, MCP2515/MCP2518FD, ADS1220/ADS123x) and were cross-checked against current vendor availability (June 2026, web — see external sources at bottom).

### Concept A — core build

| Part | Qty | ~Price | Notes / search term |
|---|---|---|---|
| **Raspberry Pi Pico 2** (RP2350) | 1–2 | $5–6 ea | "Raspberry Pi Pico 2 RP2350"; buy pre-soldered headers. Maxes stack signal via Rust + Embassy firmware ([POC](../concepts/poc-mini-molecule-cloud-workbench.md)). |
| **MCP2515 CAN module** (w/ transceiver) | 2 | $7–10/pair | "MCP2515 CAN module"; need two for a real bus. v2 only — v1 runs over USB serial. Mirrors Adom's CAN-FD bus ([Adom Technical Architecture](../concepts/adom-technical-architecture.md)). |
| **ADS1220 24-bit ADC breakout** | 1 | $10–15 | "ADS1220 ADC module"; mirrors Adom's `ads1220` precision ADC ([Adom Technical Architecture](../concepts/adom-technical-architecture.md)). Cheaper alt: ADS1115 (~$7). |
| **USB cable** (→ micro-USB, *data*) | 1 | $5 | Pico 2 is micro-USB; confirm data-capable. |
| **Breadboard** (830-point) | 1–2 | $6/pair | "830 breadboard" |
| **Jumper wires** (M-M/M-F/F-F kit) | 1 kit | $7 | "dupont jumper wire kit" |
| **Component-under-test grab bag** (resistors, pot, LEDs, thermistor, small DC fan/motor for PWM) | 1 | $10–13 | "electronics component starter kit"; gives the ADC something to read and PWM to drive. |

**Core subtotal: ~$40–55** on Amazon Prime — above the Arche's ~$20–30 estimate ([POC](../concepts/poc-mini-molecule-cloud-workbench.md)), which assumed salvage / AliExpress pricing.

**Shortcut:** the **Waveshare RP2350-CAN board** (~$10–18) combines an RP2350 + MCP2515-clone + CAN transceiver on one Pico-sized board — collapses the Pico + CAN module into one part per node; buy two.

### Concept B — programmable patch matrix (optional stretch)

The raw **CH446Q** (~$1, the Jumperless chip per [Programmable Wiring](../concepts/programmable-wiring.md) citing [Adom, Decoded](../sources/adom-decoded-and-poc-plan.md)) is an LQFP-44 SMD part — cheap but not breadboard-friendly without a breakout. Two paths:

- **Easiest:** buy a **Jumperless V5 breadboard** (built from CH446Q arrays) — Concept B in a box, ~$300.
- **Cheap but fiddly:** CH446Q chip + LQFP-44→DIP adapter, hand-soldered, ~$5.

**Recommendation:** build A first, leave B as a README stretch goal unless the wiring demo is specifically wanted — matching the Arche's own A-spine-plus-optional-B call ([POC](../concepts/poc-mini-molecule-cloud-workbench.md)).

### Gaps

The Arche specifies *what* to buy and an approximate total but no exact SKUs, vendor links, or quantities for the supporting parts (ADC, component-under-test, wires, breadboard, cables); those were filled from web vendor data, not the Arche, and prices/availability will drift. The pragmatic **v1 (browser → Web Serial → Pico, no CAN, no bridge)** needs only the Pico + ADC + supporting parts — the CAN modules and crosspoint are deferrable ([POC](../concepts/poc-mini-molecule-cloud-workbench.md) citing [Adom, Decoded](../sources/adom-decoded-and-poc-plan.md)).

## See also

- [POC: Mini-Molecule + Cloud Workbench](../concepts/poc-mini-molecule-cloud-workbench.md)
- [Adom Technical Architecture](../concepts/adom-technical-architecture.md)
- [Programmable Wiring](../concepts/programmable-wiring.md)
- [Adom, Decoded — Problem Space + a POC of Their Actual App](../sources/adom-decoded-and-poc-plan.md)
- External (web, June 2026): [Waveshare RP2350-CAN (CNX Software)](https://www.cnx-software.com/2025/04/21/raspberry-pi-pico-sized-rp2350-can-development-board-features-a-clone-of-the-mcp2515-can-bus-controller/), [Raspberry Pi Pico 2](https://www.raspberrypi.com/products/raspberry-pi-pico-2/), [CH446Q on LCSC](https://www.lcsc.com/product-detail/Analog-Switches-Multiplexers_WCH-Jiangsu-Qin-Heng-CH446Q_C109471.html), [Jumperless (DigiKey)](https://www.digikey.com/en/maker/projects/jumperless/2e62cc88ee6145bc924041dd486de76f)
