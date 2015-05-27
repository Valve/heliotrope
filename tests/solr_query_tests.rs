extern crate heliotrope;

use heliotrope::{SolrQuery, SortClause, SortOrder};

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
fn query_and_many_fields_with_set_fields_to_pairs() {
    let mut query = SolrQuery::new("abba");
    query = query.set_fields(&["id", "title"]);
    assert_eq!(query.to_pairs(),
               vec!(("wt".to_string(), "json".to_string()),
                    ("q".to_string(), "abba".to_string()),
                    ("fl".to_string(), "id, title".to_string())));
}

#[test]
fn query_and_filter_with_add_filter_to_pairs() {
    let mut query = SolrQuery::new("abba");
    query = query.add_filter("type:Person");
    assert_eq!(query.to_pairs(),
               vec!(("wt".to_string(), "json".to_string()),
                    ("q".to_string(), "abba".to_string()),
                    ("fq".to_string(), "type:Person".to_string())));
}

#[test]
fn query_and_many_filters_with_add_filter_to_pairs() {
    let mut query = SolrQuery::new("abba");
    query = query.add_filter("type:Person");
    query = query.add_filter("class:ActiveRecord");
    assert_eq!(query.to_pairs(),
               vec!(("wt".to_string(), "json".to_string()),
                    ("q".to_string(), "abba".to_string()),
                    ("fq".to_string(), "type:Person".to_string()),
                    ("fq".to_string(), "class:ActiveRecord".to_string())));
}

#[test]
fn query_and_many_filters_with_set_filters_to_pairs() {
    let mut query = SolrQuery::new("abba");
    query = query.set_filters(&["type:Person", "class:Person"]);
    assert_eq!(query.to_pairs(),
               vec!(("wt".to_string(), "json".to_string()),
                    ("q".to_string(), "abba".to_string()),
                    ("fq".to_string(), "type:Person".to_string()),
                    ("fq".to_string(), "class:Person".to_string())));
}

#[test]
fn query_and_sort_with_add_sort_to_pairs() {
    let mut query = SolrQuery::new("abba");
    query = query.add_sort("age", SortOrder::Descending);
    assert_eq!(query.to_pairs(),
               vec!(("wt".to_string(), "json".to_string()),
                    ("q".to_string(), "abba".to_string()),
                    ("sort".to_string(), "age desc".to_string())));
}

#[test]
fn query_and_many_sorts_with_add_sort_to_pairs() {
    let mut query = SolrQuery::new("abba");
    query = query.add_sort("age", SortOrder::Descending);
    query = query.add_sort("balance", SortOrder::Ascending);
    assert_eq!(query.to_pairs(),
               vec!(("wt".to_string(), "json".to_string()),
                    ("q".to_string(), "abba".to_string()),
                    ("sort".to_string(), "age desc, balance asc".to_string())));
}

#[test]
fn query_and_many_sorts_with_set_sorts_to_pairs() {
    let mut query = SolrQuery::new("abba");
    let sorts = vec!(SortClause {field: "age".to_string(), order: SortOrder::Descending},
                     SortClause {field: "balance".to_string(), order: SortOrder::Ascending});
    query = query.set_sorts(&sorts);
    assert_eq!(query.to_pairs(),
               vec!(("wt".to_string(), "json".to_string()),
                    ("q".to_string(), "abba".to_string()),
                    ("sort".to_string(), "age desc, balance asc".to_string())));
}

#[test]
fn query_and_default_start() {
    let query = SolrQuery::new("abba").start(0);
    assert_eq!(query.to_pairs(),
               vec!(("wt".to_string(), "json".to_string()),
                    ("q".to_string(), "abba".to_string())));
}

#[test]
fn query_and_non_default_start() {
    let query = SolrQuery::new("abba").start(50);
    assert_eq!(query.to_pairs(),
               vec!(("wt".to_string(), "json".to_string()),
                    ("q".to_string(), "abba".to_string()),
                    ("start".to_string(), "50".to_string())));
}

#[test]
fn query_and_default_rows() {
    let query = SolrQuery::new("abba").rows(10);
    assert_eq!(query.to_pairs(),
               vec!(("wt".to_string(), "json".to_string()),
                    ("q".to_string(), "abba".to_string())));
}

#[test]
fn query_and_non_default_rows() {
    let query = SolrQuery::new("abba").rows(25);
    assert_eq!(query.to_pairs(),
               vec!(("wt".to_string(), "json".to_string()),
                    ("q".to_string(), "abba".to_string()),
                    ("rows".to_string(), "25".to_string())));
}

#[test]
fn query_and_custom_start_and_rows() {
    let query = SolrQuery::new("abba").start(125).rows(25);
    assert_eq!(query.to_pairs(),
               vec!(("wt".to_string(), "json".to_string()),
                    ("q".to_string(), "abba".to_string()),
                    ("start".to_string(), "125".to_string()),
                    ("rows".to_string(), "25".to_string())));
}
