# Hardware Lesson 6 — Rungs 3–4: Reading Analog

## What you'll learn

How to turn a real-world voltage into a number your code can read — first the easy way (the
Pico's built-in ADC), then the precise way (the ADS1220 over SPI). The two-step approach is a
deliberate teaching trick.

## The concept

An **ADC** (analog-to-digital converter) measures a voltage and reports a number. A
potentiometer — a knob — is a *voltage divider*: turning it produces a voltage somewhere
between 0V and 3.3V. Feed that into an ADC and you can read the knob's position in software.

The clever part of this pair of rungs is the **variable isolation**: rung 3 proves the entire
browser → device → measure → stream loop using the *built-in* ADC (almost no hardware risk).
Then rung 4 swaps in the external ADS1220 over SPI — so the **only new variable** is the SPI
chip. If rung 4 breaks, you already know the loop works, so the fault must be the new chip or
its wiring. (This is the "integrate one variable at a time" discipline from
[Lesson 3](03-day-one.md), applied to the build itself.)

> ⚙ Pin numbers are derived — verify on the bench.

---

## Rung 3 — Potentiometer via the Pico's *built-in* ADC

**New concept:** analog ↔ digital, resolution, and reference voltage — at essentially zero
hardware risk.

**Wiring (⚙ derived):**

```
3V3 ──[ pot top terminal ]
          wiper (middle) ─── GP26 (ADC0)
GND ──[ pot bottom terminal ]
```

**What's happening in the silicon:** the potentiometer is a voltage divider; its wiper feeds
0–3.3V into GP26, which is **ADC0** on the Pico. The RP2350's 12-bit SAR ADC returns a number
from `0` to `4095`, where the value ≈ `(Vin / 3.3) × 4095`. So each "count" represents about
3.3V / 4096 ≈ **0.8 millivolts**. That's what "12-bit resolution against a 3.3V reference"
means concretely.

**Verify:** rotate the pot and watch the value stream to the browser. Check the endpoints — fully
one way reads ≈ 0, fully the other reads ≈ 4095.

**Write-up (the deliverable):** what "12-bit resolution against a 3.3V reference" means in
volts-per-count.

This rung also extends the self-ID spine: the molecule now announces
`capabilities: ["gpio", "adc"]`.

---

## Rung 4 — The same pot via the ADS1220 over SPI

**New concept:** SPI modes (`CPOL`/`CPHA`), register configuration, 24-bit delta-sigma
conversion, and the programmable-gain amplifier. Because only the SPI chip is new, any failure
is unambiguous.

**Wiring (⚙ derived — shares the SPI bus with the MCP2515, but a separate CS):**

```
Pico            ADS1220
GP18 (SCK)  ───  SCLK
GP19 (MOSI) ───  DIN
GP16 (MISO) ───  DOUT
GP20 (CS)   ───  CS      ← its own chip-select, distinct from the MCP2515's
GP14 (DRDY) ───  DRDY    ← "data ready" — the chip tells the Pico when a sample is ready
3V3         ───  AVDD/DVDD
GND         ───  GND
pot wiper   ───  AIN0    (AIN1 → GND)
```

Notice SCK/MOSI/MISO are *shared* with the MCP2515 from [Lesson 5](05-rung-2-can.md) — that's
how SPI works: multiple chips on the same three lines, each picked by its own CS.

**What's happening in the silicon:** the ADS1220 is a **24-bit delta-sigma** ADC. Instead of
one quick measurement (like the Pico's SAR ADC), it oversamples rapidly and noise-shapes,
trading speed for far more resolution and stability — which is why it beats the built-in ADC
for precision work. Two things to respect:

- **SPI mode (`CPOL`/`CPHA`).** SPI has four "modes" defining clock polarity and phase. Set the
  wrong one and *every byte is garbage* even though your firmware is perfect — Trap 2 from
  [Lesson 1](01-mental-model.md), the bug that's off-screen.
- **Config registers persist in-chip** (Trap 3), and it has a programmable-gain amplifier plus
  onboard IDAC current sources for ratiometric sensor reads (the thermistor trick from
  [Lesson 2](02-know-your-parts.md)).

**Verify against math — don't trust an instrument you also don't trust:** short AIN0 to AIN1
→ it should read ≈ 0. Feed a known voltage divider → your code's number must equal
`(Vin / Vref) × 2²³` *computed by hand*. If a byte looks wrong, put the logic analyzer on the
SPI lines. A correct number means you understand the conversion well enough to explain it.

**Write-up (the deliverable):** what SPI mode you used, what delta-sigma buys you over the SAR
ADC, and why you trust the number.

## Recap

- An **ADC** turns a voltage into a number; a **potentiometer** is a voltage divider you can
  read.
- **Rung 3** uses the Pico's **built-in 12-bit ADC** (`0–4095`) to prove the whole measure loop
  at near-zero risk → capability `["gpio","adc"]`.
- **Rung 4** swaps in the **ADS1220** (24-bit, SPI) as the *only* new variable — watch the
  **SPI mode**, share the SPI bus with a separate CS, and **verify against hand math.**

**Next:** [Lesson 7 — Rung 5, driving a motor with PWM](07-rung-5-pwm.md), where you finally
make something move (and learn power discipline).
</content>
