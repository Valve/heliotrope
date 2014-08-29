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
