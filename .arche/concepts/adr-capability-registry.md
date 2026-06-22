---
type: concept
title: ADR — Self-describing capability registry
created: 2026-06-22
updated: 2026-06-22
tags: [architecture, adr, poc, mini-molecule, domain-model, self-id, capabilities]
sources: [concepts/sad-mini-molecule-poc.md, concepts/ard-mini-molecule-poc.md]
status: accepted
---

# ADR — Self-describing capability registry

## Decision

We will model the molecule as a **self-describing capability registry**. On power-up it announces `{id, name, fw_version, capabilities:[...]}`, where each capability is a typed descriptor (e.g. `{kind:"gpio", channels, ops}`, `{kind:"adc", channels, bits, ref_v}`, `{kind:"pwm", channels, freq_range}`). Commands address `(capability, channel, op, args)`. **The browser renders its workbench UI from the capability list**, and the registry doubles as the AI planner's tool schema.

## Context

Self-ID is called out as "the spine, not a rung" — `{id, name, capabilities}` is the through-line every learning rung extends (`[gpio]` → `[gpio,adc]` → `[...pwm]` → over-CAN), and "literally how an Adom molecule behaves" ([Learning Hardware as a Software Dev](learning-hardware-as-a-software-dev.md), [Molecules and Workcells](molecules-and-workcells.md)). The domain model therefore has to carry weight, not be a label. Framed by [SAD — Mini-Molecule POC](sad-mini-molecule-poc.md).

## Alternatives considered

- **Fixed command set** — hardcoded `read_adc` / `set_gpio` / `set_pwm` with hand-built UI per feature. Simpler to start, but not self-describing: every rung needs new UI code, and it forfeits the "molecule announces what it can do; the workbench adapts" signal that *is* the Adom story.
- **SCPI-like instrument grammar** — a full instrument-control command language (SCPI/VISA-style). Over-engineered for three capabilities; the Arche notes these standards are ["solved and open, not the moat"](instrument-control-standards.md) — wrong altitude for a desk POC.

## Consequences

- **Enables** — adding a capability in firmware auto-surfaces a control in the browser (the evolvability fitness function); the registry is reused as the AI's tool schema, so the AI literally cannot name a capability the molecule didn't advertise; the self-ID spine matches real molecule behavior.
- **Costs** — a generic capability-driven renderer is more upfront design than three bespoke buttons; descriptor schemas must be versioned as capabilities grow.
- **Locks in** — `(capability, channel, op, args)` as the command addressing scheme across firmware, sim, AI, and browser. Invariants ("address only advertised capabilities," "channel in range," "PWM within limits") live inside the molecule aggregate (Vernon) and are echoed as TS-type guards.

## Status

`accepted`

## See also

- [SAD — Mini-Molecule POC](sad-mini-molecule-poc.md) — the solution this decision is part of.
- [ADR — Message contract via tsify](adr-message-contract-tsify.md) — the types that express this model.
- [ADR — AI constrained planner](adr-ai-constrained-planner.md) — uses this registry as its guardrail and tool schema.
- [Molecules and Workcells](molecules-and-workcells.md) — the primitive this mirrors.
