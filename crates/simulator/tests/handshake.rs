use contract::{Command, DeviceMsg};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

async fn next_text(
    ws: &mut (impl StreamExt<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin),
) -> String {
    ws.next()
        .await
        .expect("a frame")
        .expect("ok frame")
        .into_text()
        .unwrap()
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
    let cmd = Command {
        capability: "gpio".into(),
        channel: 0,
        op: "set".into(),
        args: vec![contract::Arg::Bool(true)],
    };
    ws.send(Message::Text(serde_json::to_string(&cmd).unwrap()))
        .await
        .unwrap();
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

    let cmd = Command {
        capability: "gpio".into(),
        channel: 9,
        op: "set".into(),
        args: vec![contract::Arg::Bool(true)],
    };
    ws.send(Message::Text(serde_json::to_string(&cmd).unwrap()))
        .await
        .unwrap();
    let ack = next_text(&mut ws).await;
    match serde_json::from_str::<DeviceMsg>(&ack).unwrap() {
        DeviceMsg::Ack { ok, error } => {
            assert!(!ok);
            assert!(error.is_some());
        }
        other => panic!("expected Ack, got {other:?}"),
    }
}
