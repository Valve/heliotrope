extern crate rustc_serialize;
extern crate heliotrope;
extern crate url;
extern crate hyper;

use rustc_serialize::json::Json;
use heliotrope::{HttpResponse, get};
use url::Url;
use hyper::status::StatusCode;

#[test]
fn test_ping() {
    let ping_url = "http://localhost:8983/solr/test/admin/ping?wt=json";
    let url: Url = Url::parse(ping_url).unwrap();

    let res = get(&url).unwrap();
    assert_eq!(StatusCode::Ok, res.status);
    let row_json = Json::from_str(&res.body).unwrap();
    assert_eq!("OK".to_string(),
        row_json.as_object().unwrap().get("status").unwrap().as_string().unwrap());
}
