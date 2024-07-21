use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use futures::{StreamExt, SinkExt};

#[tokio::main]
async fn main() {
    let url = "ws://127.0.0.1:8080";
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    println!("Connected to {}", url);

    let (mut write, mut read) = ws_stream.split();

    write.send(Message::Text("Ping".to_string())).await.expect("Error sending message");

    while let Some(msg) = read.next().await {
        let msg = msg.expect("Error reading message");
        println!("Received: {:?}", msg);
        if msg.is_text() || msg.is_binary() {
            write.send(Message::Text("Ping".to_string())).await.expect("Error sending message");
        }
    }
}
