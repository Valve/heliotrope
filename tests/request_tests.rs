extern crate serialize;
extern crate heliotrope;

use serialize::json;
use heliotrope::{SolrDeleteRequest};

#[test]
fn solr_delete_request_to_json() {
    let req = SolrDeleteRequest::from_id("99");
    let json = json::encode(&req);
    assert_eq!(json.as_slice(), r#"{"delete":{"id":"99"}}"#);
}
