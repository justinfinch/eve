---
name: arche-query
description: Answer a question using the project's Arche at ./.arche/ — institutional context (business, SME, ARB decisions, research) the code doesn't carry. Reads index.md, walks to the relevant pages, synthesizes a cited answer, optionally files it back as queries/<slug>.md. Use when the user asks something answerable from their Arche; says "query the Arche" / "what does the Arche say about X" / "take a look at our/the Arche" / "based on / given our Arche"; asks a question after ingesting sources; OR is about to plan, design, scope, brainstorm, set up a dev environment, tooling, or dependencies, or make any setup decision in a repo with a ./.arche/ that should be grounded in domain, architecture, or prior-research context — surfacing relevant ADRs, constraints, or prior research first. This includes being invoked from inside another skill (e.g. devbox-init, or any planning skill) whose instructions say to "look at the Arche" — surface the pages via this skill rather than reading ad hoc.
---

# arche-query

Answer a question from the project Arche.

## When this fires

Beyond direct "query the Arche" questions, treat this skill as the **cold-start orientation step** for any work in a repo that has a `./.arche/`:

- A user instruction to "take a look at our Arche", "based on the Arche", or "given our Arche" — even when paired with an action ("…and set up X") — is a trigger. Run this skill, then proceed to the action with the surfaced context.
- A planning / design / scoping / brainstorming step, or a setup decision (dev environment, tooling, dependencies) — surface relevant ADRs, constraints, and prior research *before* that step runs.
- **Invoked from inside another skill** (e.g. `devbox-init`, or any planning skill) whose own instructions say to "look at the Arche": satisfy that instruction by invoking this skill, not by reading Arche pages ad hoc. This skill does not replace the host skill's planning/brainstorming/TDD work — it only feeds grounded context into it, then control returns to the host.

## Preflight

1. Verify `./.arche/SCHEMA.md` and `./.arche/index.md` exist. If either is missing, tell the user to run `/arche-init` and `/arche-ingest` first and stop.
2. Read `./.arche/SCHEMA.md` so you cite and link in the project's house style.
3. Read `./.arche/index.md` — this is the entry point. Use it to choose which pages to read next.

## Workflow

1. **Pick candidate pages.** From the index, list the entities/concepts/queries plausibly relevant to the question. Aim for high recall — better to read one page too many than miss one. If nothing in the index looks relevant, say so and recommend an `/arche-ingest`; do not fabricate.
2. **Read the candidates.** For each candidate page, read the full file. Note the sources cited at each claim — you may need to read source summaries too if the question hinges on provenance.
3. **Synthesize an answer.** Inline-cite every non-trivial claim with **both** the Arche page that synthesizes it and the underlying source page: `... per [Concept Name](../concepts/foo.md) citing [Source Title](../sources/bar.md).` Provenance traces to the original source, not just the synthesis layer. If a claim is supported by multiple sources, cite the strongest one; the others can be implied by the Arche page's own `sources:` list.
4. **Flag gaps.** If the Arche doesn't fully answer the question, say what's missing and suggest a source to ingest. Do not guess past the Arche's coverage.
5. **Spec / architecture gap signal.** When the question is framed for building a feature, route to the upstream gap first:
   - If it's about **what to build / requirements** ("what should we build for X", "what are the requirements for Y", "scope feature Z") AND no relevant `spec-<feature>` page exists in `specs/`, surface that gap: *"The Arche has no spec filed for this feature. Want to run `/arche-specify` to grill the WHAT/WHY and file a spec before design starts?"*
   - If it's about **how to build / design** ("how should we build X", "what's the right approach for Y", "design a Z") AND no relevant ARD/SAD/ADR concept page exists, surface that gap: *"The Arche has no decision filed for this. Want to run `/arche-architect` to grill the design and file ARD/SAD/ADRs before planning starts?"*
   - If it's about **sequencing the build** ("plan this", "break it into tasks", "implementation plan for X") AND an accepted `spec-<feature>` plus covering SAD/ADRs already exist but no `plan-<feature>` page does, surface that: *"The spec and architecture are filed but there's no plan. Want to run `/arche-plan` to decompose it into an executable plan?"* (`/arche-plan` will itself gap-check the architecture and route back to `/arche-architect` if a decision is missing.)
   - Suggest; do not auto-invoke. The pipeline is `/arche-specify` → `/arche-architect` → `/arche-plan`; recommend the earliest open gap first (a missing spec before a missing decision before a missing plan).
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
