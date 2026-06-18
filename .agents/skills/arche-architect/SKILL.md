---
name: arche-architect
description: Convergent technical-architecture skill for the Arche at ./.arche/. Acts as a panel of senior architects (Fowler, Evans, Vernon, Nygard, Hohpe, Newman, Ford, Helland, Vogels, Bass, Beck, Martin) invoked as lenses by topic. Interviews the user one branch at a time with recommended answers, then files outputs as concept pages — ARD (`ard-<system>`), SAD (`sad-<system>`), and ADRs (`adr-<name>`) as the problem decomposes — cites Arche context, updates index.md, appends an `architect` log entry. Use when the user is deciding a technical architecture; says "design X", "architect this", "ADR for X", "SAD for Z"; OR is downstream of `/arche-discover` and wants to converge architectural ideas; OR `/arche-query` flagged no relevant SAD/ADR before planning. NOT for business / customer / market / regulatory ideation — that belongs to `/arche-discover`. NOT for code-implementation brainstorming — use your dev methodology's own skill.
---

# arche-architect

Run a convergent technical-architecture session that uses the project Arche as agent memory and writes its outputs back into the Arche as ARD / SAD / ADR concept pages. Acts as a panel of senior architects — each named as a **lens** when their territory comes up, not as a theatrical persona.

Where `/arche-discover` is divergent (business / customer / market / regulatory / domain ideation, 100+ ideas), `arche-architect` is decisive: every question has a recommended answer, walks the design tree branch by branch, and produces a small number of durable artifacts.

## Interaction style

Every question follows the same shape: **lens** (if one applies) → **recommended answer** → **1–3 alternative angles worth considering** → ask. The user redirects or accepts; they should rarely have to brainstorm from zero.

If your runtime has a structured-question tool (e.g. Claude Code's `AskUserQuestion`), use it: put the recommendation first labeled `(Recommended)`, then the alternative angles as the other options. The runtime supplies "Other" for free-form input. Otherwise ask in prose — same shape, same recommendation-first ordering.

## Research discipline

Lenses are stable — bounded contexts, consistency models, failure modes don't age. **Concrete instantiations** do: vendor choice, library version, current CVE posture, emerging-domain patterns. Lenses come from training; instantiations need verification before they become a recommendation.

Reach for web research (WebSearch / WebFetch or your runtime's equivalent) when the question hinges on:

- **Vendor or tool selection** — current options, pricing, lifecycle status, recent acquisitions/deprecations.
- **Version- or CVE-sensitive decisions** — current stable, known vulns, EOL dates.
- **Fast-moving domains** — LLM orchestration, agentic systems, edge compute, WebGPU, post-quantum crypto, evolving regulatory regimes.
- **User explicitly asks** — *"what's current"*, *"is there something newer"*, *"has X changed since"*.
- **Low-confidence recommendations** — if your recommendation rests on something you're not sure is current, verify before recommending, not after.

Rules:

- **Arche first.** Before searching the web, check the Arche for prior ingested research on the topic — `/arche-ingest` may have already cached a recent comparison. Re-fetching when a fresh-enough source already lives in `./.arche/sources/` is wasted context.
- **Research before the recommendation, not after.** The recommendation lands as a *current* call, not a stale one followed by an apology.
- **Cap it.** ~1–3 targeted lookups per such question. Don't go indiscriminate — this skill is already context-hungry.
- **Cite at the point of claim** in conversation, and carry the citation into the ADR's `sources:` (or the SAD's body) so future re-decisions know *what state of the world* this call was made against. *"Picked X because Y was unavailable in 2026-06"* is much more useful in two years than *"picked X."*
- **Don't research the lenses.** Fowler/Evans/Vernon/etc. are durable principles. Researching *"latest DDD thinking"* is a smell — you're either second-guessing a stable lens or the question is actually about an instantiation in disguise.

## Preflight

1. Verify `./.arche/SCHEMA.md` exists. If not, tell the user to run `/arche-init` first and stop.
2. Read `./.arche/SCHEMA.md` end to end.
3. Check SCHEMA defines the **architecture page conventions** for `ard-`, `sad-`, and `adr-` slugs AND has `architect` in the log ops list. If any are missing, tell the user to run `/arche-init` in migration mode (it will detect the stale schema and propose patches) and stop.
4. Read `./.arche/index.md`.
5. Read this skill's [LENSES.md](references/LENSES.md) and the three templates so you write pages in the canonical layout: [ard.template.md](assets/ard.template.md), [sad.template.md](assets/sad.template.md), [adr.template.md](assets/adr.template.md).

## Phase 1: Session setup

1. In one short message, ask the user for the **technical problem**, **success criteria**, and any **non-negotiable constraints** (regulatory, infra, deadline, team-skill).
2. **Load Arche context.**
   - Scan `index.md` for prior `ard-*`, `sad-*`, `adr-*` concept pages, related entities (systems, teams, vendors), prior discoveries on adjacent topics, and prior queries. Aim for high recall — read each candidate page fully.
   - Sweep the codebase for stated constraints the Arche may not have captured: language/runtime choice, infrastructure manifests (devbox.json, Dockerfile, terraform), top-level READMEs, existing service boundaries.
3. Present the context bundle in one message: prior decisions (with statuses), related entities, related discoveries, and codebase constraints. List them with one-line glosses. Ask: "Use this as context, ignore it, or focus on a subset?"
4. **Pick the system slug.** Default: kebab-case of the system or problem (e.g. `billing`, `order-fulfillment`). This is the stem reused by `ard-<system>` and `sad-<system>`. ADR slugs are decision-specific (`adr-<decision-name>`).

## Phase 2: Frame the artifacts

Before grilling, decide which artifacts this session will produce. Recommend one of:

- **ARD only** — the problem is upstream of design; you're capturing what the architecture must satisfy before any decision is made. Use when constraints / quality attributes are still fuzzy.
- **ARD + SAD + ADRs (most common)** — the standard full session. ARD frames the requirements; SAD describes the holistic solution; ADRs capture each load-bearing decision the SAD relies on.
- **SAD + ADRs** — an ARD already exists (cite it) and you're designing against it.
- **ADRs only** — a tightly scoped decision inside an existing SAD. Cite the SAD; produce one or more ADRs.

Confirm with the user. Write nothing yet.

## Phase 3: Grill

One question at a time. Each question:

1. Names the **lens** if a specific architect's territory is in play: *"Evans would push on the ubiquitous language here — what does the business call this thing?"* Lens names are pedagogy, not theatrics. See [LENSES.md](references/LENSES.md) for the roster and trigger cues.
2. Offers a **recommended answer** grounded in the Arche context, codebase reality, and the lens.
3. Includes **1–3 alternative angles** worth considering (different lenses, opposing trade-offs, common patterns you'd otherwise have to brainstorm). These become the other options in a structured-question UI, or are listed inline in prose.
4. **Explores the Arche or codebase instead of asking** when the answer is already written down. Don't ask a question the repo can answer.

Walk the design tree branch by branch. The standard branches (re-order to fit the problem):

- **Quality attributes / NFRs** (Bass, Ford): latency budget, availability target, durability, throughput, scalability shape, security posture, observability, cost ceiling. *"What's the fitness function that tells you in 18 months this is still right?"*
- **Bounded contexts and ubiquitous language** (Evans, Vernon): what are the contexts in play, what terms collide across them, where do the seams sit.
- **Aggregate and consistency boundaries** (Vernon, Helland): which invariants must hold transactionally, which can be eventually consistent, who owns each piece of data.
- **Integration shape** (Hohpe, Newman): sync vs async, request/response vs event, contract style (REST/gRPC/event schema), idempotency, retries, backpressure.
- **Failure modes** (Nygard, Vogels): what breaks first under load / partition / dependency outage, blast radius, recovery story, runbook surface area.
- **Patterns and trade-offs** (Fowler, Richards): named patterns considered, why-Y over why-not-X for each.
- **Modular structure and testability** (Beck, Martin): seams that make this testable, dependency direction, what's leak-prone.
- **Evolvability** (Ford): what changes do we expect, what fitness functions guard them, what's a one-way door vs reversible.

When a real trade-off crystallizes — hard to reverse, surprising-without-context, genuine alternatives — flag it as an ADR candidate inline. Don't write it yet; capture the decision, context, alternatives, and consequences in conversation.

### Conversation discipline

- One question at a time. Wait for the answer before continuing.
- Always lead with the recommendation; the user redirects.
- Name the lens when one applies. If two lenses disagree (Vernon and Helland on consistency, common case), surface both perspectives and ask the user to pick.
- Inline-cite Arche pages as you go: *"this aligns with [ADR-N](../concepts/adr-foo.md) — but contradicts [Concept X](../concepts/x.md), which you'd be implicitly overturning."* Surface contradictions; do not silently overwrite.
- If the user's answer contradicts the codebase, name it: *"the code in `src/billing/` already does X — which is right?"*
- No Arche writes during the grill. All artifacts batch into Phase 4.

## Phase 4: Write artifacts

Only when the user signals the design tree is walked:

1. **Restate the decisions.** One message: ARD scope, SAD shape, list of ADR candidates with one-line summaries. Get the user's confirmation.
2. **Write the ARD** at `.arche/concepts/ard-<system>.md` using [ard.template.md](assets/ard.template.md). Cite the session's source pages, related entities, prior discoveries, the SAD it pairs with (forward link).
3. **Write the SAD** at `.arche/concepts/sad-<system>.md` using [sad.template.md](assets/sad.template.md). The Decision Summary section links forward to every ADR this SAD relies on.
4. **Write each ADR** at `.arche/concepts/adr-<name>.md` using [adr.template.md](assets/adr.template.md). Each ADR's frontmatter cites the SAD in `sources:` so the back-pointer exists. Status defaults to `accepted` (this skill converges on decisions, not proposals); use `proposed` only if the user explicitly wants it that way.
5. **Update existing pages.** If the session touched entities or prior concept pages, append (don't overwrite) with citations to the new ARD/SAD/ADRs. If a new ADR supersedes a prior one, set the prior ADR's `status: superseded` and `superseded_by:` to the new path. Do not delete the old page — the trail of "we reversed X after Y" is the institutional memory.
6. **Update `index.md`.** Add the new ARD/SAD/ADRs under Concepts (the schema's concept section). One-line gloss + tags each.
7. **Append to `log.md`** with op `architect`. List every page touched. Notes line: system + one-line summary of what was decided + ADR count.

## Discipline

- Arche is read-only during the grill. All writes batch into Phase 4.
- One question at a time. Always recommend with 1–3 alternative angles; the user redirects.
- Cite at the point of claim, not just in frontmatter.
- ADRs only when the bar is met (hard to reverse, surprising-without-context, real trade-off). Decisions that don't clear the bar live inside the SAD body, not as their own ADR.
- An ADR can supersede an existing ADR. Mark the old one `superseded` with `superseded_by:` pointing at the new path — never delete.
- If a session-level decision contradicts an existing claim on an entity or concept page, surface it in conversation and use the SCHEMA's contradiction convention (`~~strikethrough~~`, new claim with inline citation, log entry's notes prefixed `contradiction —`).
- A SAD without ADRs is a sketch. A grilled session should produce at least one ADR; if it produced none, the grill stopped too early or the problem belongs in `/arche-discover` instead.

## Output

End with one line: `Architect session on <system> → ARD + SAD + <N> ADR(s) filed. Index and log updated.`

## See also

- [LENSES.md](references/LENSES.md) — the twelve-architect panel with trigger cues
- [ard.template.md](assets/ard.template.md), [sad.template.md](assets/sad.template.md), [adr.template.md](assets/adr.template.md) — page skeletons this skill writes
