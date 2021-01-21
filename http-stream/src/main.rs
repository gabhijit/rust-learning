use futures_util::StreamExt;
use hyper::{body::Body, client::HttpConnector, Client as HyperClient, Uri};
use hyper_tls::HttpsConnector;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut https_connector = HttpsConnector::new();
    https_connector.https_only(true);

    let https_client: HyperClient<HttpsConnector<HttpConnector>, Body> =
        HyperClient::builder().build(https_connector);

    let response = https_client
        .get(Uri::from_static(
            "https://cdn.kernel.org/pub/linux/kernel/v5.x/linux-5.10.8.tar.xz",
        ))
        .await
        .unwrap();

    println!("Hello, world! {:?}", response);

    let mut body = response.into_body();
    //futures_util::pin_mut!(response);

    let mut chunks: usize = 0;
    while let Some(data) = body.next().await {
        println!(
            "Data: {} {}",
            data.map_err(|e| format!("Error: {}", e)).unwrap().len(),
            chunks
        );
        chunks += 1;
    }
}
