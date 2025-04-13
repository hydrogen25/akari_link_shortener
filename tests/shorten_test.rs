use std::sync::Once;

use log::info;

static INIT: Once = Once::new();

fn init_logger() {
    INIT.call_once(|| {
        env_logger::builder().is_test(true).init();
    });
}

#[tokio::test]
async fn shorten_test() {
    init_logger();
    let key = std::env::var("key").unwrap();
    let c = akari_link_shortener::WaaAiClient::new(&key);
    let r = c
        .shorten_link_full("https://google.com", None, None)
        .await
        .unwrap();
    info!("{:#?}", r);
}

#[tokio::test]
async fn get_link_test() {
    init_logger();
    let key = std::env::var("key").unwrap();
    let c = akari_link_shortener::WaaAiClient::new(&key);
    let r = c.get_link_info("cNIS ").await.unwrap();
    info!("{:#?}", r);
}
