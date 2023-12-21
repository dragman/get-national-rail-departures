use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Location {
    #[serde(rename = "locationName")]
    location_name: String,
    crs: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Destination {
    location: Location,
}

#[derive(Debug, Deserialize, Serialize)]
struct Origin {
    location: Location,
}

#[derive(Debug, Deserialize, Serialize)]
struct Service {
    std: String,
    etd: String,
    platform: Option<i32>,
    operator: String,
    #[serde(rename = "operatorCode")]
    operator_code: String,
    length: Option<i32>,
    #[serde(rename = "serviceID")]
    service_id: String,
    origin: Vec<Origin>,
    destination: Vec<Destination>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TrainServices {
    #[serde(rename = "service")]
    services: Vec<Service>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StationBoard {
    #[serde(rename = "generatedAt")]
    generated_at: String,
    #[serde(rename = "locationName")]
    location_name: String,
    crs: String,
    #[serde(rename = "trainServices")]
    train_services: TrainServices,
}

#[derive(Debug, Deserialize, Serialize)]
struct Fault {
    faultcode: String,
    faultstring: String,
    detail: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Body {
    GetDepartureBoardResponse {
        #[serde(rename = "GetStationBoardResult")]
        station_board: StationBoard,
    },
    Fault {
        faultcode: String,
        faultstring: String,
        detail: String,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SoapResponse {
    #[serde(rename = "Body")]
    pub body: Body,
}
