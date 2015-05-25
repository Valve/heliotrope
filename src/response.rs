use serialize::{json, Decodable, Decoder};
use serialize::json::Json;
use document::{SolrDocument, SolrField};
use document::SolrValue;

pub type SolrUpdateResult = Result<SolrUpdateResponse, SolrError>;
pub type SolrQueryResult = Result<SolrQueryResponse, SolrError>;

/// SolrError
pub struct SolrError {
    /// HTTP status.
    /// When failed to connect, it will be 0 (zero).
    pub status: i32,
    /// Time it took to execute the request in milliseconds
    pub time: i32,
    /// Detailed error message
    pub message: String
}

impl Decodable for SolrError {
    fn decode<D: Decoder>(d: &mut D) -> Result<SolrError, D::Error> {
        d.read_struct("root", 0, |d| {
            d.read_struct_field("error", 0, |d| {
                Ok(SolrError{
                    message: try!(d.read_struct_field("msg", 0, Decodable::decode)),
                    status: try!(d.read_struct_field("code", 1, Decodable::decode)),
                    // TODO: implement time parsing from request header
                    time: 0})
            })
        })
    }
}

/// Solr response used for update/indexing/commit operations
#[derive(Debug, Copy, Clone)]
pub struct SolrUpdateResponse {
    /// HTTP status.
    /// When failed to connect, it will be 0 (zero).
    pub status: i32,
    /// Time it took to execute the request in milliseconds
    pub time: i32
}

impl Decodable for SolrUpdateResponse {
    fn decode<D: Decoder>(d: &mut D) -> Result<SolrUpdateResponse, D::Error> {
        d.read_struct("root", 0, |d| {
            d.read_struct_field("responseHeader", 0, |d| {
                Ok(SolrUpdateResponse{
                    status: try!(d.read_struct_field("status", 0, Decodable::decode)),
                    time: try!(d.read_struct_field("QTime", 1, Decodable::decode))
                })
            })
        })
    }
}

/// Solr query response
#[derive(Debug)]
pub struct SolrQueryResponse {
    /// HTTP status.
    /// When failed to connect, it will be 0 (zero).
    pub status: u32,
    /// Time it took to execute the request in milliseconds
    pub time: u32,
    /// Total number of rows found.
    /// Note that this will probably be different from returned subset of rows,
    /// because Solr will always use pagination
    pub total: u64,
    /// Rows offset (zero based)
    pub start: u64,
    /// Current page of found Solr documents
    pub items: Vec<SolrDocument>
}

/* 
Example JSON of query response: 
```ignore
{
  "responseHeader": {
    "status": 0,
    "QTime": 1
  },
  "response": {
    "numFound": 57,
    "start": 0,
    "docs": [
      {
        "id": 1,
        "_version_": "1478235317501689856"
      },
      {
        "id": 3,
        "_version_": "1478235317504835584"
      }
    ]
  }
}
*/
impl SolrQueryResponse {
    /// Deserializes SolrQueryResponse from JSON string
    pub fn from_json_str(json_str: &str) -> SolrQueryResult {
        let mut response = SolrQueryResponse{status: 0, time: 0, total: 0, start: 0, items: Vec::new()};
        let mut error: String = "".to_string();
        match json::from_str(json_str) {
            Ok(json) => match json {
               Json::Object(tree_map) => {
                    match tree_map.get(&"responseHeader".to_string()) {
                        Some(rh) => {
                            match rh.find("QTime"){
                                Some(time_json) => response.time = time_json.as_i64().unwrap() as u32,
                                None => error = "SolrQueryResponse JSON parsing error (responseHeader): QTime not found".to_string()
                            }
                            match rh.find("status") {
                                Some(status_json) => response.status = status_json.as_u64().unwrap() as u32,
                                None => error = "SolrQueryResponse JSON parsing error (responseHeader): status not found".to_string()
                            }
                        },
                        None => error = "SolrQueryResponse JSON parsing error: responseHeader not found".to_string()

                    }
                    match tree_map.get(&"response".to_string()) {
                        Some(rs) => {
                            match rs.find("numFound"){
                                Some(total_json) => response.total = total_json.as_u64().unwrap(),
                                None => error = "SolrQueryResponse JSON parsing error (response): numFound not found".to_string()
                            }
                            match rs.find("start") {
                                Some(start_json) => response.start = start_json.as_u64().unwrap(),
                                None => error = "SolrQueryResponse JSON parsing error (response): start not found".to_string()
                            }
                            match rs.find("docs"){
                                Some(docs_json) => {
                                    match docs_json {
                                        & Json::Array(ref docs) => {
                                            for doc_json in docs.iter() {
                                                match SolrQueryResponse::parse_doc(doc_json){
                                                    Ok(doc) => response.items.push(doc),
                                                    Err(e) => error = e
                                                }

                                            }
                                        },
                                        _ =>  error = "SolrQueryResponse JSON parsing error (response): docs is not a JSON list".to_string()
                                    }
                                },
                                None => error = "SolrQueryResponse JSON parsing error (response): docs not found".to_string()
                            }
                        },
                        None => error = "SolrQueryResponse JSON parsing error: response not found".to_string()
                    }
               },
               _ => error = "SolrQueryResponse JSON parsing error: query response is not a JSON object.".to_string()
            },
            // TODO: verify e type and additional error info
            Err(e) => error = "SolrQueryResponse JSON parsing error".to_string()
        }
        if error.len() == 0 {
            Ok(response)
        } else {
            Err(SolrError{time: 0, status: 0, message: error})
        }
    }

    fn parse_doc(doc_json: &Json) -> Result<SolrDocument, String> {
        match doc_json {
            & Json::Object(ref tm) => {
                let mut doc = SolrDocument{fields: Vec::with_capacity(tm.len())};
                for (k, json_v) in tm.iter() {
                    let v = match json_v {
                        & Json::I64(i64) => SolrValue::I64(i64),
                        & Json::U64(u64) => SolrValue::U64(u64),
                        & Json::F64(f64) => SolrValue::F64(f64),
                        & Json::String(ref string) => SolrValue::String(string.clone()),
                        & Json::Boolean(bool) => SolrValue::Boolean(bool),
                        _ => SolrValue::Null
                    };
                    doc.fields.push(SolrField{name: k.clone(), value: v});
                }
                Ok(doc)
            },
            _ => Err("SolrQueryResponse JSON parsing error (response => docs): doc is not an object".to_string())
        }
    }
}
