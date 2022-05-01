use reqwest;
use std::collections::HashMap;

fn main() {
    // Create a JSON object with HashMap.
    let mut map = HashMap::new();
    map.insert("msg", "Hello from Reqwests!");

    // Create a new client object
    let client = reqwest::blocking::Client::new();
    let resp = match client.post( "http://httpbin.org/post").json(&map).send() {
        Ok(resp) => resp.text().unwrap(),
        Err(err) => panic!("Error: {}", err)
    };
    println!("{}", resp)
}
