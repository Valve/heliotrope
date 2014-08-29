use serialize::{Decodable, Decoder};
use document::SolrDocument;

pub type SolrUpdateResult = Result<SolrUpdateResponse, SolrError>;
pub type SolrQueryResult = Result<SolrQueryResponse, SolrError>;

pub struct SolrError {
  pub status: int,
  pub time: int,
  pub message: String
}

impl<D: Decoder<E>, E> Decodable<D, E> for SolrError {
  fn decode(d: &mut D) -> Result<SolrError, E> {
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

pub struct SolrUpdateResponse {
  pub status: int,
  pub time: int
}

impl<D: Decoder<E>, E> Decodable<D, E> for SolrUpdateResponse {
  fn decode(d: &mut D) -> Result<SolrUpdateResponse, E> {
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

#[deriving(Show)]
pub struct SolrQueryResponse {
  pub status: int,
  pub time: int,
  pub total: int,
  pub start: int,
  pub items: Vec<SolrDocument>
}

impl<D: Decoder<E>, E> Decodable<D, E> for SolrQueryResponse {
  fn decode(d: &mut D) -> Result<SolrQueryResponse, E> {
    let mut resp = SolrQueryResponse{ status: 0, time: 0, total: 0, start: 0, items: Vec::new() };
    d.read_struct("root", 0, |d| {
      d.read_struct_field("responseHeader", 0, |d| {
        resp.status = try!(d.read_struct_field("status", 0, Decodable::decode));
        resp.time = try!(d.read_struct_field("QTime", 1, Decodable::decode));
        Ok(())
      });
      d.read_struct_field("response", 0, |d| {
        resp.total = try!(d.read_struct_field("numFound", 0, Decodable::decode));
        resp.start = try!(d.read_struct_field("start", 1, Decodable::decode));
        d.read_struct_field("docs", 0, |d| {
          d.read_seq(|d, len| {
            println!("NUmber of docs: {}", len);
            for i in range(0u, len) {
              resp.items.push(try!(Decodable::decode(d)));
            }
            Ok(())
          });
          Ok(())
        });
        Ok(())
      });
      Ok(())
    });
    Ok(resp)
  }
}
