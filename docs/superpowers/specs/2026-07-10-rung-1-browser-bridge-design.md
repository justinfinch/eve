# Rung 1 — Browser ↔ Pico loop via a host bridge

**Date:** 2026-07-10
**Branch:** `hardware/rung-1-browser-bridge`
**Status:** design approved, ready for implementation planning

## Goal

Close **Rung 1** of the hardware build ladder: a real RP2350 Pico announces itself over
USB serial and obeys commands sent from a browser, with the LED on **GP15** (from Rung 0)
now driven by the browser instead of a blink timer.

The north star is **hardware literacy, not a working rig** — the deliverable of each rung is
a written explanation of what is happening in the silicon, not just green tests. See the Arche:
[Learning Hardware as a Software Dev](../../../.arche/concepts/learning-hardware-as-a-software-dev.md).

## Scope decisions (settled during brainstorming)

1. **Full addressed command model now.** Commands use the capability-registry addressing
   `(capability, channel, op, args)` from day one, not a one-off "toggle LED" message. Per
   [ADR — Capability registry](../../../.arche/concepts/adr-capability-registry.md).
2. **Typed capability descriptors now.** `capabilities` upgrades from `Vec<String>` to typed
   descriptors (`{kind:"gpio", channels, ops}`), so the firmware can validate commands against
   what it advertises and the browser can render controls from the announcement.
3. **Simulator stays a full peer.** The sim is upgraded to emit typed capabilities, receive
   commands, validate them, and reply — so the entire round-trip is testable with no hardware.
   Per [ADR — Device simulator](../../../.arche/concepts/adr-device-simulator.md).
4. **Host bridge (v2 topology) instead of Web Serial (v1).** The browser talks to a Rust host
   bridge over WebSocket; the bridge owns the serial port. This consciously supersedes the
   v1-first phasing of [ADR — Phased control plane](../../../.arche/concepts/adr-phased-control-plane.md)
   — see "Arche bookkeeping" below.

## Architecture

```
                 ┌─ simulator   (fabricates messages)      ← no hardware needed
  browser ──WS───┤
                 └─ bridge  ──USB serial──►  Pico           ← real hardware
                    (relays real messages)
```

The browser speaks **only** WebSocket and cannot tell whether the simulator or a real Pico is
behind it. The simulator and the real device are interchangeable peers behind one API — this is
the Adom pattern (physical resources behind a software API, per
[John Lauer](../../../.arche/entities/john-lauer.md)) and it removes Web Serial's Chrome-only limit.

### Message framing

Framing (delimiting where one message ends) is only needed on a **raw byte stream**:

- **browser ↔ bridge (WebSocket):** message-framed by the transport — one JSON object per
  WebSocket text frame. No delimiter needed.
- **bridge ↔ Pico (serial):** raw bytes — **newline-delimited JSON (NDJSON)**, one JSON object
  per line, `\n`-terminated, both directions.

The bridge's core responsibility is to **translate between the two**: unwrap a WebSocket message
→ write it to serial + `\n`; read serial until `\n` → wrap it as a WebSocket message.

## Components

### 1. Contract (`crates/contract`) — the single source of truth

`no_std + alloc`, tsify-gated; `just gen` regenerates `web/src/contract.gen.ts`.

```rust
// Typed, self-describing capability descriptor. serde tag → {"kind":"gpio",...}
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Capability {
    Gpio { channels: u8, ops: Vec<String> },   // adc / pwm arrive at later rungs
}

pub struct SelfId {
    pub id: String,
    pub name: String,
    pub fw_version: String,
    pub capabilities: Vec<Capability>,          // was Vec<String>
}

// Host → Device: (capability, channel, op, args) addressing.
pub struct Command {
    pub capability: String,  // "gpio"
    pub channel: u8,         // 0
    pub op: String,          // "set"
    pub args: Vec<Arg>,      // [Arg::Bool(true)]
}

// Small closed value enum — deliberately NOT serde_json::Value, to keep the
// embedded target no_std-clean and the heap small.
#[serde(untagged)]
pub enum Arg { Bool(bool), Int(i64), Text(String) }

// Device → Host: framed replies (SelfId is one of them now).
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DeviceMsg {
    SelfId(SelfId),
    Ack { ok: bool, error: Option<String> },
}
```

### 2. Firmware (`firmware`) — USB serial on the Pico

- **USB CDC-ACM** via `embassy-usb`: the Pico enumerates as a virtual serial port
  (`/dev/tty.usbmodem…`), no host driver needed.
- **Concurrency:** two `async` blocks run under Embassy's executor via `join(...)` — one drives
  the USB device stack (must be serviced continuously), one runs our logic. Cooperative
  time-slicing on one core; no OS, no threads.
- **Logic:**
  1. On connect, emit the `SelfId` line advertising `Gpio { channels: 1, ops: ["set"] }`.
  2. Read bytes, buffer until `\n` → one `Command`.
  3. **Validate against advertised capability** — capability known, channel in range, op known.
     Invalid → `Ack{ok:false, error}` and no hardware action.
  4. Valid → drive **GP15** per `args`, reply `Ack{ok:true}`.
- **LED behavior change:** Rung 0 blinked on a timer; Rung 1 the LED is idle until commanded,
  then obeys the browser.
- **JSON on-chip:** `serde_json` with the `alloc` feature (contract types are heap-based).
- **Reflash quality-of-life (recommended, optional):** watch for a host opening the port at
  **1200 baud** → call `reset_to_usb_boot()` to reboot into flashing mode, removing the manual
  BOOTSEL button hold. Can be deferred without blocking the rung.

### 3. Bridge (`crates/bridge`) — new crate

The simulator with one part swapped: instead of inventing replies, it relays to a real port.

- Opens the Pico's serial port (`serialport` crate); auto-detect by Raspberry Pi USB vendor ID,
  with a `--port <path>` override.
- Accepts a WebSocket connection (`tokio-tungstenite`, as the sim does) on port **8766**.
- Runs two one-way relays concurrently:
  - *serial → browser:* buffer bytes until `\n`, forward each line as a WebSocket message.
  - *browser → serial:* write each WebSocket message to serial + `\n`.
- **Testability:** the relay core is generic over an async byte stream. Production plugs in the
  real serial port; tests plug in an in-memory `tokio::io::duplex` fake — so the framing/relay is
  exercised in CI with no Pico.

### 4. Web (`web`) — browser workbench

- Parse the tagged `DeviceMsg`: `selfid` re-renders controls, `ack` shows command result.
- **Render the LED control from the announcement**, not hardcoded: read the `Gpio` descriptor and
  draw one on/off toggle. Adding a capability in firmware later surfaces a control with little-to-no
  new browser code — the capability-registry payoff.
- On toggle, send `{capability:"gpio", channel:0, op:"set", args:[true]}` over WebSocket; show the
  returning `Ack`.
- A small **"simulator vs. device" picker** (WebSocket `8765` vs. `8766`), defaulting to the
  simulator so the UI is fully usable with no Pico plugged in.

### 5. Simulator (`crates/simulator`) — upgraded to full parity

Emit the typed `SelfId` (with `Gpio` capability), receive `Command`s, validate them the same way
the firmware does, flip an in-memory LED boolean, reply `Ack`. Enables the full round-trip — click →
command → ack → state change — end-to-end with no hardware, in CI.

## Testing

Rule: everything that can be tested without a Pico, is.

- **Contract:** round-trip serialization for `Capability`, `Command`, `Arg`, `DeviceMsg`, including
  the exact NDJSON line format; update the existing `SelfId` test for typed capabilities.
- **Simulator:** extend the handshake test — receive typed `SelfId`, send `Command`, receive `Ack`;
  assert a bad address (channel out of range) → `ok:false`.
- **Bridge:** relay test against an in-memory `tokio::io::duplex` fake serial — feed `l1\nl2\n`,
  assert two WebSocket messages; push a command, assert `command\n` is written.
- **Web:** `vitest` on the pure functions — routing a tagged `DeviceMsg`, encoding a `Command`,
  deriving the control from a capability descriptor.
- **Firmware:** no automated test (`no_std`, on-chip); covered by contract tests + bench verify.
- **Drift guard:** `just gen` regenerates `contract.gen.ts`; `just check-gen` fails if stale.

### Bench verify (the rung passes when)

Flash with `just uf2`, plug in the Pico, run the bridge + web, click the toggle → the real LED on
GP15 obeys, and the `gpio` capability round-trips into the UI.

## Build wiring (`justfile`)

- The `bridge` crate is a workspace member — `just build` / `just check` pick it up automatically
  (build, `clippy -D warnings`, `cargo test`).
- Add `just bridge` (run the bridge against a real Pico). Keep `just dev` unchanged (sim + web,
  hardware-free).
- Firmware gains deps: `embassy-usb`, `embassy-futures`, `serde_json` (alloc feature).

## Deliverable write-ups (the point of the rung)

Drafted here, refined by Justin into his own words, in `docs/lessons/hardware/writeups/`:

- **Rung 1:** where the host↔device boundary sits; how a capability list makes the board
  plug-and-play; why hardware-behind-an-API (the bridge) is the Adom pattern; why framing lives on
  the serial link but not the WebSocket.
- **Rung 0 (outstanding):** why the current-limiting resistor exists and what would physically
  happen without it.

## Arche bookkeeping

Going straight to the bridge **supersedes** the v1-first phasing in
[ADR — Phased control plane](../../../.arche/concepts/adr-phased-control-plane.md) (Web Serial v1 →
bridge v2). Recommended follow-up (non-blocking): update that ADR via `/arche-architect` or a direct
edit so the Arche reflects what was actually built. The move toward typed capability descriptors
*fulfills* [ADR — Capability registry](../../../.arche/concepts/adr-capability-registry.md) earlier
than planned — no conflict, just realized at Rung 1.

## Out of scope (later rungs)

- CAN bus (Rung 2) — the reason the bridge exists; Web Serial could never reach it.
- ADC capability / typed `Adc` descriptor (Rungs 3–4).
- PWM / motor actuation (Rung 5).
- AI planner layer (Rung 6).
- postcard-rpc binary serialization at the bridge↔firmware seam (a later contract-ADR phase).
```

