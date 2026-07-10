# Hardware Lesson 1 — How Hardware Breaks Your Software Instincts

## What you'll learn

The three mental models you carry as a software developer that are **silently false** in
hardware, and the one meta-skill that ties them together. Internalize these *before* you
wire anything — they'll save you hours of misdirected debugging.

## The concept

You've spent years building reliable instincts: catch exceptions, trust the compiler,
initialize your variables. Those instincts are excellent — for software. In the physical
world, several of them quietly invert. The danger isn't that you lack knowledge; it's that
your *existing* knowledge actively points you the wrong way. This whole lesson comes from the
project's own analysis in
[Learning Hardware as a Software Dev](../../../.arche/concepts/learning-hardware-as-a-software-dev.md).

## Trap 1 — Hardware has no `try/catch`

In software, a wrong value throws an exception you can catch, log, and recover from. Worst
case, you `git reset` and try again.

In hardware, a wrong value is **current going where it shouldn't** — and the failure is
physical and permanent. Wire a motor straight to a GPIO pin and the back-EMF spike can kill
the pin. Reverse a chip's power and it cooks. **There is no `git reset` for a fried chip.**

> **The discipline this forces:** *measure before you connect.* Use a multimeter to confirm a
> voltage or a wire *before* you apply power, not after. "Run it and see" is a software
> habit; in hardware, "see, then run" keeps your parts alive.

## Trap 2 — "It compiled, so it's right" — the bug is off-screen

In software, if it compiles and the logic is clean, the bug is *somewhere in your code*. You
narrow the search by reading code.

In hardware, perfectly clean firmware still returns garbage because of things that are
**invisible to your editor**:

- a cold (barely-touching) breadboard contact,
- the wrong SPI mode (more on `CPOL`/`CPHA` in [Lesson 5](05-rung-2-can.md) and [6](06-rungs-3-4-adc.md)),
- a byte-order flip between chips,
- a MISO wire that isn't actually connected.

> **The trap:** when "the code is correct," your instinct is to *re-read the code*. The right
> move is the opposite — "the code is correct" should send you to the **physical** layer, not
> deeper into the source.

## Trap 3 — State lives in the physical world, not in variables

In software, an uninitialized variable is a bug, but at least memory has *some* defined
behavior. You control state.

In hardware, state escapes your program entirely:

- A **floating** input pin (connected to nothing) reads random noise — there is no default
  `0`. You must add a *pull resistor* to define it.
- A chip's **configuration registers persist inside the chip** across re-flashes of your
  firmware. The MCP2515 especially will "remember" a config you set in a previous run, so a
  bug can survive a code change.
- **Capacitors and power rails carry state.** A motor starting up can momentarily drop the
  voltage ("brown-out") and reset your microcontroller; a missing decoupling capacitor causes
  mystery resets.

> **The translation:** "initialize your variables" has a hardware twin — *pull resistors,
> explicit register config, and decoupling caps.* State you don't explicitly pin down will be
> defined by physics, not by you.

## What *does* transfer

Not every instinct betrays you. The big one that survives: **"fail fast on the riskiest
assumption."** That's exactly why the build ladder moves the scary CAN bus *early* (rung 2)
instead of saving it for last — you want to hit the hardest, most failure-prone part while
you're fresh and while little else is built to confuse the diagnosis. ([Lesson 4](04-ladder-rungs-0-1.md)
covers the ladder.)

## The meta-skill: misattribution

Underneath all three traps is one pattern, and it's the deepest thing this POC teaches:

> **Physical failures wear software costumes.**

A crystal running at 8 MHz when your firmware expects 16 MHz reads, for hours, as a "CAN
bug." Static damage from last week reads as "flaky firmware." A cold contact reads as a
"code bug." The skill you're building is to **suspect the physical layer first** — to
recognize the costume.

Everything in the rest of this track (the logic analyzer, the power meter, the
"smoke-test in isolation" ritual, the diagnosis tree in [Lesson 8](08-rung-6-and-diagnosis.md))
exists to make the physical layer *observable*, so you can strip off the costume and see
what actually failed.

## Recap

- **Trap 1:** no `try/catch` — failures are physical and permanent → *measure before you
  connect.*
- **Trap 2:** "it compiled" doesn't help — the bug is often off-screen in the physical layer.
- **Trap 3:** state lives in pins, registers, and capacitors, not just variables → pull
  resistors, explicit config, decoupling caps.
- **Transfers:** "fail fast on the riskiest assumption" — why CAN comes early.
- **Meta-skill:** physical failures masquerade as software bugs; *suspect the physical layer
  first.*

**Next:** [Lesson 2 — Know your parts](02-know-your-parts.md), a guided tour of everything
you ordered.
</content>
