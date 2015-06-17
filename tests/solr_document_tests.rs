extern crate rustc_serialize;
extern crate heliotrope;

use rustc_serialize::json;
use heliotrope::{SolrDocument};

#[test]
fn empty_document_to_json(){
    let document = SolrDocument::new();
    let json = json::encode(&document);
    assert_eq!(&json.unwrap().to_string(), "{}");
}

#[test]
fn document_with_one_field_to_json(){
    let mut document = SolrDocument::new();
    document.add_field("name", "Kvothe");
    let json = json::encode(&document);
    assert_eq!(json.unwrap().to_string(), r#"{"name":"Kvothe"}"#);
}

#[test]
fn document_with_three_field_to_json(){
    let mut document = SolrDocument::new();
    document.add_field("name", "Kvothe");
    document.add_field("hobby", "Lute");
    document.add_field("friend", "Denna");
    let json = json::encode(&document);
    assert_eq!(json.unwrap().to_string(), r#"{"name":"Kvothe","hobby":"Lute","friend":"Denna"}"#);
}

#[test]
fn adding_a_field_really_adds_it(){
    let mut document = SolrDocument::new();
    document.add_field("name", "Kvothe");
    assert_eq!(document.fields.len(), 1);
}

