<p align="center">
<h2>Heliotrope</h2>
<h3>SOLR client for Rust programming language</h3>
<br/>
<a href="https://travis-ci.org/Valve/heliotrope"><img src="http://img.shields.io/travis/Valve/heliotrope/master.svg?style=flat" /></a>
</p>

## Usage

### Indexing

#### Adding new document to solr

```rust
extern crate heliotrope;

use heliotrope::{Solr, SolrDocument, SolrString, SolrI64};

fn main(){
  let url = Url::parse("http://localhost:8983/solr/test/").unwrap();
  let solr = Solr::new(url);
  let mut document = SolrDocument::new();
  document.add_field("id", SolrI64(1);
  document.add_field("type", SolrString("Book".to_string()));
  document.add_field("title", SolrString("How to train your
dragon".to_string()));
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

#### Add and commit in one step

```rust
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

#### Adding multiple document at once

```rust
let url = Url::parse("http://localhost:8983/solr/test/").unwrap();
let solr = Solr::new(url);
let mut document1 = SolrDocument::new();
document1.add_field("id", SolrI64(3));
document1.add_field("type", SolrString("Book".to_string()));
document1.add_field("title", SolrString("The Great Gatsby".to_string()));
document1.add_field("body", SolrString("In my younger and more vulnerable years..".to_string()));

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

`Solr#add` and `Solr#commit` return `SolrUpdateResult` which is of type
`IoResult<SolrUpdateResponse>`.
`SolrUpdateResponse` contains `time` and `status` fields.

### Querying

```rust
let query = SolrQuery::new("*:*");
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

`Solr#query` returns `SolrQueryResult` which is of type
`IoResult<SolrQueryResponse>`.

#### Work in progress  on other querying features (facets, sorting, highlighting etc)

### Licence

This code is MITlicenced:

Copyright (c) 2014 Valentin Vasilyev

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
