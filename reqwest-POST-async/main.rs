use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // Create a JSON object with HashMap.
    let mut map = HashMap::new();
    map.insert("msg", "Hello from Reqwests!");

    // Create a new client object
    let client = reqwest::Client::new();
    let resp = match client.post("https://httpbin.org/post")
        .json(&map).send().await {
        Ok(resp) => resp.text().await.unwrap(),
        Err(err) => panic!("Error: {}", err)
    };

    println!("{}", resp)
}
