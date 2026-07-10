# Hardware Lesson 5 — Rung 2: The CAN Bus (the big one)

## What you'll learn

The hardest and most valuable rung: building a two-node CAN bus. You'll meet SPI, differential
signaling, and the two gotchas that cause more wasted hours than anything else in the project —
plus how to de-risk them.

## Why this rung comes early

CAN is both the **highest-gotcha** technology in the build and the **Adom-signature bus**.
Following "fail fast on the riskiest assumption" ([Lesson 1](01-mental-model.md)), we hit it at
rung 2 while you're fresh and little else is built to confuse the diagnosis. If you can get
here and *explain* the two gotchas below, you can speak to CAN in an interview — which is the
whole point.

## The concepts

**SPI** (Serial Peripheral Interface) is how the Pico talks to the MCP2515 chip. It's a 4-wire
bus:

- **SCK** — clock (the Pico drives it)
- **MOSI / SI** — Master Out, Slave In (Pico → chip)
- **MISO / SO** — Master In, Slave Out (**chip → Pico**) ← remember this direction
- **CS** — chip select (Pico picks which chip it's talking to)

**CAN** is the actual network between the two nodes. It's a **differential** bus: two wires
(CANH and CANL) swing in *opposite* directions, so electrical noise that hits both cancels out.
That's why CAN is industrially robust — and why you *can't* see a meaningful logic level by
probing one wire alone. The MCP2515 is the CAN *controller* (you configure it over SPI); the
TJA1050 is the *transceiver* that turns the controller's signals into the differential pair.

## Gotcha (a) — the crystal frequency

Cheap MCP2515 modules ship with either an **8 MHz or a 16 MHz** crystal, varying by batch. The
bit-timing configuration in your firmware **must** match the actual crystal, or the two nodes
never synchronize. You already read and labeled the crystal cans in [Lesson 3](03-day-one.md) —
now set the firmware constant to match the label.

> This is the **#1 misattribution trap** in the whole project. A crystal mismatch reads as a
> "CAN bug" and can eat an entire afternoon. The 30-second labeling step in the arrival ritual
> exists precisely to defuse it.

## Gotcha (b) — the 5V / 3.3V MISO seam

The TJA1050 transceiver wants **5V**; the Pico is a **3.3V** part. On the 4-wire SPI bus, the
Pico drives SCK, MOSI, and CS — those stay at a safe 3.3V. But **MISO is driven by the
module** (it's the chip → Pico direction). So a module powered at 5V can push 5V back into a
3.3V Pico pin and damage it over time.

Mitigations, simplest first:

1. **Run the whole module at 3.3V.** Works on a short bench bus; the TJA1050 is marginal at
   3.3V but usually fine for a POC. Zero extra parts.
2. **Put the BSS138 level shifter on MISO** (or the whole SPI bus). The robust choice — and the
   shifter is a reusable bench tool. (Recall from [Lesson 2](02-know-your-parts.md): it's
   passive, so keep the SPI clock conservative, ~1–4 MHz.)
3. **Board-mod:** power only the TJA1050 at 5V and the MCP2515 logic at 3.3V. Most fiddly.

## Wiring

**SPI, Pico → its MCP2515 module (⚙ derived — verify on the bench):**

```
Pico            MCP2515 module
GP18 (SCK)  ───  SCK
GP19 (MOSI) ───  SI
GP16 (MISO) ───  SO    ◄── route through BSS138 if the module is 5V-powered
GP17 (CS)   ───  CS
GP21 (INT)  ───  INT
3V3 / 5V    ───  VCC   (see the MISO seam note above)
GND         ───  GND
```

**The CAN bus between the two nodes (differential pair + termination):**

```
 Node A (hand-built)              Node B (Waveshare reference)
 MCP2515 + TJA1050                RP2350-CAN
   CANH ───────────────────────────── CANH
   CANL ───────────────────────────── CANL
   GND  ───────────────────────────── GND
        [120Ω]                 [120Ω]   ← termination resistor at BOTH ends
```

**What's happening in the silicon:** the two 120Ω terminators at each end of the bus stop
signal reflections (an unterminated bus "echoes" and corrupts data). The differential pair
carries the data as the *voltage difference* between CANH and CANL. The MCP2515's config
registers live *inside the chip* and survive a re-flash of your firmware — Trap 3 from
[Lesson 1](01-mental-model.md), so if a setting seems "stuck," remember the state isn't in your
code.

## De-risking before two nodes ever talk

Don't try to make two nodes communicate as your first test. Climb up in safe steps:

1. **LOOPBACK mode first.** Configure your one MCP2515 in LOOPBACK — it routes transmit → receive
   *inside the chip*, with no bus and no second node. If a frame comes back, you've proven the
   whole node: chip + SPI wiring + firmware + crystal. This isolates everything *except* the bus.
2. **Then bring up the real bus.** Connect the two nodes as wired above.
3. **If it won't sync, bisect with the Golden Reference Node.** Swap your hand-built node for the
   known-good Waveshare board. If the bus now works, the fault was in your hand-wiring or
   firmware; if it still fails, look at the bus itself. This is the single most powerful
   debugging move on this rung.
4. **Put the logic analyzer on the SPI lines.** Confirm CS actually asserts (drops low) and that
   bytes are moving. CAN faults are invisible without this — you cannot debug what you cannot
   see.

## Verify

In order: LOOPBACK passes → a config register reads back what you wrote → the two nodes
acknowledge each other → a frame sent from node A appears on node B.

**Write-up (the deliverable):** what differential signaling buys you, why termination matters,
and the crystal + MISO seam *in your own words*. If you can write that paragraph honestly,
you've earned the literacy this rung exists to teach.

## Recap

- **SPI** (4 wires; MISO is chip → Pico) controls the **MCP2515**; the **TJA1050** drives the
  **differential** CAN pair; 120Ω terminators at both ends.
- **Gotcha (a):** match firmware bit-timing to the actual **crystal** (8 vs 16 MHz) — the #1
  misattribution trap.
- **Gotcha (b):** the **5V/3.3V MISO seam** — protect the Pico pin (run at 3.3V, or use the
  level shifter).
- **De-risk:** LOOPBACK → real bus → bisect with the **Golden Reference Node** → **logic
  analyzer** on SPI.

**Next:** [Lesson 6 — Rungs 3–4, reading analog](06-rungs-3-4-adc.md), a calmer pair of rungs
after the storm.
</content>
