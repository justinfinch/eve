# Hardware Lesson 2 — Know Your Parts

## What you'll learn

Every part you ordered: what it is, what job it does in the molecule, what real Adom chip
family it mirrors, and the one gotcha to remember for each. This is your reference card —
come back to it as parts arrive.

## The concept

A bill of materials (BOM) is just a parts list. But each part here was chosen deliberately to
mirror a slice of Adom's production stack, so understanding *why* each is in the box is part
of the literacy goal. The full, price-and-SKU version lives in the
[POC Hardware BOM](../../../.arche/queries/poc-hardware-bom.md); this lesson is the
teaching tour.

One habit to adopt now, before anything arrives: **the listing lies, the silkscreen
half-lies, the chip tells the truth.** Cheap modules are routinely mislabeled. So for each
part below, there's a "verify" note — the marking to check under a loupe or phone-macro
camera when it arrives. [Lesson 3](03-day-one.md) turns this into a ritual.

## The build parts

### Raspberry Pi Pico 2 W (RP2350) — the brain

The microcontroller that runs your Rust + Embassy firmware — the molecule's brain. It's the
same chip family ([RP2350](../../../.arche/concepts/adom-technical-architecture.md)) Adom
uses.

- **Job:** runs firmware; drives GPIO, reads sensors, talks SPI to the other chips.
- **Voltage:** **3.3V logic.** This matters constantly — most of your gotchas are about other
  parts wanting 5V.
- **Verify:** that it's a genuine **RP2350**, not a relabeled older RP2040.
- **Gotchas (the "W" model):** the WiFi radio is unused (harmless), but (a) the onboard LED is
  wired *through* the wireless chip, not a normal GPIO — so you'll use an **external** LED; and
  (b) pins **GP23/24/25/29 are reserved** for the radio. The wiring in these lessons stays in
  GP10–GP21 to avoid them.

### MCP2515 CAN module ×2 (with TJA1050 transceiver) — the bus

A pair of modules that together form a **CAN bus** — the rugged, two-wire network used in cars
and industrial gear, and Adom's signature bus. You need two to have a real bus (one talker,
one listener).

- **Job:** the MCP2515 is a CAN *controller* you talk to over SPI; the TJA1050 is the
  *transceiver* that turns its signals into the physical differential pair on the wire.
- **Voltage:** the **TJA1050 wants 5V**; the MCP2515 logic is happy at 3.3V. This split is the
  source of the famous "MISO seam" — [Lesson 5](05-rung-2-can.md).
- **Verify:** that the transceiver is a **TJA1050**, and — critically — **read both crystal
  cans and note whether each is 8.000 or 16.000 MHz, then label them with tape.** Cheap
  modules ship either, by batch.
- **Gotcha:** if your firmware's bit-timing assumes the wrong crystal frequency, the bus never
  syncs and it looks exactly like a software bug. Labeling the crystals now pre-empts hours of
  misdiagnosis.

### ADS1220 24-bit ADC — precision measurement

A high-precision analog-to-digital converter, mirroring Adom's `ads1220`. An ADC turns a real
voltage (from a sensor) into a number your code can read.

- **Job:** read a sensor (a potentiometer knob, a thermistor) far more precisely than the
  Pico's built-in ADC.
- **Verify:** that it talks **SPI** — listings frequently mislabel it "I2C," but **the chip is
  SPI-only.**
- **Gotcha (useful one):** it has onboard *IDAC current sources*, which let you read a
  thermistor "ratiometrically" — a precision trick the cheaper ADS1115 can't do, and a reason
  this chip was chosen.

### BSS138 4-channel level shifter — the voltage translator

A tiny board that safely translates signals between **5V and 3.3V** in both directions.

- **Job:** sit on the SPI bus (especially the MISO line) so a 5V-powered CAN module can't push
  5V into a 3.3V Pico pin and slowly damage it.
- **Verify:** nothing to decode — it's passive. (It may be labeled "I2C," but it works fine for
  SPI.)
- **Gotcha:** because it's a *passive* (MOSFET) shifter, it slightly slows fast signal edges —
  so you'll run the CAN module's SPI clock conservatively (~1–4 MHz). Plenty fast for a POC.

### 37-values component grab bag (480 pcs) — the stuff under test

The assortment that gives the molecule something to measure and drive.

- **Contains (the ones you'll use):** resistors, a **potentiometer** (a knob = adjustable
  voltage), an **NTC thermistor** (temperature sensor), **2N2222 transistors**, **1N4007
  diodes**, capacitors — plus a spare breadboard and a power module.
- **Job:** the potentiometer and thermistor are sensors for the ADC rungs; the transistor +
  diode build the motor driver in [Lesson 7](07-rung-5-pwm.md).

### DC motors ×6 (3–12V) — the actuator

Small motors with fan blades, your PWM target.

- **Job:** something to spin at variable speed, to demonstrate actuation.
- **Gotcha (important):** **never drive a motor directly from a GPIO pin.** It draws too much
  current and its back-EMF will kill the pin. It goes through a transistor, with a flyback
  diode, powered from a separate 5V rail — [Lesson 7](07-rung-5-pwm.md).

### Breadboards + dupont jumpers + USB cable

- **Breadboards:** a 2-node CAN bus wants **≥2 boards** (one per node).
- **Jumpers:** keep the male-female and female-female ones — you need them to reach the module
  pins.
- **USB cable:** the Pico 2 W is **micro-USB** and you need a **data-capable** cable (some are
  charge-only). The Waveshare reference node below is **USB-C** — you'll want both.

## The learning tools (the most important "parts")

These were **not** in the original parts list. They were added because the goal is *literacy*,
and the single best investment for literacy is making the invisible physical layer
**observable** (see [POC Unknown-Unknowns, Theme 4](../../../.arche/discoveries/poc-unknown-unknowns.md)).
Source them while your order is still open.

| Tool | What it does | Why it matters |
|------|--------------|----------------|
| **USB logic analyzer** (8-ch, 24 MHz) | Watches the actual 1s and 0s on the SPI/CAN wires, shown in **PulseView** (free) | "Your debugger is a multimeter" — this lets you *see* whether bytes are actually moving. Non-negotiable once CAN starts; CAN failures are invisible without it. |
| **Waveshare RP2350-CAN** (the *Golden Reference Node*) | A known-good, all-in-one CAN node | When your hand-built bus won't sync, swap this in to answer "is it my wiring, or the bus itself?" It's 3.3V-native with a pre-matched crystal — the very gotchas that make it trustworthy. **USB-C.** |
| **USB power meter** (inline V/A) | Shows current draw and voltage at the USB port | Current + heat are *pre-code* vital signs: near-zero = nothing's running; high + hot = a chip is backwards or shorted → cut power before it dies. The "smoke" in smoke-test, quantified. |
| **ESD wrist strap** (1 MΩ) | Grounds you so static doesn't zap chips | Software devs have zero static reflexes; ESD causes *latent* damage (works today, flaky next week) that you'll blame on firmware. Nearly-free insurance against the nastiest failure mode. |

## Recap

- **Pico 2 W** = the 3.3V brain (use an external LED; avoid GP23/24/25/29).
- **MCP2515 ×2** = the CAN bus (label the crystals! mind the 5V/3.3V seam).
- **ADS1220** = precision SPI ADC (it's SPI, not I2C).
- **BSS138** = the 5V↔3.3V translator (run SPI slow-ish).
- **Grab bag + motors** = things to measure and drive (never motor-off-a-GPIO).
- **Logic analyzer, reference node, power meter, ESD strap** = the literacy tools that make the
  invisible visible.

**Next:** [Lesson 3 — Day one: the arrival & bring-up ritual](03-day-one.md), what to do the
moment the box lands.
</content>
