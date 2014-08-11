// Copyright 2013-2014 Valentin Vasilyev.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name="heliotrope"]

extern crate serialize;
extern crate url;
extern crate http;

use url::{Url, UrlParser};
use serialize::{json, Encodable, Encoder};
use http::client::RequestWriter;
use http::method::Post;

pub struct Document<'a>{
  fields: Vec<(&'a str, &'a str)>
}

impl<'a> Document<'a> {
  pub fn new() -> Document<'a> {
    let fields: Vec<(&'a str, &'a str)> = Vec::with_capacity(10);
    Document{fields: fields}
  }

  pub fn add_field(&mut self, name: &'a str, value: &'a str) {
    self.fields.push((name, value));
  }
}

impl<'a, E, S: Encoder<E>> Encodable<S, E> for Document<'a> {
  fn encode(&self, s: &mut S) -> Result<(), E> {
    s.emit_seq(self.fields.len(), |e| {
      e.emit_map(self.fields.len(), |e| {
        let mut i = 0;
        for &(ref k, ref v) in self.fields.iter() {
          try!(e.emit_map_elt_key(i, |e| k.encode(e)));
          try!(e.emit_map_elt_val(i, |e| v.encode(e)));
          i += 1;
        }
        Ok(())
      });
    Ok(())
    })
  }
}

pub struct HttpServer {
  pub url: Url
}

impl HttpServer {
  pub fn new(url: Url) -> HttpServer {
    HttpServer {url: url}
  }

  pub fn add(&self, document: &Document) {
    let raw_json = json::encode(&document);
    let mut req: RequestWriter = RequestWriter::new(Post, update_url(&self.url)).unwrap();
    req.headers.insert_raw("Content-Type".to_string(), b"application/json");
    req.headers.content_length = Some(raw_json.len());
    req.write(raw_json.into_bytes().as_slice());
    let mut resp = match req.read_response(){
      Ok(resp) => resp,
      Err(_req) => fail!("No response available")
    };

    println!("Response status: {}", resp.status);

    let body = match resp.read_to_end(){
      Ok(body) => body,
      Err(err) => fail!("Error reading response: {}", err)
    };
  }

  pub fn add_many(&self, documents: &[Document]) {
  }


  pub fn commit(&self) {
    let url = commit_url(&update_url(&self.url));
    let mut req: RequestWriter = RequestWriter::new(Post, url).unwrap();
    req.headers.insert_raw("Content-Type".to_string(), b"application/json");
    // post commit to the server
    let mut resp = match req.read_response(){
      Ok(resp) => resp,
      Err(_req) => fail!("No response available")
    };

    println!("Response status: {}", resp.status);

    let body = match resp.read_to_end(){
      Ok(body) => body,
      Err(err) => fail!("Error reading response: {}", err)
    };
  }
}

fn update_url(url: &Url) -> Url{
  let mut url_parser = UrlParser::new();
  url_parser.base_url(url).parse("./update").unwrap()
}

fn commit_url(url: &Url) -> Url {
  let mut url_parser = UrlParser::new();
  url_parser.base_url(url).parse("./update?commit=true").unwrap()
}

