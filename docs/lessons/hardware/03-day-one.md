# Hardware Lesson 3 — Day One: The Arrival & Bring-Up Ritual

## What you'll learn

Exactly what to do the moment the parts arrive — *before* you build anything. This is a
discipline borrowed from manufacturing, and it's the highest-leverage hour in the whole
project.

## The concept

Here's a mental shift for a software developer:

> **There is no `package-lock.json` for atoms.**

When you `npm install`, a hash guarantees you got exactly the right bytes. When a box of
electronics arrives, **nothing** guarantees you got the right parts. Cheap modules are
cloned, mislabeled, shipped with the wrong crystal, dead on arrival, or missing a
surface-mount component. Every part is *guilty until proven innocent*.

So treat the box landing as **"ready to audit," not "ready to build."** Manufacturers call
this *receiving inspection*. The ritual has three steps, and skipping it is how you end up
debugging firmware for a fault that was in the part all along (the misattribution trap from
[Lesson 1](01-mental-model.md)). The full version is in the
[BOM arrival ritual](../../../.arche/queries/poc-hardware-bom.md) and the
[Hardware Manual §3](../../../.arche/stories/poc-hardware-manual-for-self.md).

## Step 1 — Audit before assembly

**Put on the ESD wrist strap first.** (Clip it to a grounded metal surface — a screw on a
plugged-in, metal-cased power supply works.) Static damage is invisible and delayed, so this
comes before you touch a single chip.

Then:

- **Decode the chip top-markings.** Use a loupe or your phone's macro camera and cross-check
  each against its datasheet. Confirm the ADS1220 really is an ADS1220, the Pico is a genuine
  **RP2350** (not a relabeled RP2040), the transceiver is a **TJA1050**. The listing lies; the
  silkscreen half-lies; *the chip tells the truth.*
- **Read both MCP2515 crystal cans** (the little silver metal boxes) — each is stamped
  `8.000` or `16.000` MHz. **Label each module with tape noting its frequency.** This
  30-second step pre-empts the single most common "CAN bug" misdiagnosis. Do it now, on a calm
  afternoon — not at hour three of debugging.
- **Visual check:** bent pins, solder bridges (blobs connecting two pins), missing
  surface-mount parts, cold joints on pre-soldered headers.

## Step 2 — Smoke-test each part in isolation

This is unit-testing for atoms: prove each part works *alone* before combining them, so any
later failure has only one possible cause.

- **Power it alone.** Connect just power and ground, watch the **USB power meter**, and
  *feel/smell* for heat. Near-zero current with no heat is a good vital sign. **High current +
  hot = a chip is backwards or shorted → cut power immediately** before it dies.
- **Verify against math, not against another instrument you also don't trust.** For the
  ADS1220: short its two inputs together → it should read ≈ 0. Feed it a known voltage divider
  → your code's number must equal `(Vin / Vref) × 2²³` *computed by hand*. If the math agrees,
  you understand the conversion; if it doesn't, you've found a real problem early.
- **Use the chips' own self-test modes.** The MCP2515 has a **LOOPBACK mode** that routes its
  transmit line straight back to its receive line *inside the chip* — so you can prove the
  whole node (chip + SPI wiring + firmware + crystal) works with **no bus and no second node**.
  Also: write a value to a config register, then read it back. A chip's datasheet "modes"
  section is a bring-up toolbox.
- **Passive parts need only a meter.** The BSS138 level shifter requires no code at all: apply
  3.3V to one side, measure the other. Knowing *which* parts are meter-only versus
  code-required is itself part of the literacy.

## Step 3 — Integrate one variable at a time

Now, and only now, combine parts — **one at a time.** Add a single new part or wire, test,
confirm, then add the next. This way any failure is unambiguously caused by *the one thing you
just changed.* It's binary-search debugging applied to physical assembly.

## The ritual as a flowchart

```
Box arrives
   │
   ▼
ESD strap on?  ──no──► put it on first
   │ yes
   ▼
AUDIT: decode chip markings vs datasheet
   │
   ▼
Read & label both MCP2515 crystal cans (8 vs 16 MHz)
   │
   ▼
Visual: bent pins / solder bridges / cold joints
   │
   ▼
SMOKE-TEST each part alone
   │
   ▼
Power meter: near-zero?  ──hot/shorted──► CUT POWER (chip backwards/dead)
   │ normal
   ▼
Self-test: LOOPBACK / register read-back / math check
   │
   ▼
Passes in isolation?  ──no──► fix this part before it contaminates a multi-part debug
   │ yes
   ▼
INTEGRATE one variable at a time
```

(Adapted from the Mermaid flowchart in the
[Hardware Manual §3](../../../.arche/stories/poc-hardware-manual-for-self.md).)

## Run it yourself (when the box lands)

You don't need code for most of this — just the power meter, a multimeter, the ESD strap, and
your phone camera. Make a checklist from the three steps above and don't skip ahead to
building until every part has passed its isolation test. Photograph each chip marking as you
verify it; that record is useful later.

## Recap

- **No `package-lock.json` for atoms** — every part is guilty until proven innocent.
- **Audit:** ESD strap on, decode chip markings, **label the MCP2515 crystals**, visual check.
- **Smoke-test in isolation:** power alone (watch current/heat), verify against *hand math*,
  use built-in self-test modes, meter the passive parts.
- **Integrate one variable at a time** so failures point at a single cause.

**Next:** [Lesson 4 — The build ladder + Rungs 0–1](04-ladder-rungs-0-1.md), where you finally
start building.
</content>
