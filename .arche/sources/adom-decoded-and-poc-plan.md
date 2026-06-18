---
type: source
title: Adom, Decoded — Problem Space + a POC of Their Actual App
created: 2026-06-17
updated: 2026-06-17
tags: [adom, electronics-prototyping, robotics, rust, embedded, ai-native-eda, interview-prep, poc]
sources: []
raw: raw/adom-decoded-and-poc-plan.md
url:
---

# Adom, Decoded — Problem Space + a POC of Their Actual App

A research + strategy brief prepared for Justin Finch (June 2026) ahead of an interview with Adom Industries. It synthesizes a primary founder interview (Dallas Innovates, Aug 2025) and the company's public GitHub org to reconstruct what Adom is building, maps the problem space and prior art, and proposes a desk-scale proof-of-concept that mirrors Adom's real architecture — ideally built on Adom's own open-source Rust crates. Adom's thesis: **"the AWS of electronics prototyping"** — a cloud-connected, robot-run factory where engineers design, wire, and test real electronics from a browser, by the hour, instead of owning a lab. The core product primitive is the **"molecule"** (a modular PCB) that snaps into a factory **"workcell"** where robot pincers physically wire molecules together and bench instruments measure them, all driven remotely with AI agents layered on top. The brief decodes Adom's stack from GitHub (Rust/Embassy firmware, CAN-FD module bus, Klipper-style motion, precision ADCs, Rust→TypeScript→browser control plane), surveys the prior art (remote labs, device farms, instrument-control standards, programmable wiring, AI-native EDA), identifies automated remote bring-up/test as the genuinely unsolved white space, and recommends a "Mini-Molecule + Cloud Workbench" POC.

## Key claims

- Adom is building a utility-priced cloud factory for electronics prototyping — "compute for atoms" — recreating Shenzhen's dense prototyping ecosystem in a US cloud lab (Part 1, quoting founder John Lauer via Dallas Innovates, Aug 2025).
- The product primitives are **molecules** (modular PCBs chosen from a library and connected "like Legos") and **workcells** (cells where robot pincers wire molecules with real wire for prototype testing) (Part 1).
- Adom's engineering stack, inferred from the public `github.com/adom-inc` org (25 repos): Rust + Embassy embedded-async firmware; CAN-FD as the module bus (`mcp2518fd`/`mcp2515`, `slcanx`); Klipper fork (8-axis) for robot motion; precision delta-sigma ADCs (`ads1220`/`ads123x`) for measurement; `tsify` (Rust→TypeScript types) confirming a Rust↔browser control plane; KiCad/Gerber/Fusion libs for design automation; "Hydrogen Desktop" as a local client/agent (Part 2).
- Prior art exists but is limited: academic **remote labs** (VISIR, MIT iLab, commercialized by LabsLand) cap at small textbook circuits; **device farms** (AWS Device Farm) prove the UX only for phones; **instrument control** (SCPI/VISA/IVI/LXI) is a solved open standard, not the moat; **programmable wiring** via crosspoint ICs hits physics limits (N² switches, ~0.1A, ±9–20V) which Adom sidesteps with robots (Part 3).
- The **AI-native EDA** race is live and funded (Diode Computers, JITX, Quilter, Flux.ai, Cofactr); the shared hard problem is that LLMs hallucinate pins/structure and a 0.1% hardware error can fry a board — so real systems pair generation with deterministic validation (Part 3).
- The genuinely unsolved white space is **automated remote bring-up and test**; capital-intensity risk is real (cautionary tale: Tempo Automation) (Part 3).
- Recommended POC: **Concept A — "Mini-Molecule + Cloud Workbench"** (self-identifying modular board driven from a browser over serial/CAN bridge, with remote measurement/actuation and an AI test layer), optionally plus **Concept B — programmable patch matrix** (CH446Q crosspoint). The "killer move" is building on Adom's own MIT/Apache-licensed crates (`postcard-rpc`, `mcp2515`, `tsify`, Embassy fork) (Parts 4–5).
- Company context: founder John Lauer (career throughline: SMS carriers → Zipwhip toll-free texting → ChiliPeppr CNC-over-browser → Adom); ~11 people as of Aug 2025; self-funded ~$10M; Fort Worth $15M incentive within a $229M Project Nimbus plan to 2033; stealth with a 2026 public launch (Parts 1, 8).

## See also

- [Adom Industries](../entities/adom-industries.md)
- [John Lauer](../entities/john-lauer.md)
- [Molecules and Workcells](../concepts/molecules-and-workcells.md)
- [Adom Technical Architecture](../concepts/adom-technical-architecture.md)
- [Remote Labs (Prior Art)](../concepts/remote-labs-prior-art.md)
- [Device Farms](../concepts/device-farms.md)
- [Instrument-Control Standards](../concepts/instrument-control-standards.md)
- [Programmable Wiring](../concepts/programmable-wiring.md)
- [AI-Native EDA](../concepts/ai-native-eda.md)
- [Automated Remote Bring-Up](../concepts/automated-remote-bring-up.md)
- [POC: Mini-Molecule + Cloud Workbench](../concepts/poc-mini-molecule-cloud-workbench.md)
