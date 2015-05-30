extern crate rustc_serialize;
extern crate heliotrope;
extern crate url;
extern crate hyper;

use rustc_serialize::json::Json;
use heliotrope::{HttpResponse, get, post_json};
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

#[test]
fn test_get_documents() {
    let docs = get_all_docs();
    assert!(docs.len() >= 0);
}

#[test]
fn test_create_document() {
    delete_all();
    let docs = get_all_docs();  
    assert_eq!(0, docs.len());

    let update_url = "http://localhost:8983/solr/test/update?&wt=json&commit=true";
    let url: Url = Url::parse(update_url).unwrap();
    let res = post_json(&url, "{add: {doc: {id: \"999\"}}}").unwrap();
    assert_eq!(StatusCode::Ok, res.status);
    
    let docs = get_all_docs();  
    assert_eq!(1, docs.len());
    let doc = &docs[0];
    assert_eq!("999", doc.as_object().unwrap().get("id").unwrap().as_string().unwrap());

    delete_all();
}

fn delete_all() {
    let delete_all_url ="http://localhost:8983/solr/test/update?q=*:*&wt=json&commit=true";
    let delete_url = Url::parse(delete_all_url).unwrap();
    post_json(&delete_url, "{delete: {query: \"*:*\"}}").unwrap();
}

fn get_all_docs() -> Vec<Json> {
    let query_url = "http://localhost:8983/solr/test/select?q=*:*&wt=json";

    let url: Url = Url::parse(query_url).unwrap();
    let res = get(&url).unwrap();
    assert_eq!(StatusCode::Ok, res.status);

    let row_json = Json::from_str(&res.body).unwrap();
    let responce_field = row_json.as_object().unwrap().get("response")
        .unwrap().as_object().unwrap();
    let docs = responce_field.get("docs").unwrap().as_array().unwrap();
    docs.clone()
}
