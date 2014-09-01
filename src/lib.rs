// Copyright 2014 Valentin Vasilyev.
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
[dependencies.heliotrope]
git = "https://github.com/Valve/heliotrope"
```

## Indexing

### Adding new document to solr

```ignore
use url::Url;
use heliotrope::{Solr, SolrDocument, SolrString, SolrI64};

fn main(){
    let url = Url::parse("http://localhost:8983/solr/test/").unwrap();
    let solr = Solr::new(url);
    let mut document = SolrDocument::new();
    document.add_field("id", SolrI64(1));
    document.add_field("type", SolrString("Book".to_string()));
    document.add_field("title", SolrString("How to train your dragon".to_string()));
    document.add_field("body", SolrString("Vala Morgulis".to_string()));
    match solr.add(&document) {
        Ok(solr_response) => println!("{:?}", solr_response),
        Err(solr_error) => println!("{:?}", solr_error)
    }
    match solr.commit() {
        Ok(solr_response) => println!("{:?}", solr_response),
        Err(solr_error) => println!("{:?}", solr_error)
    }
}
```

### Add and commit in one step

```ignore
use url::{Url};
use heliotrope::{Solr, SolrDocument, SolrString, SolrI64};

fn main(){
    let url = Url::parse("http://localhost:8983/solr/test/").unwrap();
    let solr = Solr::new(url);
    let mut document = SolrDocument::new();
    document.add_field("id", SolrI64(2));
    document.add_field("type", SolrString("Book".to_string()));
    document.add_field("title", SolrString("The Great Gatsby".to_string()));
    document.add_field("body", SolrString("In my younger and more vulnerable years..".to_string()));
    match solr.add_and_commit(&document) {
        Ok(solr_response) => println!("{:?}", solr_response),
        Err(solr_error) => println!("Status: {}, Message: {}", solr_error.status, solr_error.message)
    }
}
```

### Adding multiple document at once

```ignore
use url::{Url};
use heliotrope::{Solr, SolrDocument, SolrString, SolrI64};

let url = Url::parse("http://localhost:8983/solr/test/").unwrap();
let solr = Solr::new(url);
let mut document1 = SolrDocument::new();
document1.add_field("id", SolrI64(3));
document1.add_field("type", SolrString("Book".to_string()));
document1.add_field("title", SolrString("The Great Gatsby".to_string()));
document1.add_field("body", SolrString("In my younger and more vulnerable years".to_string()));

let mut document2 = SolrDocument::new();
document1.add_field("id", SolrI64(4));
document1.add_field("type", SolrString("Book".to_string()));
document2.add_field("title", SolrString("Moby Dick".to_string()));
document2.add_field("body", SolrString("Call me Ishmael".to_string()));

match solr.add_many_and_commit(vec!(&document1, &document2)) {
    Ok(solr_response) => println!("{:?}", solr_response),
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
Each `add_*` method on SolrQuery, that accepts a single argument has a corresponding
`set_*` method which accepts a slice of arguments and replaces existing ones.

### Pagination

```ignore
// getting third page of size 50
let query = SolrQuery::new("manufacturer:Sony").start(100).rows(50);
```
*/

#![crate_name="heliotrope"]

extern crate serialize;
extern crate url;
extern crate http;
extern crate debug;

use std::io::IoResult;
use url::{Url, UrlParser};
use serialize::{json, Decodable};
use serialize::json::Decoder as JsonDecoder;
use serialize::json::{DecoderError};
use http_utils::HttpResponse;

pub use document::{SolrField, SolrDocument, SolrValue, SolrF64, SolrI64, SolrString};
pub use response::{SolrError, SolrUpdateResult, SolrQueryResult, SolrUpdateResponse, SolrQueryResponse};
pub use query::{SolrQuery, SortOrder, Ascending, Descending, SortClause};

mod http_utils;
mod document;
mod query;
mod response;

/// Represents your API connection to Solr.
/// You will use this struct to perform operations on Solr.
pub struct Solr {
    /// Base URL to connect to Solr. Should include the core.
    /// For example http://localhost:8983/solr/production/
    pub base_url: Url,
    select_url: Url,
    update_url: Url,
    commit_url: Url
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

    /// Creates a new instance of Solr.
    pub fn new(url: &Url) -> Solr {
        Solr {base_url: url.clone(),
            select_url: Solr::build_select_url(url),
            update_url: Solr::build_update_url(url),
            commit_url: Solr::build_commit_url(url)}
    }

    /// Adds new document to Solr, without committing
    pub fn add(&self, document: &SolrDocument) -> SolrUpdateResult {
        self.add_many([document])
    }

    /// Adds new document to Solr and commits it
    pub fn add_and_commit(&self, document: &SolrDocument) -> SolrUpdateResult {
        self.add_many_and_commit([document])
    }

    /// Adds multiple documents to Solr, without committing it
    pub fn add_many(&self, documents: &[&SolrDocument]) -> SolrUpdateResult {
        let raw_json = json::encode(&documents);
        let http_result =  http_utils::post_json(&self.update_url, raw_json.as_slice());
        handle_http_result(http_result)
    }

    /// Ads multiple documents to Solr and commits them
    pub fn add_many_and_commit(&self, documents: &[&SolrDocument]) -> SolrUpdateResult {
        let raw_json = json::encode(&documents);
        println!("{}", raw_json);
        let http_result =  http_utils::post_json(&self.commit_url, raw_json.as_slice());
        handle_http_result(http_result)
    }

    /// Performs Solr commit
    pub fn commit(&self) -> SolrUpdateResult {
        let http_result = http_utils::post(&self.commit_url);
        handle_http_result(http_result)
    }

    /// Performs Solr query
    pub fn query(&self, query: &SolrQuery) -> SolrQueryResult {
        let mut query_url = self.select_url.clone();
        query_url.set_query_from_pairs(query.to_pairs().iter().map(|&(ref k, ref v)| (k.as_slice(),v.as_slice())));
        let http_result = http_utils::get(&query_url);
        handle_http_result(http_result)
    }
}

fn handle_http_result<R: Decodable<JsonDecoder, DecoderError>>(result: IoResult<HttpResponse>) -> Result<R, SolrError> {
    match result {
        Ok(http_response) => {
            match http_response.code {
                200 => {
                    let response: R = json::decode(http_response.body_str().unwrap()).unwrap();
                    Ok(response)
                },
                _ => {
                    let error: SolrError = json::decode(http_response.body_str().unwrap()).unwrap();
                    Err(error)
                }
            }
        },
        Err(err) => {
            Err(SolrError{status: 0, time: 0, message: err.desc.to_string()})
        }
    }
}

