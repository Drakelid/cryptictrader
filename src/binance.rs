use futures_util::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use serde::Deserialize;
use crate::orderbook::OrderBook;

#[derive(Debug, Deserialize)]
struct DepthUpdate {
    #[serde(rename = "b")]
    bids: Vec<(String, String)>,
    #[serde(rename = "a")]
    asks: Vec<(String, String)>,
}

pub async fn stream_depth(symbol: &str) -> tokio::io::Result<()> {
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
                            if let Some(imbalance) = ob.imbalance() {
                                println!("Imbalance: {:.4}", imbalance);
                            }
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

