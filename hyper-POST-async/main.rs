use hyper;

#[tokio::main]
async fn main() ->  Result<(), Box<dyn std::error::Error>>{
    // Create a new client object
    let client = hyper::Client::new();

    // Build out our request
    let req = hyper::Request::builder()
        .method(hyper::Method::POST)
        .uri("http://httpbin.org/post")
        .header("user-agent", "the-awesome-agent/007")
        .header("content-type", "application/json")
        .body(hyper::Body::from(r#"{msg":"This is my body from hyper!}"#))?;

    // Pass our request builder object to our client.
    let resp = client.request(req).await?;

    // Get the response body bytes.
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;

    // Convert the body bytes to utf-8
    let body = String::from_utf8(body_bytes.to_vec()).unwrap();

    println!("{}", body);
    Ok(())

}
