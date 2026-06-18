# Audience

A story without a defined audience is a status update. This sheet covers the dimensions to probe in Phase 3, the recommendation heuristics the skill should lean on, and a handful of common audience archetypes worth recognizing on sight.

The audience drives **every** framework / format / depth / diagram decision downstream. Get this right; the rest follows.

## Dimensions

Walk these in order. One question per dimension. Always lead with a recommendation grounded in what the Arche already knows about the audience (entity pages for teams, prior story pages for the same audience).

### 1. Who they are

**What to ask:** Roles, seniority, team affiliation, named individuals if known. Whether they overlap with any entity already in the Arche.

**Recommendation heuristic:**
- Default to the most likely named group based on the topic (architecture topic + recent ARD → "platform ARB"; product strategy + recent discovery → "product staff").
- If the topic touches a customer or partner, ask whether the customer's team is in the room.
- If the user names individuals, check if there's an Arche entity for them or their team.

### 2. Technical depth

**What to ask:** How deep should the technical detail go? Can the audience read a sequence diagram? Do they want the SAD-level view or the ADR-level view?

**Recommendation heuristic:**
- ARB / staff-eng / principal: SAD-level + selected ADR detail. Mermaid OK.
- Eng-management / TPM: SAD-level. Architecture diagrams OK, sequence diagrams sparingly.
- Exec staff: ARD-level (drivers, quality attributes, business outcomes). Diagrams only when they carry decision weight.
- Mixed / all-hands: pick the audience's *median* — assume the senior eng in the back row won't be impressed and the junior PM in the front won't be lost; aim for both.
- Board / external: business outcome + 1–2 diagrams max. Treat as exec-staff minus context.

### 3. What they already know

**What to ask:** Prior context, prior decisions they sat in on, recent reorgs, where their gaps are likely to be.

**Recommendation heuristic:**
- Sweep the Arche for prior story pages targeting the same audience. If the audience saw `sad-billing` last quarter, the new story builds on it; recap in one sentence, not a slide.
- Where the user says "they don't know X," check if X is documented — sometimes "they don't know" means "I haven't shown them," sometimes it means "the institution hasn't decided yet" (different problem; offer `/arche-architect`).
- Recent reorgs and turnover destroy assumed context. If the audience composition shifted in the last 6 months, treat their prior context as thinner than instinct suggests.

### 4. What they care about

**What to ask:** Cost / risk / timeline / technical fit / brand / regulatory / team morale / career — which of these does this audience weight, in what order?

**Recommendation heuristic:**
- ARB: technical fit > risk > timeline > cost.
- Exec staff: cost > risk > timeline > technical fit.
- Board: business outcome > regulatory > brand > everything else.
- Eng all-hands: career / team morale / technical fit (rarely cost — they don't control it).
- Customer team: brand > timeline > risk > cost.
- The order shapes which pillar leads in Pyramid, which complication anchors SCQA, etc.

### 5. The action ask

**What to ask:** What do you want the audience to **do, decide, or believe** after this story? Approve, fund, sign off, allocate, just-be-aware, change-their-mind, escalate, hand-off.

**Recommendation heuristic:**
- If the user can't name an ask, *stop* and surface it: *"Stories without an ask are scenery. What changes for the audience after this lands?"*
- "Just informing" is a valid ask but should be said out loud — and usually means the story is shorter than the user thinks.
- The ask determines the closing slide / section verbatim. Write it down in Phase 3 and don't lose it.
- Multiple asks → split into multiple stories. One ask per story. Mature presenters know this; instinct fights it.

### 6. Time / length budget

**What to ask:** For a deck — how many minutes do you have, including Q&A? For a narrative — how long will the audience actually read?

**Recommendation heuristic:**
- Deck slide count ≈ minutes-of-talk × 0.75 (1 slide ≈ 75 seconds of talking on average). 30-minute slot with 10 min Q&A → ~15 slides.
- Narrative length: exec audience → ≤ 800 words; technical audience → ≤ 2500 words; deep technical → 2500–5000 words. Longer than 5000 and they won't read it; cut.
- If the user says "they'll spend as long as it takes" — they're wrong. Pick the upper bound and cut.

### 7. Trade-offs to surface vs. omit

**What to ask:** What does the audience need to wrestle with, and what is deliberately out of scope?

**Recommendation heuristic:**
- Surface the trade-offs the audience *will ask about anyway*. Hiding them looks evasive; pre-empting them looks rigorous.
- Out-of-scope items should be named, not silently dropped. "We're not deciding X today — that's a separate session" preserves trust.
- ADR alternatives that were rejected for non-obvious reasons are gold — surface the rejection logic. Cite the ADR.

### 8. Consumption mode (cross-check with format)

**What to ask:** Live presented (with you in the room), async (linked, read alone), or hybrid (presented live but also circulated after)?

**Recommendation heuristic:**
- Live → deck. The presenter fills gaps verbally; the slides are scaffolding.
- Async → narrative. The reader has no presenter; the artifact must stand alone, with citations they can follow.
- Hybrid → deck **with rich speaker notes**, so the deck reads sensibly when circulated. The narrative format is also valid here if the live presentation is short.

## Archetypes

Common audiences worth recognizing — when one of these matches, the recommendation defaults shift.

### Platform ARB (Architecture Review Board)

- **Roles:** Senior eng, principals, maybe 1–2 staff PMs.
- **Depth:** SAD + ADR detail. Mermaid in full.
- **They care about:** technical fit, risk, downstream blast radius, precedent ("are we setting a pattern other teams will copy?").
- **Default ask shape:** approve / reject / send back with conditions.
- **Default framework:** SCQA.
- **Time budget:** 30–60 min, half of which is Q&A.

### Exec staff / leadership team

- **Roles:** VP+, sometimes C-level.
- **Depth:** ARD-level. Quality attributes and business outcomes, not implementation.
- **They care about:** cost, risk, timeline, brand, regulatory, what we're choosing *not* to do.
- **Default ask shape:** fund / staff / approve a strategic direction.
- **Default framework:** Pyramid.
- **Time budget:** 15–30 min. They will interrupt; design for that.

### Engineering all-hands

- **Roles:** Mixed across teams and tenures.
- **Depth:** SAD-level, lighter on jargon.
- **They care about:** career, team morale, technical fit, what they personally will be working on.
- **Default ask shape:** just-be-aware, change-their-mind about a direction, recruit volunteers.
- **Default framework:** Story Arc.
- **Time budget:** 10–20 min.

### Board / external stakeholders

- **Roles:** Investors, partners, regulators.
- **Depth:** Business outcomes and risk posture only.
- **They care about:** financial impact, regulatory standing, brand, competitive positioning.
- **Default ask shape:** approve a milestone, ratify a direction, just-be-informed.
- **Default framework:** Pyramid or PAS.
- **Time budget:** 10–20 min and you only get the first sentence to land the headline.

### Customer or partner team

- **Roles:** Their tech leads, their product, possibly their procurement.
- **Depth:** Whatever serves the customer's decision — usually SAD-level for integration stories.
- **They care about:** brand, timeline, durability, integration complexity, what they have to do.
- **Default ask shape:** sign off on an integration plan, accept a change, escalate internally.
- **Default framework:** Before/After/Bridge when the story is about a change to *their* experience; otherwise Pyramid.
- **Time budget:** highly variable; ask.

### Internal team (the people doing the work)

- **Roles:** Engineers, designers, PM on the team.
- **Depth:** ADR-level. They will implement this; they need the seams.
- **They care about:** technical fit, what they personally own, what's reversible, how it's tested.
- **Default ask shape:** kick off the work, align on sequencing, surface blockers.
- **Default framework:** Before/After/Bridge or SCQA.
- **Time budget:** 30–60 min, deeply interactive.

## Anti-patterns

- **"Tell the story to engineering."** Engineering isn't an audience; it's a department. Push for who, specifically.
- **Audience = everyone.** Mature presenters know: a story for everyone is a story for no one. Pick the primary audience; secondary audiences read the same artifact but it isn't optimized for them.
- **Inferring the audience from the topic.** "Architecture story" doesn't imply ARB — sometimes architecture stories go to execs (different framework, different depth). Ask, don't assume.
- **Reusing an audience archetype without checking.** "Standard ARB story" is a smell — every ARB has its own composition and recent context. Cross-check against entity pages.
- **No ask.** Re-flagged here because it's the single most common failure mode. A story with no ask is a status update; file it as a `query` synthesis or an `entity` page update, not as a story.
