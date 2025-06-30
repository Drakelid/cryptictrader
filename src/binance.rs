use crate::orderbook::OrderBook;
use futures_util::StreamExt;
use serde::Deserialize;
use tokio::io::{self, AsyncReadExt};
use tokio::time::{Duration, sleep};
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[derive(Debug, Deserialize)]
struct DepthUpdate {
    #[serde(rename = "b")]
    bids: Vec<(String, String)>,
    #[serde(rename = "a")]
    asks: Vec<(String, String)>,
}

pub async fn stream_depth<F>(symbol: &str, mut on_update: F) -> tokio::io::Result<()>
where
    F: FnMut(&OrderBook) + Send,
{
    let url_str = format!("wss://stream.binance.com:9443/ws/{}@depth", symbol);
    let url = url::Url::parse(&url_str).expect("invalid url");
    println!("Connecting to {}", url);

    match connect_async(url).await {
        Ok((mut ws, _)) => {
            println!("Connected");
            let mut ob = OrderBook::default();
            while let Some(msg) = ws.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Ok(update) = serde_json::from_str::<DepthUpdate>(&text) {
                            for (p, q) in update.bids {
                                if let (Ok(p), Ok(q)) = (p.parse::<f64>(), q.parse::<f64>()) {
                                    ob.update_bid(p, q);
                                }
                            }
                            for (p, q) in update.asks {
                                if let (Ok(p), Ok(q)) = (p.parse::<f64>(), q.parse::<f64>()) {
                                    ob.update_ask(p, q);
                                }
                            }
                            on_update(&ob);
                        }
                    }
                    Ok(Message::Close(_)) => break,
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("stream error: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }

    Ok(())
}

pub async fn stream_depth_offline<F>(path: &str, mut on_update: F) -> io::Result<()>
where
    F: FnMut(&OrderBook) + Send,
{
    let mut file = tokio::fs::File::open(path).await?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).await?;

    let mut ob = OrderBook::default();
    for line in buf.lines() {
        if let Ok(update) = serde_json::from_str::<DepthUpdate>(line) {
            for (p, q) in update.bids {
                if let (Ok(p), Ok(q)) = (p.parse::<f64>(), q.parse::<f64>()) {
                    ob.update_bid(p, q);
                }
            }
            for (p, q) in update.asks {
                if let (Ok(p), Ok(q)) = (p.parse::<f64>(), q.parse::<f64>()) {
                    ob.update_ask(p, q);
                }
            }
            on_update(&ob);
            sleep(Duration::from_millis(100)).await;
        }
    }
    Ok(())
}
