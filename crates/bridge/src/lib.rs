pub mod relay;

use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_serial::{SerialPort, SerialPortBuilderExt};
use tokio_tungstenite::tungstenite::Message;

use crate::relay::split_lines;

/// Bind a WebSocket listener and, for each client, relay to the serial port.
pub async fn run(ws_addr: &str, serial_path: &str, baud: u32) -> std::io::Result<()> {
    let listener = TcpListener::bind(ws_addr).await?;
    println!("bridge listening on ws://{ws_addr}, serial {serial_path} @ {baud}");

    loop {
        let (stream, _peer) = listener.accept().await?;
        let ws = match tokio_tungstenite::accept_async(stream).await {
            Ok(ws) => ws,
            Err(_) => continue,
        };
        let mut port = match tokio_serial::new(serial_path, baud).open_native_async() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("failed to open serial {serial_path}: {e}");
                continue;
            }
        };
        // Assert DTR/RTS in case a host-aware firmware gates on them. (Our firmware does
        // NOT — it announces unconditionally — but this is harmless and helps other devices.)
        let _ = port.write_data_terminal_ready(true);
        let _ = port.write_request_to_send(true);
        relay_session(ws, port).await;
    }
}

async fn relay_session<S>(
    ws: tokio_tungstenite::WebSocketStream<S>,
    port: tokio_serial::SerialStream,
) where
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
