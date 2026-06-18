---
type: concept
title: SAD — {{SYSTEM}}
created: {{DATE}}
updated: {{DATE}}
tags: [architecture, sad]
sources: []
---

# SAD — {{SYSTEM}}

Solution Architecture Document for the {{SYSTEM}} system. Holistic description of the chosen architecture. Frames against [ARD — {{SYSTEM}}](ard-{{SYSTEM}}.md) and links to every ADR that carries a load-bearing decision.

## Context

What this system does, who uses it, what it depends on, what depends on it. One paragraph.

## Drivers

The top three to five forces that shaped the architecture. Pulled from the ARD's quality attributes and constraints; cite the ARD.

- {{Driver}} — why it matters here ([ARD](ard-{{SYSTEM}}.md)).
- {{Driver}} — …

## Logical view

The components and their responsibilities. What the system is, not what it runs on. Diagrams welcome; one-line responsibility per component is mandatory.

- **{{Component}}** — {{responsibility}}.
- **{{Component}}** — …

## Process view

How the components interact at runtime. Request flow, async flow, key sequences. Name the integration style (Hohpe).

- **{{Flow}}** — {{actors, ordering, sync/async, contract style}}.
- **{{Flow}}** — …

## Data view

What data exists, who owns it, what crosses trust boundaries, what's transactional vs. eventual (Helland, Vernon).

- **{{Aggregate / dataset}}** — owner, identity, consistency boundary.
- **{{Aggregate / dataset}}** — …

## Deployment view

Where the components run. Cells, regions, redundancy story, on-call surface (Vogels, Nygard).

- **{{Component}}** — {{deployment shape, blast radius, dependency on platform services}}.
- **{{Component}}** — …

## Cross-cutting

Concerns that span every component: observability, security posture, auth model, secrets, config, build/release.

- **Observability** — {{what gets emitted, where it lands, what the fitness function checks}}.
- **Security** — {{trust boundaries, identity, secrets handling}}.
- **{{Concern}}** — …

## Fitness functions

The executable checks that say this architecture is still right (Ford). Each fitness function: what it measures, where it runs, what tripping it means.

- {{Fitness function}} — {{measure, location, action on failure}}.
- {{Fitness function}} — …

## Decision summary

Every load-bearing decision in this SAD has its own ADR. Link them here.

- [ADR — {{decision}}](adr-{{decision}}.md) — {{one-line summary}}.
- [ADR — {{decision}}](adr-{{decision}}.md) — …

## Risks and trade-offs

What this architecture is bad at, and why we accepted that trade-off.

- {{Risk / trade-off}} — {{accepted because …}}.
- {{Risk / trade-off}} — …

## See also

- [ARD — {{SYSTEM}}](ard-{{SYSTEM}}.md) — requirements this solution satisfies.
- [Related concept or entity](../concepts/example.md)
