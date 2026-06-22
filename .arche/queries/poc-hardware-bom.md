---
type: query
title: "What Hardware Do I Need to Buy (POC BOM)"
created: 2026-06-17
updated: 2026-06-22
tags: [poc, bom, hardware, parts-list, interview-prep, mini-molecule]
sources: [concepts/poc-mini-molecule-cloud-workbench.md, concepts/adom-technical-architecture.md, concepts/programmable-wiring.md, sources/adom-decoded-and-poc-plan.md, discoveries/poc-unknown-unknowns.md, concepts/learning-hardware-as-a-software-dev.md]
---

# What Hardware Do I Need to Buy (POC BOM)

> what hardware do I need to buy

## Answer

Hardware is scoped entirely around the recommended **"Mini-Molecule + Cloud Workbench"** POC — build **Concept A** as the spine, add **Concept B** only if time allows — per [POC: Mini-Molecule + Cloud Workbench](../concepts/poc-mini-molecule-cloud-workbench.md) citing [Adom, Decoded](../sources/adom-decoded-and-poc-plan.md). Part numbers below mirror the Adom chip families named in [Adom Technical Architecture](../concepts/adom-technical-architecture.md) (RP2040/RP2350, MCP2515/MCP2518FD, ADS1220/ADS123x) and were cross-checked against current vendor availability (June 2026, web — see external sources at bottom).

### Concept A — core build

| Part | Qty | ~Price | Notes / search term |
|---|---|---|---|
| **Raspberry Pi Pico 2** (RP2350) | 1–2 | $5–6 ea | "Raspberry Pi Pico 2 RP2350"; buy pre-soldered headers. Maxes stack signal via Rust + Embassy firmware ([POC](../concepts/poc-mini-molecule-cloud-workbench.md)). |
| **MCP2515 CAN module** (w/ transceiver) | 2 | $7–10/pair | "MCP2515 CAN module"; need two for a real bus. v2 only — v1 runs over USB serial. Mirrors Adom's CAN-FD bus ([Adom Technical Architecture](../concepts/adom-technical-architecture.md)). |
| **ADS1220 24-bit ADC breakout** | 1 | $10–15 | "ADS1220 ADC module"; mirrors Adom's `ads1220` precision ADC ([Adom Technical Architecture](../concepts/adom-technical-architecture.md)). Cheaper alt: ADS1115 (~$7). |
| **USB cable** (→ micro-USB, *data*) | 1 | $5 | Pico 2 is micro-USB; confirm data-capable. |
| **Breadboard** (830-point) | 1–2 | $6/pair | "830 breadboard" |
| **Jumper wires** (M-M/M-F/F-F kit) | 1 kit | $7 | "dupont jumper wire kit" |
| **Component-under-test grab bag** (resistors, pot, LEDs, thermistor, small DC fan/motor for PWM) | 1 | $10–13 | "electronics component starter kit"; gives the ADC something to read and PWM to drive. |
| **Logic level shifter** (4-channel BSS138 board) | 1 | $2 | "BSS138 4 channel level shifter"; optional but recommended. Translates the 5V MCP2515 module ↔ 3.3V Pico on the SPI bus (esp. MISO). Cheap insurance + a reusable bench tool — see [build note](#build-note--mcp2515-module-on-a-33v-pico) below. |

**Core subtotal: ~$40–55** on Amazon Prime — above the Arche's ~$20–30 estimate ([POC](../concepts/poc-mini-molecule-cloud-workbench.md)), which assumed salvage / AliExpress pricing.

**Shortcut:** the **Waveshare RP2350-CAN board** (~$10–18) combines an RP2350 + MCP2515-clone + CAN transceiver on one Pico-sized board — collapses the Pico + CAN module into one part per node; buy two.

### Selected parts (Amazon, vetted 2026-06-22)

Specific SKUs chosen for the **separate-parts** build (Pico + standalone modules, not the all-in-one Waveshare). Links are ASINs; prices drift, so confirm at checkout.

| BOM line | Picked | Amazon |
|---|---|---|
| Raspberry Pi Pico 2 (`:22`) | **Pico 2 W** (RP2350 + WiFi/BT), pre-soldered headers — W radio unused but harmless | [B0DP54FWX1](https://www.amazon.com/dp/B0DP54FWX1) |
| MCP2515 CAN module ×2 (`:23`) | HiLetgo 2pcs MCP2515 + **TJA1050** transceiver | [B01D0WSEWU](https://www.amazon.com/dp/B01D0WSEWU) |
| Logic level shifter (`:29`) | Coliao 5pcs 4-ch bidirectional 3.3↔5V (BSS138-type; "I2C" label, but fine for SPI) | [B0DLG9J81H](https://www.amazon.com/dp/B0DLG9J81H) |
| ADS1220 24-bit ADC (`:24`) | ADS1220 24-bit SPI module (listing mislabels "I2C" — chip is **SPI-only**) | [B0DPMKMGNN](https://www.amazon.com/dp/B0DPMKMGNN) |
| Breadboard (`:26`) | BOJACK breadboard + jumper kit (multiple 830/400/170-pt boards) | [B08Y59P6D1](https://www.amazon.com/dp/B08Y59P6D1) |
| Component grab bag (`:28`) | BOJACK 37-values 480pcs — incl. **potentiometer, NTC thermistor**, 830 breadboard, power module, transistors, diodes | [B099MQV8ZW](https://www.amazon.com/dp/B099MQV8ZW) |
| Dupont jumpers M-M/M-F/F-F (`:27`) | REXQualis 240pcs (10cm + 20cm) | [B0F8VDMHRT](https://www.amazon.com/dp/B0F8VDMHRT) |
| DC fan/motor for PWM (`:28`) | Sntieecr 6× mini DC motors, 3–12V, w/ fan blades | [B0922N8MCR](https://www.amazon.com/dp/B0922N8MCR) |

**Still to source (no Amazon pick yet):**

- **USB cable** — micro-USB, *data-capable* (`:25`); Pico 2 W is micro-USB.

**Notes from vetting:**

- **Breadboard/jumper overlap.** The 37-values kit *also* bundles an 830 breadboard + wires, overlapping the BOJACK breadboard kit. That's fine — a **2-node CAN bus wants ≥2 boards** (one per node). But if buying minimal: the 37-values kit alone covers breadboard + components, so the standalone breadboard kit is optional; keep the **REXQualis** dupont regardless, since the others may ship solid-core (M-M only), and you need **M-F/F-F** to reach modules.
- **Motor driver.** Don't drive the DC motor off a GPIO. Use a transistor (2N2222) + **flyback diode** (1N4007) + 0.1µF cap — *all in the 37-values kit* — and power the motor from the **5V rail / power module**, not the Pico's 3V3. See pin map in [the explainer](../assets/stories/poc-explainer-for-self.html).
- **Thermistor.** Confirmed present in the 37-values kit. Read it as a divider with a 10k fixed resistor, or use the **ADS1220's onboard IDAC current sources** for a ratiometric reading (a reason it beats the cheaper ADS1115).
- **Level shifter is passive (BSS138).** Despite the "I2C" label it works for SPI, but passive MOSFET shifters slow push-pull rising edges. Run the **MCP2515 SPI clock conservatively (~1–4 MHz)** in firmware to avoid bit errors — a POC needs no more. (For guaranteed high-speed SPI you'd use a direction-controlled shifter like TXB0104; not needed here.)
- **Pico 2 W quirks.** The "W" radio is unused by the POC (harmless). Two consequences vs a plain Pico: the **onboard LED is driven through the CYW43 wireless chip** (not a direct GPIO) — irrelevant here since the build uses an *external* LED on GP15; and **GP23/24/25/29 are reserved** for the wireless module — the wiring map (GP10–GP21) avoids them, so no conflict. Confirm on arrival it's a genuine RP2350 board, not a mislabeled RP2040.
- **Carry-over cautions:** match the MCP2515 **crystal frequency** in firmware, and keep module **MISO off a bare 3.3V GPIO** — see [build note](#build-note--mcp2515-module-on-a-33v-pico).

**Revised total (all of the above + Pico/shifter/USB): ~$75–95** — higher than the $40–55 core estimate because the kits bundle spares and extra boards you'll reuse, not because the per-line parts cost more.

### Concept B — programmable patch matrix (optional stretch)

The raw **CH446Q** (~$1, the Jumperless chip per [Programmable Wiring](../concepts/programmable-wiring.md) citing [Adom, Decoded](../sources/adom-decoded-and-poc-plan.md)) is an LQFP-44 SMD part — cheap but not breadboard-friendly without a breakout. Two paths:

- **Easiest:** buy a **Jumperless V5 breadboard** (built from CH446Q arrays) — Concept B in a box, ~$300.
- **Cheap but fiddly:** CH446Q chip + LQFP-44→DIP adapter, hand-soldered, ~$5.

**Recommendation:** build A first, leave B as a README stretch goal unless the wiring demo is specifically wanted — matching the Arche's own A-spine-plus-optional-B call ([POC](../concepts/poc-mini-molecule-cloud-workbench.md)).

### Gaps

The Arche specifies *what* to buy and an approximate total but no exact SKUs, vendor links, or quantities for the supporting parts (ADC, component-under-test, wires, breadboard, cables); those were filled from web vendor data, not the Arche, and prices/availability will drift. The pragmatic **v1 (browser → Web Serial → Pico, no CAN, no bridge)** needs only the Pico + ADC + supporting parts — the CAN modules and crosspoint are deferrable ([POC](../concepts/poc-mini-molecule-cloud-workbench.md) citing [Adom, Decoded](../sources/adom-decoded-and-poc-plan.md)).

## Tooling for *learning* (not in the original BOM)

Added from the [POC Unknown-Unknowns discovery](../discoveries/poc-unknown-unknowns.md): if the goal is hardware *literacy*, not just a working rig (see [Learning Hardware as a Software Dev](../concepts/learning-hardware-as-a-software-dev.md)), the most important "parts" are the ones that make the invisible physical layer **observable** — none of which were in the parts list. Time-sensitive: source these while the order is open.

| Item | ~Price | Why it matters for learning |
|---|---|---|
| **USB logic analyzer** (8-ch clone + PulseView) | ~$10 | "Your debugger is a multimeter" — but a logic analyzer lets you *watch* SPI/CAN bits on the wire. Non-negotiable once CAN moves early (its failures are invisible: differential pair, ACK bits, bit-timing). Being able to say "I scoped the bus and saw CS wasn't asserting" *is* the hardware-fluency signal. |
| **2nd Waveshare RP2350-CAN** (as a *reference* node) | ~$10–18 | **The Golden Reference Node.** Buy one known-good all-in-one *and* build the other CAN node from separate parts. When the bus won't sync, swap it in to bisect "my hand-wired node + firmware" vs "the bus itself." (Same part the BOM lists as the "shortcut" — here it's a diagnostic oracle, not an escape hatch.) |
| **USB power meter** (inline V/A display) | ~$10 | Current draw and heat are *pre-code* vital signs: near-zero = nothing running; too-high + hot = backwards/shorted chip → cut power before it dies. The "smoke" in smoke test, quantified. |
| **ESD wrist strap** | ~$5 | Software devs have zero ESD reflexes; static causes *latent* damage (works today, flaky next week) you'll blame on firmware. Nearly-free insurance against the worst failure mode — invisible, delayed, self-misattributing. |

### Selected learning-tool picks (Amazon, vetted 2026-06-22)

| Tool | Picked | ~Price | Amazon |
|---|---|---|---|
| USB logic analyzer | **HiLetgo 24 MHz 8-ch** (Saleae-compatible; works with PulseView/sigrok) | ~$10 | [B077LSG5P2](https://www.amazon.com/HiLetgo-Analyzer-Ferrite-Channel-Arduino/dp/B077LSG5P2) |
| Golden Reference Node | **Waveshare RP2350-CAN** (XL2515 = MCP2515 clone + SIT65HVD230 3.3V transceiver; USB-C, selectable 120Ω term) | ~$18 | [B0F4JH65HY](https://www.amazon.com/Development-Dual-architecture-Microcontroller-SIT65HVD230-Transceiver/dp/B0F4JH65HY) |
| USB power meter | **MakerHawk AT34** IPS color, 3.7–30V / 0–4A inline | ~$14 | [B07FMQZVW2](https://www.amazon.com/MakerHawk-3-7-30V-Voltage-Multimeter-Voltmeter/dp/B07FMQZVW2) |
| ESD wrist strap | **iFixit** adjustable, 1 MΩ coiled cord + alligator clip | ~$8 | [B00B2T9C8Y](https://www.amazon.com/iFixit-Anti-static-Wrist-Strap-Adjustable/dp/B00B2T9C8Y) |

**Picks subtotal: ~$50.** Prices drift; confirm at checkout (ASINs are stable, listings are not). Notes from vetting:

- **Logic analyzer.** The HiLetgo/Comidox/generic "24MHz 8CH" boards are the same Cypress FX2 design — any works with **PulseView (sigrok)**, the free open-source capture UI. 0–5.5V inputs, 1.5V threshold → reads 3.3V logic fine. A cheaper no-brand clone (~$7, e.g. [B07KW445DJ](https://www.amazon.com/Comidox-Analyzer-Device-Channel-Arduino/dp/B07KW445DJ)) is functionally identical if you want to save a few dollars.
- **Reference node.** This is the *same* Waveshare RP2350-CAN the core BOM lists as the all-in-one "shortcut" — here you buy **one** as a known-good oracle and build the *other* node from the separate Pico + HiLetgo MCP2515 module, so you can bisect bus faults. The 3.3V-native SIT65HVD230 transceiver + pre-matched crystal are exactly the gotchas that make it trustworthy as a reference. ⚠️ It's **USB-C** (the Pico 2 W is micro-USB) — you'll want a USB-C data cable for it.
- **USB power meter.** Measures *total board current drawn from USB* — perfect for the gross "is a chip shorted/backwards?" vital sign (#23), but it can't isolate per-module current. That's fine for this POC; a bench supply with a current knob would be the next step up, and isn't needed here.
- **ESD strap.** 1 MΩ inline resistor is the safety standard (limits discharge current through you). Clip the alligator to a grounded metal surface — a screw on a plugged-in metal-case PSU/chassis, or a proper ground point.

## Arrival → bring-up ritual

From the [POC Unknown-Unknowns discovery](../discoveries/poc-unknown-unknowns.md). There's **no `package-lock.json` for atoms** — no hash guarantees you got the right bytes. Treat the box landing as "ready to *audit*," not "ready to build." Adopt manufacturing's receiving-inspection discipline:

**1. Audit before assembly.**
- **Decode the chip top-markings** with a loupe/phone-macro and cross-check the datasheet — the *listing lies, the silkscreen half-lies, the chip tells the truth*. Confirm the ADS1220 is an ADS1220, the Pico is a genuine **RP2350 (not a mislabeled RP2040)**, the transceiver is a TJA1050.
- **Read both MCP2515 crystal cans** (8.000 vs 16.000 MHz), label each with tape. This 30-second step pre-empts the entire "CAN bug" misattribution spiral — discover a mismatch *now*, on a calm afternoon, not at hour three.
- Visual check: bent pins, solder bridges, missing SMD parts, cold joints on pre-soldered headers.

**2. Smoke-test each part in isolation** (unit test before integration):
- **Power it alone**, watch the USB power meter and *feel/smell* for heat → confirm a vital sign before integrating.
- **Verify against math, not against an instrument you also don't trust.** ADS1220: short AIN+ to AIN− → read ≈0; feed a known divider → the code must equal `(Vin/Vref) × 2²³` computed by hand.
- **Use the chips' own self-test modes.** The MCP2515 **LOOPBACK mode** routes its TX→RX internally — prove the CAN node (chip + SPI + firmware + crystal) on *one* node, no bus, no second node — plus write-a-config-register-then-read-it-back. The datasheet's "modes" section is a bring-up toolbox.
- **Passive parts test with a meter only.** The BSS138 level shifter needs no code: apply 3.3V one side, measure the other.

**3. Integrate one variable at a time** — so any failure is unambiguously the *one* thing you just added.

## Build note — MCP2515 module on a 3.3V Pico

Specific to the separate-parts build (Pico 2 + standalone HiLetgo-style MCP2515 module, kept separate from the all-in-one Waveshare board for learning value). Two things to check before/at assembly:

**1. Crystal frequency.** Cheap MCP2515 modules ship with either an **8 MHz or 16 MHz** crystal, and it varies by batch. The MCP2515 bit-timing config in firmware *must* match the actual crystal or the bus won't sync. Read the label on the silver crystal can when the board arrives and set the firmware constant accordingly.

**2. 5V vs 3.3V seam.** The onboard **TJA1050 transceiver wants 5V**, but the Pico 2 (RP2350) is a **3.3V** part. SPI is a 4-wire bus — SCK, MOSI, CS, MISO. The Pico drives SCK/MOSI/CS, so those stay at a safe 3.3V; but **MISO is driven by the module**, so a 5V-powered module can push 5V back into a Pico GPIO and damage it over time. Mitigations, simplest first:
   - **Run the whole module at 3.3V.** Works on a short bench bus; the TJA1050 is marginal at 3.3V but usually fine for a POC. Zero extra parts.
   - **Level shifter on MISO** (or the full SPI bus). A 4-channel BSS138 board (~$2, added to the BOM above) translates 5V ↔ 3.3V both ways. The robust choice; the level shifter is also a reusable bench tool.
   - **Board mod:** power the MCP2515 logic at 3.3V and feed only the TJA1050 5V — isolates the Pico from 5V logic. Most fiddly.

The all-in-one **Waveshare RP2350-CAN** board avoids both gotchas (3.3V-native, crystal pre-matched) — the trade-off being less to learn from the wiring.

## See also

- [POC: Mini-Molecule + Cloud Workbench](../concepts/poc-mini-molecule-cloud-workbench.md)
- [Learning Hardware as a Software Dev](../concepts/learning-hardware-as-a-software-dev.md)
- [POC Unknown-Unknowns discovery](../discoveries/poc-unknown-unknowns.md)
- [Adom Technical Architecture](../concepts/adom-technical-architecture.md)
- [Programmable Wiring](../concepts/programmable-wiring.md)
- [Adom, Decoded — Problem Space + a POC of Their Actual App](../sources/adom-decoded-and-poc-plan.md)
- External (web, June 2026): [Waveshare RP2350-CAN (CNX Software)](https://www.cnx-software.com/2025/04/21/raspberry-pi-pico-sized-rp2350-can-development-board-features-a-clone-of-the-mcp2515-can-bus-controller/), [Raspberry Pi Pico 2](https://www.raspberrypi.com/products/raspberry-pi-pico-2/), [CH446Q on LCSC](https://www.lcsc.com/product-detail/Analog-Switches-Multiplexers_WCH-Jiangsu-Qin-Heng-CH446Q_C109471.html), [Jumperless (DigiKey)](https://www.digikey.com/en/maker/projects/jumperless/2e62cc88ee6145bc924041dd486de76f)
