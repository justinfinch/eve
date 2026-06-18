# Lenses

The twelve architects this skill invokes by name. Each entry: what they push on, and the **trigger cues** that should cause you to name them in the conversation. Lenses are pedagogy — the user learns whose framework is in play — not theatrics. Do not impersonate; do not caricature; do not invent quotes.

When two lenses disagree on the same question (common with Vernon and Helland on consistency), surface both and ask the user to pick. When no lens applies cleanly, ask the question in your own voice.

---

## Martin Fowler — patterns and trade-offs

**Pushes on:** named patterns, trade-off articulation, refactoring debt, the "why-not-X" trail for rejected alternatives, integration patterns at the enterprise scale.

**Trigger cues:**
- User proposes a design without naming the pattern → ask which pattern it is, which it isn't, and what the trade-off is.
- A choice is being made without alternatives on the table → ask what else was considered and why-not.
- A pattern is being applied by reflex → ask whether the constraint that justifies it is actually present.

---

## Eric Evans — strategic DDD and ubiquitous language

**Pushes on:** ubiquitous language, bounded contexts, context maps, the seam between domains, the gap between "the term the team uses" and "the term the code uses."

**Trigger cues:**
- A term is being overloaded across contexts → ask whose language this is and where the boundary is.
- The user mixes business terms and technical terms in the same sentence → separate them.
- A design crosses what feels like a bounded context → ask whether the context map shows it.

---

## Vaughn Vernon — tactical DDD and aggregate design

**Pushes on:** aggregate boundaries, invariants enforced inside vs. across aggregates, event-driven cohesion, the "small aggregate" bias, command/event modeling.

**Trigger cues:**
- A single transaction wants to mutate state across what look like two aggregates → ask whether that's one aggregate (transactional) or two (eventually consistent).
- Invariants are being asserted without saying where they live → ask which aggregate owns each invariant.
- The user is reaching for a saga → confirm it's actually a multi-aggregate workflow and not a missing aggregate.

---

## Michael Nygard — failure modes and operational reality

**Pushes on:** failure modes, stability patterns (circuit breaker, bulkhead, timeout, backpressure), blast radius, recovery story, runbook surface.

**Trigger cues:**
- A new dependency is being introduced → ask what happens when it's down, slow, or returns garbage.
- Synchronous calls are crossing a trust or process boundary → ask about timeouts, retries, and the backpressure story.
- The user describes the happy path → ask what page-er gets paged when it breaks at 3am.

---

## Gregor Hohpe — integration shape and async semantics

**Pushes on:** integration style (file / shared db / RPC / messaging), message contracts, idempotency, the "architect elevator" framing of choice cost, sync-vs-async semantics.

**Trigger cues:**
- Two services need to talk → ask which of the four integration styles this is and why.
- Async messaging is proposed → ask about idempotency, ordering guarantees, and delivery semantics.
- The cost of a decision is unclear → ask "what does this cost to undo six months from now?"

---

## Sam Newman — service boundaries and decomposition

**Pushes on:** service boundary justification, the cost of distribution, contract evolution, monolith-first vs decompose-first, what the seam buys you.

**Trigger cues:**
- A new service is being proposed → ask what the seam buys that a module boundary wouldn't.
- Multiple services share a database → flag it as a coupling smell that nullifies the seam.
- Decomposition is being driven by team structure, not domain → name Conway's law and check whether that's actually intended.

---

## Neal Ford — evolutionary architecture and fitness functions

**Pushes on:** fitness functions (the executable check that the architecture is still right), evolvability, one-way doors vs reversible decisions, architectural characteristics over time.

**Trigger cues:**
- A characteristic is asserted (e.g. "must be fast") without a measurement → ask what the fitness function looks like and where it runs.
- A decision is irreversible → name it as a one-way door and confirm the user knows.
- The user is optimizing for today's load → ask what the fitness function for "still right in 18 months" would check.

---

## Pat Helland — data on the inside vs. outside

**Pushes on:** data ownership, immutable data crossing trust boundaries, the difference between data-at-rest and data-in-flight, identity and reference vs. value, eventual consistency as a first-class concern.

**Trigger cues:**
- Data is shared between services → ask who owns it and whether what crosses is a snapshot, a reference, or a stream.
- An ID is being passed around → ask whose ID space it lives in.
- A consistency boundary is being assumed without naming it → name it and confirm what's transactional vs eventual.

---

## Werner Vogels — distributed-systems reality

**Pushes on:** "everything fails all the time," cell-based architecture, blast radius containment, operational simplicity, the cost of consistency at scale.

**Trigger cues:**
- A design assumes a dependency stays up → ask what happens when it doesn't and how the system degrades.
- Strong consistency is being asserted across a partition → ask whether the latency and availability cost was priced in.
- Operational complexity is creeping → ask which on-call rotation owns this at 3am.

---

## Len Bass — quality attributes taxonomy

**Pushes on:** the quality-attribute scenario format (stimulus → environment → response → measure), the taxonomy of -ilities (availability, modifiability, performance, security, testability, usability), tactics that move each attribute.

**Trigger cues:**
- An NFR is asserted vaguely ("highly available", "scalable") → reframe as a quality-attribute scenario with measurable response.
- A tactic is being applied → name which attribute it serves and what it costs the others.
- Multiple attributes are in tension → make the trade-off explicit.

---

## Kent Beck — simple design and testability

**Pushes on:** the four rules of simple design (passes tests, reveals intention, no duplication, fewest elements), test-driven seams, "make the change easy, then make the easy change."

**Trigger cues:**
- A design adds elements without paying for them → ask which simple-design rule each element earns.
- A part of the design is hard to test → name it as a seam problem and ask where the test boundary should sit.
- The user is over-engineering for future needs → ask what the test for "needed now" would look like.

---

## Robert C. Martin — dependency direction and clean architecture

**Pushes on:** SOLID principles (especially Dependency Inversion and Single Responsibility), the dependency rule (concentric layers, dependencies point inward), use-case / interactor boundaries, what's policy vs. mechanism.

**Trigger cues:**
- Dependencies point from domain code to infrastructure → name the inversion and ask whether an abstraction belongs at the boundary.
- A class or module has multiple reasons to change → ask whether that's one responsibility or two.
- Business rules are leaking into delivery mechanism (controllers, ORMs) → name the layer violation.

---

## Using the panel

- Lenses are tools, not personalities. A single question may invoke two or three; some questions invoke none.
- If you find yourself naming the same lens for every question, you're forcing the framing. Re-read this file and pick the one whose territory actually applies.
- The user can name a lens too: *"what would Nygard say about this?"* — when they do, answer in that lens's territory and skip your own framing.
- Lenses are a vocabulary the Arche user shares with you for shorthand. They are not a substitute for engaging with the user's actual problem.
