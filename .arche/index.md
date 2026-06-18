---
type: index
created: 2026-06-16
updated: 2026-06-17
---

# Arche Index

Catalog of every page in this Arche. Read this first when answering queries. See [SCHEMA.md](SCHEMA.md) for conventions.

## Sources

- [Adom, Decoded — Problem Space + a POC of Their Actual App](sources/adom-decoded-and-poc-plan.md) — research + strategy brief reconstructing what Adom is building and proposing a desk-scale POC. `adom` `interview-prep` `poc`

## Entities

- [Adom Industries](entities/adom-industries.md) — stealth North Texas startup building "the AWS of electronics prototyping" (robot-run cloud factory). `adom` `company` `cloud-factory`
- [John Lauer](entities/john-lauer.md) — Adom founder; career pattern of putting physical resources behind a software API (Zipwhip, ChiliPeppr). `adom` `founder`

## Concepts

- [Molecules and Workcells](concepts/molecules-and-workcells.md) — Adom's core primitives: modular PCBs ("molecules") robot-wired in factory "workcells." `adom` `product-primitive`
- [Adom Technical Architecture](concepts/adom-technical-architecture.md) — stack decoded from GitHub: Rust/Embassy, CAN-FD, Klipper, ADCs, Rust→TS→browser. `adom` `architecture` `decoded`
- [Remote Labs (Prior Art)](concepts/remote-labs-prior-art.md) — VISIR/iLab/LabsLand; web-driven real circuits, capped by relay-matrix limits. `prior-art` `remote-labs`
- [Device Farms](concepts/device-farms.md) — AWS Device Farm et al. prove the UX but only for phones; custom electronics is white space. `prior-art` `device-farms`
- [Instrument-Control Standards](concepts/instrument-control-standards.md) — SCPI/VISA/IVI/LXI; solved and open, not the moat. `prior-art` `standards`
- [Programmable Wiring](concepts/programmable-wiring.md) — switch fabrics (relay matrices, crosspoint ICs) and their physics limits; Adom uses robots instead. `prior-art` `switch-fabric`
- [AI-Native EDA](concepts/ai-native-eda.md) — Diode/JITX/Quilter/Flux/Cofactr race; LLMs hallucinate and can fry boards. `prior-art` `competitors`
- [Automated Remote Bring-Up](concepts/automated-remote-bring-up.md) — the genuinely unsolved white space: end-to-end remote test. `white-space` `remote-test`
- [POC: Mini-Molecule + Cloud Workbench](concepts/poc-mini-molecule-cloud-workbench.md) — recommended desk-scale POC (Concepts A–D) mirroring Adom's stack. `poc` `interview-prep`

## Queries

_None yet._

## Discoveries

_None yet. Run `/arche-discover` to facilitate a discovery / ideation session._

## Stories

_None yet. Run `/arche-tell` to produce a presentation-ready HTML artifact from Arche content._
