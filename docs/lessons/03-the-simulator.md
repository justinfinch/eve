# Lesson 3 — The Simulator (a software molecule)

## What you'll learn

How a small Rust program pretends to be a hardware board: it listens on a network port,
and whenever the browser connects, it sends one `SelfId` message. You'll meet `tokio`
(async Rust) and WebSockets along the way.

## The concept

Eventually a real microcontroller will announce itself. But you can't develop a browser
UI against a chip you haven't flashed yet. So the **simulator** is a stand-in: a normal
program running on your laptop that behaves, on the network, exactly like the real board
will. It speaks the same message (`SelfId` from [Lesson 2](02-the-contract.md)) over the
same channel (a WebSocket).

Two new ideas:

- **WebSocket** — a long-lived, two-way connection between a browser and a server, built
  on top of HTTP. Unlike a normal web request (ask once, get one response), a WebSocket
  stays open so either side can send messages at any time. Perfect for a board streaming
  live data. The simulator listens at `ws://127.0.0.1:8765`.
- **Async / `tokio`** — networking means a lot of waiting (for connections, for data).
  Rust's `async`/`await` lets one program juggle many connections without blocking.
  **tokio** is the runtime that drives that. You'll see `async fn` and `.await`.

## Walk the code

The simulator is two files plus a test. Logic lives in the **library** (`lib.rs`); the
**binary** (`main.rs`) is a thin wrapper that starts it.

### `crates/simulator/src/lib.rs`

```rust
use contract::SelfId;
use futures_util::SinkExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;
```

It imports `SelfId` straight from the `contract` crate — that's the SSOT in action.
`tokio_tungstenite` is the WebSocket library; `tungstenite` is the underlying protocol
implementation.

```rust
/// The canonical self-description this software molecule advertises.
pub fn self_id() -> SelfId {
    SelfId {
        id: "mol-001".into(),
        name: "Mini-Molecule".into(),
        fw_version: "0.1.0".into(),
        capabilities: Vec::new(),
    }
}
```

`self_id()` returns the one value this molecule advertises. Notice `capabilities` is an
empty `Vec` — the Foundation claims nothing yet. (Compare this to the identical literal
in the firmware in [Lesson 6](06-the-firmware.md): same shape, same values, one source.)

```rust
/// Accept WebSocket connections forever; emit one SelfId frame per connection.
pub async fn serve(listener: TcpListener) {
    while let Ok((stream, _peer)) = listener.accept().await {
        tokio::spawn(handle(stream));
    }
}
```

`serve` is the accept loop. `listener.accept().await` waits for the next client to
connect. `tokio::spawn(handle(stream))` hands that connection off to a background task
so the loop can immediately wait for the *next* client. This is how one program serves
many connections at once.

```rust
async fn handle(stream: TcpStream) {
    let mut ws = match tokio_tungstenite::accept_async(stream).await {
        Ok(ws) => ws,
        Err(_) => return,            // not a valid WebSocket handshake → drop it
    };
    let json = serde_json::to_string(&self_id()).expect("SelfId serializes");
    let _ = ws.send(Message::Text(json)).await;
}
```

For each connection: complete the WebSocket handshake, turn `self_id()` into JSON text
(serde, from [Lesson 2](02-the-contract.md)), and send exactly **one** text frame. Then
the function returns and the connection closes. Simple and deterministic: connect → get
your identity → done.

### `crates/simulator/src/main.rs`

```rust
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8765";
    let listener = TcpListener::bind(addr).await.expect("bind sim port");
    println!("simulator listening on ws://{addr}");
    simulator::serve(listener).await;
}
```

`#[tokio::main]` sets up the async runtime so `main` can be `async`. It binds port
`8765` on localhost and hands the listener to `serve`. That's the whole program — bind
a port, serve forever. The address `127.0.0.1:8765` is exactly what the browser app
connects to in [Lesson 5](05-the-web-workbench.md).

### `crates/simulator/tests/handshake.rs` — the end-to-end test

```rust
#[tokio::test]
async fn client_receives_one_selfid_frame_on_connect() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();  // :0 = any free port
    let addr = listener.local_addr().unwrap();
    tokio::spawn(simulator::serve(listener));

    let (mut ws, _resp) = connect_async(format!("ws://{addr}")).await.unwrap();
    let msg = ws.next().await.expect("a frame").expect("ok frame");
    let text = msg.into_text().unwrap();

    let parsed: SelfId = serde_json::from_str(text.as_str()).unwrap();
    assert_eq!(parsed, simulator::self_id());
    assert!(parsed.capabilities.is_empty());
}
```

This is a true round-trip: it starts the real server on a random free port (`:0` so it
never collides with anything), connects as a client, reads the frame, parses it back
into a `SelfId`, and asserts it equals what the simulator advertises. This is the proof
that the wire bytes are exactly the message — the same guarantee the browser relies on.

### `crates/simulator/Cargo.toml`

```toml
[dependencies]
contract = { path = "../contract", features = ["std"] }   # the SSOT, with std turned on
serde_json = "1"
tokio = { version = "1", features = ["rt-multi-thread", "macros", "net"] }
tokio-tungstenite = "0.24"
futures-util = "0.3"
```

The simulator depends on `contract` with the **`std` feature** on — it runs on your
laptop, so it has the full standard library (unlike the firmware). `tokio` provides the
async runtime; `tokio-tungstenite` adds WebSockets on top.

## Run it yourself

Run the end-to-end test:

```bash
devbox run -- cargo test -p simulator --test handshake
```

You should see `client_receives_one_selfid_frame_on_connect` pass.

Run the actual simulator and poke it by hand:

```bash
devbox run -- cargo run -p simulator
# prints: simulator listening on ws://127.0.0.1:8765
```

Leave that running. It's now waiting for connections — the browser app ([Lesson 5](05-the-web-workbench.md))
is what connects to it. Press `Ctrl-C` to stop.

## Recap

- The **simulator** is a laptop program that behaves on the network like the real board
  will: connect to it, and it sends one `SelfId`.
- It uses **`tokio`** (async runtime) and **WebSockets** to listen on `ws://127.0.0.1:8765`.
- It imports `SelfId` from **`contract`** — the same single source of truth — and the
  handshake test proves the bytes on the wire *are* that message.

**Next:** [Lesson 4 — Codegen](04-codegen.md): how the TypeScript version of `SelfId` is
generated from Rust so the browser and the simulator can never disagree.
</content>
