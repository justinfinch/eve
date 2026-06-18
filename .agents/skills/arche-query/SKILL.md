---
name: arche-query
description: Answer a question using the project's Arche at ./.arche/ — the Arche holds **institutional context** (business domain, SME knowledge, ARB-style architectural decisions, research) that the code doesn't carry. Reads index.md, walks to the relevant pages, synthesizes an answer with inline citations to Arche pages and their sources, and optionally files the synthesis back as queries/<slug>.md when the user wants it kept. Use when the user asks something that should be answered from their Arche, says "query the Arche" / "what does the Arche say about X", asks a question after having ingested sources, **OR** is about to plan / design / scope / brainstorm a feature in a repo with a ./.arche/ and could benefit from surfacing relevant ADRs, domain constraints, customer context, or prior research before the planning step runs. Fires as a cold-start orientation step alongside agentic dev methodologies (e.g. superpowers, mattpocock skills) — does not replace their planning/brainstorming/TDD skills, only feeds context into them.
---

# arche-query

Answer a question from the project Arche.

## Preflight

1. Verify `./.arche/SCHEMA.md` and `./.arche/index.md` exist. If either is missing, tell the user to run `/arche-init` and `/arche-ingest` first and stop.
2. Read `./.arche/SCHEMA.md` so you cite and link in the project's house style.
3. Read `./.arche/index.md` — this is the entry point. Use it to choose which pages to read next.

## Workflow

1. **Pick candidate pages.** From the index, list the entities/concepts/queries plausibly relevant to the question. Aim for high recall — better to read one page too many than miss one. If nothing in the index looks relevant, say so and recommend an `/arche-ingest`; do not fabricate.
2. **Read the candidates.** For each candidate page, read the full file. Note the sources cited at each claim — you may need to read source summaries too if the question hinges on provenance.
3. **Synthesize an answer.** Inline-cite every non-trivial claim with **both** the Arche page that synthesizes it and the underlying source page: `... per [Concept Name](../concepts/foo.md) citing [Source Title](../sources/bar.md).` Provenance traces to the original source, not just the synthesis layer. If a claim is supported by multiple sources, cite the strongest one; the others can be implied by the Arche page's own `sources:` list.
4. **Flag gaps.** If the Arche doesn't fully answer the question, say what's missing and suggest a source to ingest. Do not guess past the Arche's coverage.
5. **Architectural-gap signal.** If the question is framed for planning/design/scoping a feature ("how should we build X", "what's the right approach for Y", "design a Z") AND no relevant ARD/SAD/ADR concept page exists in the Arche, surface that gap explicitly: *"The Arche has no decision filed for this. Want to run `/arche-architect` to grill the design and file ARD/SAD/ADRs before planning starts?"* Suggest it; do not auto-invoke.
6. **Offer to file the synthesis.** If the answer is non-trivial and reusable, ask: "Want me to file this as `queries/<slug>.md`?" Default: do not file unless asked or unless the question itself was framed as an Arche investigation.

## Filing a query back

If the user says yes:

1. Create `.arche/queries/<slug>.md` using this skill's [query.template.md](assets/query.template.md) as the layout (frontmatter: `type: query`, today's date, tags, `sources:` listing every Arche page and source you cited).
2. Body per template: the question (verbatim) as a blockquote, then the answer with citations preserved.
3. Add an entry under Queries in `index.md`.
4. Append a `query` entry to `log.md`.

## Discipline

- Citations are mandatory — every non-obvious claim needs a link to the Arche page that synthesizes it AND the underlying source page. Uncited claims should not appear; if you'd write one, you don't know enough from the Arche and should flag the gap instead.
- Prefer answers grounded in concept/entity pages over raw source summaries — concepts and entities exist precisely to be the synthesis layer.
- Do not modify any Arche page during a query unless you're filing the synthesis back. Queries are read-mostly.

## Output

A direct answer to the user's question, with inline citations. No preamble.
