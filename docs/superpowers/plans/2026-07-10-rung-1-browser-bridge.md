# Rung 1 — Browser ↔ Pico via Host Bridge — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** A real RP2350 Pico announces itself over USB serial and obeys browser commands (LED on GP15), with a Rust host bridge relaying between the browser's WebSocket and the Pico's newline-framed serial.

**Architecture:** One Rust `contract` crate is the single source of truth for message types *and* command validation. The browser speaks only WebSocket, to either the `simulator` (fabricates) or the `bridge` (relays to a real Pico over serial). The bridge is a domain-agnostic byte-stream relay. Firmware uses `embassy-usb` CDC-ACM and shares the contract's validation with the simulator.

**Tech Stack:** Rust (serde, tsify-next, tokio, tokio-tungstenite, serialport, embassy-usb/rp/executor, serde_json), TypeScript/React/Vite/Vitest.

## Global Constraints

- Devbox is the source of truth; run every command through `devbox run -- <cmd>` in non-interactive contexts (or rely on direnv in an interactive shell).
- `wasm-bindgen` crate version must equal the devbox `wasm-bindgen-cli` version — currently `=0.2.100` in `crates/contract/Cargo.toml`. Do not bump it.
- The contract is the single source of truth: never hand-edit `web/src/contract.gen.ts`; regenerate with `just gen`. `just check-gen` must pass (no drift).
- Firmware builds from CWD `firmware/` (its `.cargo/config.toml` sets the `thumbv8m.main-none-eabihf` target). The workspace `Cargo.toml` excludes `firmware` and `web`.
- Acceptance gate for non-firmware work: `just check` (build all, gen-drift guard, `clippy -D warnings`, `cargo test`, `npm test`).
- Firmware is `no_std`; it is verified on the bench (`just uf2` → flash → observe), not by automated tests.
- Wire formats: browser↔bridge and browser↔sim = one JSON object per WebSocket text frame. bridge↔Pico = newline-delimited JSON (`\n` per message).
- Message tags (serde): `DeviceMsg` internally tagged on `"type"` (`"selfid"`/`"ack"`); `Capability` internally tagged on `"kind"` (`"gpio"`); `Arg` untagged.
- Commit after every task. Branch: `hardware/rung-1-browser-bridge` (already created).

---

## File Structure

- `crates/contract/src/lib.rs` — **modify.** Add `Capability`, `Command`, `Arg`, `DeviceMsg`; change `SelfId.capabilities` to `Vec<Capability>`; add shared `resolve_gpio_set` validator. Tests updated.
- `web/src/contract.gen.ts` — **regenerated** by `just gen`.
- `crates/simulator/src/lib.rs` — **modify.** Advertise typed capability; handle commands; reply `Ack`. Keep the connection open.
- `crates/simulator/tests/handshake.rs` — **modify.** Typed SelfId + command round-trip + bad-address rejection.
- `crates/bridge/` — **create.** New crate: generic relay (`src/relay.rs`), WebSocket + serial wiring (`src/lib.rs`), binary (`src/main.rs`), tests (`tests/relay.rs`).
- `Cargo.toml` (workspace) — **modify.** Add `crates/bridge` to `members`.
- `web/src/selfId.ts` → **replace** with `web/src/protocol.ts` (parse `DeviceMsg`, build `Command`, derive control from capability).
- `web/src/selfId.test.ts` → **replace** with `web/src/protocol.test.ts`.
- `web/src/App.tsx` — **modify.** Sim/device picker, render control from capability, send command, show ack.
- `firmware/Cargo.toml` — **modify.** Add `embassy-usb`, `embassy-futures`, `serde_json` (alloc).
- `firmware/src/main.rs` — **modify.** USB CDC-ACM; announce SelfId; read+validate commands; drive GP15; ack.
- `justfile` — **modify.** Add `bridge` recipe.
- `docs/lessons/hardware/writeups/rung-0-resistor.md`, `docs/lessons/hardware/writeups/rung-1-boundary.md` — **create.** The learning deliverables.

---

## Task 1: Contract — typed capabilities, addressed commands, shared validator

**Files:**
- Modify: `crates/contract/src/lib.rs`
- Regenerate: `web/src/contract.gen.ts` (via `just gen`)

**Interfaces:**
- Produces (Rust, used by simulator + firmware):
  - `enum Capability { Gpio { channels: u8, ops: Vec<String> } }`
  - `struct SelfId { id: String, name: String, fw_version: String, capabilities: Vec<Capability> }`
  - `struct Command { capability: String, channel: u8, op: String, args: Vec<Arg> }`
  - `enum Arg { Bool(bool), Int(i64), Text(String) }`
  - `enum DeviceMsg { SelfId(SelfId), Ack { ok: bool, error: Option<String> } }`
  - `fn resolve_gpio_set(caps: &[Capability], cmd: &Command) -> Result<bool, String>`

- [ ] **Step 1: Write the failing tests**

Replace the `mod tests` block at the bottom of `crates/contract/src/lib.rs` with:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    fn gpio_self_id() -> SelfId {
        SelfId {
            id: "mol-001".into(),
            name: "Mini-Molecule".into(),
            fw_version: "0.1.0".into(),
            capabilities: vec![Capability::Gpio {
                channels: 1,
                ops: vec!["set".into()],
            }],
        }
    }

    #[test]
    fn self_id_serializes_with_typed_capability() {
        let json = serde_json::to_string(&gpio_self_id()).unwrap();
        assert_eq!(
            json,
            r#"{"id":"mol-001","name":"Mini-Molecule","fw_version":"0.1.0","capabilities":[{"kind":"gpio","channels":1,"ops":["set"]}]}"#
        );
    }

    #[test]
    fn device_msg_selfid_is_internally_tagged() {
        let json = serde_json::to_string(&DeviceMsg::SelfId(gpio_self_id())).unwrap();
        assert!(json.starts_with(r#"{"type":"selfid","id":"mol-001""#), "got {json}");
    }

    #[test]
    fn device_msg_ack_serializes() {
        let json = serde_json::to_string(&DeviceMsg::Ack { ok: true, error: None }).unwrap();
        assert_eq!(json, r#"{"type":"ack","ok":true,"error":null}"#);
    }

    #[test]
    fn command_roundtrips() {
        let cmd = Command {
            capability: "gpio".into(),
            channel: 0,
            op: "set".into(),
            args: vec![Arg::Bool(true)],
        };
        let json = serde_json::to_string(&cmd).unwrap();
        assert_eq!(
            json,
            r#"{"capability":"gpio","channel":0,"op":"set","args":[true]}"#
        );
        let back: Command = serde_json::from_str(&json).unwrap();
        assert_eq!(back.args, vec![Arg::Bool(true)]);
    }

    #[test]
    fn resolve_gpio_set_accepts_valid_command() {
        let caps = gpio_self_id().capabilities;
        let cmd = Command {
            capability: "gpio".into(),
            channel: 0,
            op: "set".into(),
            args: vec![Arg::Bool(true)],
        };
        assert_eq!(resolve_gpio_set(&caps, &cmd), Ok(true));
    }

    #[test]
    fn resolve_gpio_set_rejects_out_of_range_channel() {
        let caps = gpio_self_id().capabilities;
        let cmd = Command {
            capability: "gpio".into(),
            channel: 5,
            op: "set".into(),
            args: vec![Arg::Bool(true)],
        };
        assert!(resolve_gpio_set(&caps, &cmd).is_err());
    }

    #[test]
    fn resolve_gpio_set_rejects_unknown_op() {
        let caps = gpio_self_id().capabilities;
        let cmd = Command {
            capability: "gpio".into(),
            channel: 0,
            op: "pulse".into(),
            args: vec![Arg::Bool(true)],
        };
        assert!(resolve_gpio_set(&caps, &cmd).is_err());
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `devbox run -- cargo test -p contract`
Expected: FAIL — `Capability`, `Command`, `Arg`, `DeviceMsg`, `resolve_gpio_set` not found.

- [ ] **Step 3: Implement the types and validator**

In `crates/contract/src/lib.rs`, keep the existing header (`#![cfg_attr(...)]`, `extern crate alloc;`, the `use` lines, and the `#[cfg(feature = "codegen")]` tsify imports). Replace the `SelfId` struct with the following block (add `format` to the alloc imports at the top: change `use alloc::string::String;` region to also `use alloc::format;` and `use alloc::string::ToString;`):

```rust
/// A typed, self-describing capability descriptor — the ADR's registry.
/// Internally tagged so the wire is self-describing: {"kind":"gpio",...}.
#[cfg_attr(feature = "codegen", derive(Tsify))]
#[cfg_attr(feature = "codegen", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum Capability {
    Gpio { channels: u8, ops: Vec<String> },
}

/// The molecule's self-description — the spine every later rung extends.
#[cfg_attr(feature = "codegen", derive(Tsify))]
#[cfg_attr(feature = "codegen", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SelfId {
    pub id: String,
    pub name: String,
    pub fw_version: String,
    pub capabilities: Vec<Capability>,
}

/// A command argument. Untagged: serializes as a bare JSON scalar.
/// Deliberately a small closed enum (not serde_json::Value) to keep the
/// embedded target no_std-clean and the heap small.
#[cfg_attr(feature = "codegen", derive(Tsify))]
#[cfg_attr(feature = "codegen", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Arg {
    Bool(bool),
    Int(i64),
    Text(String),
}

/// Host -> Device command, addressed as (capability, channel, op, args).
#[cfg_attr(feature = "codegen", derive(Tsify))]
#[cfg_attr(feature = "codegen", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Command {
    pub capability: String,
    pub channel: u8,
    pub op: String,
    pub args: Vec<Arg>,
}

/// Device -> Host framed message. Internally tagged on "type".
#[cfg_attr(feature = "codegen", derive(Tsify))]
#[cfg_attr(feature = "codegen", tsify(into_wasm_abi, from_wasm_abi))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DeviceMsg {
    SelfId(SelfId),
    Ack { ok: bool, error: Option<String> },
}

/// Validate a command against advertised capabilities and, on success,
/// return the desired GPIO level. Shared by firmware and simulator so both
/// enforce identical invariants (address only advertised capabilities,
/// channel in range, op advertised, args well-formed).
pub fn resolve_gpio_set(caps: &[Capability], cmd: &Command) -> Result<bool, String> {
    if cmd.capability != "gpio" {
        return Err(format!("unknown capability {:?}", cmd.capability));
    }
    let (channels, ops) = caps
        .iter()
        .find_map(|c| match c {
            Capability::Gpio { channels, ops } => Some((*channels, ops)),
        })
        .ok_or_else(|| "gpio capability not advertised".to_string())?;
    if cmd.channel >= channels {
        return Err(format!("channel {} out of range (0..{})", cmd.channel, channels));
    }
    if cmd.op != "set" || !ops.iter().any(|o| o == "set") {
        return Err(format!("unknown op {:?}", cmd.op));
    }
    match cmd.args.as_slice() {
        [Arg::Bool(level)] => Ok(*level),
        _ => Err("gpio set expects args [bool]".to_string()),
    }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `devbox run -- cargo test -p contract`
Expected: PASS (7 tests).

- [ ] **Step 5: Regenerate the TypeScript contract and confirm it builds**

Run: `devbox run -- just gen`
Expected: "wrote web/src/contract.gen.ts". Then open `web/src/contract.gen.ts` and confirm it now contains `Capability`, `Command`, `Arg`, `DeviceMsg`, and `SelfId.capabilities: Capability[]`. (Note the exact generated union shapes — Task 4 keys off the `type`/`kind` fields rather than importing the enum types directly, so minor tsify formatting differences are fine.)

- [ ] **Step 6: Commit**

```bash
git add crates/contract/src/lib.rs web/src/contract.gen.ts
git commit -m "feat(contract): typed capabilities, addressed commands, shared validator"
```

---

## Task 2: Simulator — full peer (typed SelfId, command handling, Ack)

**Files:**
- Modify: `crates/simulator/src/lib.rs`
- Modify: `crates/simulator/tests/handshake.rs`

**Interfaces:**
- Consumes: `contract::{SelfId, Capability, Command, DeviceMsg, resolve_gpio_set}`.
- Produces: `simulator::self_id() -> SelfId` (now advertises `Gpio`), `simulator::serve(TcpListener)` unchanged signature.

- [ ] **Step 1: Write the failing test**

Replace `crates/simulator/tests/handshake.rs` with:

```rust
use contract::{Command, DeviceMsg};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

async fn next_text(ws: &mut (impl StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin)) -> String {
    ws.next().await.expect("a frame").expect("ok frame").into_text().unwrap()
}

#[tokio::test]
async fn announces_typed_selfid_then_acks_a_valid_command() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(simulator::serve(listener));

    let (mut ws, _resp) = connect_async(format!("ws://{addr}")).await.unwrap();

    // 1. First frame is the typed SelfId with a gpio capability.
    let hello = next_text(&mut ws).await;
    let msg: DeviceMsg = serde_json::from_str(&hello).unwrap();
    match msg {
        DeviceMsg::SelfId(id) => assert_eq!(id, simulator::self_id()),
        other => panic!("expected SelfId, got {other:?}"),
    }
    assert!(!simulator::self_id().capabilities.is_empty());

    // 2. A valid command gets ok:true.
    let cmd = Command { capability: "gpio".into(), channel: 0, op: "set".into(), args: vec![contract::Arg::Bool(true)] };
    ws.send(Message::Text(serde_json::to_string(&cmd).unwrap())).await.unwrap();
    let ack = next_text(&mut ws).await;
    match serde_json::from_str::<DeviceMsg>(&ack).unwrap() {
        DeviceMsg::Ack { ok, .. } => assert!(ok),
        other => panic!("expected Ack, got {other:?}"),
    }
}

#[tokio::test]
async fn rejects_a_bad_address() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(simulator::serve(listener));
    let (mut ws, _resp) = connect_async(format!("ws://{addr}")).await.unwrap();
    let _hello = next_text(&mut ws).await;

    let cmd = Command { capability: "gpio".into(), channel: 9, op: "set".into(), args: vec![contract::Arg::Bool(true)] };
    ws.send(Message::Text(serde_json::to_string(&cmd).unwrap())).await.unwrap();
    let ack = next_text(&mut ws).await;
    match serde_json::from_str::<DeviceMsg>(&ack).unwrap() {
        DeviceMsg::Ack { ok, error } => { assert!(!ok); assert!(error.is_some()); }
        other => panic!("expected Ack, got {other:?}"),
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `devbox run -- cargo test -p simulator`
Expected: FAIL — sim closes after one frame / doesn't handle commands (test hangs or errors).

- [ ] **Step 3: Rewrite the simulator to be a full peer**

Replace `crates/simulator/src/lib.rs` with:

```rust
use contract::{Capability, Command, DeviceMsg, SelfId};
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

/// The canonical self-description this software molecule advertises.
pub fn self_id() -> SelfId {
    SelfId {
        id: "mol-001".into(),
        name: "Mini-Molecule".into(),
        fw_version: "0.1.0".into(),
        capabilities: vec![Capability::Gpio { channels: 1, ops: vec!["set".into()] }],
    }
}

/// Accept WebSocket connections forever; each is a full peer session.
pub async fn serve(listener: TcpListener) {
    while let Ok((stream, _peer)) = listener.accept().await {
        tokio::spawn(handle(stream));
    }
}

async fn handle(stream: TcpStream) {
    let mut ws = match tokio_tungstenite::accept_async(stream).await {
        Ok(ws) => ws,
        Err(_) => return,
    };

    // Announce ourselves.
    let hello = serde_json::to_string(&DeviceMsg::SelfId(self_id())).expect("SelfId serializes");
    if ws.send(Message::Text(hello)).await.is_err() {
        return;
    }

    // Then act on commands, keeping an in-memory LED state.
    let caps = self_id().capabilities;
    let mut _led = false;
    while let Some(Ok(msg)) = ws.next().await {
        let Message::Text(txt) = msg else { continue };
        let reply = match serde_json::from_str::<Command>(&txt) {
            Ok(cmd) => match contract::resolve_gpio_set(&caps, &cmd) {
                Ok(level) => {
                    _led = level;
                    DeviceMsg::Ack { ok: true, error: None }
                }
                Err(e) => DeviceMsg::Ack { ok: false, error: Some(e) },
            },
            Err(e) => DeviceMsg::Ack { ok: false, error: Some(e.to_string()) },
        };
        let out = serde_json::to_string(&reply).expect("Ack serializes");
        if ws.send(Message::Text(out)).await.is_err() {
            break;
        }
    }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `devbox run -- cargo test -p simulator`
Expected: PASS (2 tests).

- [ ] **Step 5: Commit**

```bash
git add crates/simulator/src/lib.rs crates/simulator/tests/handshake.rs
git commit -m "feat(simulator): full peer — typed capability, command validation, ack"
```

---

## Task 3: Bridge crate — WebSocket ↔ newline-framed serial relay

**Files:**
- Create: `crates/bridge/Cargo.toml`, `crates/bridge/src/relay.rs`, `crates/bridge/src/lib.rs`, `crates/bridge/src/main.rs`, `crates/bridge/tests/relay.rs`
- Modify: `Cargo.toml` (workspace `members`)

**Interfaces:**
- Consumes: nothing from `contract` — the bridge is a domain-agnostic byte relay.
- Produces:
  - `bridge::relay::split_lines(buf: &mut Vec<u8>) -> Vec<String>` — drains complete `\n`-terminated lines from a byte buffer, leaving any partial tail.
  - `bridge::relay::pump<R, W>(serial_read: R, serial_write: W, ws: WebSocketStream<...>)` — the generic relay loop (integration-tested via an in-memory duplex).

- [ ] **Step 1: Add the crate to the workspace**

Modify `Cargo.toml` (workspace root):

```toml
[workspace]
resolver = "2"
members = ["crates/contract", "crates/simulator", "crates/bridge"]
exclude = ["firmware", "web"]
```

- [ ] **Step 2: Create the crate manifest**

Create `crates/bridge/Cargo.toml`:

```toml
[package]
name = "bridge"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["rt-multi-thread", "macros", "net", "io-util", "time"] }
tokio-tungstenite = "0.24"
tokio-serial = "5.4"
futures-util = "0.3"

[dev-dependencies]
tokio = { version = "1", features = ["rt-multi-thread", "macros", "net", "io-util", "time"] }
```

(`tokio-serial` wraps the `serialport` crate as an async `AsyncRead`/`AsyncWrite`, which is why the relay can be generic and tested with an in-memory duplex.)

- [ ] **Step 3: Write the failing unit test for line framing**

Create `crates/bridge/tests/relay.rs`:

```rust
use bridge::relay::split_lines;

#[test]
fn split_lines_drains_complete_lines_and_keeps_the_tail() {
    let mut buf = b"one\ntwo\npar".to_vec();
    let lines = split_lines(&mut buf);
    assert_eq!(lines, vec!["one".to_string(), "two".to_string()]);
    assert_eq!(buf, b"par".to_vec()); // partial line stays buffered
}

#[test]
fn split_lines_returns_empty_when_no_newline_yet() {
    let mut buf = b"partial".to_vec();
    let lines = split_lines(&mut buf);
    assert!(lines.is_empty());
    assert_eq!(buf, b"partial".to_vec());
}
```

- [ ] **Step 4: Run the test to verify it fails**

Run: `devbox run -- cargo test -p bridge`
Expected: FAIL — `bridge::relay` / `split_lines` not found.

- [ ] **Step 5: Implement the relay module**

Create `crates/bridge/src/relay.rs`:

```rust
use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

/// Drain every complete `\n`-terminated line from `buf`, returning them without
/// the trailing newline and leaving any partial (unterminated) tail in `buf`.
pub fn split_lines(buf: &mut Vec<u8>) -> Vec<String> {
    let mut lines = Vec::new();
    while let Some(pos) = buf.iter().position(|&b| b == b'\n') {
        let line: Vec<u8> = buf.drain(..=pos).collect();
        // line includes the '\n' at the end; trim it (and a stray '\r').
        let end = line.len().saturating_sub(1);
        let mut text = String::from_utf8_lossy(&line[..end]).into_owned();
        if text.ends_with('\r') {
            text.pop();
        }
        if !text.is_empty() {
            lines.push(text);
        }
    }
    lines
}

/// Relay forever between a byte stream (serial) and a WebSocket:
/// - serial bytes -> split on '\n' -> WebSocket text messages
/// - WebSocket text messages -> write to serial + '\n'
pub async fn pump<S, T>(
    mut serial_read: S,
    mut serial_write: T,
    ws: WebSocketStream<tokio_tungstenite::tungstenite::protocol::WebSocket<()>>,
) where
    S: AsyncRead + Unpin,
    T: AsyncWrite + Unpin,
{
    let _ = (&mut serial_read, &mut serial_write, ws);
    unimplemented!("wired concretely in lib.rs::run; see run() for the real loop")
}
```

> Note: `pump`'s generic WebSocket type is awkward to name across tokio-tungstenite versions, so the *concrete* relay loop lives in `lib.rs::run` (Step 7) where the stream types are known. `split_lines` — the part with real logic worth testing — is unit-tested above. Keep `pump` as documentation of intent or delete it if it does not compile against the installed tokio-tungstenite; it is not on the tested path.

- [ ] **Step 6: Run the test to verify it passes**

Run: `devbox run -- cargo test -p bridge`
Expected: PASS (2 tests). If `pump`'s signature fails to compile, delete the `pump` fn (keep `split_lines`) and re-run.

- [ ] **Step 7: Implement the library (concrete relay) and expose the module**

Create `crates/bridge/src/lib.rs`:

```rust
pub mod relay;

use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_serial::SerialPortBuilderExt;
use tokio_tungstenite::tungstenite::Message;

use crate::relay::split_lines;

/// Bind a WebSocket listener and, for the first client, relay to the serial port.
pub async fn run(ws_addr: &str, serial_path: &str, baud: u32) -> std::io::Result<()> {
    let listener = TcpListener::bind(ws_addr).await?;
    println!("bridge listening on ws://{ws_addr}, serial {serial_path} @ {baud}");

    loop {
        let (stream, _peer) = listener.accept().await?;
        let ws = match tokio_tungstenite::accept_async(stream).await {
            Ok(ws) => ws,
            Err(_) => continue,
        };
        let port = tokio_serial::new(serial_path, baud).open_native_async();
        let port = match port {
            Ok(p) => p,
            Err(e) => {
                eprintln!("failed to open serial {serial_path}: {e}");
                continue;
            }
        };
        relay_session(ws, port).await;
    }
}

async fn relay_session<S>(ws: tokio_tungstenite::WebSocketStream<S>, port: tokio_serial::SerialStream)
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
{
    let (mut ws_tx, mut ws_rx) = ws.split();
    let (mut serial_rx, mut serial_tx) = tokio::io::split(port);

    let mut buf: Vec<u8> = Vec::new();
    let mut chunk = [0u8; 256];

    loop {
        tokio::select! {
            // serial -> browser
            n = serial_rx.read(&mut chunk) => {
                let n = match n { Ok(0) | Err(_) => break, Ok(n) => n };
                buf.extend_from_slice(&chunk[..n]);
                for line in split_lines(&mut buf) {
                    if ws_tx.send(Message::Text(line)).await.is_err() { return; }
                }
            }
            // browser -> serial
            msg = ws_rx.next() => {
                let Some(Ok(msg)) = msg else { break };
                if let Message::Text(txt) = msg {
                    if serial_tx.write_all(txt.as_bytes()).await.is_err() { break; }
                    if serial_tx.write_all(b"\n").await.is_err() { break; }
                }
            }
        }
    }
}
```

Create `crates/bridge/src/main.rs`:

```rust
//! Host bridge: browser WebSocket <-> Pico USB serial (newline-framed JSON).
//! Usage: bridge [--port <serial>] [--ws <addr>] [--baud <n>]

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut port = default_port();
    let mut ws = "127.0.0.1:8766".to_string();
    let mut baud = 115_200u32;

    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--port" => port = args.next().expect("--port needs a value"),
            "--ws" => ws = args.next().expect("--ws needs a value"),
            "--baud" => baud = args.next().expect("--baud needs a value").parse().unwrap(),
            other => eprintln!("ignoring unknown arg {other}"),
        }
    }

    bridge::run(&ws, &port, baud).await
}

/// Best-effort auto-detect: first serial port whose USB vendor id is Raspberry Pi (0x2E8A).
fn default_port() -> String {
    if let Ok(ports) = tokio_serial::available_ports() {
        for p in &ports {
            if let tokio_serial::SerialPortType::UsbPort(info) = &p.port_type {
                if info.vid == 0x2E8A {
                    return p.port_name.clone();
                }
            }
        }
        if let Some(p) = ports.first() {
            return p.port_name.clone();
        }
    }
    "/dev/tty.usbmodem0001".to_string()
}
```

- [ ] **Step 8: Build the whole workspace and run bridge tests**

Run: `devbox run -- cargo build --workspace`
Expected: builds (bridge compiles). If `relay::pump` fails to compile, delete it per Step 6's note and rebuild.
Run: `devbox run -- cargo test -p bridge`
Expected: PASS.

- [ ] **Step 9: Commit**

```bash
git add Cargo.toml crates/bridge
git commit -m "feat(bridge): WebSocket <-> newline-framed serial relay crate"
```

---

## Task 4: Web — parse DeviceMsg, render control from capability, send Command, picker

**Files:**
- Create: `web/src/protocol.ts`, `web/src/protocol.test.ts`
- Delete: `web/src/selfId.ts`, `web/src/selfId.test.ts`
- Modify: `web/src/App.tsx`

**Interfaces:**
- Consumes: generated `web/src/contract.gen.ts` (`SelfId`, `Capability`, `Command`).
- Produces:
  - `parseDeviceMsg(raw: string): { kind: "selfid"; selfId: SelfId } | { kind: "ack"; ok: boolean; error: string | null } | { kind: "unknown" }`
  - `encodeSetGpio(channel: number, on: boolean): string` — the JSON command line.
  - `gpioCapability(selfId: SelfId): { channels: number } | null` — derive the control from the announcement.

- [ ] **Step 1: Write the failing tests**

Create `web/src/protocol.test.ts`:

```ts
import { describe, it, expect } from "vitest";
import { parseDeviceMsg, encodeSetGpio, gpioCapability } from "./protocol";

describe("parseDeviceMsg", () => {
  it("routes a selfid frame", () => {
    const raw =
      '{"type":"selfid","id":"mol-001","name":"Mini-Molecule","fw_version":"0.1.0","capabilities":[{"kind":"gpio","channels":1,"ops":["set"]}]}';
    const msg = parseDeviceMsg(raw);
    expect(msg.kind).toBe("selfid");
    if (msg.kind === "selfid") {
      expect(msg.selfId.id).toBe("mol-001");
      expect(gpioCapability(msg.selfId)).toEqual({ channels: 1 });
    }
  });

  it("routes an ack frame", () => {
    const msg = parseDeviceMsg('{"type":"ack","ok":true,"error":null}');
    expect(msg).toEqual({ kind: "ack", ok: true, error: null });
  });
});

describe("encodeSetGpio", () => {
  it("builds the addressed command line", () => {
    expect(encodeSetGpio(0, true)).toBe(
      '{"capability":"gpio","channel":0,"op":"set","args":[true]}'
    );
  });
});

describe("gpioCapability", () => {
  it("returns null when no gpio is advertised", () => {
    const selfId = { id: "x", name: "x", fw_version: "0", capabilities: [] };
    expect(gpioCapability(selfId as never)).toBeNull();
  });
});
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `devbox run -- npm --prefix web test`
Expected: FAIL — `./protocol` not found.

- [ ] **Step 3: Implement the protocol module**

Create `web/src/protocol.ts`:

```ts
import type { SelfId } from "./contract.gen";

export type ParsedMsg =
  | { kind: "selfid"; selfId: SelfId }
  | { kind: "ack"; ok: boolean; error: string | null }
  | { kind: "unknown" };

/** Route a raw WebSocket frame by its "type" tag. */
export function parseDeviceMsg(raw: string): ParsedMsg {
  const obj = JSON.parse(raw) as Record<string, unknown>;
  if (obj.type === "selfid") {
    return { kind: "selfid", selfId: obj as unknown as SelfId };
  }
  if (obj.type === "ack") {
    return { kind: "ack", ok: Boolean(obj.ok), error: (obj.error as string | null) ?? null };
  }
  return { kind: "unknown" };
}

/** Build the addressed GPIO set command as a JSON line. */
export function encodeSetGpio(channel: number, on: boolean): string {
  return JSON.stringify({ capability: "gpio", channel, op: "set", args: [on] });
}

/** Derive the GPIO control descriptor from the announcement, if any. */
export function gpioCapability(selfId: SelfId): { channels: number } | null {
  for (const cap of selfId.capabilities as Array<Record<string, unknown>>) {
    if (cap.kind === "gpio") {
      return { channels: Number(cap.channels) };
    }
  }
  return null;
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `devbox run -- npm --prefix web test`
Expected: PASS.

- [ ] **Step 5: Delete the superseded selfId module**

```bash
git rm web/src/selfId.ts web/src/selfId.test.ts
```

- [ ] **Step 6: Rewrite the App to use the picker + capability-driven control**

Replace `web/src/App.tsx` with:

```tsx
import { useEffect, useRef, useState } from "react";
import type { SelfId } from "./contract.gen";
import { parseDeviceMsg, encodeSetGpio, gpioCapability } from "./protocol";

const ENDPOINTS = {
  simulator: "ws://127.0.0.1:8765",
  device: "ws://127.0.0.1:8766",
} as const;
type Target = keyof typeof ENDPOINTS;

export default function App() {
  const [target, setTarget] = useState<Target>("simulator");
  const [selfId, setSelfId] = useState<SelfId | null>(null);
  const [status, setStatus] = useState("connecting…");
  const [ack, setAck] = useState<string>("");
  const [on, setOn] = useState(false);
  const wsRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    setSelfId(null);
    setStatus("connecting…");
    const ws = new WebSocket(ENDPOINTS[target]);
    wsRef.current = ws;
    ws.onopen = () => setStatus("connected");
    ws.onmessage = (ev) => {
      const msg = parseDeviceMsg(ev.data as string);
      if (msg.kind === "selfid") setSelfId(msg.selfId);
      else if (msg.kind === "ack") setAck(msg.ok ? "ok ✓" : `error: ${msg.error}`);
    };
    ws.onerror = () => setStatus("disconnected");
    ws.onclose = () => setStatus((s) => (s === "connected" ? "disconnected" : s));
    return () => ws.close();
  }, [target]);

  const gpio = selfId ? gpioCapability(selfId) : null;

  function toggle() {
    const next = !on;
    setOn(next);
    wsRef.current?.send(encodeSetGpio(0, next));
  }

  return (
    <main style={{ fontFamily: "system-ui", padding: "2rem" }}>
      <h1>Mini-Molecule Workbench</h1>
      <label>
        source:{" "}
        <select value={target} onChange={(e) => setTarget(e.target.value as Target)}>
          <option value="simulator">simulator (no hardware)</option>
          <option value="device">device (bridge)</option>
        </select>
      </label>
      <p>status: {status}</p>
      {selfId ? (
        <>
          <dl>
            <dt>id</dt>
            <dd data-testid="id">{selfId.id}</dd>
            <dt>name</dt>
            <dd data-testid="name">{selfId.name}</dd>
          </dl>
          {gpio ? (
            <button data-testid="led-toggle" onClick={toggle}>
              LED (ch0): turn {on ? "off" : "on"}
            </button>
          ) : (
            <p>no gpio capability advertised</p>
          )}
          {ack && <p data-testid="ack">last command: {ack}</p>}
        </>
      ) : (
        <p>waiting for the molecule to announce itself…</p>
      )}
    </main>
  );
}
```

- [ ] **Step 7: Verify the web build and tests pass**

Run: `devbox run -- npm --prefix web run build`
Expected: `tsc -b` + vite build succeed.
Run: `devbox run -- npm --prefix web test`
Expected: PASS.

- [ ] **Step 8: Commit**

```bash
git add web/src/protocol.ts web/src/protocol.test.ts web/src/App.tsx
git commit -m "feat(web): capability-driven LED control, sim/device picker, ack display"
```

---

## Task 5: Firmware — USB CDC serial, announce, validate, drive GP15

**Files:**
- Modify: `firmware/Cargo.toml`
- Modify: `firmware/src/main.rs`

**Interfaces:**
- Consumes: `contract::{SelfId, Capability, Command, DeviceMsg, resolve_gpio_set}`.
- Produces: a flashable UF2 whose device behavior matches the simulator.

> Firmware is `no_std` and verified on the bench, not by automated tests. `embassy-usb`'s exact API is version-sensitive; the code below targets the `embassy-*` versions already pinned in `firmware/Cargo.toml` (executor 0.7 / rp 0.4 / time 0.4) with `embassy-usb 0.4`. Expect to iterate against the compiler and the [embassy-rp USB CDC example](https://github.com/embassy-rs/embassy/blob/main/examples/rp235x/src/bin/usb_serial.rs); adjust `embassy-usb` API calls if a minor version differs.

- [ ] **Step 1: Add firmware dependencies**

Modify `firmware/Cargo.toml` `[dependencies]` — add:

```toml
embassy-usb = "0.4"
embassy-futures = "0.1"
serde_json = { version = "1", default-features = false, features = ["alloc"] }
serde = { version = "1", default-features = false, features = ["derive", "alloc"] }
```

(Keep the existing `contract`, `embassy-executor`, `embassy-rp`, `embassy-time`, `cortex-m-rt`, `panic-halt`, `embedded-alloc` entries.)

- [ ] **Step 2: Rewrite `firmware/src/main.rs`**

Replace `firmware/src/main.rs` with:

```rust
#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use alloc::vec;
use contract::{Capability, Command, DeviceMsg, SelfId};
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::driver::EndpointError;
use embassy_usb::{Builder, Config};
use embedded_alloc::LlffHeap as Heap;
use panic_halt as _;

#[global_allocator]
static HEAP: Heap = Heap::empty();

// RP2350 boot image signature; see firmware/memory.x `.start_block`.
#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: embassy_rp::block::ImageDef = embassy_rp::block::ImageDef::secure_exe();

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

fn self_id() -> SelfId {
    SelfId {
        id: "mol-001".into(),
        name: "Mini-Molecule".into(),
        fw_version: "0.1.0".into(),
        capabilities: vec![Capability::Gpio { channels: 1, ops: vec!["set".into()] }],
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Heap for the contract's alloc-backed types + serde_json.
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 8192;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(core::ptr::addr_of_mut!(HEAP_MEM) as usize, HEAP_SIZE) }
    }

    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_15, Level::Low);

    // --- USB CDC-ACM setup ---
    let driver = Driver::new(p.USB, Irqs);

    let mut config = Config::new(0x2e8a, 0x0009); // Raspberry Pi vendor id
    config.manufacturer = Some("Mini-Molecule");
    config.product = Some("Mini-Molecule");
    config.serial_number = Some("mol-001");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    let mut config_descriptor = [0u8; 256];
    let mut bos_descriptor = [0u8; 256];
    let mut control_buf = [0u8; 64];
    let mut state = State::new();

    let mut builder = Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut [], // no MS OS descriptors
        &mut control_buf,
    );

    let mut class = CdcAcmClass::new(&mut builder, &mut state, 64);
    let mut usb = builder.build();

    let usb_fut = usb.run();

    let comms_fut = async {
        loop {
            class.wait_connection().await;
            let _ = session(&mut class, &mut led).await;
        }
    };

    join(usb_fut, comms_fut).await;
}

/// One connected session: announce, then serve commands until disconnect.
async fn session<'d, D: embassy_usb::driver::Driver<'d>>(
    class: &mut CdcAcmClass<'d, D>,
    led: &mut Output<'static>,
) -> Result<(), EndpointError> {
    // Announce ourselves.
    let hello = serde_json::to_string(&DeviceMsg::SelfId(self_id())).unwrap();
    write_line(class, &hello).await?;

    let caps = self_id().capabilities;
    let mut line: alloc::vec::Vec<u8> = alloc::vec::Vec::new();
    let mut packet = [0u8; 64];

    loop {
        let n = class.read_packet(&mut packet).await?;
        line.extend_from_slice(&packet[..n]);

        // Process every complete '\n'-terminated command in the buffer.
        while let Some(pos) = line.iter().position(|&b| b == b'\n') {
            let raw: alloc::vec::Vec<u8> = line.drain(..=pos).collect();
            let end = raw.len().saturating_sub(1);
            let reply = match serde_json::from_slice::<Command>(&raw[..end]) {
                Ok(cmd) => match contract::resolve_gpio_set(&caps, &cmd) {
                    Ok(level) => {
                        led.set_level(if level { Level::High } else { Level::Low });
                        DeviceMsg::Ack { ok: true, error: None }
                    }
                    Err(e) => DeviceMsg::Ack { ok: false, error: Some(e) },
                },
                Err(_) => DeviceMsg::Ack {
                    ok: false,
                    error: Some(String::from("invalid command json")),
                },
            };
            let out = serde_json::to_string(&reply).unwrap();
            write_line(class, &out).await?;
        }
    }
}

/// Write a JSON string + '\n', chunked to the 64-byte CDC packet size.
async fn write_line<'d, D: embassy_usb::driver::Driver<'d>>(
    class: &mut CdcAcmClass<'d, D>,
    text: &str,
) -> Result<(), EndpointError> {
    let mut bytes = alloc::vec::Vec::with_capacity(text.len() + 1);
    bytes.extend_from_slice(text.as_bytes());
    bytes.push(b'\n');
    for chunk in bytes.chunks(64) {
        class.write_packet(chunk).await?;
    }
    // If the last chunk was exactly 64 bytes, send a zero-length packet so the
    // host sees the transfer boundary.
    if bytes.len() % 64 == 0 {
        class.write_packet(&[]).await?;
    }
    Ok(())
}
```

- [ ] **Step 3: Build the firmware**

Run: `cd firmware && devbox run -- cargo build`
Expected: compiles. Fix any `embassy-usb` API drift against the linked example (constructor arg counts and `CdcAcmClass`/`Builder` signatures are the usual culprits). Re-run until it builds.

- [ ] **Step 4: Lint the firmware**

Run: `cd firmware && devbox run -- cargo clippy -- -D warnings`
Expected: no warnings.

- [ ] **Step 5: Produce the UF2**

Run: `devbox run -- just uf2`
Expected: "Wrote firmware/firmware.uf2".

- [ ] **Step 6: Commit**

```bash
git add firmware/Cargo.toml firmware/src/main.rs
git commit -m "feat(firmware): USB CDC serial — announce SelfId, validate+apply gpio commands"
```

---

## Task 6: Bench bring-up, justfile recipe, and write-ups

**Files:**
- Modify: `justfile`
- Create: `docs/lessons/hardware/writeups/rung-0-resistor.md`, `docs/lessons/hardware/writeups/rung-1-boundary.md`

- [ ] **Step 1: Add a `bridge` recipe to the justfile**

Append to `justfile`:

```make
# Run the host bridge against a real Pico (auto-detects the RP2350 serial port).
# Pass a port explicitly if auto-detect picks the wrong one: `just bridge --port /dev/tty.usbmodemXXXX`.
bridge *ARGS:
    cargo run -p bridge -- {{ARGS}}
```

- [ ] **Step 2: Full acceptance gate for the non-firmware parts**

Run: `devbox run -- just check`
Expected: "check passed" (build all four parts, gen-drift guard, clippy, `cargo test`, `npm test`).

- [ ] **Step 3: Bench bring-up (the rung passes here)**

Manual, with the Pico + LED from Rung 0 on GP15:
1. Flash: hold BOOTSEL, plug in the Pico, drop `firmware/firmware.uf2` on the RP2350 drive (or use the 1200-baud reboot if wired — deferred here).
2. Find the port: `devbox run -- cargo run -p bridge` — it prints the serial path it opened.
3. In another shell: `devbox run -- npm --prefix web run dev`, open the app in Chrome, choose **device (bridge)**.
4. Confirm: `id`/`name` render, a **LED (ch0)** button appears (rendered from the announced capability), clicking it turns the real LED on/off, and `last command: ok ✓` shows.
5. Sanity: switch the picker to **simulator** and confirm the same UI works with no hardware.

Record the result (worked / what broke) in the Rung 1 write-up.

- [ ] **Step 4: Write the Rung 0 deliverable (still outstanding)**

Create `docs/lessons/hardware/writeups/rung-0-resistor.md`:

```markdown
# Rung 0 write-up — why the current-limiting resistor exists

**Wiring:** GP15 → 330Ω → LED anode; LED cathode → GND.

**What the resistor does.** GP15 is a push-pull output: driven high it sources ~3.3V.
An LED is not a resistor — past its forward voltage it behaves like a near-short, so the
current is set by whatever *else* is in the loop. With a 330Ω resistor and an LED forward
drop of ~1.8V, the current is (3.3 − 1.8) / 330 ≈ **4.5 mA** — safe for both the LED and
the GPIO pad.

**Without it.** Nothing limits the loop current; the LED tries to pull far more than the
pin's ~12 mA design limit. Best case the LED burns out; worse case the GPIO pad (or the
whole chip) is damaged. This is Trap 1 — current going where it shouldn't.

**How I verified it (multimeter as debugger).** Before connecting the LED, I measured GP15
as the firmware toggled it and confirmed it swings 0V ↔ 3.3V. Then I measured across the
resistor to confirm current direction and magnitude.
```

(Refine into your own words after re-measuring on the bench.)

- [ ] **Step 5: Write the Rung 1 deliverable**

Create `docs/lessons/hardware/writeups/rung-1-boundary.md`:

```markdown
# Rung 1 write-up — the host↔device boundary and the bridge

**What I built.** The Pico announces `{id, name, capabilities:[{kind:"gpio",...}]}` over USB
serial. A Rust **bridge** owns the serial port and relays to the browser over a WebSocket. The
browser renders an LED control *from the announced capability* and sends back an addressed
command `(capability, channel, op, args)`; the Pico validates it and drives GP15.

**Where the boundary sits.** The browser never touches hardware. It speaks one protocol
(WebSocket) to an API. Behind that API is either the simulator (fabricated) or the bridge (a
real Pico). This is the Adom pattern — physical resources behind a software API — and it means
the sim and the real board are interchangeable peers.

**Framing — and why it only matters on the serial link.** A WebSocket hands you whole messages;
a serial port hands you a raw byte stream with no message boundaries. So on the serial link I
delimit each message with a newline (`\n`); on the WebSocket I don't need to. The bridge's whole
job is translating between the two.

**Plug-and-play via the capability list.** The board describes what it can do; the workbench
draws controls from that description. Nothing in the browser hardcodes "the LED button" — it
draws a control for whatever GPIO the board advertises. At Rung 3 an ADC readout should appear
the same way, from its announcement.

**Result on the bench.** <fill in: did the real LED respond? what surprised me?>
```

- [ ] **Step 6: Commit**

```bash
git add justfile docs/lessons/hardware/writeups
git commit -m "chore(rung-1): bridge recipe + rung 0/1 write-ups"
```

---

## Self-Review Notes

- **Spec coverage:** contract types + shared validator (T1), simulator parity (T2), bridge relay generic-over-stream + tests (T3), web capability-driven control + picker + ack (T4), firmware USB CDC (T5), justfile + write-ups + bench verify (T6), Arche ADR-supersession note recorded in the spec (non-blocking follow-up, not a code task). All spec sections mapped.
- **Framing** taught and implemented only on the serial link (bridge + firmware); WebSocket paths carry one JSON object per frame.
- **DRY:** `resolve_gpio_set` lives once in `contract` and is called by both simulator and firmware.
- **Known risk:** `embassy-usb` API version drift (T5) and `tokio-tungstenite` generic type naming (`relay::pump`, T3) — both flagged inline with fallbacks; neither is on an automated-test path.
```

