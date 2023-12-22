mod models;
mod requests;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use requests::{GetDepartureBoardRequest, API_CONFIG};
use reqwest::Client;
use url::Url;

async fn departures_to(from: &str, to: Option<&str>) -> models::Response {
    let client = Client::new();

    let url: Url = GetDepartureBoardRequest::new(from, to).into();
    let key = &API_CONFIG.key;

    println!("GET {}", url.as_str());

    let response = client
        .get(url.to_string())
        .header("x-apikey", key)
        .send()
        .await
        .expect("Unable to send GET request");

    let body_bytes = response.bytes().await.expect("Didn't get bytes");

    // Attempt deserialization
    let result = serde_json::from_slice::<models::Response>(&body_bytes);

    // Handle deserialization result
    match result {
        Ok(body) => body,
        Err(err) => {
            // Deserialization failed, handle the error as needed
            println!("{}", String::from_utf8_lossy(&body_bytes));
            panic!("Failed to deserialize: {}", err);
        }
    }
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    if let Some(query_map) = event.query_string_parameters_ref() {
        let from_crs = query_map.first("from");

        if from_crs.is_none() {
            return Ok(Response::builder()
                .status(400)
                .body("No from query provided, expected from".into())
                .expect("failed to render response"));
        }

        let to_crs = query_map.first("to");

        let departures_response =
            serde_json::to_value(departures_to(from_crs.unwrap(), to_crs).await)
                .expect("Unable to serialise response to JSON!");

        let resp = Response::builder()
            .status(200)
            .header("content-type", "text/json")
            .body(departures_response.to_string().into())
            .map_err(Box::new)?;
        return Ok(resp);
    } else {
        return Ok(Response::builder()
            .status(400)
            .body("No queries provided, expected from and to".into())
            .expect("failed to render response"));
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
