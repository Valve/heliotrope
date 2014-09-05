use serialize::{Encodable, Encoder};

#[deriving(Show)]
pub enum SolrValue {
    SolrI64(i64),
    SolrU64(u64),
    SolrF64(f64),
    SolrString(String),
    SolrBoolean(bool),
    SolrNull
}
impl<S: Encoder<E>, E> Encodable<S, E> for SolrValue {
    fn encode(&self, e: &mut S) -> Result<(), E> {
        match *self {
            SolrI64(v) => v.encode(e),
            SolrU64(v) => v.encode(e),
            SolrF64(v) => v.encode(e),
            SolrString(ref v) => v.encode(e),
            SolrBoolean(v) => v.encode(e),
            SolrNull => "null".encode(e)
        }
    }
}
/// SolrDocument field
#[deriving(Show)]
pub struct SolrField {
    pub name: String,
    pub value: SolrValue
}

/// SolrDocument to be used to either index or query.
#[deriving(Show)]
pub struct SolrDocument {
    /// Collection of document fields
    pub fields: Vec<SolrField>
}

impl SolrDocument {
    /// Creates new empty SolrDocument
    pub fn new() -> SolrDocument {
        let fields: Vec<SolrField> = Vec::with_capacity(10);
        SolrDocument{fields: fields}
    }

    /// Adds a field to the document
    pub fn add_field(&mut self, name: &str, value: &str) {
        self.fields.push(SolrField{name: name.to_string(), value: SolrString(value.to_string())});
    }
}

impl<E, S: Encoder<E>> Encodable<S, E> for SolrDocument {
    fn encode(&self, s: &mut S) -> Result<(), E> {
        let mut i = 0u;
        s.emit_struct("SolrDocument", self.fields.len(), |e| {
            for field in self.fields.iter() {
                try!(e.emit_struct_field(field.name.as_slice(), i, |e| field.value.encode(e)));
                i = i + 1;
            }
            Ok(())
        })
    }
}
