# Hardware Lesson 8 — Rung 6 + the Diagnosis Playbook

## What you'll learn

The final rung — the AI layer, which is pure software layered on everything below — and the
single most important hardware skill: a debugging order that starts at the physical layer
instead of the code.

## Rung 6 — the AI layer + shareable permalink

**New hardware concept:** none. This rung is entirely software, layered last *on purpose*.

The flow: a natural-language intent in the browser → an AI produces a test plan → the plan runs
on the molecule → a plain-English readout of the result → a shareable permalink so anyone can
reproduce the run.

It sits cleanly on top because every rung below already exposes a **clean capability over the
self-ID spine**. The AI doesn't poke pins directly — it works through the same
capability-described interface the browser already uses. (The project deliberately constrains
the AI to a validated, capability-schema'd, simulator-first test plan — never a raw actuator
command — see
[ADR — AI constrained planner](../../../.arche/concepts/adr-ai-constrained-planner.md) and
[ADR — Capability registry](../../../.arche/concepts/adr-capability-registry.md).)

You build this with the software skills you already have — it's the cap on the stack, not a new
hardware challenge. The molecule's final capability list reflects everything it can do, and the
AI reads that list to know what's possible.

## The diagnosis playbook

This is the payoff of the whole track. When a rung misbehaves, **run the misattribution check
before you open the firmware.** Your software instinct is to start at the code; literacy is
starting at power. Work down this order:

```
Symptom: "it doesn't work"
   │
   ▼
Power meter: is current sane?
   ├─ near-zero ───► nothing's running. Check power, contacts, and whether CS asserts.
   ├─ too high + hot ► CUT POWER. A chip is backwards / shorted / dead.
   └─ normal
        │
        ▼
   Did it EVER work?
   ├─ worked, now flaky ► suspect ESD damage / a cold breadboard contact / a loose dupont wire.
   └─ never worked
        │
        ▼
   Does a single part pass its self-test in isolation?
   ├─ no ──► bad / clone / DOA part. Bisect with the Golden Reference Node.
   └─ yes
        │
        ▼
   Logic analyzer: are the bits actually moving?
   ├─ no bits / CS not asserting ► wiring / pin map / SPI mode (CPOL-CPHA).
   └─ bits move, value wrong ► crystal mismatch / byte-order / a stale config register.
        │
        ▼
   ONLY NOW suspect the firmware logic.
```

(Adapted from the Mermaid diagnosis tree in the
[Hardware Manual §5](../../../.arche/stories/poc-hardware-manual-for-self.md).)

**The order is the lesson:** power → history → isolation → wire-level → *then* code. A software
developer's instinct is to jump straight to the last box. The literacy you've been building is
the discipline to start at the first one. Each tool you bought maps to a step: the **power
meter** answers the first question, the **Golden Reference Node** the isolation question, the
**logic analyzer** the wire-level question.

## The ask (how to actually do this track)

Build Concept A **one rung at a time, validating each in isolation before stacking the next.**
For every rung, write the short *"what's actually happening in the silicon"* note — the
physics, why this chip, the failure modes, the mapping between code and silicon. **That trail
is the deliverable.**

Treat success as **being able to explain the hardware**, not as making the demo pass. And when
it breaks: **suspect the physical layer first.**

This closes the loop with [Lesson 0](00-orientation.md): the repo becomes a textbook you
authored by building it — half software ([the software track](../README.md)), half hardware
(this one), unified by the one `SelfId` contract that runs through both.

## Recap

- **Rung 6** is pure software — the AI layer rides on the capability spine every hardware rung
  built up; it never touches pins directly.
- **The diagnosis playbook:** power → history → isolation → wire-level → **then** code. Start at
  the top, not the bottom.
- **The ask:** one rung at a time, write the silicon note each time, optimize for *explanation*
  over a passing demo, and suspect the physical layer first.

**You've finished the hardware track.** Head back to the [hardware index](README.md) or the
[main lessons index](../README.md) anytime — and when the parts land, start at
[Lesson 3's arrival ritual](03-day-one.md).
</content>
