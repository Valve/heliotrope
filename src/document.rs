use serialize::{Encodable, Encoder};

#[deriving(Show)]
pub enum SolrValue {
    I64(i64),
    U64(u64),
    F64(f64),
    String(String),
    Boolean(bool),
    Null
}
impl<S: Encoder<E>, E> Encodable<S, E> for SolrValue {
    fn encode(&self, e: &mut S) -> Result<(), E> {
        match *self {
            SolrValue::I64(v) => v.encode(e),
            SolrValue::U64(v) => v.encode(e),
            SolrValue::F64(v) => v.encode(e),
            SolrValue::String(ref v) => v.encode(e),
            SolrValue::Boolean(v) => v.encode(e),
            SolrValue::Null => "null".encode(e)
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
        self.fields.push(SolrField{name: name.to_string(), value: SolrValue::String(value.to_string())});
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
