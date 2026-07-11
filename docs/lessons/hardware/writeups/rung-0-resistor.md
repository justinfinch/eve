# Rung 0 write-up — why the current-limiting resistor exists

**Wiring:** GP15 → 330Ω → LED anode; LED cathode → GND.

**What the resistor does.** GP15 is a push-pull output: driven high it sources ~3.3V. An LED
is not a resistor — past its forward voltage it behaves like a near-short, so the current in the
loop is set by whatever *else* is in the loop. With a 330Ω resistor and an LED forward drop of
~1.8V, the current is (3.3 − 1.8) / 330 ≈ **4.5 mA** — safe for both the LED and the GPIO pad.

**Without it.** Nothing limits the loop current; the LED tries to pull far more than the pin's
~12 mA design limit. Best case the LED burns out; worse case the GPIO pad (or the whole chip) is
damaged. This is Trap 1 — current going where it shouldn't.

**How I verified it (multimeter as debugger).** Before connecting the LED, I measured GP15 as the
firmware toggled it and confirmed it swings 0V ↔ 3.3V. Then I measured across the resistor to
confirm current direction and magnitude.

<!-- Refine into your own words after re-measuring on the bench:
     - What did the multimeter actually read across the resistor?
     - Compute the measured current from V/R and compare to the 4.5 mA estimate. -->
