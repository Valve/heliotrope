extern crate heliotrope;

use heliotrope::{SolrQuery, SortClause, Ascending, Descending};

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

#[test]
fn query_and_sort_with_add_sort_to_pairs() {
    let mut query = SolrQuery::new("abba");
    query = query.add_sort("age", Descending);
    assert_eq!(query.to_pairs(),
               vec!(("wt".to_string(), "json".to_string()),
                    ("q".to_string(), "abba".to_string()),
                    ("sort".to_string(), "age desc".to_string())));
}

#[test]
fn query_and_many_sorts_with_add_sort_to_pairs() {
    let mut query = SolrQuery::new("abba");
    query = query.add_sort("age", Descending);
    query = query.add_sort("balance", Ascending);
    assert_eq!(query.to_pairs(),
               vec!(("wt".to_string(), "json".to_string()),
                    ("q".to_string(), "abba".to_string()),
                    ("sort".to_string(), "age desc, balance asc".to_string())));
}

#[test]
fn query_and_many_sorts_with_set_sorts_to_pairs() {
    let mut query = SolrQuery::new("abba");
    let sorts = vec!(SortClause {field: "age".to_string(), order: Descending},
                     SortClause {field: "balance".to_string(), order: Ascending});
    query = query.set_sorts(sorts.as_slice());
    assert_eq!(query.to_pairs(),
               vec!(("wt".to_string(), "json".to_string()),
                    ("q".to_string(), "abba".to_string()),
                    ("sort".to_string(), "age desc, balance asc".to_string())));
}
