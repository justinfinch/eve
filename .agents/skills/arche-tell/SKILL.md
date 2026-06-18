---
name: arche-tell
description: Tell a story from the project's Arche at ./.arche/ — produce a shareable HTML artifact (reveal.js deck OR scrollable narrative) for communicating architecture, product strategy, or decision rationale to a defined audience. Interviews the user to lock topic, audience, action ask, and narrative framework (Pyramid, SCQA, Story Arc, Before/After/Bridge, PAS), grounding every claim in cited Arche pages (ARDs, SADs, ADRs, concept/entity pages, discoveries). Files outputs as `stories/<slug>.md` + `assets/stories/<slug>.html`, updates index.md, and appends a `story` log entry. Use when the user says "tell the story of X", "communicate our architecture to Y", "present this to Z", "turn this into a deck", or wants shareable HTML built from Arche content. NOT for ideation (use `/arche-discover`); NOT for designing a system (use `/arche-architect`) — this skill assumes the context exists and packages it for an audience.
---

# arche-tell

Run a convergent storytelling session that uses the project Arche as the source of truth and produces a shareable HTML artifact — a slide deck or a single-page scrollable narrative — backed by a first-class `stories/<slug>.md` page in the Arche.

The HTML is **designed**, not generated from a fixed template. Pick the underlying tools (reveal.js / impress.js / plain CSS slides for decks; memo / long-form / landing / report shape for narratives; Mermaid / SVG / CSS / Chart.js / ASCII for diagrams) to fit the topic and audience. Guidance lives in [DESIGN.md](references/DESIGN.md).

Where `/arche-architect` makes decisions (ARDs/SADs/ADRs) and `/arche-discover` explores ideas (discoveries), `arche-tell` **communicates** them: it takes institutional context that already exists in the Arche, locks an audience and an ask, and shapes the narrative for that audience.

## Interaction style

Every question follows the same shape: **recommended answer** → **1–3 alternative angles worth considering** → ask. The user redirects or accepts; they should rarely brainstorm from zero.

If your runtime has a structured-question tool (e.g. Claude Code's `AskUserQuestion`), use it: put the recommendation first labeled `(Recommended)`, then alternatives as the other options. Otherwise ask in prose — same shape, same recommendation-first ordering.

## Preflight

1. Verify `./.arche/SCHEMA.md` exists. If not, tell the user to run `/arche-init` first and stop.
2. Read `./.arche/SCHEMA.md` end to end.
3. Check SCHEMA defines the `story` page type AND has `story` in the log ops list. If either is missing, tell the user to run `/arche-init` in migration mode (it will detect the stale schema and propose patches) and stop.
4. Ensure `./.arche/stories/` exists; if not, create it with a `.gitkeep`. Ensure `./.arche/assets/stories/` exists; if not, create it with a `.gitkeep`.
5. Read `./.arche/index.md`.
6. Read this skill's [FRAMEWORKS.md](references/FRAMEWORKS.md), [AUDIENCE.md](references/AUDIENCE.md), [DESIGN.md](references/DESIGN.md), and the Arche page template [story.template.md](assets/story.template.md).

## Phase 1: Session setup

1. In one short message, ask the user for the **topic** (what story they want to tell) and any **time pressure** (when the artifact is needed, how long the audience has).
2. **Load Arche context.**
   - Scan `index.md` for entities, concepts, queries, and discoveries plausibly relevant to the topic. For architecture topics, look hard for `ard-*`, `sad-*`, `adr-*` concept pages — they are the spine of the story. Aim for high recall.
   - Read each candidate page fully. Note their `sources:` lists — for non-trivial claims you may need to walk into source summaries too.
   - Check `./.arche/stories/` for prior stories on the same topic. If any exist, read them — reuse the slug stem with a suffix (e.g. `auth-rewrite-for-arb-2.md`), and let the new story build on (not duplicate) the prior one.
3. Present the context bundle in one message: *"Here's what the Arche has for `<topic>`: N ARDs/SADs/ADRs, M concept/entity pages, K discoveries, J prior stories."* List each with a one-line gloss and link. Ask: *"Use this as the spine, ignore it, or focus on a subset?"*
4. **Pick the slug.** Default: `<topic>-for-<audience>` in kebab-case (e.g. `billing-architecture-for-arb`, `auth-rewrite-for-staff-eng`). The audience tag in the slug matters — the same topic told to different audiences is a different story, not a revision. Date stays in frontmatter only.

## Phase 2: Frame the story

Before interviewing, lock the frame. Two questions, recommendation-first:

1. **Framework.** Recommend one of *Pyramid / SCQA / Story Arc / Before-After-Bridge / Problem-Agitate-Solution* based on topic + audience cues. Full catalog and trigger cues in [FRAMEWORKS.md](references/FRAMEWORKS.md). Common heuristics:
   - **Pyramid Principle** — exec / decision audiences, time-pressed, "what should we do about X."
   - **SCQA** — ARB / staff-eng audiences who need to feel the problem before the answer is acceptable.
   - **Story Arc** — vision / transformation pitches, change stories, all-hands.
   - **Before/After/Bridge** — migrations, re-architectures, strategic shifts.
   - **Problem-Agitate-Solution** — audiences who underestimate the problem.
2. **Format.** Recommend *deck* or *narrative* based on consumption mode:
   - Live present / time-boxed → **deck**.
   - Async / linkable / reads on their own time → **narrative**.
   - "Both" is not a v1 option — pick the primary audience's consumption mode. The user can re-run for the other format.
   - Frontmatter records only the top-level format. The underlying *style* — reveal.js vs. plain-CSS slides for decks; memo vs. long-form vs. landing vs. report for narratives — is picked in Phase 4 and described in the story body. See [DESIGN.md](references/DESIGN.md) for the palette.

Confirm both with the user before continuing. Write nothing yet.

## Phase 3: Audience interview

One question at a time. Each question recommendation-first. Dimensions to walk (full prompts and recommendation heuristics in [AUDIENCE.md](references/AUDIENCE.md)):

1. **Who they are** — roles, seniority, technical depth. *"Recommended: platform ARB (mixed senior eng + 1–2 product), assumes deep DDD/event-driven literacy. Other angles: exec staff (shallower tech depth, sharper cost/risk focus); engineering all-hands (mixed tenure)."*
2. **What they already know** — prior context, prior decisions they sat in on, where their gaps are. Pull from Arche: if the audience already saw `sad-billing` last quarter, the story builds on it; it doesn't re-explain it.
3. **What they care about** — cost, risk, timeline, technical fit, brand, regulatory, team morale. The story's emphasis follows this.
4. **The action ask** — what you want them to *do, decide, or believe* after the artifact. Stories without an ask are scenery — refuse to proceed without one. *"Recommended: approve the migration plan. Other angles: rubber-stamp budget; sign off on architecture; just inform."*
5. **Time / length budget** — minutes for a live deck, scroll-depth for a narrative. Drives slide count / section count.
6. **Trade-offs to surface vs. omit** — what the audience must wrestle with vs. what's out of scope. Mature stories name what they don't cover; immature ones pretend it doesn't exist.

Walk one dimension at a time. Cite Arche pages inline as the audience picture takes shape: *"this aligns with [Customer Segment](../entities/customer-acme.md) — does the audience include their account team?"*

### Conversation discipline

- One question at a time. Wait for the answer before continuing.
- Always lead with the recommendation; the user redirects.
- Surface contradictions: if the user's audience model contradicts a known entity page (e.g. they describe the ARB as "execs" but the Arche has them as senior eng), name it and ask which is right.
- No Arche writes during the interview. All artifacts batch into Phase 5.

## Phase 4: Outline and visual approach

Only when the frame and audience are locked:

1. **Draft the outline** in the chosen framework's skeleton (see [FRAMEWORKS.md](references/FRAMEWORKS.md)). For a deck: list slide titles + one-line speaker note per slide. For a narrative: list section headings + one-line summary per section. Inline-cite the Arche page each section/slide leans on.
2. **Pick the underlying style** per [DESIGN.md](references/DESIGN.md):
   - **Deck format** → recommend the deck framework: *reveal.js* (default, most cases), *impress.js* (vision/transformation with bold spatial concept), or *plain-CSS slides* (≤8 slides, no presenter mode). Recommendation-first; user can override.
   - **Narrative format** → recommend the narrative shape: *memo* (exec/board, ≤800 words), *long-form* (technical audience, 800–2500), *landing* (mixed audience, hero + scroll, 1500–4000), or *report* (formal, 2500+).
3. **Plan the diagrams.** For each diagram proposed, name **(a) the section/slide it lands in**, **(b) the claim it carries**, and **(c) the tool**: Mermaid, inline SVG, CSS+HTML, Chart.js, D3, ASCII, or embedded image. Pick the simplest tool that fits the content — see DESIGN.md's tool palette. If a cited SAD/ADR already contains a diagram, prefer **copying it through verbatim** in whatever form it's in. Budget: typically 1–3 per deck, 2–5 per narrative.
4. **Show the outline + style + diagram plan in one message.** Highlight any sections where the Arche is thin (you'd be claiming without a citation) and offer to either drop the section or recommend an `/arche-ingest` first.
5. **Get the user's sign-off.** They can: accept; reorder; cut sections; expand sections; swap a diagram tool; or change framework / format / style (loops back to Phase 2). No HTML until they accept.

## Phase 5: Write artifacts

Only when the outline is signed off:

1. **Write the story page** at `.arche/stories/<slug>.md` using [story.template.md](assets/story.template.md). Frontmatter: `type: story`, today's date, tags, the audience block, the action ask, the framework, the format, and `html: assets/stories/<slug>.html` pointing at the rendered file. Body: the outline expanded — each section/slide has its narrative text + speaker notes (decks) + cited Arche pages + any diagram source (Mermaid block, inline SVG, ASCII art, CSS-diagram markup, embed instruction). The `## Style` section names the chosen deck framework / narrative shape and any tool choices that aren't obvious from the body. **The .md is the source of truth.** The HTML is generated from it; re-renders should be possible from the .md alone.
2. **Design the HTML** at `.arche/assets/stories/<slug>.html`, following [DESIGN.md](references/DESIGN.md). The HTML is **authored** — not produced by copying a fixed template:
   - Pick typography, color, spacing per DESIGN.md's principles, tuned to the audience and the topic's emotional valence.
   - For **deck format**, structure the HTML around the chosen deck framework (reveal.js / impress.js / plain CSS slides). Pull the framework's CSS/JS from a pinned CDN major version. Include speaker notes when the framework supports them; otherwise as collapsed `<details>` per slide.
   - For **narrative format**, plain semantic HTML + CSS. Match the chosen narrative shape (memo / long-form / landing / report).
   - Diagrams: implement each in the tool chosen in Phase 4. Multiple tools in the same artifact is fine.
   - The hero/opening and the closing/ask follow the patterns in DESIGN.md — audience tag, single-sentence subtitle, framed accent-colored ask.
   - Self-contained discipline (DESIGN.md): one file, no build step, works when double-clicked.
3. **Sources frontmatter** on the story page lists every Arche page cited (entities, concepts including ARD/SAD/ADRs, discoveries, queries, sources). The body inline-cites at the point of claim.
4. **Update existing pages.** If the session leaned heavily on a particular SAD or entity page, append a short `## See also` entry on those pages with a citation back to this story. Bump their `updated:`. Do not rewrite.
5. **Update `index.md`.** Add the story under a `## Stories` section (create the section if missing). One-line gloss, link, audience tag.
6. **Append to `log.md`** with op `story`. List every page touched (story page + HTML file + any back-link updates + index.md). Notes line: `<topic> → <format> for <audience>`.

## Discipline

- Arche is read-only during the interview. All writes batch into Phase 5.
- Every non-trivial claim in the story needs an inline citation to an Arche page. If you would write an uncited claim, you're inventing — flag the gap and offer an `/arche-ingest` or `/arche-architect` instead.
- The .md is the source of truth; the HTML is a derived artifact. Re-render the HTML from the .md; never edit only the HTML.
- Refuse to produce a story without an explicit action ask. "Just informing" is a valid ask but the user must say it.
- A story for audience A and a story for audience B on the same topic are **different stories**, not revisions. Different slugs, different audience tags, both filed.
- Stories age. When the underlying SAD/ADR they cite is superseded, the story is stale — do not edit the story silently to track the new decision; re-render or retire it.
- Pick the simplest diagram tool that carries the claim. Defaulting to Mermaid because it's familiar is the most common failure mode — see DESIGN.md's tool palette. If the user needs marketing-polish branded visuals, hand off to a designer; the Arche's role is correctness, not pixels.
- Self-contained HTML — no build step, opens by double-click. CDN libraries pinned to a major version are fine.

## Output

End with two lines:

```
Story `<slug>` filed → .arche/stories/<slug>.md + .arche/assets/stories/<slug>.html
Open the HTML in a browser to present or share.
```

## See also

- [FRAMEWORKS.md](references/FRAMEWORKS.md) — narrative-framework catalog with trigger cues and skeletons
- [AUDIENCE.md](references/AUDIENCE.md) — audience-dimension cheat sheet, interview prompts, and common archetypes
- [DESIGN.md](references/DESIGN.md) — visual principles, deck/narrative style palette, diagram tool palette, and pattern snippets
- [story.template.md](assets/story.template.md) — Arche story page skeleton (the markdown source of truth; the HTML is designed per story, not templated)
