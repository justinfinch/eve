---
name: arche-specify
description: Convergent feature-specification skill for the Arche at ./.arche/. Turns a feature idea into a technology-agnostic WHAT/WHY specification — testable requirements, measurable success criteria, user scenarios, ubiquitous language — grounded in Arche context (business / domain / SME / prior decisions) and stored back as a `spec-<feature>` page under ./.arche/specs/. Grills the user one question at a time with recommended answers, caps unresolved questions as ≤3 impact-prioritized [NEEDS CLARIFICATION] markers, and runs a requirements-quality gate before handing off. Use when the user wants to specify a feature, write or sharpen requirements, says "spec out X", "write a spec for Y", "what should we build for Z", or is downstream of /arche-discover and ready to scope a concrete feature. NOT for the technical HOW — designing systems, choosing patterns / integrations, writing ADRs — that's /arche-architect, which the spec feeds. NOT for business / customer / market / regulatory ideation — that's /arche-discover.
---

# arche-specify

Run a convergent feature-specification session that uses the project Arche as agent memory and writes its output back as a `spec-<feature>` page. The spec captures **WHAT the feature must do and WHY** — in user and business language, technology-agnostic — so it can ground everything downstream.

Position in the pipeline:

`/arche-discover` (business / market / domain ideation) → **`/arche-specify` (feature WHAT/WHY)** → `/arche-architect` (technical HOW: ARD / SAD / ADR) → your dev methodology's planning + implementation.

This skill is decisive like `/arche-architect`, not divergent like `/arche-discover`: every question carries a recommended answer, walks the spec tree branch by branch, and converges on one durable artifact with a quality gate.

## What goes in a spec (and what doesn't)

A spec describes the **problem and the required behavior**, never the solution:

- **In:** the problem and why it matters, target users and scenarios, testable functional requirements, measurable success criteria, the ubiquitous language, scope boundaries (non-goals), assumptions and dependencies.
- **Out:** frameworks, languages, data models, API shapes, deployment, patterns, vendor choices — every technical *HOW*. The moment a question becomes "how do we build it," it belongs to `/arche-architect`. Defer it; do not answer it here.

If the user's request is actually architectural ("how should we structure the service", "what queue do we use", "design the schema"), say so and route to `/arche-architect`. A spec that names technologies has leaked.

## Interaction style

Every question follows the same shape: **recommended answer** → **1–3 alternative angles worth considering** → ask. The user redirects or accepts; they should rarely brainstorm from zero.

If your runtime has a structured-question tool (e.g. Claude Code's `AskUserQuestion`), use it: put the recommendation first labeled `(Recommended)`, then the alternatives as the other options. The runtime supplies "Other" for free-form input. Otherwise ask in prose — same shape, recommendation-first.

## Preflight

1. Verify `./.arche/SCHEMA.md` exists. If not, tell the user to run `/arche-init` first and stop.
2. Read `./.arche/SCHEMA.md` end to end.
3. Check SCHEMA defines the **`spec` page type** (row pointing at `specs/<slug>.md`), the **`specify` log op**, and that `status:` / `superseded_by:` apply to spec pages. If any are missing, tell the user to run `/arche-init` in migration mode (it will detect the stale schema and propose patches) and stop.
4. Ensure `./.arche/specs/` exists. If not, create it with a `.gitkeep`.
5. Read `./.arche/index.md`.
6. Read this skill's [spec.template.md](assets/spec.template.md) so you write the page in the canonical layout.

## Phase 1: Session setup

1. In one short message, ask the user for the **feature**, the **problem / why** it exists, the rough **definition of success**, and any **non-negotiable constraints**.
2. **Load Arche context — via `/arche-query`.** Don't read pages ad hoc; invoke `/arche-query` to surface the institutional context that should ground the spec: relevant entities, concepts, prior discoveries, and any existing ARD/SAD/ADR for the system this feature touches. This is what "grounded in the Arche" means — the spec's requirements and language descend from filed context, not from a cold start. Also sweep the codebase only for *stated* product behavior already shipped (so the spec doesn't re-specify what exists); do not derive requirements from implementation.
3. Present the context bundle in one message: prior decisions (with statuses), related entities, related discoveries, prior specs on adjacent features. One-line glosses each. Ask: "Use this as context, ignore it, or focus on a subset?"
4. **Pick the feature slug.** Default: `spec-<feature>` where `<feature>` is a 2–4 word action-noun kebab-case stem (e.g. `spec-user-onboarding`, `spec-bulk-export`). If `specs/spec-<feature>.md` already exists and this is a revision, edit in place; if it's a distinct feature that collides, disambiguate per SCHEMA's slug rules. Date stays in frontmatter.

## Phase 2: Frame the spec

Before grilling, confirm scope with the user:

- **One feature, one spec** (default). If the request is really an epic, recommend slicing it into named features and specifying the first; capture the rest as non-goals or a follow-up list.
- Confirm this is a **WHAT/WHY spec**, not architecture. If it's architectural, redirect to `/arche-architect`.
- If an `/arche-discover` session motivated this, cite it as the spec's origin.

Write nothing yet.

## Phase 3: Grill

One question at a time, recommendation-first. Walk the spec tree branch by branch (re-order to fit the feature):

- **Problem & why** — the user/business need; the cost of not doing it. Ground in Arche context.
- **Target users & scenarios** — who, primary flow, alternate flows, edge conditions. In user language, observable behavior.
- **Functional requirements** — what the system must do, each phrased so it's **testable**. Number them FR-1, FR-2…
- **Success criteria** — how you'll know it worked, each **measurable and technology-agnostic**. Number them SC-1, SC-2… A criterion that names a tool or framework is wrong — restate the underlying measure.
- **Ubiquitous language** — the terms in play. Define **what each thing IS, not what it does** (grill-with-docs discipline). Reconcile every term against existing Arche entity/concept pages — if the feature redefines a known term, surface the collision; capture aliases to avoid so the team doesn't drift.
- **Non-goals & scope boundary** — apply YAGNI ruthlessly. What are we explicitly *not* building this pass, and why.
- **Assumptions & dependencies** — what's treated as true; what other feature/system/team/party this relies on.

Disciplines during the grill:

- One question at a time. Wait for the answer. Always lead with the recommendation; the user redirects.
- **Explore the Arche or codebase instead of asking** when the answer is already written down. Don't ask a question the repo or a filed page can answer.
- **Inline-cite Arche pages as you go:** *"this aligns with [Concept X](../concepts/x.md) — but the customer signal in [Discovery Y](../discoveries/y.md) suggests…"*. Surface contradictions; do not silently overwrite.
- **Defer every HOW** to `/arche-architect`. When a technical question surfaces, note it as a hand-off item and move on — do not let it into the spec.
- **`[NEEDS CLARIFICATION]` discipline.** When a genuine ambiguity can't be resolved by your recommendation or by the Arche, mark it rather than guessing. Cap at **three** total; if more surface, prioritize **scope > security > UX > technical** and fold the rest into assumptions or non-goals. Present each open clarification as concrete options and wait for the user's pick.
- **No Arche writes during the grill.** All artifacts batch into Phase 4.

## Phase 4: Write the spec + quality gate

Only when the user signals the spec tree is walked:

1. **Restate.** One message: the feature in a sentence, the FR/SC counts, the non-goals, and any open clarifications. Get confirmation.
2. **Write the spec** at `.arche/specs/spec-<feature>.md` using [spec.template.md](assets/spec.template.md). Frontmatter: `type: spec`, today's date, tags, `context_pages:` (the Arche pages `/arche-query` loaded in Phase 1), `sources:` (every Arche page cited inline). Set `status: accepted` once the user approves the gate below; use `proposed` if they want it left as a draft.
3. **Run the quality gate** (the template's `## Quality gate` checklist) as a self-review: no implementation detail leaked, every FR testable, every SC measurable and tech-agnostic, no placeholders/contradictions/undefined terms, non-goals explicit, open clarifications ≤3, ubiquitous language reconciled with Arche pages. Fix inline; **bounded to 3 iterations** — if an item still fails, record it as a `[NEEDS CLARIFICATION]` rather than looping. This is the hard approval gate: the user must accept before any handoff.
4. **Update existing pages.** If the session sharpened the ubiquitous language, append (don't overwrite) the affected entity/concept pages with citations to this spec, and add this spec to their `sources:`; add those pages to the spec's `sources:` (forward and back). If a new domain term warrants its own page, create the entity/concept page citing this spec. Don't promote technical decisions — those are `/arche-architect`'s to file.
5. **Update `index.md`.** Add the spec under a `## Specs` section (create it if missing). One-line gloss + tags. Add any new entity/concept pages under their sections.
6. **Append to `log.md`** with op `specify`. List every page touched. Notes line: feature + FR/SC counts + open-clarification count.
7. **Hand off.** Recommend `/arche-architect` to converge the technical HOW — it will cite this spec in the ARD/SAD it produces. If the architecture for this system is already settled in the Arche (a current SAD/ADRs cover it), recommend the user's dev-methodology planning skill instead, with the spec as its input. Suggest; do not auto-invoke.

## Discipline

- Arche is read-only during the grill. All writes batch into Phase 4.
- WHAT/WHY only. The first technology name in a spec is a defect — defer it to `/arche-architect`.
- Ground in the Arche via `/arche-query`, not ad-hoc reads. A spec with no cited context either skipped grounding or the feature is genuinely greenfield (say which).
- Every functional requirement testable; every success criterion measurable and tech-agnostic.
- `[NEEDS CLARIFICATION]` markers capped at 3, impact-prioritized. Guessing past a real ambiguity is worse than marking it.
- A spec can supersede an earlier spec for the same feature. Mark the old one `status: superseded` with `superseded_by:` pointing at the replacement — never delete; the trail of how the requirement changed is institutional memory.
- If a requirement contradicts an existing Arche claim, surface it in conversation and use the SCHEMA's contradiction convention (`~~strikethrough~~`, new claim with inline citation, log notes prefixed `contradiction —`).

## Output

End with one line: `Spec on <feature> → spec-<feature> filed with <N> requirements, <M> success criteria, <K> open clarification(s). Index and log updated. Recommend /arche-architect next.`

## See also

- [spec.template.md](assets/spec.template.md) — the page skeleton this skill writes
