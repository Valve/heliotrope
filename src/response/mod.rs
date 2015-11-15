pub use self::update::{SolrUpdateResponse, SolrUpdateResult};
pub use self::query::{SolrQueryResponse, SolrQueryResult};
pub use self::ping::{SolrPingResponse, SolrPingResult};

mod update;
mod query;
mod ping;

use rustc_serialize::{json, Decodable, Decoder};
use rustc_serialize::json::Json;


/// SolrError
pub struct SolrError {
    /// HTTP status.
    /// When failed to connect, it will be 0 (zero).
    pub status: i32,
    /// Time it took to execute the request in milliseconds
    pub time: i32,
    /// Detailed error message
    pub message: String
}

impl Decodable for SolrError {
    fn decode<D: Decoder>(d: &mut D) -> Result<SolrError, D::Error> {
        d.read_struct("root", 0, |d| {
            d.read_struct_field("error", 0, |d| {
                Ok(SolrError{
                    message: try!(d.read_struct_field("msg", 0, Decodable::decode)),
                    status: try!(d.read_struct_field("code", 1, Decodable::decode)),
                    // TODO: implement time parsing from request header
                    time: 0})
            })
        })
    }
}
