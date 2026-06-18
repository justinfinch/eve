---
type: concept
title: ADR — {{DECISION}}
created: {{DATE}}
updated: {{DATE}}
tags: [architecture, adr]
sources: []
status: accepted
---

# ADR — {{DECISION}}

## Decision

One paragraph. What was decided. Lead with the verb: *We will …* / *We chose …*. No hedging.

## Context

What problem prompted this decision. What constraints applied. Why this came up now. Cite the ARD or SAD that frames it: [SAD — {{SYSTEM}}](sad-{{SYSTEM}}.md), [ARD — {{SYSTEM}}](ard-{{SYSTEM}}.md). Cite the discovery or query that surfaced it if applicable.

## Alternatives considered

The most load-bearing section. For each alternative on the table:

- **{{Alternative}}** — what it would have looked like; why-not (the specific reason it was rejected, not generic "complexity").
- **{{Alternative}}** — …

If there were no real alternatives, this should not be an ADR — fold the decision into the SAD body instead.

## Consequences

What this enables, what it costs, what it locks in. Be honest about the downside; the trail of accepted-trade-offs is what makes this useful in 18 months.

- **Enables** — {{what becomes easier or possible}}.
- **Costs** — {{what becomes harder, more expensive, more rigid}}.
- **Locks in** — {{what becomes a one-way door or sticky default}}.

## Status

`accepted` — the decision is live.

When this ADR is reversed, do not delete this page. Set `status: superseded` in the frontmatter and `superseded_by: concepts/adr-<new>.md`. The trail of "we tried this, reversed it after …" is the institutional memory the Arche exists to preserve.

## See also

- [SAD — {{SYSTEM}}](sad-{{SYSTEM}}.md) — the solution this decision is part of.
- [Related ADR](adr-related.md)
