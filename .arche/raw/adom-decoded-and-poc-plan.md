# Adom, Decoded — Problem Space + a POC of Their Actual App

*Prepared for Justin Finch · June 2026. Goal: understand what Adom is really building well enough to show up with a working slice of it.*

---

## TL;DR

Adom is building **"the AWS of electronics prototyping"** — a cloud-connected, robot-run factory where you design, wire, and test real electronics from a browser, by the hour, instead of owning a lab or flying to Shenzhen. Their core primitive is the **"molecule"**: a modular PCB that snaps into a factory **"workcell,"** where robot pincers physically wire molecules together and bench instruments measure them — all driven remotely, with AI agents on top.

Their public GitHub org all but confirms the architecture: **Rust + Embassy embedded firmware, CAN-FD as the module bus, Klipper-style multi-axis motion for the robotics, precision ADCs for measurement, and a Rust→TypeScript→browser control plane.**

The single best POC you can build is a **desk-scale "molecule + cloud workbench"**: a modular sensor/actuator board that self-identifies on a bus and is driven from a browser over a serial/CAN bridge, with an AI layer that runs a test and explains the result. **And the killer move: build it on top of Adom's own open-source Rust crates** (their `mcp2515`, `postcard-rpc`, `tsify` repos are public). Walking in having used their actual libraries to build a mini version of their actual product is about as strong a signal as exists.

---

## Part 1 — What Adom is actually building (high confidence, sourced)

This is no longer guesswork. In an Aug 2025 interview with Dallas Innovates, founder John Lauer laid it out, and the company's public GitHub fills in the engineering.

**The pitch, in Lauer's words:**
- *"Think of it as a data center, but for atoms, not bits."* / *"compute for atoms."* / **"the Amazon Web Services of electronics prototyping."**
- The problem he's attacking: *"What takes minutes in software can take months in hardware."*
- The model: engineers anywhere design, prototype, and test hardware *"from the comfort of their laptops"* in a *"programmable AI cloud lab,"* while *"connected robots produce a prototype in near real time and ship it overnight."*
- The business model is utility pricing: rent millions of dollars of test equipment for *"a few dollars"* an hour, shared across *"23 other people using it that day."* CapEx → shared OpEx. He's explicitly recreating Shenzhen's dense component-and-prototyping ecosystem in a US cloud factory.
- AI is woven in from the start: *"a network of custom-built software agents tied to Adom's knowledge base,"* layered over Claude, ChatGPT, Gemini. End-state: *"you'll ask the AI to design your electronics for you."*

**The key product concept — "molecules" and "workcells":**
- **Molecules** = Adom's term for modular PCBs designed to plug into factory **workcells**. A UT Dallas advisor describes them perfectly: *"Instead of buying an Arduino or Raspberry Pi, you'd choose from a library of these modules and connect them like Legos. Then you'd do what you need to do — measurements, firmware development, testing — all remotely."*
- A **workcell** is the physical heart of the factory: a cell where **robot pincers connect molecules with wires** for prototype testing. (Texas A&M built them a wire-bending rig to automate those connections; UT Austin worked on an AI-trained robotic arm.)
- This is "software-defined hardware": a reconfigurable bank of modular boards + instruments you reprogram and rewire remotely, time-shared like cloud compute.

**Status / scale (context for your interview):** ~11 people as of Aug 2025, "essentially co-founders," self-funded ~$10M by Lauer, Fort Worth approved a $15M incentive package (Project Nimbus, $229M over four phases to 2033), still officially in stealth with a public launch targeted for 2026. They're hiring straight out of MIT/Stanford/CMU/Georgia Tech/UT — which means **your 20 years of shipping + delivery judgment is a differentiator against a young team.**

---

## Part 2 — The architecture, decoded from their GitHub

The org `github.com/adom-inc` is public, has 25 repos, and the tagline is the whole thesis: **"Hardware development needs to be as fluid as software development. Let the revolution begin."** What's in there tells you exactly how they build:

**Language & framework spine — Rust, embedded-async:**
- `embassy` (fork) — modern **Rust async embedded** framework. This is their firmware foundation.
- `postcard-rpc` — an **RPC layer** over the `postcard` wire format (embedded Rust ↔ host messaging).
- `tsify` — generates **TypeScript types from Rust**. This is the smoking gun for a **Rust backend ↔ TypeScript browser frontend** control plane.
- `assign-resources`, `modular-bitfield`, `spi-pio` — embedded-Rust plumbing.

**The module bus — CAN-FD everywhere:**
- `mcp2518fd` / `mcp2515` — Rust `#![no_std]` drivers for **CAN / CAN-FD controllers**.
- `slcanx` / `slcan_fd` — **Serial-Line CAN** (CAN tunneled over serial; the bridge between modules and a host).
- `socketcan-rs` — Linux SocketCAN access in Rust.
- **Read:** molecules talk to each other and the workcell over **CAN-FD**, bridged to a Linux host. (CAN = robust, multi-drop, industrial — exactly right for a bus of pluggable modules. It's also straight out of the EE job's "I2C, SPI, **CANBUS**" list.)

**The robotics — Gcode motion control:**
- `klipper` (fork) — *"to allow use of up to 8 axes at once."* Klipper is high-performance 3D-printer/CNC motion firmware. This is the **robot-pincer / wire-bending motion layer**, and it maps word-for-word to the Robotics Engineer JD: *"generating steps from a microcontroller driven from Gcode to make robot arms move along specified paths."*
- `tmc2240` (Trinamic **stepper driver**), `as5047d` (absolute **magnetic encoder**) — closed-loop motion hardware drivers.

**The measurement & power front-end:**
- `ads1220` / `ads123x` (precision **delta-sigma ADCs**), `cdcx913` (**PLL clock**), `Modular_BMS` (battery management, C++). This is the instrumentation that turns a molecule into something you can *measure* remotely.

**EDA / design automation libraries:**
- `kicad_lib`, `gerber_lib`, `fusion-360-electronics-adom-library` — Rust libraries to **programmatically read/modify KiCad, Gerber, and Fusion 360 Electronics files.** This is the "hardware as code" / automated-design layer.

**The desktop/client:**
- `hd-wsl2-image` — *"Golden WSL2 rootfs image for **Hydrogen Desktop**."* "Hydrogen Desktop" appears to be their client/local-agent environment (note the chemistry theme — atoms, molecules, hydrogen). A local agent bridging the browser to hardware is **exactly the SPJS/ChiliPeppr pattern Lauer invented**, generalized.

**Putting it together:** modular PCBs ("molecules") on a **CAN-FD** bus, running **Rust/Embassy** firmware, physically wired by **Klipper-driven** robot arms (steppers + encoders), measured by **precision ADCs**, all exposed through a **Rust→TypeScript→browser** control plane with **AI agents** on top, designed via **code that manipulates KiCad/Gerber** files. That's the machine.

---

## Part 3 — The problem space (why this is hard, and what's genuinely new)

To talk about Adom credibly, know the prior art and where the hard, unsolved parts are.

**Remote labs are old news — but limited.** Academic "remote labs" have let students wire and measure real circuits over the web for ~15 years. The canonical system, **VISIR**, realizes a student's wiring with a **relay switching matrix** and pre-validates every circuit before energizing it for safety; it's commercialized by **LabsLand**. **MIT's iLab** defined the standard three-tier pattern (browser client / lab server / broker). The catch: relay matrices cap you at ~17 nodes of textbook circuits, and each rig is expensive, so the field *shares* scarce rigs rather than scaling them. Adom's "molecules + robots" is a bet on breaking past that ceiling to arbitrary, real prototypes.

**"Device farms" prove the UX — but only for phones.** AWS Device Farm and Firebase Test Lab let you drive a *real physical device in a data center* from your browser. The embedded/IoT equivalent barely exists — people hand-roll Raspberry Pi rigs. **A general-purpose, customer-facing device farm for custom electronics is open white space**, and it's a chunk of what Adom is claiming.

**Instrument control is a solved, open standard — not the moat.** **SCPI** (universal ASCII instrument commands), **VISA/PyVISA**, **IVI**, and **LXI/HiSLIP** (instruments as network endpoints) mean "drive a power supply / scope / DMM from code over Ethernet" is mature and open-source. The moat isn't the protocol — it's the **orchestration, multiplexing, safety, and reliability** layer, plus bridging physical instruments to a cloud/AI control plane. (That bridge is, again, the SPJS pattern.)

**The "programmable wiring" problem is real silicon.** Letting software connect any node to any node means a **switch fabric** — relay matrices (instrument-grade: Pickering, up to 300V/2A, thousands of crosspoints, but bulky/expensive) or **analog crosspoint ICs** (ADI ADG2128 = 96 switches; the cheap CH446Q used by the open-source **Jumperless** programmable breadboard). The hard limits are physics: every solid-state switch adds ~tens of ohms and parasitic capacitance, caps current to ~0.1A and voltage to ~±9–20V, and a full any-to-any crossbar grows as N² switches. **Adom sidesteps the pure-crossbar ceiling by using robots to physically wire modules** — which is why the robotics hire matters.

**AI-native EDA is a live, funded race.** **Diode Computers** (a16z-backed, code-based/open-source PCBs LLMs can author, "weeks to minutes") is the closest public proxy to Adom's *design* layer; **JITX** (hardware-as-code), **Quilter** (physics-driven autorouting), **Flux.ai** (AI copilot), and **Cofactr** (AI procurement) each own a slice. The honest hard part everyone hits: **LLMs hallucinate pins/structure, and in hardware a 0.1% error can fry a board** — so real systems pair generation with deterministic/formal validation. *(This is your eval-harness wheelhouse — worth flagging in an interview.)*

**The genuinely unsolved white space** — and the hardest part of Adom's pitch — is the *"testing electronics remotely"* half: **automated remote bring-up and test.** Manual test-fixture design alone takes 2–6 weeks; nobody convincingly automates end-to-end remote bring-up today. Plus Adom carries real capital-intensity risk (the cautionary tale is **Tempo Automation**, a software-driven PCBA factory that went public and struggled).

---

## Part 4 — POC concepts (a working slice of their app)

Design principle: **mirror their real architecture at desk scale.** Every concept below is a recognizable piece of the actual product, not a generic Arduino demo. Ordered by how directly they hit Adom's thesis.

### ★ Concept A — "Mini-Molecule + Cloud Workbench" (recommended spine)
A small modular board (your "molecule") that:
1. **Self-identifies** on a bus — on power-up it announces `{id, name, capabilities:[adc, gpio, pwm]}`. (Plug-and-play modules = the molecule concept.)
2. Is driven from a **browser workbench** over a **serial/CAN→WebSocket bridge** (their `slcanx` + `tsify` + Hydrogen-Desktop-style local agent, and Lauer's own SPJS lineage).
3. Exposes **remote measurement + actuation**: read an ADC channel, toggle GPIO, set a PWM, stream a value live.
4. Has an **AI layer**: natural language → a test plan ("measure channel 0 and tell me if the resistor is within 5%"), and an AI-written plain-English read of the captured data, plus a **shareable permalink** of the run (reproducibility = their whole reliability ethos).

*Why it wins:* it's the end-to-end vertical slice — molecule + bus + cloud control plane + AI + reproducibility — in one demo. **Stack signal maxed if the firmware is Rust + Embassy on an RP2040/RP2350 (Raspberry Pi Pico), with CAN via an MCP2515 module.** Scope: a couple of focused weekends for a serial v1; add CAN + Rust + AI as v2.

### ★ Concept B — "Programmable Patch Matrix" (the *programmable* add-on)
A handful of relays or an analog **crosspoint IC** (CH446Q, ~$2, the Jumperless chip) that lets the browser **programmatically wire a component into a measurement path**, then measures it — *"wire it in software, measure it remotely."* This is the literal "reconfigure hardware without touching it" core of a programmable factory. Bolt it onto Concept A and you've demonstrated *programmable wiring* + *remote measurement* together — uncannily close to a workcell in miniature. Slightly fiddlier hardware; highest "I understand the hard part" payoff.

### Concept C — "Remote Bring-Up Box" (the *testing remotely* slice)
An Arduino acting as a **test fixture** for a board-under-test: power-cycle it, poke an input, measure the output, run an automated pass/fail suite, report to a browser dashboard — a hobby-scale **"CI/CD for hardware."** This aims straight at the unsolved white space (remote bring-up/test) and pairs beautifully with your eval-harness angle. Less visually flashy than A+B.

### Concept D — "Gcode Molecule Mover" (the *robotics* slice)
Drive a 2-axis motion rig from **Gcode typed in the browser** → serial → stepper pulses to position/"place" a molecule — echoing their Klipper 8-axis fork and *"take a 3D design → compute robot movements → drive arms."* Coolest robotics signal, but needs steppers/rails (~$30–50) and the most build time. Great as a stretch if you want to lean robotics.

**Recommendation:** Build **A as the spine, add B if time allows.** Together they demonstrate *molecule + programmable wiring + remote measurement + AI + reproducibility* — five of Adom's pillars in one runnable demo. Keep C's pass/fail framing in the README to flag the "remote test" insight even if you don't fully build it.

---

## Part 5 — The killer move: build it on Adom's own open source

Adom's crates are public and MIT/Apache licensed. Building your POC **on their actual libraries** turns "I made a demo" into "I used your stack to build a mini version of your product." Concretely:

- Firmware messaging with **`postcard-rpc`** (their repo) instead of rolling your own protocol.
- CAN with **`mcp2515`** (their repo) on a $2 module — you'd literally be running Adom's driver.
- **`tsify`** (their repo) to generate the browser's TypeScript types from your Rust message structs — exactly their Rust→TS pattern.
- Firmware on **Embassy** (their fork) on a Raspberry Pi Pico.

Even using *one or two* of these and saying so in the README/interview is a standout move: it proves you read their code, you can work in their stack, and you ramp on unfamiliar Rust fast (your whole FDE thesis, demonstrated rather than asserted).

---

## Part 6 — Bill of materials (recommended POC)

You already have an Arduino + breadboard, so v1 can cost ~$0. To match their stack, add cheap parts:

| Item | Why | Approx cost |
|---|---|---|
| Raspberry Pi Pico (RP2040) or Pico 2 (RP2350) | Runs Embassy/Rust; RP2350 matches Jumperless/their MCU era | $4–6 |
| MCP2515 CAN module (×2 if you want a real bus) | Use Adom's own `mcp2515` Rust driver; demonstrates the CAN-FD-style bus | ~$2–4 each |
| Assorted resistors / LEDs / a potentiometer / photoresistor | Components to measure/characterize | ~$10 (kit) |
| CH446Q crosspoint IC (Concept B) | Programmable analog wiring, the Jumperless chip | ~$2–5 |
| Small relay module (alt for Concept B) | Simpler programmable wiring | ~$5 |
| (Stretch, Concept D) 28BYJ-48 stepper + ULN2003, or NEMA17 + A4988 | Gcode motion | $5–15 |

Total for a strong A+B build: **~$20–30.** You don't need their $182M of equipment to demo the *idea* of their equipment.

---

## Part 7 — Architecture sketch (Concept A)

```
 Browser Workbench (TypeScript/React)
   • discover molecules → show capabilities
   • run measurement → live plot (Chart.js)
   • "Run test" (NL) → AI test plan → result + plain-English readout
   • Share permalink (reproducible run)
        │  WebSocket (JSON / postcard-rpc-shaped)
        ▼
 Host Bridge (Rust)  ← echoes SPJS/Hydrogen Desktop + slcanx
   • serial or SocketCAN ↔ WebSocket
   • TS types generated via tsify
        │  USB serial  /  CAN (MCP2515)
        ▼
 Molecule firmware (Rust + Embassy on RP2040/RP2350)
   • on boot: announce {id, name, capabilities}
   • commands: read_adc(ch), set_gpio(pin,v), set_pwm(ch,duty)
   • (Concept B) set_switch(x,y) → CH446Q crosspoint wiring
        │
        ▼
 Hardware: ADC channel + a component under test (+ optional crosspoint matrix)
```

A pragmatic v1 can skip the Rust bridge and talk **browser → Web Serial API → Pico** directly (Chrome-only; Lauer pioneered exactly this browser-serial path). Add the Rust bridge + CAN in v2 to match their stack more literally.

---

## Part 8 — How to use this in the interview

- Lead with the thesis, not the trivia: *"You're building the AWS of electronics prototyping — compute for atoms — with molecules on a CAN bus, wired by Klipper-driven robots, behind a Rust-to-browser control plane. So I built a tiny molecule that self-identifies on a bus and runs a remote measurement from the browser, using your `mcp2515` and `tsify` crates."* That sentence alone separates you from every other applicant.
- Tie it to Lauer's pattern: his whole career is *"put a hard-to-reach physical resource behind a clean software API and sell it as a utility"* (SMS carriers → Zipwhip's toll-free texting at 99.999% uptime → CNC-over-browser with ChiliPeppr → now the whole electronics lab). Show you see the throughline.
- Plant your unique value: 20 years shipping production systems, **reliability/uptime discipline** (their Zipwhip DNA), and **eval harnesses for AI correctness** — the exact antidote to the "LLMs hallucinate and fry boards" problem that the AI-native EDA field is stuck on.
- Be honest about hardware: you're strong software, ramping fast on the embedded/Rust/hardware side — and the POC *is the proof* you do that by building, not talking.

---

## Sources

- Adom careers & about: https://adom.inc/careers · https://adom.inc/about
- Dallas Innovates (primary interview — "data center for atoms," molecules, workcells, AWS-of-prototyping, funding): https://dallasinnovates.com/shenzhen-speed-in-the-cloud-north-texas-adom-industries-plans-229m-robotics-driven-electronics-factory/ · https://dallasinnovates.com/fort-worth-lands-adom-industries-229m-ai-native-cloud-factory-and-hq/
- Adom GitHub org (architecture signals): https://github.com/adom-inc · https://github.com/orgs/adom-inc/repositories
- John Lauer / Serial Port JSON Server (browser-to-serial lineage): https://github.com/johnlauer/serial-port-json-server · https://github.com/chilipeppr
- Remote labs / VISIR / iLab: https://ieeexplore.ieee.org/document/6305453/ · http://icampus.mit.edu/projects/ilabs/ · https://labsland.com/
- Device farms: https://aws.amazon.com/device-farm/
- Programmable wiring / Jumperless / crosspoint: https://architeuthis-flux.github.io/JumperlessV5/readme · https://github.com/Architeuthis-Flux/JumperlessV5 · https://www.analog.com/en/products/adg2128.html
- Instrument automation: https://www.ivifoundation.org/About-IVI/scpi.html · https://github.com/pyvisa/pyvisa · https://developer.chrome.com/docs/capabilities/serial
- AI-native EDA: https://blog.diode.computer/series-a-announcement · https://www.jitx.com/ · https://www.quilter.ai/ · https://www.flux.ai/
