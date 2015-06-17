use rustc_serialize::{Encodable, Encoder};

/// Represents a document(s) delete request
#[derive(Debug)]
pub struct SolrDeleteRequest {
    ids: Option<Vec<String>>,
    query: Option<String>
}

impl SolrDeleteRequest {
    /// Creates a SolrDeleteRequest that will delete by a given Id
    pub fn from_id(id: &str) -> SolrDeleteRequest {
        SolrDeleteRequest { ids: Some(vec![id.to_string()]), query: None }
    }

    pub fn from_ids(ids: &Vec<String>) -> SolrDeleteRequest {
        SolrDeleteRequest {ids: Some(ids.clone()), query: None}
    }

    pub fn from_query(query: &str) -> SolrDeleteRequest {
        SolrDeleteRequest {ids: None, query: Some(query.to_string())}
    }
}

impl Encodable for SolrDeleteRequest {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        let result = if let Some(ref ids) = self.ids {
            e.emit_struct("SolrDeleteRequest", 1, |e| {
                e.emit_struct_field("delete", 0, |e| {
                    e.emit_seq(ids.len(), |e| {
                        for (i,id) in ids.iter().enumerate() {
                             e.emit_seq_elt(i, |e| {
                                e.emit_struct("id_struct", 1, |e| {
                                    e.emit_struct_field("id", 0, |e| id.encode(e))
                                })
                             });
                        }
                        Ok(())
                    })
                })
            })
        } else if let Some(ref query) = self.query {
           e.emit_struct("SolrDeleteRequest", 1, |e| {
                e.emit_struct_field("delete", 0, |e| {
                    e.emit_struct("id_struct", 1, |e| {
                        e.emit_struct_field("query", 0, |e| query.encode(e));
                        Ok(())
                    })
                })
            }) 
        } else {
            unreachable!()
        };
        result
    }
}
