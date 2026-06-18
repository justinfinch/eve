---
type: concept
title: Instrument-Control Standards
created: 2026-06-17
updated: 2026-06-17
tags: [prior-art, instrument-control, scpi, visa, ivi, lxi, standards]
sources: [sources/adom-decoded-and-poc-plan.md]
---

# Instrument-Control Standards

The mature, open stack for driving lab instruments (power supplies, scopes, DMMs) from code over a network — solved and not the moat; the moat is orchestration, safety, and bridging to a cloud/AI control plane ([source](../sources/adom-decoded-and-poc-plan.md)).

## Explanation

**SCPI** (universal ASCII instrument commands), **VISA/PyVISA**, **IVI**, and **LXI/HiSLIP** (instruments as network endpoints) make "drive a power supply / scope / DMM from code over Ethernet" mature and open-source. The moat isn't the protocol — it is the **orchestration, multiplexing, safety, and reliability** layer, plus bridging physical instruments to a cloud/AI control plane. That bridge is, again, [John Lauer](../entities/john-lauer.md)'s SPJS browser-to-serial pattern ([source](../sources/adom-decoded-and-poc-plan.md)).

## Examples

- SCPI universal command set; VISA/PyVISA host libraries; IVI driver model; LXI/HiSLIP for networked instruments ([source](../sources/adom-decoded-and-poc-plan.md)).

## See also

- [Adom Technical Architecture](adom-technical-architecture.md)
- [Automated Remote Bring-Up](automated-remote-bring-up.md)
- [John Lauer](../entities/john-lauer.md)
