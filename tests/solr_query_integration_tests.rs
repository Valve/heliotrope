extern crate url;
extern crate heliotrope;

use url::Url;
use heliotrope::{SolrClient, SolrQuery};


#[test]
fn simple_query_test() {
    let url = Url::parse("http://localhost:8983/solr/test/").unwrap();
    let client = SolrClient::new(&url);
    let query = SolrQuery::new("*:*");
    match client.query(&query) {
        Ok(query_response) => println!("{:?}", query_response),
        Err(e) => panic!(e.message)
    }
}
