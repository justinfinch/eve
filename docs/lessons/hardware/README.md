# Hardware Lessons — Building the Real Molecule

The [first lesson track](../README.md) taught the **software Foundation**: a *simulated*
molecule announcing itself to a browser. This track is the other half — the **physical
build**: taking the parts you've ordered and bringing them up into a real molecule, one
careful step at a time.

> **The one rule (read this twice).** The goal of this hardware work is **literacy, not a
> working rig**. For a software developer, "just get it working" is a trap — a live bus you
> can't explain is worth less than a humbler one you deeply understand. So every lesson
> ends the same way: **write down what's actually happening in the silicon.** That write-up
> *is* the deliverable.
>
> And its corollary, which governs all debugging: **when something breaks, suspect the
> physical layer first.** In hardware, physical failures wear software costumes.
>
> Both come straight from the project's own research — see
> [Learning Hardware as a Software Dev](../../../.arche/concepts/learning-hardware-as-a-software-dev.md)
> and the [POC Hardware Manual](../../../.arche/stories/poc-hardware-manual-for-self.md).

## How this track relates to the software track

The software you already built doesn't get thrown away — the hardware *grows into it*:

- The `SelfId` message (`{id, name, capabilities}`) you met in the software lessons is the
  **same spine** the physical molecule announces. Each hardware rung you complete adds a
  string to that `capabilities` list (`["gpio"]` → `["gpio","adc"]` → …).
- The simulator was a stand-in for a board. These lessons replace it, step by step, with a
  real one — but the *browser side* barely changes, because the contract is the same.

## A note before you start

These lessons describe parts that are **on order**. Read 0–4 now (mental model, your parts,
and the day-one ritual) so that when the box lands you're *ready to audit*, not scrambling.
Lessons 5–9 are the bench build — return to them at the workbench, one rung at a time.

⚙ **Pin numbers are "derived — verify on the bench."** The exact GPIO assignments in these
lessons come from the Pico 2 pinout + the chosen modules, **not** from the project's locked
research. The research only fixes the *safe range* (GP10–GP21, external LED on GP15, avoid
GP23/24/25/29 on the Pico 2 **W**). Always confirm against your actual board.

## The lessons

| # | Lesson | What it covers |
|---|--------|----------------|
| 0 | [Orientation & the one rule](00-orientation.md) | What you're building physically, and why "literacy not rig" |
| 1 | [How hardware breaks your software instincts](01-mental-model.md) | The 3 traps + the misattribution meta-skill |
| 2 | [Know your parts](02-know-your-parts.md) | Every part you ordered, what it is, what it mirrors at Adom |
| 3 | [Day one — arrival & bring-up ritual](03-day-one.md) | Audit → smoke-test → integrate; "no package-lock for atoms" |
| 4 | [The build ladder (overview) + Rungs 0–1](04-ladder-rungs-0-1.md) | Learning order vs feature order; LED + browser loop |
| 5 | [Rung 2 — the CAN bus (the big one)](05-rung-2-can.md) | SPI, differential signaling, the crystal + MISO gotchas |
| 6 | [Rungs 3–4 — reading analog](06-rungs-3-4-adc.md) | Built-in ADC, then the ADS1220 over SPI |
| 7 | [Rung 5 — driving a motor with PWM](07-rung-5-pwm.md) | PWM, transistors, flyback diodes, power rails |
| 8 | [Rung 6 + the diagnosis playbook](08-rung-6-and-diagnosis.md) | The AI layer, and how to debug physical-first |

## The deeper references

These lessons are a teaching layer over the project's own bench documentation. When you
want the full, dense version, go to the source:

- [POC Hardware Manual — Build It, Understand It](../../../.arche/stories/poc-hardware-manual-for-self.md)
  — the 4500-word bench reference these lessons are based on.
- [POC Hardware BOM](../../../.arche/queries/poc-hardware-bom.md) — exact SKUs, prices, and
  build notes.
- [Learning Hardware as a Software Dev](../../../.arche/concepts/learning-hardware-as-a-software-dev.md)
  — the traps, the ladder, the misattribution meta-skill.
</content>
