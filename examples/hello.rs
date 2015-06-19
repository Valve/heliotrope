// This example requires core test and running Solr server on localhost:8983
extern crate heliotrope;
extern crate url;

use heliotrope::{Solr, SolrDocument, SolrQuery};
use url::Url;


fn main(){
    let base_url = "http://localhost:8983/solr/test/";
    let url: Url = Url::parse(base_url).unwrap();
    let client = Solr::new(&url);

    println!("Starting example hello...");
    println!("Removing all documents from core test on Solr {}", url);
    client.delete_by_query("*:*");

    let mut doc = SolrDocument::new();
    doc.add_field("id", "1");
    doc.add_field("city", "London");
    println!("Prepared SolrDocument to insert {:?}", doc);

    client.add_and_commit(&doc);

    let query_all = SolrQuery::new("*:*");
    println!("Getting all documents by query *:*");
    let results = client.query(&query_all);
    if let Ok(resp) = results {
        println!("Retrieved results {:?}", resp);
    }
}
