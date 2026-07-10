# Hardware Lesson 0 — Orientation & the One Rule

## What you'll learn

What you're physically building, how it connects to the software you already wrote, and the
single rule that should govern every decision you make at the bench.

## The concept

In the software track you built a **simulated** molecule: a program on your laptop that
announced `{id, name, capabilities}` to a browser over a WebSocket. That proved the *shape*
of the system. This track builds the **real thing** — a small electronics board (a
"molecule") that:

- **self-identifies** (announces who it is and what it can do),
- is **driven from a browser**,
- **reads a sensor** (a knob, a temperature sensor), and
- **drives an actuator** (an LED, a motor),

…end to end, on real silicon. This is **Concept A — the Mini-Molecule + Cloud Workbench**
(see [POC: Mini-Molecule + Cloud Workbench](../../../.arche/concepts/poc-mini-molecule-cloud-workbench.md)).

You already have the software skills for half of it. This track is the *other* half: the
physical build, and — more importantly — the **understanding** of what's happening
underneath.

### Why this maps onto a real company's product

The project deliberately mirrors the stack of a real startup, **Adom**, which builds
"molecules" (modular PCBs) robot-wired in factory "workcells." Every part you'll touch was
chosen to mirror a real Adom chip family (see
[Adom Technical Architecture](../../../.arche/concepts/adom-technical-architecture.md)). So
when you learn the MCP2515 CAN bus or the ADS1220 ADC, you're not learning a toy — you're
learning a desk-scale version of a production system.

## The one rule

> **The deliverable is hardware literacy, not a working rig.**

This is the most important sentence in the whole track, and it's counter-intuitive for a
software developer. Our instinct is: *make the demo pass.* In hardware, that instinct
betrays you. A blinking LED you can't explain teaches you nothing; a circuit you can *reason
about* — even a simpler one — is the actual prize.

So every lesson in this track ends with the same homework: **write down what's actually
happening in the silicon.** Why does this resistor exist? What would happen without it? What
is the chip doing electrically? That written trail is the deliverable. Think of the repo as
a textbook you're authoring by building it.

This framing isn't arbitrary — it came out of a structured pre-mortem of this exact POC,
which explicitly reframed the goal from "working rig" to "hardware literacy" (see
[Learning Hardware as a Software Dev](../../../.arche/concepts/learning-hardware-as-a-software-dev.md)
and the [POC Unknown-Unknowns discovery](../../../.arche/discoveries/poc-unknown-unknowns.md)).

## The corollary: suspect the physical layer first

The rule has a debugging twin:

> **When something breaks, suspect the physical layer first.**

In software, a bug is in your code. In hardware, a "bug" is often a crystal running at the
wrong frequency, a cold breadboard contact, a chip damaged by static last week, or a wire in
the wrong hole — and all of these **masquerade as software bugs**. You'll spend three hours
debugging firmware when the real problem was a loose jumper.

Learning to flip that instinct — to reach for the multimeter and the logic analyzer *before*
the debugger — is the single deepest skill this whole POC is training. [Lesson 1](01-mental-model.md)
unpacks exactly why your instincts mislead you here.

## How the pieces will come together

You won't build it all at once. The build is a **ladder** of rungs, each adding one new
hardware concept on top of the last:

```
Rung 0  LED            → GPIO, current-limiting, 3.3V logic
Rung 1  Browser loop   → self-ID over USB; capabilities: ["gpio"]
Rung 2  CAN bus        → SPI + differential signaling (the hard one)
Rung 3  Built-in ADC   → analog → digital, the easy way
Rung 4  ADS1220 ADC    → precision ADC over SPI
Rung 5  PWM a motor    → actuation + power discipline
Rung 6  AI layer       → pure software, layered on top
```

The order is chosen for *learning safety*, not for how the demo flows — [Lesson 4](04-ladder-rungs-0-1.md)
explains why. Each rung extends the same `capabilities` list your software molecule already
knows how to announce.

## Recap

- You're building **Concept A**: a real molecule that self-identifies, is browser-driven,
  measures, and actuates — the physical counterpart to the simulator.
- The parts mirror a real company's (Adom's) stack, so the skills transfer.
- **The one rule:** the deliverable is *literacy, not a working rig* — write down what's
  happening in the silicon at every step.
- **The corollary:** when it breaks, *suspect the physical layer first.*

**Next:** [Lesson 1 — How hardware breaks your software instincts](01-mental-model.md).
</content>
