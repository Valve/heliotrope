// Copyright 2013-2014 Valentin Vasilyev.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name="heliotrope"]
//#![warn(unstable)]

extern crate serialize;
extern crate url;
extern crate http;
extern crate debug;

use std::io::IoResult;
use url::{Url, UrlParser};
use serialize::{json, Encodable, Encoder, Decodable, Decoder};
use http_utils::HttpResponse;

mod http_utils;

pub type SolrResult = Result<SolrResponse, SolrError>;

pub struct SolrDocument<'a>{
  fields: Vec<(&'a str, &'a str)>
}

impl<'a> SolrDocument<'a> {
  pub fn new() -> SolrDocument<'a> {
    let fields: Vec<(&'a str, &'a str)> = Vec::with_capacity(10);
    SolrDocument{fields: fields}
  }

  pub fn add_field(&mut self, name: &'a str, value: &'a str) {
    self.fields.push((name, value));
  }
}

impl<'a, E, S: Encoder<E>> Encodable<S, E> for SolrDocument<'a> {
  fn encode(&self, s: &mut S) -> Result<(), E> {
    let mut i = 0u;
    s.emit_struct("SolrDocument", self.fields.len(), |e| {
      for &(ref k, ref v) in self.fields.iter() {
        try!(e.emit_struct_field(*k, i, |e| v.encode(e)));
        i = i + 1;
      }
      Ok(())
    })
  }
}


pub struct Solr {
  pub base_url: Url,
  update_url: Url,
  commit_url: Url
}

impl Solr {

  fn build_update_url(url: &Url) -> Url{
    let mut url_parser = UrlParser::new();
    url_parser.base_url(url).parse("./update").unwrap()
  }

  fn build_commit_url(url: &Url) -> Url {
    let mut url_parser = UrlParser::new();
    url_parser.base_url(url).parse("./update?commit=true").unwrap()
  }

  pub fn new(url: &Url) -> Solr {
    Solr {base_url: url.clone(),
      update_url: Solr::build_update_url(url),
      commit_url: Solr::build_commit_url(url)}
  }

  pub fn add(&self, document: &SolrDocument) -> SolrResult {
    self.add_many([document])
  }

  pub fn add_and_commit(&self, document: &SolrDocument) -> SolrResult {
    self.add_many_and_commit([document])
  }

  pub fn add_many(&self, documents: &[&SolrDocument]) -> SolrResult {
    let raw_json = json::encode(&documents);
    let http_result =  http_utils::post_json(&self.update_url, raw_json.as_slice());
    handle_http_result(http_result)
  }

  pub fn add_many_and_commit(&self, documents: &[&SolrDocument]) -> SolrResult {
    let raw_json = json::encode(&documents);
    println!("{}", raw_json);
    let http_result =  http_utils::post_json(&self.commit_url, raw_json.as_slice());
    handle_http_result(http_result)
  }

  pub fn commit(&self) -> SolrResult {
    let http_result = http_utils::post(&self.commit_url);
    handle_http_result(http_result)
  }

}

fn handle_http_result(result: IoResult<HttpResponse>) -> SolrResult {
  match result {
    Ok(http_response) => {
      match http_response.code {
        200 => {
          let response: SolrResponse = json::decode(http_response.body_str().unwrap()).unwrap();
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

pub struct SolrError {
  pub status: int,
  pub time: int,
  pub message: String
}

impl<D: Decoder<E>, E> Decodable<D, E> for SolrError {
  fn decode(d: &mut D) -> Result<SolrError, E> {
    d.read_struct("root", 0, |d| {
      d.read_struct_field("error", 0, |d| {
        Ok(SolrError{
          message: try!(d.read_struct_field("msg", 0, |d| Decodable::decode(d))),
          status: try!(d.read_struct_field("code", 1, |d| Decodable::decode(d))),
          // TODO: implement time parsing from request header
          time: 0})
      })
    })
  }
}

pub struct SolrResponse {
  pub status: int,
  pub time: int
}

impl<D: Decoder<E>, E> Decodable<D, E> for SolrResponse {
  fn decode(d: &mut D) -> Result<SolrResponse, E> {
    d.read_struct("root", 0, |d| {
      d.read_struct_field("responseHeader", 0, |d| {
        Ok(SolrResponse{
          status: try!(d.read_struct_field("status", 0, |d| Decodable::decode(d))),
          time: try!(d.read_struct_field("QTime", 1, |d| Decodable::decode(d)))
        })
      })
    })
  }
}
