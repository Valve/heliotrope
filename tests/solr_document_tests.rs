extern crate serialize;
extern crate heliotrope;


use serialize::json;
use heliotrope::SolrDocument;

#[test]
fn empty_document_to_json(){
  let document = SolrDocument::new();
  let json = json::encode(&document);
  assert_eq!(json.as_slice(), "{}");
}

#[test]
fn document_with_one_field_to_json(){
  let mut document = SolrDocument::new();
  document.add_field("name", "Kvothe");
  let json = json::encode(&document);
  assert_eq!(json.as_slice(), r#"{"name":"Kvothe"}"#);
}

#[test]
fn document_with_three_field_to_json(){
  let mut document = SolrDocument::new();
  document.add_field("name", "Kvothe");
  document.add_field("hobby", "Lute");
  document.add_field("friend", "Denna");
  let json = json::encode(&document);
  assert_eq!(json.as_slice(), r#"{"name":"Kvothe","hobby":"Lute","friend":"Denna"}"#);
}
