use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8765";
    let listener = TcpListener::bind(addr).await.expect("bind sim port");
    println!("simulator listening on ws://{addr}");
    simulator::serve(listener).await;
}
