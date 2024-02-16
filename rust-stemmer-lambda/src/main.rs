use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

use rust_stemmers::{Algorithm, Stemmer};
use serde::{Deserialize, Serialize};
use serde_json::json;


#[derive(Deserialize)]
struct Request {
    phrase: String,
    // for now just English and Spanish available
    language: String,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}
#[derive(Debug, Serialize)]
struct BadLanguageError{
    req_id: String,
    msg: String,
}

impl std::error::Error for BadLanguageError {
    // this implementation required `Debug` and `Display` traits
}

impl std::fmt::Display for BadLanguageError {
    /// Display the error struct as a JSON string
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err_as_json = json!(self).to_string();
        write!(f, "{}", err_as_json)
    }
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let phrase = event.payload.phrase;

    //create the stemmer
    let stemmer: Result<Stemmer, BadLanguageError> =
        match event.payload.language.to_lowercase().as_str() {
            "spanish" => Ok(Stemmer::create(Algorithm::Spanish)),
            "english" => Ok(Stemmer::create(Algorithm::English)),
            other @ _ => {
                let lang_error = BadLanguageError {
                    req_id: event.context.request_id,
                    msg: format!("Invalid language parameter, only 'Spanish' and 'English' available, but '{other}' received!"),
                };
                return Err(Box::new(lang_error));
            },
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
