Heliotrope
==========

## SOLR client for Rust programming language.

### Usage

```rust
extern crate heliotrope;

use heliotrope::{HttpServer, Document};

fn main(){
  let url = Url::parse("http://localhost:8983/solr/test/").unwrap();
  let solr = HttpServer::new(url);
  let mut document = Document::new();
  document.add_field("id", "100");
  document.add_field("type", "Person");
  document.add_field("Name", "How to train your dragon");
  document.add_field("Body", "Vala Morgulis");
  solr.add(&document);
  solr.commit();
}
```
