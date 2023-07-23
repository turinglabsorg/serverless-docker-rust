use lambda_http::{run, service_fn, Body, Error, Request, Response};

// This is the handler function our Lambda will call
async fn handler(_event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    let uri = _event.uri();
    let method = _event.method();
    let headers = _event.headers();
    let body = _event.body();

    // Print the request to stdout, so it will show up in the logs
    println!("uri: {}", uri);
    println!("method: {}", method);
    println!("headers: {:?}", headers);
    println!("body: {:?}", body);

    // Return something that implements IntoResponse.
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body("Hello 🦀 from 🐳!".into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Enable tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();
    // Running handler function
    run(service_fn(handler)).await
}