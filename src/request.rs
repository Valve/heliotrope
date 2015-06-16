use rustc_serialize::{Encodable, Encoder};

/// Represents a document(s) delete request
#[derive(Debug)]
pub struct SolrDeleteRequest {
    ids: Vec<String>
}

impl SolrDeleteRequest {
    /// Creates a SolrDeleteRequest that will delete by a given Id
    pub fn from_id(id: &str) -> SolrDeleteRequest {
        SolrDeleteRequest { ids: vec![id.to_string()] }
    }

    pub fn from_ids(ids: &Vec<String>) -> SolrDeleteRequest {
        SolrDeleteRequest {ids: ids.clone()}
    }
}

impl Encodable for SolrDeleteRequest {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_struct("SolrDeleteRequest", 1, |e| {
            e.emit_struct_field("delete", 0, |e| {
                e.emit_seq(self.ids.len(), |e| {
                    for (i,id) in self.ids.iter().enumerate() {
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
    }
}
