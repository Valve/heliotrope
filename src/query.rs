pub trait ToUrlParam {
    fn to_url_param(&self) -> String;
}

/// Represents Solr query.
/// You'll need to build the query and pass it to Solr to execute.
/// This struct is immutable, ie returns modified clone of itself when building.
/// This is done to enable chaining.
pub struct SolrQuery {
    query: String,
    fields: Option<Vec<String>>,
    filters: Option<Vec<String>>,
    sorts: Option<Vec<SortClause>>
}

impl SolrQuery {
    /// Creates a new SolrQuery with only query term inside it
    pub fn new(query: &str) -> SolrQuery {
        SolrQuery{query: query.to_string(), fields: None, filters: None, sorts: None}
    }

    /// Adds field (l) to the list of returned fields
    pub fn add_field(&self, field: &str) -> SolrQuery {
        let mut fields = self.fields.clone();
        fields = match fields {
            Some(mut f) => {
                f.push(field.to_string());
                Some(f)
            },
            None => Some(vec!(field.to_string()))
        };
        SolrQuery{ query: self.query.clone(),
                   fields: fields,
                   filters: self.filters.clone(),
                   sorts: self.sorts.clone() }
    }

    /// Sets fields (fl) as the list of returned fields.
    /// The already set fields are overwritten.
    pub fn set_fields(&self, fields: &[&str]) -> SolrQuery {
        let mut new_fields = Vec::with_capacity(fields.len());
        new_fields.extend(fields.iter().map(|x| x.to_string()));
        SolrQuery { query:self.query.clone(),
                    fields: Some(new_fields),
                    filters: self.filters.clone(),
                    sorts:self.sorts.clone() }
    }

    /// Adds query filter (fq)
    pub fn add_filter(&self, filter: &str) -> SolrQuery {
        let mut filters = self.filters.clone();
        filters = match filters {
            Some(mut f) => {
                f.push(filter.to_string());
                Some(f)
            },
            None => Some(vec!(filter.to_string()))
        };
        SolrQuery { query: self.query.clone(),
                    fields: self.fields.clone(),
                    filters: filters,
                    sorts: self.sorts.clone() }
    }

    /// Sets query filters (fq)
    /// Already existing filters are overwritten.
    pub fn set_filters(&self, filters: &[&str]) -> SolrQuery {
        let mut new_filters = Vec::with_capacity(filters.len());
        new_filters.extend(filters.iter().map(|x| x.to_string()));
        SolrQuery { query: self.query.clone(),
                    fields: self.fields.clone(),
                    filters: Some(new_filters),
                    sorts: self.sorts.clone() }
    }

    /// Adds sort to query.
    pub fn add_sort(&self, field: &str, order: SortOrder) -> SolrQuery {
        let mut sorts = self.sorts.clone();
        let sort_clause = SortClause{field: field.to_string(), order: order};
        sorts = match sorts {
            Some(mut s) => {
                s.push(sort_clause);
                Some(s)
            },
            None => Some(vec!(sort_clause))
        };
        SolrQuery {
            query: self.query.clone(),
            fields: self.fields.clone(),
            filters: self.filters.clone(),
            sorts: sorts
        }
    }

    /// Sets sorts for fields.
    /// Already existing sorts will be overwritten
    pub fn set_sorts(&self, sorts: &[SortClause]) -> SolrQuery {
        let mut new_sorts = Vec::with_capacity(sorts.len());
        new_sorts.push_all(sorts);
        SolrQuery {
            query: self.query.clone(),
            fields: self.fields.clone(),
            filters: self.filters.clone(),
            sorts: Some(new_sorts) }
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

        match self.filters {
            Some(ref f) => {
                vec.extend(f.iter().map(|x| ("fq".to_string(), x.clone())));
            },
            _ => ()
        }

        match self.sorts {
            Some(ref s) => {
                let sort_url_params: Vec<String> = s.clone()
                                              .iter()
                                              .map(|x| x.to_url_param())
                                              .collect();
                let formatted_sorts = format!("{:#}", sort_url_params);
                vec.push(("sort".to_string(), formatted_sorts));
            },
            _ => ()
        }
        vec
    }
}

/// Represents sort ordering for a field
#[deriving(Clone)]
pub enum SortOrder {
    Ascending,
    Descending
}

impl ToUrlParam for SortOrder {
    fn to_url_param(&self) -> String {
        match *self {
            Ascending => "asc".to_string(),
            Descending => "desc".to_string()
        }
    }
}

/// A utility struct to hold sorting for a field
#[deriving(Clone)]
pub struct SortClause {
    pub field: String,
    pub order: SortOrder
}

impl ToUrlParam for SortClause {
    fn to_url_param(&self) -> String {
        format!("{} {}", self.field, self.order.to_url_param())
    }
}
