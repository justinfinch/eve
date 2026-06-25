use contract::SelfId;
use futures_util::StreamExt;
use tokio::net::TcpListener;
use tokio_tungstenite::connect_async;

#[tokio::test]
async fn client_receives_one_selfid_frame_on_connect() {
    // Bind an ephemeral port so the test never collides with a running sim.
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(simulator::serve(listener));

    let (mut ws, _resp) = connect_async(format!("ws://{addr}")).await.unwrap();
    let msg = ws.next().await.expect("a frame").expect("ok frame");
    let text = msg.into_text().unwrap();

    // SC-3: the wire bytes parse as SelfId and match what the sim advertises.
    let parsed: SelfId = serde_json::from_str(text.as_str()).unwrap();
    assert_eq!(parsed, simulator::self_id());
    assert!(parsed.capabilities.is_empty());
}
