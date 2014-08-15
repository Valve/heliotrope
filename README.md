Heliotrope
==========

## SOLR client for Rust programming language.

### Usage


#### Adding new document to solr

```rust
extern crate heliotrope;

use heliotrope::{Solr, Document};

fn main(){
  let url = Url::parse("http://localhost:8983/solr/test/").unwrap();
  let solr = Solr::new(url);
  let mut document = Document::new();
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
  let mut document = Document::new();
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

`Solr#add` and `Solr#commit` return `SolrResult` which is of type
`IoResult<SolrResponse>`.
`SolrResponse` contains `time` and `status` fields.


