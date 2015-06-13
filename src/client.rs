use SolrError;
use SolrDocument;
use SolrQuery;
use SolrPingResponse;
use SolrUpdateResponse;
use SolrQueryResponse;

pub struct SolrClient;

impl SolrClient {
    fn add(doc: &SolrDocument) -> Result<SolrUpdateResponse, SolrError> {
        unimplemented!()
    }

    fn add_with_commit(doc: &SolrDocument) -> Result<SolrUpdateResponse, SolrError> {
        unimplemented!()
    }

    fn add_many(docs: &[&SolrDocument]) -> Result<SolrUpdateResponse, SolrError> {
        unimplemented!()
    }

    fn add_many_with_commit(docs: &[&SolrDocument]) -> Result<SolrUpdateResponse, SolrError> {
        unimplemented!()
    }

    fn commit() -> Result<SolrUpdateResponse, SolrError> {
        unimplemented!()
    }

    fn delete_by_id(id: &str) -> Result<SolrUpdateResponse, SolrError> {
        unimplemented!()
    }

    fn delete_by_ids(ids: &[&str]) -> Result<SolrUpdateResponse, SolrError> {
        unimplemented!()
    }

    fn delete_by_query(query: &str) -> Result<SolrUpdateResponse, SolrError> {
        unimplemented!()
    }

    fn get_by_id(id: &str) -> Result<SolrDocument, SolrError> {
        unimplemented!()
    }

    fn get_by_ids(ids: &[&str]) -> Result<Vec<SolrDocument>, SolrError> {
        unimplemented!()
    }

    fn optimize() -> Result<SolrUpdateResponse, SolrError> {
        unimplemented!()
    }

    fn ping() -> Result<SolrPingResponse, SolrError> {
        unimplemented!()
    }

    fn query(query: &SolrQuery) -> Result<SolrQueryResponse, SolrError> {
        unimplemented!()
    }

    fn rollback() -> Result<SolrUpdateResponse, SolrError> {
        unimplemented!()
    }
}