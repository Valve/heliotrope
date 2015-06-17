use rustc_serialize::{Encodable, Encoder};

#[derive(Debug, PartialEq)]
pub enum SolrValue {
    I64(i64),
    U64(u64),
    F64(f64),
    String(String),
    Boolean(bool),
    Null
}
impl Encodable for SolrValue {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
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
#[derive(Debug)]
pub struct SolrField {
    pub name: String,
    pub value: SolrValue
}

/// SolrDocument to be used to either index or query.
#[derive(Debug)]
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

impl Encodable for SolrDocument {
    fn encode<E: Encoder>(&self, s: &mut E) -> Result<(), E::Error> {
        let mut i = 0usize;
        s.emit_struct("SolrDocument", self.fields.len(), |e| {
            for field in self.fields.iter() {
                try!(e.emit_struct_field(&field.name, i, |e| field.value.encode(e)));
                i = i + 1;
            }
            Ok(())
        })
    }
}
