---
type: concept
title: Molecules and Workcells
created: 2026-06-17
updated: 2026-06-17
tags: [adom, product-primitive, modular-pcb, software-defined-hardware, robotics]
sources: [sources/adom-decoded-and-poc-plan.md]
---

# Molecules and Workcells

Adom's two core product primitives: a **molecule** is a modular PCB designed to plug into a factory **workcell**, where robots physically wire molecules together and bench instruments measure them — "software-defined hardware," time-shared like cloud compute ([source](../sources/adom-decoded-and-poc-plan.md)).

## Explanation

A **molecule** is Adom's term for a modular PCB chosen from a library and connected "like Legos." A UT Dallas advisor's framing: instead of buying an Arduino or Raspberry Pi, you pick from a library of these modules, connect them, and do measurements, firmware development, and testing — all remotely ([source](../sources/adom-decoded-and-poc-plan.md)).

A **workcell** is the physical heart of the factory: a cell where **robot pincers connect molecules with wires** for prototype testing. Texas A&M built Adom a wire-bending rig to automate the connections; UT Austin worked on an AI-trained robotic arm ([source](../sources/adom-decoded-and-poc-plan.md)).

Together they constitute "software-defined hardware" — a reconfigurable bank of modular boards plus instruments that are reprogrammed and rewired remotely and time-shared like cloud compute. Crucially, Adom's choice to **physically wire modules with robots** sidesteps the physics ceiling of pure switch-fabric approaches (see [Programmable Wiring](programmable-wiring.md)), which is why the robotics hire matters ([source](../sources/adom-decoded-and-poc-plan.md)).

## Examples

- A molecule self-identifies on power-up, announcing its `{id, name, capabilities}` so the system can treat it as plug-and-play — the behavior the recommended [POC](poc-mini-molecule-cloud-workbench.md) replicates at desk scale ([source](../sources/adom-decoded-and-poc-plan.md)).
- The chemistry naming theme runs through the product: atoms → molecules → "Hydrogen Desktop" client ([source](../sources/adom-decoded-and-poc-plan.md)).

## See also

- [Adom Industries](../entities/adom-industries.md)
- [Adom Technical Architecture](adom-technical-architecture.md)
- [Programmable Wiring](programmable-wiring.md)
- [POC: Mini-Molecule + Cloud Workbench](poc-mini-molecule-cloud-workbench.md)
