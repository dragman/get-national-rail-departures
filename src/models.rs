use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Origin {
    #[serde(rename = "locationName")]
    location_name: String,
    crs: String,
    #[serde(rename = "assocIsCancelled")]
    assoc_is_cancelled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Destination {
    #[serde(rename = "locationName")]
    location_name: String,
    crs: String,
    via: Option<String>,
    #[serde(rename = "assocIsCancelled")]
    assoc_is_cancelled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct TrainService {
    #[serde(rename = "futureCancellation")]
    future_cancellation: bool,
    #[serde(rename = "futureDelay")]
    future_delay: bool,
    origin: Vec<Origin>,
    destination: Vec<Destination>,
    std: String,
    etd: String,
    platform: Option<String>,
    operator: String,
    #[serde(rename = "operatorCode")]
    operator_code: String,
    #[serde(rename = "isCircularRoute")]
    is_circular_route: bool,
    #[serde(rename = "isCancelled")]
    is_cancelled: bool,
    #[serde(rename = "filterLocationCancelled")]
    filter_location_cancelled: bool,
    #[serde(rename = "serviceType")]
    service_type: String,
    length: i32,
    #[serde(rename = "detachFront")]
    detach_front: bool,
    #[serde(rename = "isReverseFormation")]
    is_reverse_formation: bool,
    #[serde(rename = "serviceID")]
    service_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Xmlns {
    #[serde(rename = "Count")]
    count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDepBoardWithDetailsResult {
    #[serde(rename = "trainServices")]
    train_services: Vec<TrainService>,
    #[serde(rename = "Xmlns")]
    xmlns: Xmlns,
    #[serde(rename = "generatedAt")]
    generated_at: String,
    #[serde(rename = "locationName")]
    location_name: String,
    crs: String,
    #[serde(rename = "filterLocationName")]
    filter_location_name: Option<String>,
    #[serde(rename = "filtercrs")]
    filter_crs: Option<String>,
    #[serde(rename = "filterType")]
    filter_type: String,
    #[serde(rename = "platformAvailable")]
    platform_available: bool,
    #[serde(rename = "areServicesAvailable")]
    are_services_available: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResult {
    #[serde(rename = "Message")]
    message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    GetDepBoardWithDetailsResult(GetDepBoardWithDetailsResult),
    ErrorResult(ErrorResult),
}

#[cfg(test)]
mod test {
    use crate::models::Response;
    use serde_json::from_str;
    use test_case::test_case;

    #[test_case("long.json")]
    #[test_case("error.json")]
    fn test_parse_data(file: &str) {
        let file_path = format! {"tests/data/{}", file};
        let test_data = std::fs::read_to_string(file_path).expect("Failed to read file!");

        let response: Response = from_str(&test_data).expect("Parsing failed!");
        println!("{:#?}", response)
    }
}
