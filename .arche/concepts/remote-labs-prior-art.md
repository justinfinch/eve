---
type: concept
title: Remote Labs (Prior Art)
created: 2026-06-17
updated: 2026-06-17
tags: [prior-art, remote-labs, visir, ilab, labsland, education]
sources: [sources/adom-decoded-and-poc-plan.md]
---

# Remote Labs (Prior Art)

Academic systems that have let students wire and measure real circuits over the web for ~15 years — the closest prior art to Adom's "design and measure real hardware from a browser," but limited to small textbook circuits on scarce, shared rigs ([source](../sources/adom-decoded-and-poc-plan.md)).

## Explanation

The canonical system, **VISIR**, realizes a student's wiring with a **relay switching matrix** and pre-validates every circuit before energizing it for safety; it is commercialized by **LabsLand**. **MIT's iLab** defined the standard three-tier pattern: browser client / lab server / broker ([source](../sources/adom-decoded-and-poc-plan.md)).

The ceiling: relay matrices cap you at roughly 17 nodes of textbook circuits, and each rig is expensive — so the field *shares* scarce rigs rather than scaling them. Adom's "molecules + robots" approach is a bet on breaking past that ceiling to arbitrary, real prototypes (see [Molecules and Workcells](molecules-and-workcells.md)) ([source](../sources/adom-decoded-and-poc-plan.md)).

## Examples

- VISIR (relay-matrix remote lab, pre-validated for safety) → commercialized by LabsLand ([source](../sources/adom-decoded-and-poc-plan.md)).
- MIT iLab three-tier architecture (client / lab server / broker) ([source](../sources/adom-decoded-and-poc-plan.md)).

## See also

- [Device Farms](device-farms.md)
- [Programmable Wiring](programmable-wiring.md)
- [Adom Industries](../entities/adom-industries.md)
