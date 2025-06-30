use cryptictrader::orderbook::OrderBook;

#[tokio::test]
async fn imbalance_basic() {
    let mut ob = OrderBook::default();
    ob.update_bid(100.0, 1.0);
    ob.update_ask(101.0, 1.0);
    assert_eq!(ob.imbalance().unwrap(), 0.0);
    ob.update_bid(100.0, 2.0);
    assert!(ob.imbalance().unwrap() > 0.0);
}
