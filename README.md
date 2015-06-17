<p align="center">
<h2>Heliotrope</h2>
<h3>SOLR client for Rust programming language</h3>
<br/>
<a href="https://travis-ci.org/Valve/heliotrope"><img src="https://travis-ci.org/Valve/heliotrope.svg?branch=unstable" /></a>
<a href="https://crates.io/crates/heliotrope"><img src="https://img.shields.io/crates/v/heliotrope.svg"/></a>
<a href="https://crates.io/crates/heliotrope"><img src="https://img.shields.io/crates/d/heliotrope.svg"/></a>
</p>


```toml
[dependencies]
heliotrope = "*"
```

### [Documentation](http://valve.github.io/heliotrope/heliotrope/index.html)


#### TODO

* facets
* deleting docs
* highlighting
* etc

#### Try it!

```bash
git clone https://github.com/Valve/heliotrope
cd heliotrope
cargo run --example hello
```

#### Usage

Add dependency to your project by Cargo.toml

```toml
[dependencies]
heliotrope = "*"
```

```rust
extern crate heliotrope;
extern crate url;

use heliotrope::{Solr, SolrDocument, SolrQuery};
use url::Url;


fn main(){
    let base_url = "http://localhost:8983/solr/test/";
    let url: Url = Url::parse(base_url).unwrap();
    let client = Solr::new(&url);

    let doc = SolrDocument::new();
    doc.add_field("id", "1");
    doc.add_field("city", "London");

    client.add_and_commit(&doc);

    let query_all = SolrQuery::new("*:*");
    let results = client.query(&query_all);
    if let Ok(resp) = results {
        println!("Retreived results {:?}", resp);
    }
}
```

### Licence

Copyright 2015 Valentin Vasilyev, Dzmitry Misiuk

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
option. This file may not be copied, modified, or distributed
except according to those terms.
