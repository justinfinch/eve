use contract::SelfId;
use futures_util::SinkExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

/// The canonical self-description this software molecule advertises.
pub fn self_id() -> SelfId {
    SelfId {
        id: "mol-001".into(),
        name: "Mini-Molecule".into(),
        fw_version: "0.1.0".into(),
        capabilities: Vec::new(),
    }
}

/// Accept WebSocket connections forever; emit one SelfId frame per connection.
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
    let json = serde_json::to_string(&self_id()).expect("SelfId serializes");
    // `.into()` adapts String to whatever the tungstenite Text payload type is.
    let _ = ws.send(Message::Text(json.into())).await;
}
