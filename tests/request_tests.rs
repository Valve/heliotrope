extern crate rustc_serialize;
extern crate heliotrope;

use rustc_serialize::json;
use heliotrope::{SolrDeleteRequest};

#[test]
fn solr_delete_request_to_json() {
    let req = SolrDeleteRequest::from_id("99");
    let json = json::encode(&req);
    assert_eq!(json.unwrap().to_string(), r#"{"delete":{"id":"99"}}"#);
}
