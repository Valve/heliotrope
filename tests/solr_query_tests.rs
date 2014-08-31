extern crate heliotrope;

use heliotrope::{SolrQuery};

#[test]
fn query_only_query_to_pairs() {
    let query = SolrQuery::new("abba");
    assert_eq!(query.to_pairs(), vec!(("wt".to_string(), "json".to_string()),
                                      ("q".to_string(), "abba".to_string())));
}

#[test]
fn query_and_field_with_add_field_to_pairs() {
    let mut query = SolrQuery::new("abba");
    query = query.add_field("score");
    assert_eq!(query.to_pairs(),
               vec!(("wt".to_string(), "json".to_string()),
                    ("q".to_string(), "abba".to_string()),
                    ("fl".to_string(), "score".to_string())));
}

#[test]
fn query_and_many_fields_with_add_field_to_pairs() {
    let mut query = SolrQuery::new("abba");
    query = query.add_field("*");
    query = query.add_field("score");
    assert_eq!(query.to_pairs(),
               vec!(("wt".to_string(), "json".to_string()),
                    ("q".to_string(), "abba".to_string()),
                    ("fl".to_string(), "*, score".to_string())));
}

#[test]
fn query_and_many_fields_with_set_fields_to_paris() {
    let mut query = SolrQuery::new("abba");
    query = query.set_fields(&["id", "title"]);
    assert_eq!(query.to_pairs(),
               vec!(("wt".to_string(), "json".to_string()),
                    ("q".to_string(), "abba".to_string()),
                    ("fl".to_string(), "id, title".to_string())));
}

