use contract::{Command, DeviceMsg, SelfId};
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

/// The canonical self-description this software molecule advertises — sourced from
/// the contract crate so the simulator and the real firmware can't drift.
pub fn self_id() -> SelfId {
    contract::default_self_id()
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
                    DeviceMsg::Ack {
                        ok: true,
                        error: None,
                    }
                }
                Err(e) => DeviceMsg::Ack {
                    ok: false,
                    error: Some(e),
                },
            },
            Err(e) => DeviceMsg::Ack {
                ok: false,
                error: Some(e.to_string()),
            },
        };
        let out = serde_json::to_string(&reply).expect("Ack serializes");
        if ws.send(Message::Text(out)).await.is_err() {
            break;
        }
    }
}
