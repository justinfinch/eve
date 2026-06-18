---
type: concept
title: ARD — {{SYSTEM}}
created: {{DATE}}
updated: {{DATE}}
tags: [architecture, ard]
sources: []
---

# ARD — {{SYSTEM}}

Architecture Requirements Document for the {{SYSTEM}} system. Captures what any architecture for this system must satisfy, ahead of any specific design. Paired with [SAD — {{SYSTEM}}](sad-{{SYSTEM}}.md).

## Stakeholders

- **{{Role}}** — what they need from this system, what they will judge it on.
- **{{Role}}** — …

## Functional requirements

What the system must do. One bullet per capability. No design.

- {{Capability}} — {{one-line description}}.
- {{Capability}} — …

## Quality attributes

Each as a scenario in the form **stimulus → environment → response → measure** (Bass).

- **{{Attribute}}** — when {{stimulus}} occurs in {{environment}}, the system shall {{response}}, measured by {{metric and threshold}}.
- **{{Attribute}}** — …

## Constraints

Non-negotiable. Cite the source: regulation, contract, infra reality, team skill.

- {{Constraint}} ([source](../sources/example.md)).
- {{Constraint}} — …

## Assumptions

Things treated as true for this design pass. If any of these falls, the ARD needs revisiting.

- {{Assumption}}.
- {{Assumption}} — …

## Risks

Known risks to either the requirements themselves (we got them wrong) or the achievability of them (we got them right but they're hard).

- {{Risk}} — likelihood / impact / mitigation strategy if any.
- {{Risk}} — …

## See also

- [SAD — {{SYSTEM}}](sad-{{SYSTEM}}.md) — the solution designed against this ARD.
- [Related ADR or concept](../concepts/example.md)
