use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{StreamExt};

#[tokio::main]
async fn main() {
    let url = url::Url::parse("wss://stream.binance.com:9443/ws/btcusdt@depth").unwrap();
    println!("Connecting to {}", url);
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected");
    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("{}", text);
            }
            Ok(Message::Binary(_)) => {},
            Ok(Message::Ping(_)) | Ok(Message::Pong(_)) => {},
            Ok(Message::Close(_)) => break,
            Ok(Message::Frame(_)) => {},
            Err(e) => { eprintln!("error: {}", e); break; }
        }
    }
}
