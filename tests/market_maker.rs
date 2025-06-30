use cryptictrader::market_maker::MarketMaker;

#[test]
fn quote_skew() {
    let mm = MarketMaker::new(5.0, 1.0);
    let (bid, ask) = mm.quote(100.0, 101.0, 0.5);
    assert!(bid > 100.0);
    assert!(ask > 101.0);
}
