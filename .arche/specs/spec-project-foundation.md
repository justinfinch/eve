---
type: spec
title: Spec — Project Foundation
created: 2026-06-22
updated: 2026-06-22
tags: [spec, poc, mini-molecule, foundation, skeleton, monorepo]
status: accepted
sources: [concepts/sad-mini-molecule-poc.md, concepts/ard-mini-molecule-poc.md, concepts/adr-message-contract-tsify.md, concepts/adr-capability-registry.md, concepts/adr-device-simulator.md, concepts/adr-phased-control-plane.md, concepts/learning-hardware-as-a-software-dev.md, concepts/molecules-and-workcells.md]
context_pages: [concepts/sad-mini-molecule-poc.md, concepts/ard-mini-molecule-poc.md, concepts/adr-message-contract-tsify.md, concepts/adr-capability-registry.md, concepts/adr-device-simulator.md, concepts/adr-phased-control-plane.md, concepts/learning-hardware-as-a-software-dev.md, concepts/molecules-and-workcells.md]
---

# Spec — Project Foundation

The starting point for the Mini-Molecule POC's software: a single repository skeleton that does one trivial thing **end to end** — a software molecule announces who it is, and a browser workbench shows it — so that every later capability is added *into a structure whose load-bearing seams already work*, rather than onto a blank page. For the builder (Justin), grounded in the POC's hardware-literacy goal where the control plane is the self-built *instrument* every later rung reads from. **WHAT and WHY only** — the technical HOW (workspace tool, codegen tool, transport, languages) is already settled in the [SAD](../concepts/sad-mini-molecule-poc.md) and its ADRs, which this spec feeds and does not restate.

## Problem & why

A POC with a fully-decided architecture ([SAD](../concepts/sad-mini-molecule-poc.md) + seven accepted ADRs) but no code yet needs a *first* increment that is small enough to finish in one sitting yet shaped so the architecture's hardest commitments are exercised, not just declared.

- The architecture rests on two **one-way-door** commitments: the message **contract is the single source of truth** that every component imports ([ADR — Message contract](../concepts/adr-message-contract-tsify.md)), and the **logical protocol is the invariant** the whole system is written against ([ADR — Phased control plane](../concepts/adr-phased-control-plane.md)). A foundation that stubs each component in isolation would let both commitments rot unverified until they are expensive to fix.
- The POC's deliverable is **hardware literacy**, and its discipline is "build the instrument before the experiments" — the browser↔device loop and the self-ID spine come *first*, as the software-native readout every later rung depends on ([Learning Hardware as a Software Dev](../concepts/learning-hardware-as-a-software-dev.md)). The foundation *is* that instrument in skeleton form.
- Success for the POC is dual: *it works* **and** *it reads as Adom's stack* ([ARD](../concepts/ard-mini-molecule-poc.md)). The foundation is the first chance to bank the fidelity signal (a generated, single-source-of-truth contract) at its cheapest moment — before any component has a duplicate type to reconcile.
- **Why now:** it is the gating increment. Rung 1 ("build the instrument first") cannot start until the seams it extends exist.

## Goals & non-goals

**Goals**

- A single repository skeleton in which a software molecule's self-identity travels the *real* seam — defined once in the contract, advertised by the simulator, received and shown by the web workbench — with **no hardware attached**.
- The contract proven as the **single source of truth**: the web workbench consumes a *generated* representation of the contract type, never a hand-maintained duplicate.
- The embedded toolchain proven early: the firmware member compiles board-free, retiring the "Rust + Embassy ramp" schedule risk at rung zero rather than discovering it later ([ARD risks](../concepts/ard-mini-molecule-poc.md)).
- A repeatable, hardware-free check that the whole skeleton builds and the contract↔consumer link has not drifted.

**Non-goals** — explicitly out of scope this pass (YAGNI). Each is a later rung or a deferred ADR concern, not part of proving the seam.

- **Real capabilities and commands** (`gpio` / `adc` / `pwm` read/toggle/set) — deferred; the foundation advertises an *empty* capability list. Rungs 0–5.
- **Live streaming / subscriptions** — deferred to the measurement rungs.
- **AI test planning and plain-English readout** — deferred ([ADR — AI planner](../concepts/adr-ai-constrained-planner.md)), Rung 6.
- **Run artifacts and shareable permalinks** — deferred ([ADR — Reproducible run artifacts](../concepts/adr-reproducible-run-artifacts.md)).
- **CAN multi-node and the v2 host bridge** — deferred; the foundation is single-node and bridge-free ([ADR — Phased control plane](../concepts/adr-phased-control-plane.md)).
- **The firmware runtime path (real board over Web Serial)** — deferred; firmware is build-only this pass, because a hardware-free foundation is the point and the simulator is a first-class peer ([ADR — Device simulator](../concepts/adr-device-simulator.md)).
- **Real hardware bring-up / physical literacy** — owned by the [POC Hardware Manual](../stories/poc-hardware-manual-for-self.md), out of scope here.
- **Cross-browser support, authentication, and any security surface** — none exist yet; deferred with the v2 bridge.

## User scenarios

In user language; observable behavior, not implementation.

- **Primary** — As the builder, when I check out the repository fresh and follow the README, I bring up the simulator and the workbench and **see the molecule's identity displayed in the browser**, so that I have a working instrument to extend.
- **Alternate (verify the seam, no UI)** — As the builder, when I run the foundation's automated check, it builds every part and confirms the browser-facing type still matches the contract, so that I know the single-source-of-truth link is intact.
- **Alternate (extend later)** — As the future-self maintainer, when I add a new field or capability to the contract, the generated consumer reflects it without my hand-editing a second copy, so that adding a rung never means reconciling duplicate definitions.
- **Edge (drift)** — when the browser-facing type no longer matches the contract, the automated check shall **fail** rather than pass silently.
- **Edge (no hardware)** — when no physical device is attached, the primary scenario shall still succeed end to end.

## Functional requirements

- **FR-1** — The system shall be a single repository housing four named parts — a **contract**, a **simulator**, a **web workbench**, and a **firmware** part — built and managed together as one workspace.
- **FR-2** — The contract shall define exactly one **self-identification** value carrying an identifier, a human-readable name, a firmware-version field, and a capability list, where the capability list is **empty** in this pass.
- **FR-3** — The representation of the self-identification type used by the web workbench shall be **generated from the contract**, with no hand-maintained duplicate of that type anywhere in the repository.
- **FR-4** — On start, the simulator shall advertise a self-identification value conforming to the contract, **without any hardware attached**.
- **FR-5** — The web workbench shall obtain the advertised self-identification value from the simulator and **display its identifier and name** to the user.
- **FR-6** — The firmware part shall **compile for its intended target with no board attached** and shall not be required for the primary scenario to succeed.
- **FR-7** — The system shall provide a **single repeatable check** that builds every part and **fails if** regenerating the web-facing type from the contract would differ from what is committed.
- **FR-8** — The repository shall include onboarding instructions sufficient for a developer to reach the primary scenario from a clean checkout without prior project knowledge.

## Success criteria

Measurable and technology-agnostic.

- **SC-1** — From a clean checkout, a developer following only the README reaches the displayed-identity primary scenario in **≤ 15 minutes** *(threshold adjustable)*.
- **SC-2** — The end-to-end primary scenario succeeds with **zero hardware attached** in **100%** of foundation acceptance runs.
- **SC-3** — The identifier and name shown in the browser are **byte-for-byte identical** to the value the simulator advertised (the seam transforms nothing).
- **SC-4** — Regenerating the web-facing type from the contract produces **zero differences** from the committed version; introducing a deliberate mismatch makes the check **fail 100%** of the time.
- **SC-5** — A single documented command builds **all four parts green**; deliberately breaking any one part makes that command fail (proving the workspace truly binds them, not that they pass independently).
- **SC-6** — The number of **hand-maintained duplicate definitions** of the self-identification type across all parts is **zero**.

## Ubiquitous language

What each term *is*. Reconciled against existing Arche pages; no term is redefined here.

| Term | Definition (what it *is*) | Arche page | Aliases to avoid |
| :--- | :------------------------ | :--------- | :--------------- |
| Molecule | A self-describing device (here, a software stand-in) that announces its identity and capabilities on connect. | [Molecules and Workcells](../concepts/molecules-and-workcells.md) | board, node, device-under-test |
| Self-identification | The announced value `{id, name, fw_version, capabilities[]}`; the **spine** every later capability extends — empty-listed in this pass. | [ADR — Capability registry](../concepts/adr-capability-registry.md) | handshake, hello-packet |
| Capability registry | The self-describing model the molecule advertises; the browser renders from it and the AI later uses it as a guardrail. Empty in the foundation. | [ADR — Capability registry](../concepts/adr-capability-registry.md) | command set, API |
| Contract | The single, authoritative definition of the messages crossing between components; the dependency hub every part imports. | [ADR — Message contract](../concepts/adr-message-contract-tsify.md) | schema, types, interface |
| Simulator | A software molecule that implements the same contract as real firmware and is an **interchangeable peer** of it, not a test stub. | [ADR — Device simulator](../concepts/adr-device-simulator.md) | mock, fake, stub |
| Web workbench | The browser-side surface that connects to a molecule (real or simulated) and presents it; renders *from* what the molecule advertises. | [SAD](../concepts/sad-mini-molecule-poc.md) | dashboard, frontend, UI (generic) |
| Foundation | This first increment: the skeleton whose load-bearing seams already work, so later rungs are added into structure, not onto a blank page. | (this spec) | boilerplate, scaffolding (generic) |
| Rung | One step on the hardware-literacy learning ladder; a unit of later capability the foundation is built to receive. | [Learning Hardware as a Software Dev](../concepts/learning-hardware-as-a-software-dev.md) | phase, milestone, sprint |

## Clarifications

- **Resolved** — What must the foundation prove? → A **trivial end-to-end vertical slice** through the real seams (contract → simulator → web), not isolated per-component stubs (2026-06-22; chosen as most faithful to the contract-as-hub and simulator-as-peer ADRs).
- **Resolved** — Is firmware in scope? → **Build-only** workspace member: compiles board-free, not on the runtime path (2026-06-22; proves the Embassy toolchain early per the ARD ramp risk, while keeping the foundation hardware-free).
- **Resolved** — Must the contract be proven as single source of truth now? → **Yes**, banked on day one: the web workbench consumes a generated type, and the drift check exists from the start (2026-06-22; the [tsify ADR](../concepts/adr-message-contract-tsify.md) calls this "banked on day one").
- **Resolved** — Minimal self-identification shape? → The **full spine** `{id, name, fw_version, capabilities[]}` with an **empty** capability list, not a reduced `{id, name}` (2026-06-22; `capabilities` is "the spine, not a rung" per the capability-registry ADR — the faithful minimum is "advertises nothing yet," not a different shape).
- **Resolved** — Is an automated check in scope? → **Yes**, a minimal build-all-plus-drift check; the banked drift guard only has force if something runs it (2026-06-22).

## Assumptions & dependencies

- **Assumption** — The ≤ 15-minute onboarding threshold (SC-1) is the builder's target; if a one-time toolchain install dominates first-run time, the threshold measures the *second* clean run.
- **Assumption** — An empty capability list is a valid, well-formed self-identification value (the molecule may legitimately advertise nothing yet).
- **Dependency** — The deployment and build shape this realizes is fixed by [SAD — Mini-Molecule POC](../concepts/sad-mini-molecule-poc.md) and its ADRs; this spec states the WHAT/WHY, those state the HOW.
- **Dependency** — The hardware-literacy trail and physical bring-up are owned by [Learning Hardware as a Software Dev](../concepts/learning-hardware-as-a-software-dev.md) and the [POC Hardware Manual](../stories/poc-hardware-manual-for-self.md); this foundation is their software counterpart and does not duplicate them.

## Quality gate

- [x] No implementation detail — no tech, framework, data model, or API shape leaked in (workspace/codegen/transport/language names deferred to the SAD/ADRs).
- [x] Every functional requirement is testable.
- [x] Every success criterion is measurable and technology-agnostic.
- [x] No placeholder text, contradictions, or undefined terms remain.
- [x] Non-goals make the scope boundary explicit.
- [x] Open clarifications ≤ 3 (zero open; five resolved).
- [x] Ubiquitous-language terms reconcile with existing Arche entity/concept pages (no silent redefinition).

## See also

- [SAD — Mini-Molecule POC](../concepts/sad-mini-molecule-poc.md) — the settled architecture (Deployment & Build views) this foundation realizes.
- [ARD — Mini-Molecule POC](../concepts/ard-mini-molecule-poc.md) — the requirements whose foundational subset this spec scopes.
- [Learning Hardware as a Software Dev](../concepts/learning-hardware-as-a-software-dev.md) — "build the instrument first"; this foundation is that instrument in skeleton.
- [ADR — Message contract via tsify](../concepts/adr-message-contract-tsify.md) · [ADR — Capability registry](../concepts/adr-capability-registry.md) · [ADR — Device simulator](../concepts/adr-device-simulator.md) · [ADR — Phased control plane](../concepts/adr-phased-control-plane.md) — the decisions the seam exercises.
