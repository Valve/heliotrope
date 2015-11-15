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
    let solr = SolrClient::new(url);
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
    let solr = SolrClient::new(url);
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
let solr = SolrClient::new(url);
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
let solr = SolrClient::new(url);
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

pub use self::client::SolrClient;
pub use self::document::SolrDocument;
pub use self::query::{SolrQuery, SortClause, SortOrder};
pub use self::request::SolrDeleteRequest;

mod http_utils;
mod document;
mod query;
mod request;
mod response;
mod client;
