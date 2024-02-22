use lambda_runtime::{run, service_fn, Error, LambdaEvent};
#[allow(unused_imports)]
use reverse_geocoder::{ReverseGeocoder, SearchResult};
use serde::{Deserialize, Serialize};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
    lat: String,
    lon: String,
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let lat = event.payload.lat.parse::<f64>().unwrap();
    let lon = event.payload.lon.parse::<f64>().unwrap();

    let geocoder = ReverseGeocoder::new();
    let coords = (lat, lon);
    let search_result = geocoder.search(coords);

    let distance = search_result.distance;
    let city = search_result.record;

    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("city: {}, distance: {}", city, distance),
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
