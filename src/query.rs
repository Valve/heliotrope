/// Represents Solr query.
/// You'll need to build the query and pass it to Solr to execute
pub struct SolrQuery {
  query: String
}

impl SolrQuery {
  /// Creates a new SolrQuery with only query term inside it
  pub fn new(query: &str) -> SolrQuery {
    SolrQuery{query: query.to_string()}
  }

  /// Converts this query to a vector of pairs, suitable for URL percent encoding
  pub fn to_pairs(&self) -> Vec<(&str, &str)> {
    vec!(("q", self.query.as_slice()), ("wt", "json"))
  }
}
