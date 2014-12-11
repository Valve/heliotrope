use serialize::{Encodable, Encoder};

/// Represents a document(s) delete request
#[deriving(Show)]
pub struct SolrDeleteRequest {
    id: String
}

impl SolrDeleteRequest {
    /// Creates a SolrDeleteRequest that will delete by a given Id
    pub fn from_id(id: &str) -> SolrDeleteRequest {
        SolrDeleteRequest { id: id.to_string() }
    }
}

impl<E, S: Encoder<E>> Encodable<S, E> for SolrDeleteRequest {
    fn encode(&self, s: &mut S) -> Result<(), E> {
        s.emit_struct("SolrDeleteRequest", 1, |e| {
            e.emit_struct_field("delete", 0, |e| {
                e.emit_struct("delete", 1, |e| {
                    try!(e.emit_struct_field("id", 0, |e| self.id.encode(e)));
                    Ok(())
                })
            })
        })
    }
}
