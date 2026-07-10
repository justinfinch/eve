# Hardware Lesson 4 — The Build Ladder + Rungs 0–1

## What you'll learn

Why the build is ordered the way it is (it's *not* the order the demo flows in), and how to
build the first two rungs: a blinking LED, and the browser-to-board loop that ties back to
the software you already wrote.

## The concept: learning order ≠ feature order

The *product* sequence is: self-ID → bridge → measure → AI. That demos well. But it's a bad
*learning* order, because it doesn't introduce hardware concepts safely or cumulatively.

So the build is re-ordered into a **ladder**, sorted by hardware-concept difficulty and
"blast radius" (how much damage a mistake can do). Same parts, same end product — just a
sequence designed to teach. This comes from
[Learning Hardware as a Software Dev](../../../.arche/concepts/learning-hardware-as-a-software-dev.md)
and the [Hardware Manual §4](../../../.arche/stories/poc-hardware-manual-for-self.md).

Three principles shape the ladder:

1. **Build the instrument before the experiments.** The browser↔device loop (rung 1) is a
   software dev's home turf — build it early so every later rung has a familiar, software-native
   readout to test against.
2. **Self-ID is the spine, not a rung.** The `{id, name, capabilities}` message is the
   through-line. Each rung *appends* a capability: `["gpio"]` → `["gpio","adc"]` →
   `["gpio","adc","pwm"]`. This is literally how an Adom
   [molecule](../../../.arche/concepts/molecules-and-workcells.md) behaves — and it's the exact
   `SelfId` you built in the software track.
3. **Bloody-on-CAN early.** CAN is the highest-gotcha *and* the Adom-signature tech, so it
   moves to rung 2 — "fail fast on the riskiest assumption." It's de-risked by LOOPBACK mode and
   the Golden Reference Node.

```
Rung 0  External LED        GPIO, current-limit resistor, 3.3V logic
Rung 1  Browser loop        self-ID over USB; capabilities: ["gpio"]   ← ties to the software track
Rung 2  2-node CAN bus      SPI + differential signaling   (the hard one)
Rung 3  Built-in ADC        analog → digital, ~zero risk
Rung 4  ADS1220 over SPI    precision ADC; only the SPI chip is new
Rung 5  PWM a motor         actuation + power discipline
Rung 6  AI layer            pure software, on top
```

> ⚙ **Pin numbers below are derived — verify on the bench.** They come from the Pico 2 pinout
> and the chosen modules, not the locked research. The research fixes only: external LED on
> **GP15**, safe range **GP10–GP21**, avoid **GP23/24/25/29** on the W.

---

## Rung 0 — Power the Pico, blink an *external* LED

**New concept:** GPIO, the current-limiting resistor, 3.3V logic, and using a multimeter as
your debugger. This is the safest possible first contact with the board.

**Wiring (⚙ derived):**

```
Pico GP15 ──[ 330Ω ]──►|── GND
                       LED
       (GP15 → resistor → LED anode; LED cathode → GND)
```

**What's happening in the silicon:** GP15 is a *push-pull output* — when your firmware drives
it high, it sources about 3.3V. The 330Ω resistor limits the current to roughly
(3.3 − 1.8) / 330 ≈ **4.5 mA**, so neither the LED nor the GPIO pad burns out. (1.8V is the
LED's forward voltage drop.) Remove the resistor and you'd push far too much current — Trap 1
from [Lesson 1](01-mental-model.md), current going where it shouldn't.

Why *external*? On the Pico 2 **W**, the onboard LED is wired behind the wireless chip, not a
plain GPIO — so an external LED on GP15 is both necessary and clearer.

**Verify:** *before* connecting the LED, measure GP15 with a multimeter as your firmware
toggles it — confirm it swings 0V ↔ 3.3V. Then measure across the resistor to confirm current
direction.

**Write-up (the deliverable):** why a current-limiting resistor exists, and what would
physically happen without it.

---

## Rung 1 — Browser↔device loop + self-ID spine `["gpio"]`

**New concept:** serialization, the host↔device boundary, and capability announcement. This is
your home turf — you're building the *instrument* you'll use to test every later rung.

**Wiring:** none beyond rung 0 — this uses the Pico's built-in USB. The browser talks to the
Pico directly via the **Web Serial API** (Chrome only). That browser-to-serial pattern is
exactly what Adom's founder [John Lauer](../../../.arche/entities/john-lauer.md) pioneered.

**What's happening:** on power-up, the firmware announces
`{id, name, capabilities: ["gpio"]}` over USB serial. The browser parses that message and
renders a control for the LED. **This is the same `SelfId` contract from the software track** —
except now a real board sends it instead of the simulator. Every later rung appends one more
capability to that list.

> **The connection to your software lessons:** in the software track, the *simulator* sent
> `SelfId` over a WebSocket and the browser displayed it. Here, the *real Pico* sends the same
> shape over Web Serial. The contract is the invariant; only the sender changed. That's the
> whole point of having defined the message once (see the
> [ADR — Phased control plane](../../../.arche/concepts/adr-phased-control-plane.md): the
> message contract is the invariant, not the transport).

**Verify:** toggle the LED from the browser; confirm the capability string round-trips
(browser sees `["gpio"]`).

**Write-up:** where the host↔device boundary sits, and how a capability list makes the board
"plug-and-play."

## Recap

- The ladder is ordered for **safe, cumulative learning**, not demo flow.
- **Self-ID is the spine** — each rung appends a capability to the same `SelfId` you already
  know.
- **Rung 0:** external LED on GP15 + a current-limit resistor — your first GPIO and your first
  "measure before connect."
- **Rung 1:** the browser↔Pico loop over Web Serial — the same contract as the simulator, now
  from real silicon.

**Next:** [Lesson 5 — Rung 2, the CAN bus](05-rung-2-can.md) — the hardest and most
instructive rung.
</content>
