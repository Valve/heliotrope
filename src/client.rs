use url::{Url, UrlParser};
use rustc_serialize::{json};
use hyper::error::Error;

use http_utils;
use http_utils::HttpResponse;
use document::SolrDocument;
use query::SolrQuery;
use request::SolrDeleteRequest;
use response::SolrError;
use response::{SolrPingResponse, SolrPingResult};
use response::{SolrQueryResponse, SolrQueryResult};
use response::{SolrUpdateResponse, SolrUpdateResult};

/// Represents your API connection to Solr.
/// You use this struct to perform operations on Solr.
pub struct SolrClient {
    // Base URL to connect to Solr. Should include the core.
    // For example http://localhost:8983/solr/production/
    pub base_url: Url,
    select_url: Url,
    update_url: Url,
    commit_url: Url,
    rollback_url: Url,
    optimize_url: Url,
    pub ping_url: Url
}

impl SolrClient {

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

    fn build_ping_url(url: &Url) -> Url {
        let mut url_parser = UrlParser::new();
        url_parser.base_url(url).parse("./admin/ping?wt=json").unwrap()
    }

    fn build_rollback_url(url: &Url) -> Url {
        let mut url_parser = UrlParser::new();
        url_parser.base_url(url).parse("./update?rollback=true").unwrap()
    }

    fn build_optimize_url(url: &Url) -> Url {
        let mut url_parser = UrlParser::new();
        url_parser.base_url(url).parse("./update?optimize=true").unwrap()
    }

    /// Creates a new instance of Solr.
    pub fn new(url: &Url) -> SolrClient {
        SolrClient {base_url: url.clone(),
            select_url: SolrClient::build_select_url(url),
            update_url: SolrClient::build_update_url(url),
            commit_url: SolrClient::build_commit_url(url),
            ping_url: SolrClient::build_ping_url(url),
            rollback_url: SolrClient::build_rollback_url(url),
            optimize_url: SolrClient::build_optimize_url(url)}
    }

    /// Issues a ping request to check if the server is alive.
    pub fn ping(&self) -> Result<SolrPingResponse, SolrError> {
        let http_result = http_utils::get(&self.ping_url);
        // TODO `
        match http_result {
            Ok(http_response) => match SolrPingResponse::from_json_str(&http_response.body) {
                Ok(spr) => Ok(spr),
                // TODO: insert actual builder_error inside solr_error
                Err(err) => Err(SolrError{status: 0, time: 0, message: format!("Error parsing ping response JSON: {}", err.message)})
            },
            Err(_) => Err(SolrError{status: 0, time: 0, message: "Network error".to_string()})
        }
    }

    /// Performs Solr query
    pub fn query(&self, query: &SolrQuery) -> SolrQueryResult {
        let mut query_url = self.select_url.clone();
        query_url.set_query_from_pairs(query.to_pairs().iter().map(|&(ref x, ref y)| (&x[..], &y[..])));
        let http_result = http_utils::get(&query_url);
        handle_http_query_result(http_result)
    }

    // TODO DRY
    /// Adds new document to Solr, without committing
    pub fn add(&self, document: &SolrDocument) -> SolrUpdateResult {
        self.add_many(&[document])
    }

    /// Adds new document to Solr and commits it
    pub fn add_and_commit(&self, document: &SolrDocument) -> SolrUpdateResult {
        self.add_many_and_commit(&[document])
    }

    /// Adds multiple documents to Solr, without committing it
    pub fn add_many(&self, documents: &[&SolrDocument]) -> SolrUpdateResult {
        let raw_json = json::encode(&documents);
        match raw_json {
            Ok(body) => {
                let http_result =  http_utils::post_json(&self.update_url, &body);
                handle_http_update_result(http_result)
            },
            Err(err) => Err(SolrError{status: 0, time: 0, message: "Error serialize solr document to json".to_string()})
        }
    }

    /// Ads multiple documents to Solr and commits them
    pub fn add_many_and_commit(&self, documents: &[&SolrDocument]) -> SolrUpdateResult {
        let raw_json = json::encode(&documents);
        match raw_json {
            Ok(body) => {
                let http_result =  http_utils::post_json(&self.commit_url, &body);
                handle_http_update_result(http_result)
            },
            Err(err) => Err(SolrError{status: 0, time: 0, message: "Error serialize solr document to json".to_string()})
        }
    }

    /// Performs an explicit commit, causing pending documents to be indexed
    pub fn commit(&self) -> SolrUpdateResult {
        let http_result = http_utils::post_json(&self.commit_url, "");
        handle_http_update_result(http_result)
    }

    /// Performs a rollback of all non-committed documents
    pub fn rollback(&self) -> SolrUpdateResult {
        let http_result = http_utils::post_json(&self.rollback_url, "");
        handle_http_update_result(http_result)
    }

    /// Performs an explicit optimize, causing a merge of all segments to one.
    pub fn optimize(&self) -> SolrUpdateResult {
        let http_result = http_utils::post_json(&self.optimize_url, "");
        handle_http_update_result(http_result)
    }

    /// Deletes a single document by a unique ID
    pub fn delete_by_id(&self, id: &str) -> SolrUpdateResult {
        let delete_request = SolrDeleteRequest::from_id(id);
        let raw_json = json::encode(&delete_request);
        match raw_json {
            Ok(body) => {
                let http_result =  http_utils::post_json(&self.commit_url, &body);
                handle_http_update_result(http_result)
            },
            Err(err) => Err(SolrError{status: 0, time: 0, message: "Error serialize solr document to json".to_string()})
        }
    }

    /// Deletes a list of documents by IDs
    pub fn delete_by_ids(&self, ids: &Vec<String>) -> SolrUpdateResult {
        let delete_request = SolrDeleteRequest::from_ids(&ids);
        let raw_json = json::encode(&delete_request);
        match raw_json {
            Ok(body) => {
                let http_result =  http_utils::post_json(&self.commit_url, &body);
                handle_http_update_result(http_result)
            },
            Err(err) => Err(SolrError{status: 0, time: 0, message: "Error serialize solr document to json".to_string()})
        }
    }

    /// Deletes documents from the index by query
    pub fn delete_by_query(&self, query: &str) -> SolrUpdateResult {
        let delete_request = SolrDeleteRequest::from_query(query);
        let raw_json = json::encode(&delete_request);
        match raw_json {
            Ok(body) => {
                let http_result =  http_utils::post_json(&self.commit_url, &body);
                handle_http_update_result(http_result)
            },
            Err(err) => Err(SolrError{status: 0, time: 0, message: "Error serialize solr document to json".to_string()})
        }
    }
}

fn handle_http_update_result(http_result: Result<HttpResponse, Error>) -> SolrUpdateResult {
    match http_result {
        Ok(response) => {
            match json::decode::<SolrUpdateResponse>(&response.body) {
                Ok(sur) => Ok(sur),
                Err(err) => Err(SolrError{status: 0, time: 0, message: format!("Parse error: {}", err)})
            }
        },
        Err(err) => Err(SolrError{status: 0, time: 0, message: format!("Http error: {}", err)})
    }
}

// TODO add handling http error
fn handle_http_query_result(http_result: Result<HttpResponse, Error>) -> SolrQueryResult {
    match http_result {
        Ok(response) => {
            match SolrQueryResponse::from_json_str(&response.body) {
                Ok(qp) => Ok(qp),
                Err(err) => Err(err)
            }
        },
        Err(err) => Err(SolrError{status: 0, time: 0, message: format!("Http error: {}", err)})
    }
}
