---
type: spec
title: Spec — {{FEATURE}}
created: {{DATE}}
updated: {{DATE}}
tags: [spec]
status: proposed
sources: []
context_pages: []
---

# Spec — {{FEATURE}}

One-paragraph orientation: what this feature is, who it's for, and the problem it solves. **WHAT and WHY only — no HOW.** No frameworks, no data models, no API shapes, no technology choices. Those belong in `/arche-architect` (the ARD/SAD/ADR this spec feeds).

## Problem & why

The user/business need this exists to serve. Cite the Arche context that motivates it — discovery sessions, customer/SME signal, domain constraints.

- {{Problem statement}} ([motivating context](../discoveries/example.md)).
- Why now / why it matters.

## Goals & non-goals

**Goals** — what success looks like at the feature level.

- {{Goal}}.

**Non-goals** — explicitly out of scope this pass (YAGNI). Naming them prevents scope creep and tells `/arche-architect` where the boundary is.

- {{Non-goal}} — deferred because {{reason}}.

## User scenarios

Primary and alternate flows in user language. Each scenario is observable behavior, not implementation.

- **Primary** — As a {{role}}, when {{situation}}, I {{action}} so that {{outcome}}.
- **Alternate** — …
- **Edge** — when {{boundary condition}}, the system shall {{observable response}}.

## Functional requirements

What the system must do. **Testable** — each requirement is something you could write an acceptance test against. Numbered for reference from downstream ARD/SAD/ADRs and plans.

- **FR-1** — {{The system shall …}}.
- **FR-2** — …

## Success criteria

**Measurable and technology-agnostic** (spec-kit discipline). A metric with a threshold, stated without naming any tool, framework, or implementation. Numbered.

- **SC-1** — {{measure}} reaches {{threshold}} under {{condition}}.
- **SC-2** — …

## Ubiquitous language

The terms this feature introduces or relies on — **what things ARE, not what they do** (grill-with-docs glossary discipline). Each term: a succinct definition, the Arche entity/concept it maps to, and aliases to avoid so the team doesn't drift.

| Term | Definition (what it *is*) | Arche page | Aliases to avoid |
| :--- | :------------------------ | :--------- | :--------------- |
| {{Term}} | {{one-line definition}} | [{{entity/concept}}](../entities/example.md) | {{alias}}, {{alias}} |

## Clarifications

Genuine ambiguities surfaced during specification. Resolved ones record the decision; any remaining open marker is capped at three total and prioritized **scope > security > UX > technical**.

- **Resolved** — {{question}} → {{decision}} ({{date or rationale}}).
- **[NEEDS CLARIFICATION: {{open question}}]** — {{why it matters / options}}.

## Assumptions & dependencies

- **Assumption** — {{treated as true for this spec}}; if it falls, the spec needs revisiting.
- **Dependency** — {{other feature, system, team, or external party}} this relies on ([Arche page](../entities/example.md)).

## Quality gate

Self-review checklist, run before the spec is accepted (max 3 fix iterations). Every box must be checked or the unmet item recorded as a `[NEEDS CLARIFICATION]` above.

- [ ] No implementation detail — no tech, framework, data model, or API shape leaked in.
- [ ] Every functional requirement is testable.
- [ ] Every success criterion is measurable and technology-agnostic.
- [ ] No placeholder text, contradictions, or undefined terms remain.
- [ ] Non-goals make the scope boundary explicit.
- [ ] Open clarifications ≤ 3, each impact-prioritized.
- [ ] Ubiquitous-language terms reconcile with existing Arche entity/concept pages (no silent redefinition).

## See also

- [Motivating discovery / context pages](../discoveries/example.md)
- [ARD — {{system}}](../concepts/ard-example.md) — the architecture requirements `/arche-architect` derives from this spec (added once that session runs).
