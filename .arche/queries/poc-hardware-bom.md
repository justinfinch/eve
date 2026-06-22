---
type: query
title: "What Hardware Do I Need to Buy (POC BOM)"
created: 2026-06-17
updated: 2026-06-22
tags: [poc, bom, hardware, parts-list, interview-prep, mini-molecule]
sources: [concepts/poc-mini-molecule-cloud-workbench.md, concepts/adom-technical-architecture.md, concepts/programmable-wiring.md, sources/adom-decoded-and-poc-plan.md]
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
- [Adom Technical Architecture](../concepts/adom-technical-architecture.md)
- [Programmable Wiring](../concepts/programmable-wiring.md)
- [Adom, Decoded — Problem Space + a POC of Their Actual App](../sources/adom-decoded-and-poc-plan.md)
- External (web, June 2026): [Waveshare RP2350-CAN (CNX Software)](https://www.cnx-software.com/2025/04/21/raspberry-pi-pico-sized-rp2350-can-development-board-features-a-clone-of-the-mcp2515-can-bus-controller/), [Raspberry Pi Pico 2](https://www.raspberrypi.com/products/raspberry-pi-pico-2/), [CH446Q on LCSC](https://www.lcsc.com/product-detail/Analog-Switches-Multiplexers_WCH-Jiangsu-Qin-Heng-CH446Q_C109471.html), [Jumperless (DigiKey)](https://www.digikey.com/en/maker/projects/jumperless/2e62cc88ee6145bc924041dd486de76f)
