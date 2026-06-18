---
name: arche-discover
description: Facilitate a structured discovery / ideation session grounded in the project's Arche at ./.arche/ — for **business, domain, customer, market, or regulatory** topics. Loads relevant entity/concept/query pages as agent memory, runs facilitation across 60+ creativity techniques (Yes-And, First Principles, SCAMPER, Six Hats, Reverse Brainstorming, Cross-Pollination, etc.) with anti-bias domain pivots and energy checkpoints, then files the session as discoveries/<slug>.md, promotes user-selected top ideas to concept or entity pages, and appends a `discovery` log entry. Use when the user wants to explore unknown unknowns about the business, customer, market, or regulatory landscape; surface non-technical strategic options or risks; says "help me think through X" / "explore options for Y" for non-technical topics; or wants to use prior Arche context as a launchpad. NOT for technical architecture (designing systems, choosing patterns, deciding integrations) — use `/arche-architect` for that; it converges on ARD/SAD/ADR artifacts. NOT for code-implementation brainstorming (refactor / test structure) — use your dev methodology's own brainstorming skill (e.g. superpowers' `brainstorming`). Adapted from BMAD-METHOD's bmad-brainstorming skill.
---

# arche-discover

Run a structured discovery / ideation session that uses the project Arche as agent memory and writes its outputs back into the Arche. Scope: business, customer, market, or regulatory topics — the kinds of things that belong in the Arche as institutional context (see `.arche/SCHEMA.md`).

**Out of scope — delegate explicitly:**

- Technical architecture (designing a system, choosing patterns, deciding integrations, writing ADRs) → use `/arche-architect`, which is convergent and produces ARD/SAD/ADR concept pages with senior-architect lenses (Fowler, Evans, Vernon, Nygard, Hohpe, Newman, Ford, Helland, Vogels, Bass, Beck, Martin). If a discovery session here surfaces a strong architectural direction, suggest `/arche-architect` to converge it rather than promoting to an ADR directly.
- Code-implementation brainstorming (how to refactor X, how to test Y) → use your dev methodology's own brainstorming skill.

## Preflight

1. Verify `./.arche/SCHEMA.md` exists. If not, tell the user to run `/arche-init` first and stop.
2. Read `./.arche/SCHEMA.md` end to end.
3. Check SCHEMA defines a `discovery` page type and a `discovery` log op. If missing, tell the user to run `/arche-init` in migration mode (it will detect the stale schema and propose patches) and stop.
4. Ensure `./.arche/discoveries/` exists. If not, create it with a `.gitkeep`.
5. Read `./.arche/index.md`.

## Phase 1: Session setup

1. In one short message, ask the user for **topic**, **goals**, and any **constraints**.
2. **Load Arche context.** Scan `index.md` for entities, concepts, and prior queries plausibly relevant to the topic — aim for high recall. Read each candidate page fully. Also check `./.arche/discoveries/` for prior sessions on the same or adjacent topic; if any exist, read them — do not re-explore ground already covered.
3. Present the context bundle in one message: `"Here's what the Arche already knows about <topic>: N entities, M concepts, K prior discovery session(s)."` List them with one-line glosses. Ask: "Use this as context, ignore it, or focus on a subset?"
4. **Pick the slug.** Default: kebab-case of the topic. If `discoveries/<topic>.md` already exists, use `<topic>-session-2`, `<topic>-session-3`, etc. (per SCHEMA's discovery slug rule). Date stays in frontmatter only.

## Phase 2: Choose technique mode

Offer four modes (full prompts in [FACILITATION.md](references/FACILITATION.md)):

- **User-Selected** — user browses [TECHNIQUES.md](references/TECHNIQUES.md) and picks.
- **AI-Recommended** — facilitator picks 2–3 based on topic + Arche context. When substantial Arche context was loaded, prefer the techniques flagged as arche-leveraged in TECHNIQUES.md.
- **Random** — pick a wild card from any category. Good for stuck thinking.
- **Progressive Flow** — sequence across phases: divergent → analogical → convergent.

## Phase 3: Facilitate

Run the chosen technique(s) per [FACILITATION.md](references/FACILITATION.md). Non-negotiable disciplines:

- **One idea/provocation at a time.** Present, wait for the user, build together. No batch lists.
- **Anti-bias domain pivot every ~10 ideas.** Consciously shift to an orthogonal domain (UX → business → physics → social → ethics → governance → ...). LLMs drift toward semantic clustering; counter it.
- **Inline-cite the Arche when relevant.** When an idea touches a known page: `"this connects to [Concept Foo](../concepts/foo.md) — what if we extended it by..."`. Citations belong in the conversation now and in the discovery page later.
- **Energy checkpoint every 4–5 exchanges.** Offer: continue / switch technique / deepen / break. Never auto-conclude.
- **Aim for 100+ collaboratively developed ideas** before suggesting organization. Quantity unlocks quality. Ideas count only when they emerge through dialogue or are accepted/developed by the user.
- **No Arche writes during facilitation.** Capture ideas in-conversation using FACILITATION.md's idea format. All Arche edits batch into Phase 4.

## Phase 4: Organize and promote

Only when the user signals readiness to wrap up:

1. **Cluster** ideas into 3–6 themes; identify breakthrough concepts and cross-cutting threads.
2. **Prioritize** with the user across impact / feasibility / innovation / alignment. See FACILITATION.md.
3. **Write `.arche/discoveries/<slug>.md`** using this skill's [discovery.template.md](assets/discovery.template.md) as the layout. Frontmatter: `type: discovery`, today's date, topic, techniques used, total idea count, `context_pages:` (Arche pages loaded in Phase 1), and `sources:` — bidirectional per SCHEMA: the union of (a) Arche pages cited inline during the session and (b) any pages this discovery promoted ideas to (filled in after Phase 4 step 4). Body: themes with full idea inventory, prioritized top ideas with action plans, technique narrative + creative breakthroughs, and a `## See also` section listing every Arche page touched.
4. **Promote top ideas.** For each user-selected top idea:
   - Extends an existing concept/entity page → append with inline citation to this discovery; bump `updated:`; add the discovery to that page's `sources:` list; **and** add the page to this discovery's `sources:` list (forward and back).
   - New concept warranted → create `concepts/<slug>.md` from `/arche-ingest`'s [concept.template.md](../arche-ingest/assets/concept.template.md) with `sources: [discoveries/<slug>.md]`; add the new page to this discovery's `sources:` list.
   - If unsure whether to extend or create, ask. Slug churn is expensive.
5. **Update `index.md`.** Add the discovery under a `## Discoveries` section (create the section if missing). Add any new concept/entity pages under their sections.
6. **Append to `log.md`** with op `discovery`. List every page touched. Notes line: topic + idea count + count of promoted ideas.

## Discipline

- Arche is read-only during facilitation. All writes are batched into Phase 4.
- Promote only what the user explicitly picks. The discovery page is the home for the full inventory — concept pages stay dense and signal-rich.
- If a prior discovery on the same topic exists, the new session must build on it, not duplicate it. Cite it in the new discovery's `## See also`.
- If the conversation drifts into question-answer instead of true collaboration, pause and reset to one-idea-at-a-time facilitation.

## Output

End with one line: `Discovery session on <topic> → <N> ideas across <T> brainstorming techniques, <M> promoted. Filed as discoveries/<slug>.md.`

## See also

- [TECHNIQUES.md](references/TECHNIQUES.md) — the 61-technique library
- [FACILITATION.md](references/FACILITATION.md) — coaching patterns, idea format, energy checkpoints, prioritization framework
- [discovery.template.md](assets/discovery.template.md) — page skeleton this skill writes

_Adapted from [bmad-code-org/BMAD-METHOD](https://github.com/bmad-code-org/BMAD-METHOD)'s `bmad-brainstorming` skill (MIT-licensed)._
