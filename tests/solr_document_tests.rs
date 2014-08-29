extern crate serialize;
extern crate heliotrope;


use serialize::json;
use heliotrope::{SolrDocument, SolrString};

#[test]
fn empty_document_to_json(){
    let document = SolrDocument::new();
    let json = json::encode(&document);
    assert_eq!(json.as_slice(), "{}");
}

#[test]
fn document_with_one_field_to_json(){
    let mut document = SolrDocument::new();
    document.add_field("name", SolrString("Kvothe".to_string()));
    let json = json::encode(&document);
    assert_eq!(json.as_slice(), r#"{"name":"Kvothe"}"#);
}

#[test]
fn document_with_three_field_to_json(){
    let mut document = SolrDocument::new();
    document.add_field("name", SolrString("Kvothe".to_string()));
    document.add_field("hobby", SolrString("Lute".to_string()));
    document.add_field("friend", SolrString("Denna".to_string()));
    let json = json::encode(&document);
    assert_eq!(json.as_slice(), r#"{"name":"Kvothe","hobby":"Lute","friend":"Denna"}"#);
}

#[test]
fn adding_a_field_really_adds_it(){
    let mut document = SolrDocument::new();
    document.add_field("name", SolrString("Kvothe".to_string()));
    assert_eq!(document.fields.len(), 1);
}

