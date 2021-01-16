use hyper::{Client, Uri};

async fn get_url() {
    let client = Client::new();

    let url = "http://registry-1.docker.io/v2/".parse::<Uri>().unwrap();

    let resp = client.get(url).await.unwrap();

    println!("Response: {}", resp.status());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_get_url() {
        get_url().await;

        assert!(true);
    }
}
