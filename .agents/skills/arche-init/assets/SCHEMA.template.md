---
type: schema
created: {{DATE}}
updated: {{DATE}}
---

# Arche Schema

This file tells the LLM how to maintain `./.arche/`. The operation skills (`/arche-ingest`, `/arche-query`, `/arche-lint`, `/arche-discover`) read it before acting. Keep it short and authoritative — when conventions change, edit this file rather than the skills.

Based on Andrej Karpathy's LLM Wiki pattern: humans curate, the LLM maintains.

## What belongs here (and what doesn't)

This Arche captures **institutional context that does not live in the code**:

- **Business domain** — customer context, product positioning, market signals, the *why* this product exists.
- **SME knowledge** — subject-matter expert insights, interview transcripts, regulatory/compliance constraints, the things "the senior person in the room" carries in their head.
- **Architectural decisions (ARD, SAD, ADR)** — solution-architecture work as a chain of concept-page conventions: requirements (ARD) → chosen solution (SAD) → individual reversible decisions (ADRs) with alternatives and supersession trail. See [Architecture pages](#architecture-pages-ard-sad-adr) below.
- **Feature specifications (spec)** — the WHAT/WHY of a feature to be built: testable requirements, measurable (technology-agnostic) success criteria, user scenarios, ubiquitous language. Upstream-of-code *intent*, not derived-from-code documentation; it grounds and feeds the architecture work rather than describing existing code. See [Feature specs](#feature-specs-spec) below — `/arche-specify` produces them.
- **Implementation plans (plan)** — the durable *execution blueprint* for an accepted spec: dependency-ordered, independently reviewable tasks with a file/interface map, each traced to a requirement and a filed architectural decision. The blueprint (decomposition + architectural grounding + traceability) is institutional context — "how we decided to build this, traced to why" — distinct from the **transient** execution *state* (ticked checkboxes, debug notes, commit history), which stays in the PR / working tree and is **not** captured here. See [Implementation plans](#implementation-plans-plan) below — `/arche-plan` produces them.
- **Research** — papers, articles, competitor analyses, prior art relevant to product or architecture.

This Arche **does not** capture:

- Code documentation, API references, module diagrams — those live with the code (`docs/`, doc comments, generated references).
- In-flight execution *state* — ticked checkboxes, in-progress TODOs, debugging notes, commit-by-commit history — those live in PR descriptions, commits, or your dev methodology's working artifacts. (The durable *plan of record* — task decomposition + architectural grounding + traceability, produced by `/arche-plan` — does belong here; its transient build state does not. Keep the line sharp: a `plan` page that accumulates checkbox state has drifted into TODO-tracker territory.)
- Generated content (changelogs, dependency lists, build outputs).

**Rule of thumb:** if a question is answered by *"read the code,"* it doesn't belong here. If a question is answered by *"ask the senior architect or product owner what we decided and why,"* it does.

**How it plugs into dev workflows:** the Arche is *consumed* by your agentic dev/implementation skills during planning, design, and brainstorming phases — it surfaces ADRs, domain constraints, customer context, and prior research that should inform the work. It is **not** written to by coding sessions. Coding artifacts stay with the code.

## Three layers

1. **Raw** — immutable source files stored in `raw/`. PDFs, transcripts, downloaded articles, pasted text saved as `.md`. The human (or `/arche-ingest`) drops files here; the LLM reads them but never modifies them. This is the audit trail.
2. **Arche** — markdown pages the LLM owns: source summaries (which cite back into `raw/`), entity pages, concept pages, query-result pages, and discovery (session) pages.
3. **Schema** — this file. The LLM-readable contract that keeps the Arche coherent.

`raw/` doubles as a drop zone: dragging a file into `raw/` is the capture step. Processing is a separate step (`/arche-ingest`), so capture stays frictionless.

## Page types

| Type       | Path                     | Purpose                                                                  |
| :--------- | :----------------------- | :----------------------------------------------------------------------- |
| raw        | `raw/<filename>`         | The actual source file (PDF, MD, transcript). Immutable. Not summarized in-place. |
| source     | `sources/<slug>.md`      | LLM-written summary + key claims for one raw file (or external URL)      |
| entity     | `entities/<slug>.md`     | A person, org, system, place — facts aggregated across sources            |
| concept    | `concepts/<slug>.md`     | An idea, pattern, or technique — explanation + examples                  |
| query      | `queries/<slug>.md`      | A filed-back synthesis from a `/arche-query` worth keeping                |
| spec       | `specs/<slug>.md`        | Feature specification (`/arche-specify`): technology-agnostic WHAT/WHY for a feature — problem, user scenarios, testable requirements, measurable success criteria, ubiquitous language; grounded in Arche context and feeds `/arche-architect`. Slug `spec-<feature>` |
| plan       | `plans/<slug>.md`        | Implementation plan (`/arche-plan`): durable execution blueprint for an accepted spec — dependency-ordered reviewable tasks, file/interface map, traceability to FR/SC and to filed SAD/ADRs, architect gap-check verdict. The blueprint is durable; build state stays in the PR. Slug `plan-<feature>` |
| discovery  | `discoveries/<slug>.md`  | Captured discovery / ideation session (`/arche-discover`): full idea inventory from a facilitated brainstorming session, themes, technique narrative; cites concept/entity pages and may be cited back by them |
| story      | `stories/<slug>.md`      | A communication artifact (`/arche-tell`) that packages Arche content for a defined audience and ask: outline, audience block, framework, format, and inline-cited narrative; pairs with a rendered HTML file at `assets/stories/<slug>.html` |

### Architecture pages (ARD, SAD, ADR)

The Arche captures technical-architecture work as three slug conventions **layered on concept pages** — not separate page types. They form a chain: an **ARD** frames what any architecture for a system must satisfy, a **SAD** describes the chosen solution, and **ADRs** capture each load-bearing decision the SAD relies on. `/arche-architect` is the operation skill that produces and updates them.

| Slug                       | Convention                            | Body sections                                                                                     |
| :------------------------- | :------------------------------------ | :------------------------------------------------------------------------------------------------ |
| `ard-<system>`             | Architecture Requirements Document    | Stakeholders / Functional requirements / Quality attributes (stimulus → environment → response → measure) / Constraints / Assumptions / Risks |
| `sad-<system>`             | Solution Architecture Document        | Context / Drivers / Logical view / Process view / Data view / Deployment view / Cross-cutting / Fitness functions / Decision summary (links to ADRs) / Risks and trade-offs |
| `adr-<short-decision-name>` | Architecture Decision Record         | Decision / Context / Alternatives considered / Consequences / Status                              |

Examples: `ard-billing`, `sad-billing`, `adr-event-driven-billing`, `adr-session-token-storage`.

**Pairing.** An ARD and SAD for the same system share the system stem and link to each other. The SAD's *Decision summary* lists every ADR; each ADR's `sources:` frontmatter cites the SAD so navigation is bidirectional.

**Status (ARD, SAD, and ADR).** One of: `proposed` | `accepted` | `superseded`. If `superseded`, set the `superseded_by:` frontmatter field to the replacement page. Do not delete superseded pages — the trail of "we tried X, reversed it after Y" is precisely the institutional memory the Arche exists to preserve. SADs and ARDs can supersede each other across major redesigns the same way ADRs do.

**Citation.** Cite sources in the body the same as any other concept page: the discovery session or research that produced the decision, the SME who flagged the constraint, the prior ADR being superseded.

**When to write each.** `/arche-architect` recommends the scope at session start: ARD only (requirements not yet clear), full ARD + SAD + ADRs (most common), SAD + ADRs (design against an existing ARD), or ADRs only (tightly scoped decision inside an existing SAD). Decisions that don't clear the ADR bar — hard to reverse, surprising-without-context, real trade-off — stay inside the SAD body rather than getting their own ADR.

### Feature specs (spec)

The Arche captures feature-level requirements as `spec` pages under `specs/`, with the slug convention `spec-<feature>` (2–4 word action-noun stem, e.g. `spec-user-onboarding`). `/arche-specify` is the operation skill that produces them.

A spec is **WHAT/WHY, never HOW** — it describes the problem and required behavior in user/business language and is deliberately technology-agnostic. It sits **upstream of architecture**: `/arche-specify` grounds the spec in Arche context (via `/arche-query`) and hands off to `/arche-architect`, whose ARD/SAD/ADRs cite the spec back.

| Slug             | Convention            | Body sections                                                                                                                              |
| :--------------- | :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| `spec-<feature>` | Feature specification | Problem & why / Goals & non-goals / User scenarios / Functional requirements (testable, FR-n) / Success criteria (measurable, tech-agnostic, SC-n) / Ubiquitous language / Clarifications / Assumptions & dependencies / Quality gate |

**Discipline.** Functional requirements are testable; success criteria are measurable and name no technology. Unresolved ambiguities become `[NEEDS CLARIFICATION]` markers, capped at three and prioritized scope > security > UX > technical. A `## Quality gate` checklist is self-reviewed before the spec is accepted.

**Status & supersession.** Specs use the same `status:` (`proposed` | `accepted` | `superseded`) and `superseded_by:` convention as architecture pages. A revised spec supersedes the prior one; do not delete — the trail of how the requirement changed is institutional memory.

### Implementation plans (plan)

The Arche captures the durable execution blueprint for a feature as `plan` pages under `plans/`, slug `plan-<feature>` (reusing the spec's feature stem, e.g. `plan-bulk-export`). `/arche-plan` is the operation skill that produces them.

A plan is the **HOW-sequence** — it instantiates an already-decided design; it does not make new architectural decisions. It sits **downstream of architecture**: `/arche-plan` grounds the plan in the spec and its SAD/ADRs (via `/arche-query`), runs an architect gap-check first (halts and routes to `/arche-architect` if a behavior lacks a covering decision), then decomposes the work and hands off to the team's dev methodology to execute.

| Slug             | Convention          | Body sections                                                                                                                                                          |
| :--------------- | :------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `plan-<feature>` | Implementation plan | Goal / Architecture / Tech stack / Arche grounding (+ architect gap-check verdict) / Global constraints / File structure / Tasks (files, interfaces, traces, steps) / Traceability (FR/SC → task) / Self-review / Execution handoff |

**Discipline.** Tasks are dependency-ordered and right-sized (smallest unit worth independent review). Every spec FR/SC maps to a task and every task traces to a requirement. No placeholders — exact paths, exact steps, type/signature consistency across tasks. The default per-step ritual is TDD but is swappable per the executing methodology; the durable contract is the decomposition + interfaces + traceability. **Durable blueprint, transient state out:** ticked checkboxes, debug notes, and commit history stay in the PR / working tree, never written back to the plan page.

**Architect gate.** `/arche-plan` does not invent architecture. If a spec behavior would force a load-bearing decision not recorded in a SAD/ADR (or contradicts a current one), the planning session halts and routes to `/arche-architect`; the plan resumes once the decision is filed. The "no new ARD/ADR required" verdict (or the gap that was routed) is recorded in the plan's *Arche grounding* section.

**Status & supersession.** Plans use the same `status:` (`proposed` | `accepted` | `superseded`) and `superseded_by:` convention as specs and architecture pages. A revised plan (spec changed, architecture changed) supersedes the prior one; do not delete — the trail of how the build approach changed is institutional memory. A plan's `spec:` frontmatter points at the spec it builds, and the spec back-links to the plan in its `## See also`.

## Slug rules

- Kebab-case, ASCII only, no dates in filename.
- Stable: prefer renaming via redirect (leave a stub pointing to the new file) over deleting.
- Discoveries re-run on the same topic use `-session-N` suffixes (e.g., `auth-rewrite.md`, then `auth-rewrite-session-2.md`). Date stays in frontmatter only.

### Slug derivation

When generating a slug from a source (URL, file path, or pasted text):

1. Pick a stem: URL → page title or final path segment; file → filename without extension; pasted text → user-supplied title or first heading.
2. Lowercase. Strip accents (NFD then drop combining marks). Replace any non-ASCII or non-alphanumeric character with `-`. Collapse repeated `-`. Trim leading/trailing `-`.
3. On collision in the target directory: if the new raw file is byte-identical to the existing one, treat it as already-ingested and skip; otherwise append `-2`, `-3`, etc.

## Page frontmatter

Every page (except files in `raw/`, which are not Arche pages) starts with YAML frontmatter:

```yaml
---
type: source | entity | concept | spec | plan | query | discovery | story | schema | index | log
title: Human-readable title
created: YYYY-MM-DD
updated: YYYY-MM-DD
tags: [tag1, tag2]
sources: [sources/foo.md]            # entities/concepts/queries/discoveries/stories/plans cite their sources
raw: raw/foo.pdf                     # source pages only — points to the raw file if one exists
url: https://...                     # source pages only — canonical URL if web-based
status: proposed | accepted | superseded   # ARD, SAD, ADR concept pages, spec pages, and plan pages
superseded_by: concepts/adr-new.md   # ARD, SAD, ADR, spec, and plan pages — when superseded, points to the replacement
context_pages: [concepts/foo.md]     # spec, discovery, and plan pages — the Arche pages loaded to ground the session
spec: specs/spec-foo.md              # plan pages only — the accepted spec this plan builds
audience: ...                        # story pages only — who the artifact is for
action_ask: ...                      # story pages only — what the audience should do/decide/believe
framework: pyramid | scqa | ...      # story pages only — narrative framework used
format: deck | narrative             # story pages only — rendered HTML shape
html: assets/stories/<slug>.html     # story pages only — path to the rendered artifact
---
```

For a source page, at least one of `raw:` or `url:` must be set. If both: `raw:` is the snapshot, `url:` is the canonical location.

`schema`, `index`, and `log` are reserved for the three system files at the Arche root (`SCHEMA.md`, `index.md`, `log.md`). All other pages use one of the content types.

For `discovery` pages, `sources:` is bidirectional: it lists both (a) the Arche pages cited inline during the session AND (b) the entity/concept pages the discovery promoted ideas to. This keeps navigation symmetric with the forward references those pages add back.

For `spec` pages, `context_pages:` lists the Arche pages loaded to ground the spec (via `/arche-query`), and `sources:` lists the pages cited inline — including any entity/concept page whose ubiquitous-language definition the spec sharpened (those pages cite the spec back). The downstream ARD/SAD that `/arche-architect` derives cites the spec in *its* `sources:`; the spec's `## See also` forward-links to that ARD once it exists.

For `plan` pages, `spec:` points at the accepted spec the plan builds, `context_pages:` lists the Arche pages `/arche-query` loaded to ground the plan (spec, SAD, ADRs, ARD, entities), and `sources:` lists the pages cited inline. The plan is downstream of those pages; the spec it builds back-links to the plan in its `## See also` (and adds it to `sources:`), keeping navigation symmetric.

For `story` pages, `sources:` lists every Arche page cited inline in the story (entities, concepts including ARD/SAD/ADR, discoveries, queries, sources). The story is a downstream consumer of those pages — they do not need a back-link unless the story revealed an issue or revision worth tracking, in which case the affected page appends a short `## See also` entry citing the story (and bumps `updated:`).

## Cross-linking

- Use relative markdown links: `[Title](../entities/foo.md)`. Do not use `[[wikilinks]]`.
- Every entity and concept page lists its sources in the `sources:` frontmatter array AND links to them inline at the point of claim.
- Source pages link out to the entities and concepts they touch (in a `## See also` section).

## index.md

The catalog. Read first when answering queries. Organized by section: Sources, Entities, Concepts, Specs, Plans, Queries, Discoveries, Stories. Each entry: bullet with title, one-line gloss, link, tags.

Append new entries on ingest. Never remove without leaving a redirect note.

## log.md

Append-only chronological record. Entry format:

```
## [YYYY-MM-DD] <op> | <title>
- pages touched: path1.md, path2.md
- notes: one-line summary
```

Ops: `ingest`, `query`, `lint`, `manual` (human edit), `init`, `migrate`, `specify`, `plan`, `discovery`, `architect`, `story`.

**Contradiction marker.** When an ingest finds a source that contradicts an existing claim, the log entry's notes line starts with `contradiction —`. Example:

```
## [2026-05-27] ingest | New paper on X
- pages touched: sources/new-paper.md, entities/foo.md, index.md
- notes: contradiction — new paper disputes earlier dating in entities/foo.md (struck through, replacement cited)
```

`/arche-lint` scans for this prefix to find unresolved contradictions. A `~~strikethrough~~` claim counts as **resolved** when the same paragraph contains a follow-up claim with a `[source link](...)` citation; otherwise lint flags it.

## Operations summary

- **ingest**: place the raw file in `raw/` (or capture a URL snapshot there) → write a summary page in `sources/` linking back to `raw/` via the `raw:` field → update index → revise affected entity/concept pages → append log entry. Touch as many pages as the source warrants; 10–15 is normal.
  - **Batch mode**: `/arche-ingest` can also process every file in `raw/` not yet referenced from any `sources/` page — useful after dropping several files in at once.
- **query**: read index → read relevant pages → answer with inline citations. If the synthesis is reusable, file it as `queries/<slug>.md` and update index + log.
- **lint**: scan for contradictions, stale dates, orphan pages (no inbound links), orphan raw files (raw file with no source page citing it), broken links, frontmatter drift, gaps in coverage. Report findings; do not auto-fix without confirmation.
- **specify**: convergent feature-specification session (`/arche-specify`). Grounds in Arche context (via `/arche-query`) → grills the user one question at a time with recommended answers, keeping it technology-agnostic → files a `specs/spec-<feature>.md` page with testable functional requirements, measurable success criteria, user scenarios, ubiquitous language, and a self-reviewed quality gate; unresolved ambiguities become ≤3 impact-prioritized `[NEEDS CLARIFICATION]` markers → sharpens affected entity/concept pages with citations → updates index and log. Sits between `/arche-discover` (which can motivate it) and `/arche-architect` (which derives the ARD/SAD/ADRs from it). The WHAT/WHY only — never the technical HOW.
- **plan**: convergent implementation-planning session (`/arche-plan`). Grounds in the spec and its architecture (via `/arche-query`) → runs an **architect gap-check first**: if any spec behavior lacks a covering SAD/ADR or would force a load-bearing decision not yet recorded, it halts and routes to `/arche-architect` before planning → otherwise decomposes the work into a `plans/plan-<feature>.md` page with a file/interface map, dependency-ordered right-sized tasks (TDD steps by default, swappable), exact paths, traceability from every FR/SC to a task, and a self-review gate → back-links the spec → updates index and log. Produce-only: hands off to the team's dev methodology to execute; transient checkbox/execution state stays in the PR, not the Arche. Sits downstream of `/arche-architect` (which it gates on) and consumes `/arche-specify`'s spec.
- **discovery**: facilitated discovery / ideation session (`/arche-discover`) for business, domain, customer, market, or regulatory topics. Reads Arche context (relevant entity/concept/query/prior-discovery pages) → runs interactive brainstorming with the user → files the session as `discoveries/<slug>.md` with the full idea inventory → promotes user-selected top ideas to concept or entity pages with citations back → updates index and log. Aims for 100+ ideas before organization. **Not** for technical-architecture work — that belongs to `/arche-architect`. **Not** for code-implementation brainstorming — that belongs to your dev methodology's own brainstorming skill.
- **architect**: convergent technical-architecture session (`/arche-architect`) acting as a panel of senior-architect lenses (Fowler, Evans, Vernon, Nygard, Hohpe, Newman, Ford, Helland, Vogels, Bass, Beck, Martin). Reads Arche context + codebase constraints → grills the user one branch at a time with recommended answers → files outputs as ARD (`concepts/ard-<system>.md`), SAD (`concepts/sad-<system>.md`), and one or more ADRs (`concepts/adr-<name>.md`) as the problem decomposes → updates index and log. Pairs with `/arche-discover` (which feeds it) and `/arche-query` (which surfaces "no relevant SAD/ADR" gaps that motivate a session).
- **story**: communication-artifact session (`/arche-tell`). Reads Arche context for a topic → interviews the user on audience, action ask, and narrative framework (Pyramid / SCQA / Story Arc / Before-After-Bridge / PAS) → files outputs as `stories/<slug>.md` (source page, frontmatter includes audience, framework, format, html-path) AND a self-contained HTML artifact at `assets/stories/<slug>.html` (designed per story — deck or scrollable narrative; underlying tool palette covers reveal.js / impress.js / plain-CSS slides; diagrams via Mermaid / SVG / CSS / Chart.js / D3 / ASCII / embedded image as appropriate) → updates index and log. The `.md` is the source of truth; the HTML is derived. Stories age as their cited ARDs/SADs/ADRs change — when a citation goes `superseded`, the story is stale and should be re-rendered or retired.
- **migrate**: `/arche-init` re-run against an existing Arche. Detects missing or stale system files (e.g., absent `_templates/`, outdated SCHEMA), proposes additive changes, applies what the user accepts, and logs the result. Never rewrites content pages.

Empty subdirectories carry a `.gitkeep` file so git tracks the structure; these have no other meaning.

## Editing rules for the LLM

- Update `updated:` frontmatter on every page touched.
- Never delete a claim without leaving a `~~strikethrough~~` and a note in log.md citing the contradicting source.
- Prefer adding to existing pages over creating near-duplicates. If unsure whether something is a new entity or a section of an existing one, ask.
- Quote sparingly. Paraphrase and cite.

## Conventions the human controls

Edit this section freely; the LLM respects it:

- **Tone**: neutral, dense, no filler.
- **Length**: source summaries ≤ 400 words, entity/concept pages grow as needed.
- **Tags**: free-form, lowercase, kebab-case.
