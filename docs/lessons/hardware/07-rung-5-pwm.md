# Hardware Lesson 7 — Rung 5: Driving a Motor with PWM

## What you'll learn

How to make something physically move at a speed you control — and the power-handling
discipline that keeps you from frying a pin or browning out your board. This rung is where
"current going where it shouldn't" stops being theory.

## The concepts

**PWM** (Pulse-Width Modulation) is how you get "analog-like" control from a digital pin. The
pin switches fully on and fully off very fast; the fraction of time it's on — the **duty
cycle** — sets the *average* power delivered. 25% duty ≈ a quarter of full power → a slow motor;
75% → fast. The motor's inertia smooths the pulses into steady spin.

**Why you can't just wire a motor to a GPIO:** two reasons, both fatal to the pin.

1. A motor draws far more **current** than a GPIO can safely source.
2. When a motor switches off, its coil dumps a high-voltage **back-EMF spike** (an inductive
   kick) back into the circuit — "current going where it shouldn't," exactly Trap 1 from
   [Lesson 1](01-mental-model.md), and permanent.

So the GPIO doesn't power the motor — it just *switches* a transistor that controls the motor's
current from a separate supply. A **flyback diode** absorbs the back-EMF spike.

## Wiring

⚙ Derived — verify on the bench. **Never drive the motor from the Pico's 3V3 pin.**

```
                          +5V rail  (power module — NOT the Pico's 3V3)
                             │
                         [ motor ]
                             │
                1N4007  ►|───┤   (flyback diode across the motor; cathode toward +5V)
                             │
 Pico GP13 ──[ 1kΩ ]── base  │ collector
                       2N2222 (NPN transistor)
                          emitter ── GND ── (shared/common ground with the Pico)
```

**What's happening in the silicon:**

- **The transistor (2N2222)** is an electronic switch. A small current into its *base* (from
  GP13 through the 1kΩ resistor) lets a much larger current flow from *collector* to *emitter* —
  through the motor. The GPIO controls a current it could never supply directly.
- **The 1kΩ base resistor** limits the small control current into the base.
- **The flyback diode (1N4007)** sits across the motor. In normal running it does nothing; the
  instant the transistor switches off, the motor's collapsing magnetic field tries to drive a
  voltage spike, and the diode gives that energy a safe loop to dissipate instead of punching
  through the transistor. Orientation matters — cathode (the banded end) toward +5V.
- **Separate 5V rail + common ground.** The motor is powered from the 5V power module, not the
  Pico, because a motor starting up can momentarily sag the voltage ("brown-out") and reset the
  Pico — Trap 3 (state in the power rail) from [Lesson 1](01-mental-model.md). But the two
  supplies **must share a ground**, or the transistor has no common reference to switch against.

## Verify

- **Before powering:** confirm the diode orientation with your multimeter's diode-test mode
  (it conducts one way only). A backwards flyback diode is useless and can short the rail.
- **Then:** sweep the duty cycle 0 → 100% from the browser and watch the motor speed track it.

**Write-up (the deliverable):** why the transistor and the flyback diode each exist, and what a
given duty cycle maps to *physically* (average voltage / power at the motor).

This rung extends the spine to `capabilities: ["gpio", "adc", "pwm"]`.

## Power discipline, summarized

This rung is really a lesson in three habits that don't exist in software:

1. **A control signal is not a power source** — switch big loads through a transistor.
2. **Inductive loads kick back** — always give them a flyback path.
3. **Power is shared state** — isolate noisy loads (motors) onto their own rail, but tie grounds
   together.

## Recap

- **PWM** = fast on/off switching; **duty cycle** sets average power → motor speed.
- **Never** drive a motor from a GPIO — use a **2N2222 transistor** switched by the pin, motor
  powered from a **separate 5V rail** with a **common ground**.
- The **1N4007 flyback diode** clamps the back-EMF spike that would otherwise destroy the
  transistor — check its orientation *before* powering.

**Next:** [Lesson 8 — Rung 6 + the diagnosis playbook](08-rung-6-and-diagnosis.md), the
software cap and how to debug everything physical-first.
</content>
