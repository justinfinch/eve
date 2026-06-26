---
type: concept
title: ADR — AI as a constrained, sim-first planner
created: 2026-06-22
updated: 2026-06-22
tags: [architecture, adr, poc, mini-molecule, ai, llm, safety, tool-use]
sources: [concepts/sad-mini-molecule-poc.md, concepts/ard-mini-molecule-poc.md]
status: accepted
---

# ADR — AI as a constrained, sim-first planner

## Decision

We will architect the AI layer as a **constrained planner, never a raw actuator**. The AI receives the molecule's capability descriptor plus the user's natural-language request and emits a **structured, typed test-plan DSL** (a sequence of capability-addressed steps) via structured/tool-use output. The plan is **validated against the capability registry and safety bounds before any execution**, runs **sim-first** by default ([ADR — Device simulator](adr-device-simulator.md)), and is promoted to hardware only as the *same validated plan*. A second AI call turns result samples into a plain-English readout. Default provider **Claude** (latest capable model, structured tool-use); exact model, params, and tool-schema pinned at implementation against the `claude-api` reference.

## Context

The Arche is explicit that [LLMs hallucinate and can fry boards](ai-native-eda.md) — and hardware has no `try/catch`; a wrong value is current going where it shouldn't, physical and permanent ([Learning Hardware as a Software Dev](learning-hardware-as-a-software-dev.md)). The AI layer is the POC's headline differentiator but also its sharpest failure mode. The capability registry ([ADR — Capability registry](adr-capability-registry.md)) already provides a typed schema that doubles as a tool schema and a validation boundary. Framed by [SAD — Mini-Molecule POC](sad-mini-molecule-poc.md).

## Alternatives considered

- **AI emits raw hardware commands** — LLM directly issues GPIO/PWM/ADC writes. Rejected on safety: this is the exact "AI can fry boards" failure mode, with no validation seam and no sandbox.
- **AI explains only, no actuation** — AI narrates human-driven test results but never generates a plan. Safest and simplest, but drops the "NL → test plan" half that is the headline Adom-style demo.
- **Defer the AI layer entirely** — ship workbench + sim + hardware first. Reasonable de-risking, but loses the chance to bake the capability-schema-as-tool-schema seam in from the start, when it's cheapest.

## Consequences

- **Enables** — the natural-language → test-plan → readout demo, with a hard safety guarantee: a plan addressing an unadvertised capability or an out-of-bounds value cannot pass validation, so prompt injection cannot manufacture an out-of-schema hardware command (AI safety fitness function); free AI experimentation against the simulator.
- **Costs** — a DSL + validator to design and maintain; an LLM dependency (API key, latency, cost) held server-side in the v2 AI function.
- **Locks in** — the capability registry as the authoritative tool schema and the simulator as the default execution target; the AI never gets a path to hardware that bypasses validation.

## Status

`accepted`

## See also

- [SAD — Mini-Molecule POC](sad-mini-molecule-poc.md) — the solution this decision is part of.
- [ADR — Capability registry](adr-capability-registry.md) — the guardrail and tool schema.
- [ADR — Device simulator](adr-device-simulator.md) — the safety sandbox for sim-first execution.
- [AI-Native EDA](ai-native-eda.md) — the "LLMs can fry boards" risk this designs around.
