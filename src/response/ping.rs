use rustc_serialize::{json, Decodable, Decoder};
use rustc_serialize::json::Json;
use response::SolrError;

pub type SolrPingResult = Result<SolrPingResponse, SolrError>;

#[derive(Debug)]
pub struct SolrPingResponse {
    /// HTTP status.
    /// When failed to connect, it will be 0 (zero).
    pub status: u32,
    /// Time it took to execute the request in milliseconds
    pub time: u32,
    /// Ping status
    pub ping_status: String
}

impl SolrPingResponse {
    /// Deserializes SolrPingResponse from JSON string
    pub fn from_json_str(json_str: &str) -> Result<SolrPingResponse, SolrError> {
        let mut response = SolrPingResponse{status: 0, time: 0, ping_status: "null".to_string()};
        let mut error: String = "".to_string();
        match Json::from_str(json_str) {
            Ok(json) => match json {
               Json::Object(tree_map) => {
                    match tree_map.get(&"status".to_string()){
                        Some(st) => response.ping_status = st.as_string().unwrap().to_string(),
                        None => error = "SolrPingResponse JSON parsing error: ping status not found".to_string()
                    }
                    match tree_map.get(&"responseHeader".to_string()) {
                        Some(rh) => {
                            match rh.find("QTime"){
                                Some(time_json) => response.time = time_json.as_i64().unwrap() as u32,
                                None => error = "SolrPingResponse JSON parsing error (responseHeader): QTime not found".to_string()
                            }
                            match rh.find("status") {
                                Some(status_json) => response.status = status_json.as_u64().unwrap() as u32,
                                None => error = "SolrPingResponse JSON parsing error (responseHeader): status not found".to_string()
                            }
                            // TODO add params field
                        },
                        None => error = "SolrPingResponse JSON parsing error: responseHeader not found".to_string()

                    }
               },
               _ => error = "SolrPingResponse JSON parsing error: query response is not a JSON object.".to_string()
            },
            // TODO: verify e type and additional error info
            Err(e) => error = "SolrPingResponse JSON parsing error".to_string()
        }
        if error.len() == 0 {
            Ok(response)
        } else {
            Err(SolrError{time: 0, status: 0, message: error})
        }
    }
}

