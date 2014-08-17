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

use heliotrope::{Solr, SolrDocument};

fn main(){
  let url = Url::parse("http://localhost:8983/solr/test/").unwrap();
  let solr = Solr::new(url);
  let mut document = SolrDocument::new();
  document.add_field("id", "100");
  document.add_field("type", "Book");
  document.add_field("title", "How to train your dragon");
  document.add_field("body", "Vala Morgulis");
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
  document.add_field("id", "101");
  document.add_field("type", "Book");
  document.add_field("title", "The Great Gatsby");
  document.add_field("body", "In my younger and more vulnerable years..");
  match solr.add_and_commit(&document) {
    Ok(solr_response) => println!("{:?}", solr_response),
    Err(solr_error) => println!("Status: {}, Message: {}", solr_error.status, solr_error.message)
  }
}
```

#### Adding multiple document at once

```rust
fn main(){
  let url = Url::parse("http://localhost:8983/solr/test/").unwrap();
  let solr = Solr::new(url);
  let mut document1 = SolrDocument::new();
  document1.add_field("id", "101");
  document1.add_field("type", "Book");
  document1.add_field("title", "The Great Gatsby");
  document1.add_field("body", "In my younger and more vulnerable years..");

  let mut document2 = SolrDocument::new();
  document2.add_field("title", "Moby Dick");
  document2.add_field("body", "Call me Ishmael");

  match solr.add_many_and_commit(vec!(&document1, &document2)) {
    Ok(solr_response) => println!("{:?}", solr_response),
    Err(solr_error) => println!("Status: {}, Message: {}", solr_error.status, solr_error.message)
  }
}
```

`Solr#add` and `Solr#commit` return `SolrResult` which is of type
`IoResult<SolrResponse>`.
`SolrResponse` contains `time` and `status` fields.

### Querying

Work in progress..

### Licence

This code is MITlicenced:

Copyright (c) 2014 Valentin Vasilyev

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
