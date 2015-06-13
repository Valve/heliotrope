// Copyright 2015 Valentin Vasilyev.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*!

Heliotrope is a Solr client for the [Rust](http://rust-lang.org/) programming language.

It builds with [Cargo](http://crates.io/).
To use it in your project, add this to your `Cargo.toml` file:

```Cargo
heliotrope = "~0.0.3"
```

## Indexing

### Adding new document to solr

```ignore
use url::Url;
use heliotrope::{Solr, SolrDocument};

fn main(){
    let url = Url::parse("http://localhost:8983/solr/test/").unwrap();
    let solr = Solr::new(url);
    let mut document = SolrDocument::new();
    document.add_field("id", "1".to_string());
    document.add_field("type", "Book".to_string());
    document.add_field("title", "How to train your dragon".to_string());
    document.add_field("body", "Vala Morgulis".to_string());
    match solr.add(&document) {
        Ok(solr_response) => println!("{}", solr_response),
        Err(solr_error) => println!("{}", solr_error)
    }
    match solr.commit() {
        Ok(solr_response) => println!("{}", solr_response),
        Err(solr_error) => println!("{}", solr_error)
    }
}
```

### Add and commit in one step

```ignore
use url::{Url};
use heliotrope::{Solr, SolrDocument};

fn main(){
    let url = Url::parse("http://localhost:8983/solr/test/").unwrap();
    let solr = Solr::new(url);
    let mut document = SolrDocument::new();
    document.add_field("id", "2".to_string());
    document.add_field("type", "Book".to_string());
    document.add_field("title", "The Great Gatsby".to_string());
    document.add_field("body", "In my younger and more vulnerable years..".to_string());
    match solr.add_and_commit(&document) {
        Ok(solr_response) => println!("{}", solr_response),
        Err(solr_error) => println!("Status: {}, Message: {}", solr_error.status, solr_error.message)
    }
}
```

### Adding multiple document at once

```ignore
use url::{Url};
use heliotrope::{Solr, SolrDocument};

let url = Url::parse("http://localhost:8983/solr/test/").unwrap();
let solr = Solr::new(url);
let mut document1 = SolrDocument::new();
document1.add_field("id", "3".to_string());
document1.add_field("type", "Book".to_string());
document1.add_field("title", "The Great Gatsby".to_string());
document1.add_field("body", "In my younger and more vulnerable years".to_string());

let mut document2 = SolrDocument::new();
document1.add_field("id", "4".to_string());
document1.add_field("type", "Book".to_string());
document2.add_field("title", "Moby Dick".to_string());
document2.add_field("body", "Call me Ishmael".to_string());

match solr.add_many_and_commit(vec!(&document1, &document2)) {
    Ok(solr_response) => println!("{}", solr_response),
    Err(solr_error) => println!("Status: {}, Message: {}", solr_error.status, solr_error.message)
}
```

## Querying

```ignore
extern crate url;
use url::{Url};
use heliotrope::{Solr, SolrDocument, SolrQuery, Descending};

let url = Url::parse("http://localhost:8983/solr/test/").unwrap();
let solr = Solr::new(url);
let query = SolrQuery::new("*:*").add_sort("year", Descending);
match solr.query(&query) {
    Ok(solr_response) => {
        println!("Status: {}", solr_response.status);
        println!("Time: {}", solr_response.time);
        println!("Total rows found: {}", solr_response.total);
        println!("Offset: {}", solr_response.start);
        for item in solr_response.items.iter() {
            println!("{}", item);
        }
    }
    Err(solr_error) => println!("Status: {}, Message: {}", solr_error.status, solr_error.message)
}
```

### Query options chaining

```ignore
let query = SolrQuery::new("*:*")
    .add_field("score")
    .add_field("*")
    .add_sort("age", Descending)
    .add_filter("type:Person");
```

#### Replacing arguments
Each `add_*` method on SolrQuery, that accepts a single argument, has a corresponding
`set_*` method which accepts a slice of arguments and replaces existing ones.

```ignore
let query = SolrQuery::new("*:*")
    .set_fields(["id", "title", "score"])
    .set_filters(["type:Person", "class:AR"]);
```

### Pagination

```ignore
// getting third page of size 50
let query = SolrQuery::new("manufacturer:Sony").start(100).rows(50);
```

### Delete documents by ID

```ignore
solr.delete_by_id("99");
```

Note that `delete_by_id` commits automatically after every delete request
*/

#![crate_name="heliotrope"]

extern crate rustc_serialize;
extern crate url;
extern crate hyper;

use url::{Url, UrlParser};
use rustc_serialize::{json};
use hyper::error::Error;

pub use document::{SolrValue, SolrField, SolrDocument};
pub use request::{SolrDeleteRequest};
pub use response::{SolrError, SolrUpdateResult, SolrQueryResult, SolrUpdateResponse, SolrQueryResponse, SolrPingResponse};
pub use query::{SolrQuery, SortOrder, SortClause};
pub use http_utils::{HttpResponse, get, post_json};

mod http_utils;
mod document;
mod query;
mod request;
mod response;
mod client;

/// Represents your API connection to Solr.
/// You use this struct to perform operations on Solr.
pub struct Solr {
    // Base URL to connect to Solr. Should include the core.
    // For example http://localhost:8983/solr/production/
    pub base_url: Url,
    select_url: Url,
    update_url: Url,
    commit_url: Url,
    pub ping_url: Url
}

impl Solr {

    fn build_update_url(url: &Url) -> Url{
        let mut url_parser = UrlParser::new();
        url_parser.base_url(url).parse("./update").unwrap()
    }

    fn build_select_url(url: &Url) -> Url {
        let mut url_parser = UrlParser::new();
        url_parser.base_url(url).parse("./select").unwrap()
    }

    fn build_commit_url(url: &Url) -> Url {
        let mut url_parser = UrlParser::new();
        url_parser.base_url(url).parse("./update?commit=true").unwrap()
    }

    fn build_ping_url(url: &Url) -> Url {
        let mut url_parser = UrlParser::new();
        url_parser.base_url(url).parse("./admin/ping?wt=json").unwrap()
    }

    /// Creates a new instance of Solr.
    pub fn new(url: &Url) -> Solr {
        Solr {base_url: url.clone(),
            select_url: Solr::build_select_url(url),
            update_url: Solr::build_update_url(url),
            commit_url: Solr::build_commit_url(url),
            ping_url: Solr::build_ping_url(url)}
    }

    pub fn ping(&self) -> Result<SolrPingResponse, SolrError>  {
        let http_result = http_utils::get(&self.ping_url);
        // TODO `
        match http_result {
            Ok(http_response) => match SolrPingResponse::from_json_str(&http_response.body) {
                Ok(spr) => Ok(spr),
                // TODO: insert actual builder_error inside solr_error
                Err(err) => Err(SolrError{status: 0, time: 0, message: format!("Error parsing ping response JSON: {}", err.message)})
            },
            Err(_) => Err(SolrError{status: 0, time: 0, message: "Network error".to_string()})
        }
    }

    ///// Performs Solr query
    pub fn query(&self, query: &SolrQuery) -> SolrQueryResult {
        let mut query_url = self.select_url.clone();
        query_url.set_query_from_pairs(query.to_pairs().iter().map(|&(ref x, ref y)| (&x[..], &y[..])));
        let http_result = http_utils::get(&query_url);
        handle_http_query_result(http_result)
    }

    // TODO DRY
    ///// Adds new document to Solr, without committing
    pub fn add(&self, document: &SolrDocument) -> SolrUpdateResult {
        self.add_many(&[document])
    }

    ///// Adds new document to Solr and commits it
    pub fn add_and_commit(&self, document: &SolrDocument) -> SolrUpdateResult {
        self.add_many_and_commit(&[document])
    }

    ///// Adds multiple documents to Solr, without committing it
    pub fn add_many(&self, documents: &[&SolrDocument]) -> SolrUpdateResult {
        let raw_json = json::encode(&documents);
        match raw_json {
            Ok(body) => {
                let http_result =  http_utils::post_json(&self.update_url, &body);
                handle_http_update_result(http_result)
            },
            Err(err) => Err(SolrError{status: 0, time: 0, message: "Error serialize solr document to json".to_string()})
        }
    }

    ///// Ads multiple documents to Solr and commits them
    pub fn add_many_and_commit(&self, documents: &[&SolrDocument]) -> SolrUpdateResult {
        let raw_json = json::encode(&documents);
        match raw_json {
            Ok(body) => {
                let http_result =  http_utils::post_json(&self.commit_url, &body);
                handle_http_update_result(http_result)
            },
            Err(err) => Err(SolrError{status: 0, time: 0, message: "Error serialize solr document to json".to_string()})
        }
    }

    ///// Performs Solr commit
    pub fn commit(&self) -> SolrUpdateResult {
        let http_result = http_utils::post_json(&self.commit_url, "");
        handle_http_update_result(http_result)
    }

    ///// Deletes document with the given id
    pub fn delete_by_id(&self, id: &str) -> SolrUpdateResult {
        let delete_request = SolrDeleteRequest::from_id(id);
        let raw_json = json::encode(&delete_request);
        match raw_json {
            Ok(body) => {
                let http_result =  http_utils::post_json(&self.commit_url, &body);
                handle_http_update_result(http_result)
            },
            Err(err) => Err(SolrError{status: 0, time: 0, message: "Error serialize solr document to json".to_string()})
        }
    }
}

fn handle_http_update_result(http_result: Result<HttpResponse, Error>) -> SolrUpdateResult {
    match http_result {
        Ok(response) => {
            match json::decode::<SolrUpdateResponse>(&response.body) {
                Ok(sur) => Ok(sur),
                Err(err) => Err(SolrError{status: 0, time: 0, message: format!("Parse error: {}", err)})
            }
        },
        Err(err) => Err(SolrError{status: 0, time: 0, message: format!("Http error: {}", err)})
    }
}

// TODO add handling http error
fn handle_http_query_result(http_result: Result<HttpResponse, Error>) -> SolrQueryResult {
    match http_result {
        Ok(response) => {
            match SolrQueryResponse::from_json_str(&response.body) {
                Ok(qp) => Ok(qp),
                Err(err) => Err(err)
            }
        },
        Err(err) => Err(SolrError{status: 0, time: 0, message: format!("Http error: {}", err)})
    }
}
