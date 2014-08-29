use serialize::{json, Encodable, Encoder, Decodable, Decoder};

#[deriving(Show, Decodable)]
pub enum SolrValue {
  SolrF64(f64),
  SolrI64(i64),
  SolrString(String)
}

impl<S: Encoder<E>, E> Encodable<S, E> for SolrValue {
  fn encode(&self, e: &mut S) -> Result<(), E> {
    match *self {
      SolrF64(v) => v.encode(e),
      SolrI64(v) => v.encode(e),
      SolrString(ref v) => v.encode(e)
    }
  }
}

#[deriving(Show)]
pub struct SolrField {
  pub name: String,
  pub value: SolrValue
}
#[deriving(Show)]
pub struct SolrDocument {
  pub fields: Vec<SolrField>
}

impl SolrDocument {
  pub fn new() -> SolrDocument {
    let fields: Vec<SolrField> = Vec::with_capacity(10);
    SolrDocument{fields: fields}
  }

  pub fn add_field(&mut self, name: &str, value: SolrValue) {
    self.fields.push(SolrField{name: name.to_string(), value: value});
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

impl<E, D: Decoder<E>> Decodable<D, E> for SolrDocument {
  fn decode(d: &mut D) -> Result<SolrDocument, E> {
    d.read_map(|d, len| {
      let mut doc = SolrDocument{fields: Vec::with_capacity(len)};
      for i in range(0u, len) {
        let field_name: String = try!(d.read_map_elt_key(i, Decodable::decode));
        // TODO: match correct SolrValue
        let field_value = SolrString(try!(d.read_map_elt_val(i, Decodable::decode)));
        doc.fields.push(SolrField{name: field_name, value: field_value});
      }
      Ok(doc)
    })
  }
}

