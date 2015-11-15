use rustc_serialize::{json, Decodable, Decoder};
use rustc_serialize::json::Json;
use response::SolrError;

pub type SolrUpdateResult = Result<SolrUpdateResponse, SolrError>;

#[derive(Debug, Copy, Clone)]
pub struct SolrUpdateResponse {
    /// HTTP status.
    /// When failed to connect, it will be 0 (zero).
    pub status: i32,
    /// Time it took to execute the request in milliseconds
    pub time: i32
}

impl Decodable for SolrUpdateResponse {
    fn decode<D: Decoder>(d: &mut D) -> Result<SolrUpdateResponse, D::Error> {
        d.read_struct("root", 0, |d| {
            d.read_struct_field("responseHeader", 0, |d| {
                Ok(SolrUpdateResponse{
                    status: try!(d.read_struct_field("status", 0, Decodable::decode)),
                    time: try!(d.read_struct_field("QTime", 1, Decodable::decode))
                })
            })
        })
    }
}
