---
type: concept
title: ADR — Reproducible run artifacts (sim-replayable permalinks)
created: 2026-06-22
updated: 2026-06-22
tags: [architecture, adr, poc, mini-molecule, reproducibility, permalink]
sources: [concepts/sad-mini-molecule-poc.md, concepts/ard-mini-molecule-poc.md]
status: accepted
---

# ADR — Reproducible run artifacts (sim-replayable permalinks)

## Decision

We will make a test run an **immutable, self-contained artifact**: a permalink encodes (or short-IDs) the **validated plan + the capability snapshot it ran against + the result samples**. Opening a permalink can **re-run the plan against the simulator** ([ADR — Device simulator](adr-device-simulator.md)), so every shared run is reproducible — by anyone, with or without hardware. POC implementation: URL-encoded self-contained payload for small runs, with a tiny key-value store + short id as the upgrade path for large captures.

## Context

Reproducibility is the POC's fifth pillar ([POC concept](poc-mini-molecule-cloud-workbench.md)), and a shareable permalink is the named deliverable. A link to *live state* isn't reproducible once the hardware is gone. Helland's framing: what crosses the boundary should be an immutable *value* (a snapshot), not a reference to live state. The simulator and the single-source contract make true replay cheap. Framed by [SAD — Mini-Molecule POC](sad-mini-molecule-poc.md).

## Alternatives considered

- **Same design but kept in the SAD body** — fold it in as "just serialize and share." Rejected: the *replay-against-the-sim* property is non-obvious and load-bearing — it's *why* the sim and contract-as-invariant pay off for sharing — and deserves its own page.
- **Server-stored runs only** — every run persisted server-side behind an id, no self-contained URL. Rejected: needs a backend from day 1 and couples sharing to that service staying up — heavier than a POC needs.
- **Screenshot / export only** — let users screenshot or export results. Rejected: trivial but drops genuine reproducibility — you can't re-run a screenshot, so the pillar becomes cosmetic.

## Consequences

- **Enables** — true reproducibility (the fitness function for the pillar): a permalink re-executes deterministically against the sim, forever; sharing needs no backend for small runs.
- **Costs** — the run artifact schema must version alongside the contract; large captures eventually need the short-id store.
- **Locks in** — the run artifact (`plan + capability snapshot + samples`) as the unit of sharing, and the simulator as its replay engine — binding this decision to [ADR — Device simulator](adr-device-simulator.md) and [ADR — Message contract](adr-message-contract-tsify.md).

## Status

`accepted`

## See also

- [SAD — Mini-Molecule POC](sad-mini-molecule-poc.md) — the solution this decision is part of.
- [ADR — Device simulator](adr-device-simulator.md) — the replay engine.
- [ADR — AI constrained planner](adr-ai-constrained-planner.md) — produces the validated plans that become artifacts.
