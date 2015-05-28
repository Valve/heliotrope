extern crate rustc_serialize;
extern crate heliotrope;
extern crate url;
extern crate hyper;

use rustc_serialize::json::Json;
use heliotrope::{HttpResponse, get, Solr};
use url::Url;
use hyper::status::StatusCode;

#[test]
fn test_instantiate() {
    let base_url = "http://localhost:8983/solr/test/";
    let url: Url = Url::parse(base_url).unwrap();
    let client = Solr::new(&url);

    let solr_query_result = client.ping();
    match solr_query_result {
    	Ok(resp) => {assert_eq!("OK", resp.ping_status)},
    	Err(e) => {panic!(e.message)}
    }
}
