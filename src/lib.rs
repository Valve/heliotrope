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
use serialize::json::Decoder as JsonDecoder;
use serialize::json::{DecoderError};
use http_utils::HttpResponse;

pub use document::{SolrDocument, SolrValue, SolrF64, SolrI64, SolrString};
pub use response::{SolrError, SolrUpdateResult, SolrQueryResult, SolrUpdateResponse, SolrQueryResponse};
pub use query::{SolrQuery};

mod http_utils;
mod document;
mod query;
mod response;

pub struct Solr {
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

  pub fn new(url: &Url) -> Solr {
    Solr {base_url: url.clone(),
      select_url: Solr::build_select_url(url),
      update_url: Solr::build_update_url(url),
      commit_url: Solr::build_commit_url(url)}
  }

  pub fn add(&self, document: &SolrDocument) -> SolrUpdateResult {
    self.add_many([document])
  }

  pub fn add_and_commit(&self, document: &SolrDocument) -> SolrUpdateResult {
    self.add_many_and_commit([document])
  }

  pub fn add_many(&self, documents: &[&SolrDocument]) -> SolrUpdateResult {
    let raw_json = json::encode(&documents);
    let http_result =  http_utils::post_json(&self.update_url, raw_json.as_slice());
    handle_http_result(http_result)
  }

  pub fn add_many_and_commit(&self, documents: &[&SolrDocument]) -> SolrUpdateResult {
    let raw_json = json::encode(&documents);
    println!("{}", raw_json);
    let http_result =  http_utils::post_json(&self.commit_url, raw_json.as_slice());
    handle_http_result(http_result)
  }

  pub fn commit(&self) -> SolrUpdateResult {
    let http_result = http_utils::post(&self.commit_url);
    handle_http_result(http_result)
  }

  pub fn query(&self, query: &SolrQuery) -> SolrQueryResult {
    let mut query_url = self.select_url.clone();
    query_url.set_query_from_pairs(query.to_pairs().iter().map(|&(k,v)| (k,v)));
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

