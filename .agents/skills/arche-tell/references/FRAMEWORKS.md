# Narrative Frameworks

A small, opinionated catalog. The skill **recommends** one based on the topic + audience and shows the others as alternatives — the user can override. Each entry includes: what it is, when to use it, the section/slide skeleton, and an example opening and closing.

These are tools, not laws. The framework shapes the spine; the Arche supplies the meat.

## 1. Pyramid Principle (Barbara Minto)

**What it is.** Answer first. Lead with the single "so what" — the recommendation, the call, the headline — then group the supporting reasoning into 2–4 pillars. Every level of the hierarchy answers a single question the level above implies.

**Use when:**
- Audience is time-pressed (execs, ARB sign-off, board, busy staff+).
- A *decision* is being asked for, not an exploration.
- The audience is more interested in *what to do* than *how we got here*.

**Skeleton:**
1. Title + audience tag + date.
2. **The ask.** One sentence: what the audience should approve / decide / believe.
3. **Why this is the right call.** 2–4 pillars (one slide / section per pillar). Each pillar starts with its own one-line claim, then 2–3 supporting points cited from the Arche.
4. **What we considered and rejected.** Brief, with the reason. ADR alternatives are gold here.
5. **What we need from you.** Restate the ask as a verb with a date.

**Opening line example:** *"We should migrate billing to the event-driven architecture in `sad-billing`, starting next quarter — three reasons, then your questions."*

**Closing line example:** *"What we need: sign-off on the migration plan by 2026-06-20 so vendor procurement can start."*

## 2. SCQA — Situation, Complication, Question, Answer (also Minto)

**What it is.** Minto's narrative form. Set the scene the audience already agrees with (Situation), introduce the change that breaks it (Complication), state the question the room is now asking (Question), then answer (Answer). The audience feels the problem before the solution arrives — which makes the solution acceptable.

**Use when:**
- Audience is technical and resistant to top-down conclusions (ARB, staff-eng forums).
- The problem is non-obvious — the audience would dismiss the answer if you led with it.
- You need to *re-anchor* a familiar situation as no-longer-stable.

**Skeleton:**
1. **Situation** — what's been true. Cite the SAD / entity pages that describe the current state. *"For the last 18 months, billing has been a synchronous monolith. It's worked."*
2. **Complication** — what changed or broke. New scale, new regulation, new dependency, surprising failure mode. Cite the discoveries / sources that surfaced it.
3. **Question** — the question the room is now asking. *"What do we do about this?"* Make it explicit; do not assume it.
4. **Answer** — the recommendation, with 2–4 pillars (sub-claims cited from the Arche).
5. **Trade-offs** — what we lose to get the answer. Surface the ADR alternatives that lost.
6. **The ask** — what the audience does next.

**Opening line example:** *"For most of last year, billing was a fine synchronous service. Then Acme onboarded — and the durability story stopped working."*

**Closing line example:** *"We're proposing event-driven billing. It costs us 8 weeks and a 2x ops surface. The room needs to decide whether the durability guarantee is worth it."*

## 3. Story Arc — Setup / Conflict / Resolution

**What it is.** Three-act narrative. Establish the world, introduce conflict, resolve. The oldest structure in storytelling; the audience absorbs it without conscious effort.

**Use when:**
- Vision / transformation pitches (re-org, re-platform, new product line).
- All-hands or company-wide audiences — mixed depth, mixed engagement.
- You want the audience to *feel* the journey, not just be informed of an outcome.

**Skeleton:**
1. **Setup** — who we are, what we've been doing, what's been working. Anchor in entity pages (the team, the product, the customer).
2. **Conflict** — the tension. What we've outgrown, what's at stake, what happens if we don't act. Drama is allowed; exaggeration isn't — cite the discoveries / sources that surfaced the conflict.
3. **Resolution** — where we're going, how we get there, what life looks like on the other side. The SAD is usually the spine here.
4. **What's next** — concrete near-term steps, named owners, dates.

**Opening line example:** *"Two years ago we built billing for ten customers. We have four hundred now. The thing that took us here can't take us further."*

**Closing line example:** *"Next quarter we start. Here's the first three milestones, here's who owns each."*

## 4. Before / After / Bridge

**What it is.** Show the current state ("Before"), the desired state ("After"), then the path between them ("Bridge"). Tight, visual, ideal for migrations and re-architectures.

**Use when:**
- Migrations, re-architectures, replatforming, vendor switches.
- Strategic shifts where the *delta* is the story (not the destination on its own).
- Audiences who need to see "we know where we are" before they trust "we know where we're going."

**Skeleton:**
1. **Before** — current state, warts and all. Architecture diagram (Mermaid from the current SAD), pain-points list cited from discoveries / incident postmortems / SME interviews.
2. **After** — target state. Architecture diagram (Mermaid from the new SAD), what gets better and by how much (cite NFR targets from the ARD).
3. **Bridge** — the migration plan. Phases, milestones, what's reversible vs. one-way doors (cite the ADRs). Risk register.
4. **The ask** — approve the bridge, fund the bridge, or pick the bridge variant.

**Opening line example:** *"Today: one shared Postgres, three monoliths, twelve-minute deploys. Target: event-driven, isolated stores, deploy-per-service. Here's how we get there in three phases."*

**Closing line example:** *"We're asking for sign-off on phase 1 — eight weeks, two engineers, reversible. Phases 2 and 3 are designed but not yet committed."*

## 5. Problem / Agitate / Solution (PAS)

**What it is.** Marketing-style. Name the problem. *Agitate* it — show the cost of not solving it, the worst-case trajectory. Then offer the solution as relief.

**Use when:**
- Audience underestimates the problem ("it's fine, we'll deal with it later").
- Funding / staffing requests where the case for *doing anything* is the bar, not the case for *this specific solution*.
- Risk and security stories — vulnerabilities, technical debt, single-points-of-failure that haven't broken yet.

**Skeleton:**
1. **Problem** — name it precisely. Cite the entity / source / discovery that captured it.
2. **Agitate** — concrete cost. What it costs us today (cite metrics or incidents from sources), what it costs us in 6 / 12 / 24 months on the current trajectory, what the worst-case looks like.
3. **Solution** — the recommendation. Brief on *what*; lean on the linked SAD / ADRs for *how*.
4. **The ask** — usually a resource ask: people, budget, calendar time.

**Caveat:** Use PAS sparingly with technical audiences — they detect manipulation. Works best when the cost is genuinely under-appreciated, not when you're inflating it.

**Opening line example:** *"Our auth middleware stores session tokens in a way legal flagged in March. We have not fixed it."*

**Closing line example:** *"We're asking for two engineers for one quarter to rewrite it. The alternative is a regulatory finding I'd rather not own."*

## Picking the framework

Quick triage (the skill should reason through these before recommending):

| Audience signal                              | Lean toward          |
| :------------------------------------------- | :------------------- |
| Exec / time-pressed / decision-asking        | Pyramid              |
| Technical / skeptical / problem-first        | SCQA                 |
| Mixed / all-hands / vision                   | Story Arc            |
| Migration / re-platform / "before vs after"  | Before/After/Bridge  |
| Under-appreciated problem / funding ask      | PAS                  |

When two are close, prefer **Pyramid for decks** and **SCQA for narratives** — Pyramid's answer-first shape fits live-presented slides where the audience can't go back; SCQA's slow build works better for async reading where the reader can re-scan.

## Anti-patterns

- **Burying the ask.** Every framework above ends with what the audience does next. If the artifact has no ask, it's scenery, not a story.
- **Pillar inflation.** Pyramid with 7 pillars isn't Pyramid; it's a list. Cap at 4. If you have more, group them.
- **Uncited drama.** PAS's "agitate" and Story Arc's "conflict" can fabricate if not Arche-grounded. The drama must trace to a real source.
- **Framework drift mid-story.** Pick one and finish it. If a section "doesn't fit the framework," cut the section or change the framework — don't mix.
- **Too many diagrams.** One diagram per slide is noise. Use diagrams where the spatial relationship is the claim — typically 1–3 per deck, 2–5 per narrative. Diagram tool palette and selection guidance live in [DESIGN.md](DESIGN.md).
