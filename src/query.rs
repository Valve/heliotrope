static DEFAULT_START: u64 = 0;
static DEFAULT_ROWS: u32 = 10;

pub trait ToUrlParam {
    fn to_url_param(&self) -> String;
}

/// Represents Solr query.
/// You'll need to build the query and pass it to Solr to execute.
/// This struct is immutable, ie returns modified clone of itself when building.
/// This is done to enable chaining.
#[deriving(Clone)]
pub struct SolrQuery {
    query: String,
    fields: Option<Vec<String>>,
    filters: Option<Vec<String>>,
    sorts: Option<Vec<SortClause>>,
    start: u64,
    rows: u32
}

impl SolrQuery {
    /// Creates a new SolrQuery with only query term inside it
    pub fn new(query: &str) -> SolrQuery {
        SolrQuery { query: query.to_string(),
            fields: None,
            filters: None,
            sorts: None,
            start: 0,
            rows: DEFAULT_ROWS }

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
        let mut solr_query = self.clone();
        solr_query.fields = fields;
        solr_query
    }

    /// Sets fields (fl) as the list of returned fields.
    /// The already set fields are overwritten.
    pub fn set_fields(&self, fields: &[&str]) -> SolrQuery {
        let mut new_fields = Vec::with_capacity(fields.len());
        new_fields.extend(fields.iter().map(|x| x.to_string()));
        let mut solr_query = self.clone();
        solr_query.fields = Some(new_fields);
        solr_query
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
        let mut solr_query = self.clone();
        solr_query.filters = filters;
        solr_query
    }

    /// Sets query filters (fq)
    /// Already existing filters are overwritten.
    pub fn set_filters(&self, filters: &[&str]) -> SolrQuery {
        let mut new_filters = Vec::with_capacity(filters.len());
        new_filters.extend(filters.iter().map(|x| x.to_string()));
        let mut solr_query = self.clone();
        solr_query.filters = Some(new_filters);
        solr_query
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
        let mut solr_query = self.clone();
        solr_query.sorts = sorts;
        solr_query
    }

    /// Sets sorts for fields.
    /// Already existing sorts will be overwritten
    pub fn set_sorts(&self, sorts: &[SortClause]) -> SolrQuery {
        let mut new_sorts = Vec::with_capacity(sorts.len());
        new_sorts.push_all(sorts);
        let mut solr_query = self.clone();
        solr_query.sorts = Some(new_sorts);
        solr_query
    }

    /// Sets initial offset (zero based).
    /// This method is only required when you want to get non-zero offset,
    /// because by default it's 0 (zero)
    pub fn start(&self, start: u64) -> SolrQuery {
        let mut solr_query = self.clone();
        solr_query.start = start;
        solr_query
    }

    /// Sets number of rows to be returned.
    /// This method is only required when you want to get non-default page size,
    /// which is 10.
    pub fn rows(&self, rows: u32) -> SolrQuery {
        let mut solr_query = self.clone();
        solr_query.rows = rows;
        solr_query
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

        if self.start != DEFAULT_START {
            vec.push(("start".to_string(), self.start.to_string()));
        }

        if self.rows != DEFAULT_ROWS {
            vec.push(("rows".to_string(), self.rows.to_string()));
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
            SortOrder::Ascending => "asc".to_string(),
            SortOrder::Descending => "desc".to_string()
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
