mod models;
use dotenv::dotenv;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use reqwest::Client;
use serde_json::Value;
use serde_xml_rs::from_str;
use std::env;

async fn departures_to(from: &str, to: Option<&str>) -> Value {
    dotenv().ok();

    let client = Client::new();
    let key = env::var("API_KEY").unwrap();
    let endpoint_url = env::var("API_URL").unwrap();

    // Build the SOAP request payload
    let mut soap_request = format!(
        r#"
        <soap-env:Envelope xmlns:soap-env="http://schemas.xmlsoap.org/soap/envelope/">
        <soap-env:Header>
          <ns0:AccessToken xmlns:ns0="http://thalesgroup.com/RTTI/2013-11-28/Token/types">
            <ns0:TokenValue>{}</ns0:TokenValue>
          </ns0:AccessToken>
        </soap-env:Header>
        <soap-env:Body>
          <ns0:GetDepartureBoardRequest xmlns:ns0="http://thalesgroup.com/RTTI/2021-11-01/ldb/">
            <ns0:numRows>10</ns0:numRows>
            <ns0:crs>{}</ns0:crs>
    "#,
        key, from
    );

    // Optionally add filterCrs
    if let Some(to_crs) = to {
        let filter_crs = format!(r#"<ns0:filterCrs>{}</ns0:filterCrs>"#, to_crs);
        soap_request.push_str(&filter_crs)
    }

    let soap_request_end: String = String::from(
        r#"
        </ns0:GetDepartureBoardRequest>
        </soap-env:Body>
        </soap-env:Envelope>
    "#,
    );

    soap_request.push_str(&soap_request_end);

    let response = client
        .post(endpoint_url)
        .header("Content-Type", "text/xml")
        .body(soap_request)
        .send()
        .await;

    let response_text = response.unwrap().text().await.unwrap();

    let soap_response: models::SoapResponse =
        from_str(&response_text).expect("Failed to deserialise soap response");

    serde_json::to_value(&soap_response.body).unwrap()
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

        let departures_response = departures_to(from_crs.unwrap(), to_crs).await;

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

#[cfg(test)]
mod test {
    use serde_xml_rs::from_str;

    use crate::models::SoapResponse;

    use test_case::test_case;

    #[test_case("foh.xml")]
    #[test_case("example.xml")]
    #[test_case("soap_error.xml")]
    fn test_parse_data(file: &str) {
        let file_path = format! {"tests/data/{}", file};
        let test_data = std::fs::read_to_string(file_path).expect("Failed to read file!");

        let response: SoapResponse = from_str(&test_data).expect("Parsing failed!");
        println!("{:#?}", response)
    }
}
