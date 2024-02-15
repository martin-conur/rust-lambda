use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

use rust_stemmers::{Algorithm, Stemmer};
use serde::{Deserialize, Serialize};

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Deserialize)]
struct Request {
    phrase: String,
    // for now just English and Spanish available
    language: String,
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
#[derive(Debug)]
struct BadLanguageError;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let phrase = event.payload.phrase;

    //create the stemmer
    let stemmer: Result<Stemmer, BadLanguageError> =
        match event.payload.language.to_lowercase().as_str() {
            "spanish" => Ok(Stemmer::create(Algorithm::Spanish)),
            "english" => Ok(Stemmer::create(Algorithm::English)),
            _ => Err(BadLanguageError),
        };

    
    let stemmed_phrase = phrase.split(" ")
        .map(|c| stemmer.as_ref().unwrap().stem(&c))
        .collect::<Vec<_>>()
        .join(" ");

    // Prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!(
            "Original phrase: {}; Stemmed phrase: {}",
            phrase, stemmed_phrase
        ),
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
