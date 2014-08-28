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

mod http_utils;

pub type SolrUpdateResult = Result<SolrUpdateResponse, SolrError>;
pub type SolrQueryResult = Result<SolrQueryResponse, SolrError>;

#[deriving(Show, Decodable)]
pub enum SolrValue {
  SolrF64(f64),
  SolrI64(i64),
  SolrString(String)
}

impl<S: Encoder<E>, E> Encodable<S, E> for SolrValue {
  fn encode(&self, e: &mut S) -> Result<(), E> {
    match *self {
      SolrF64(v) => v.encode(e),
      SolrI64(v) => v.encode(e),
      SolrString(ref v) => v.encode(e)
    }
  }
}

#[deriving(Show)]
pub struct SolrField {
  pub name: String,
  pub value: SolrValue
}

#[deriving(Show)]
pub struct SolrDocument {
  pub fields: Vec<SolrField>
}


impl SolrDocument {
  pub fn new() -> SolrDocument {
    let fields: Vec<SolrField> = Vec::with_capacity(10);
    SolrDocument{fields: fields}
  }

  pub fn add_field(&mut self, name: &str, value: SolrValue) {
    self.fields.push(SolrField{name: name.to_string(), value: value});
  }
}

impl<E, S: Encoder<E>> Encodable<S, E> for SolrDocument {
  fn encode(&self, s: &mut S) -> Result<(), E> {
    let mut i = 0u;
    s.emit_struct("SolrDocument", self.fields.len(), |e| {
      for field in self.fields.iter() {
        try!(e.emit_struct_field(field.name.as_slice(), i, |e| field.value.encode(e)));
        i = i + 1;
      }
      Ok(())
    })
  }
}

impl<E, D: Decoder<E>> Decodable<D, E> for SolrDocument {
  fn decode(d: &mut D) -> Result<SolrDocument, E> {
    d.read_map(|d, len| {
      let mut doc = SolrDocument{fields: Vec::with_capacity(len)};
      for i in range(0u, len) {
        let field_name: String = try!(d.read_map_elt_key(i, Decodable::decode));
        // TODO: match correct SolrValue
        let field_value = SolrString(try!(d.read_map_elt_val(i, Decodable::decode)));
        doc.fields.push(SolrField{name: field_name, value: field_value});
      }
      Ok(doc)
    })
  }
}


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
          message: try!(d.read_struct_field("msg", 0, Decodable::decode)),
          status: try!(d.read_struct_field("code", 1, Decodable::decode)),
          // TODO: implement time parsing from request header
          time: 0})
      })
    })
  }
}

pub struct SolrUpdateResponse {
  pub status: int,
  pub time: int
}

impl<D: Decoder<E>, E> Decodable<D, E> for SolrUpdateResponse {
  fn decode(d: &mut D) -> Result<SolrUpdateResponse, E> {
    d.read_struct("root", 0, |d| {
      d.read_struct_field("responseHeader", 0, |d| {
        Ok(SolrUpdateResponse{
          status: try!(d.read_struct_field("status", 0, Decodable::decode)),
          time: try!(d.read_struct_field("QTime", 1, Decodable::decode))
        })
      })
    })
  }
}

#[deriving(Show)]
pub struct SolrQueryResponse {
  pub status: int,
  pub time: int,
  pub total: int,
  pub start: int,
  pub items: Vec<SolrDocument>
}

impl<D: Decoder<E>, E> Decodable<D, E> for SolrQueryResponse {
  fn decode(d: &mut D) -> Result<SolrQueryResponse, E> {
    let mut resp = SolrQueryResponse{ status: 0, time: 0, total: 0, start: 0, items: Vec::new() };
    d.read_struct("root", 0, |d| {
      d.read_struct_field("responseHeader", 0, |d| {
        resp.status = try!(d.read_struct_field("status", 0, Decodable::decode));
        resp.time = try!(d.read_struct_field("QTime", 1, Decodable::decode));
        Ok(())
      });
      d.read_struct_field("response", 0, |d| {
        resp.total = try!(d.read_struct_field("numFound", 0, Decodable::decode));
        resp.start = try!(d.read_struct_field("start", 1, Decodable::decode));
        d.read_struct_field("docs", 0, |d| {
          d.read_seq(|d, len| {
            println!("NUmber of docs: {}", len);
            for i in range(0u, len) {
              resp.items.push(try!(Decodable::decode(d)));
            }
            Ok(())
          });
          Ok(())
        });
        Ok(())
      });
      Ok(())
    });
    Ok(resp)
  }
}

pub struct SolrQuery {
  query: String
}

impl SolrQuery {
  pub fn new(query: &str) -> SolrQuery {
    SolrQuery{query: query.to_string()}
  }

  pub fn to_pairs(&self) -> Vec<(&str, &str)> {
    vec!(("q", self.query.as_slice()), ("wt", "json"))
  }
}
