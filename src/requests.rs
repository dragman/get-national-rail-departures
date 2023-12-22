use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::env;
use url::Url;

#[derive(Debug)]
pub struct ApiConfig {
    pub url: Url,
    pub key: String,
}

impl ApiConfig {
    fn new() -> Self {
        dotenv().ok();
        ApiConfig {
            url: Url::parse(&env::var("API_URL").expect("No env var API_URL set"))
                .expect("Unable to parse API_URL value"),
            key: env::var("API_KEY").expect("No env var API_KEY set"),
        }
    }
}

trait ApiRequest {
    fn api_config() -> &'static ApiConfig;
}

#[derive(Default)]
pub struct GetDepartureBoardRequest {
    crs: String,
    filter_crs: Option<String>,
    num_rows: Option<i32>,
    filter_type: Option<String>,
    time_offset: Option<i32>,
    time_window: Option<i32>,
}

impl GetDepartureBoardRequest {
    const REQUEST_PATH: &'static str = "GetDepBoardWithDetails";

    pub fn new(crs: &str, filter_crs: Option<&str>) -> Self {
        GetDepartureBoardRequest {
            crs: String::from(crs),
            filter_crs: filter_crs.map(str::to_string),
            ..Default::default()
        }
    }

    fn append_query_param(url: &mut Url, param_name: &str, param_value: Option<impl ToString>) {
        if let Some(value) = param_value {
            url.query_pairs_mut()
                .append_pair(param_name, &value.to_string());
        }
    }
}

impl ApiRequest for GetDepartureBoardRequest {
    fn api_config() -> &'static ApiConfig {
        &API_CONFIG
    }
}

impl Into<Url> for GetDepartureBoardRequest {
    fn into(self) -> Url {
        let mut url = Self::api_config()
            .url
            .join(format!("{}/{}", Self::REQUEST_PATH, &self.crs).as_str())
            .unwrap();

        Self::append_query_param(&mut url, "filterCrs", self.filter_crs);
        Self::append_query_param(&mut url, "numRows", self.num_rows);
        Self::append_query_param(&mut url, "filterType", self.filter_type);
        Self::append_query_param(&mut url, "timeOffset", self.time_offset);
        Self::append_query_param(&mut url, "timeWindow", self.time_window);

        url
    }
}

pub static API_CONFIG: Lazy<ApiConfig> = Lazy::new(|| ApiConfig::new());

#[cfg(test)]
mod test {
    use super::{ApiConfig, GetDepartureBoardRequest};
    use test_case::test_case;
    use url::Url;

    #[test]
    fn can_init_config() {
        let config = ApiConfig::new();
        println!("{:?}", config)
    }

    #[test_case("XXX", None => "https://api1.raildata.org.uk/1010-live-departure-board-dep/LDBWS/api/20220120/GetDepBoardWithDetails/XXX"; "empty filter")]
    #[test_case("XXX", Some("YYY") => "https://api1.raildata.org.uk/1010-live-departure-board-dep/LDBWS/api/20220120/GetDepBoardWithDetails/XXX?filterCrs=YYY"; "with filter")]
    fn can_build_request_url(crs: &str, filter_crs: Option<&str>) -> String {
        let url: Url = GetDepartureBoardRequest::new(crs, filter_crs).into();
        url.to_string()
    }

    #[test]
    fn can_build_request_url_with_num_rows() {
        let mut req = GetDepartureBoardRequest::new("XXX", Some("YYY"));
        req.num_rows = Some(99);

        let url: Url = req.into();
        assert_eq!(url.to_string(), "https://api1.raildata.org.uk/1010-live-departure-board-dep/LDBWS/api/20220120/GetDepBoardWithDetails/XXX?filterCrs=YYY&numRows=99")
    }
}
