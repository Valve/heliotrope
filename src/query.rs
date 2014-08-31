/// Represents Solr query.
/// You'll need to build the query and pass it to Solr to execute.
/// This struct is immutable, ie returns modified clone of itself when building.
/// This is done to enable chaining.
pub struct SolrQuery {
    query: String,
    fields: Option<Vec<String>>,
    sorts: Option<Vec<String>>
}

impl SolrQuery {
    /// Creates a new SolrQuery with only query term inside it
    pub fn new(query: &str) -> SolrQuery {
        SolrQuery{query: query.to_string(), fields: None, sorts: None}
    }

    /// Adds field (fl) to the list of returned fields
    pub fn add_field(&self, field: &str) -> SolrQuery {
        let mut fields = self.fields.clone();
        fields = match fields {
            Some(mut f) => {
                f.push(field.to_string());
                Some(f)
            },
            None => Some(vec!(field.to_string()))
        };
        SolrQuery{query: self.query.clone(), fields: fields, sorts: self.sorts.clone()}
    }

    /// Sets fields (fl) as the list of returned fields.
    /// The already set fields are overwritten.
    pub fn set_fields(&self, fields: &[&str]) -> SolrQuery {
        let mut new_fields = Vec::with_capacity(fields.len());
        new_fields.extend(fields.iter().map(|x| x.to_string()));
        SolrQuery{query:self.query.clone(), fields: Some(new_fields), sorts:self.sorts.clone()}
    }

    /// Converts this query to a vector of pairs, suitable for URL percent encoding
    pub fn to_pairs(&self) -> Vec<(String, String)> {
        // usually will be wt, q and something else
        let mut vec = Vec::with_capacity(3);
        vec.push(("wt".to_string(), "json".to_string()));
        vec.push(("q".to_string(), self.query.to_string()));
        match self.fields {
            Some(ref f) => {
                let formatted_fields = format!("{:#}", f.clone());
                vec.push(("fl".to_string(), formatted_fields));
            },
            _ => ()
        }
        vec
    }
}
