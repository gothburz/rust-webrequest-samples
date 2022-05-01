#[tokio::main]
async fn main() {
    let resp = match reqwest::get("https://httpbin.org/ip").await {
        Ok(resp) => resp.text().await.unwrap(),
        Err(err) => panic!("Error: {}", err)
    };
    println!("{}", resp)
}
