use std::error::FromError;
use url::Url;
use hyper::{HttpResult, HttpError};
use hyper::net::Fresh;
use hyper::client::Request;
use hyper::header::common::{ContentType, ContentLength};


pub struct HttpResponse {
    pub code: u16,
    pub body: String
}

pub fn get(url: &Url) -> HttpResult<HttpResponse> {
    let mut req = Request::get(url.clone()).unwrap();
    req.headers_mut().set(ContentType(from_str("application/json").unwrap()));
    req.headers_mut().set(ContentLength(0));
    make_request(req)
}

pub fn post(url: &Url) -> HttpResult<HttpResponse> {
    let mut req = Request::post(url.clone()).unwrap();
    req.headers_mut().set(ContentType(from_str("application/json").unwrap()));
    req.headers_mut().set(ContentLength(0));
    make_request(req)
}

pub fn post_json(url: &Url, json: &str) -> HttpResult<HttpResponse> {
    //let mut req: RequestWriter = RequestWriter::new(Post, url.clone()).unwrap();
    //req.headers.insert_raw("Content-Type".to_string(), b"application/json").unwrap();
    //req.headers.content_length = Some(json.len());
    //try!(req.write(json.to_string().into_bytes().as_slice()));
    //make_request(req)
    let mut req = Request::post(url.clone()).unwrap();
    req.headers_mut().set(ContentType(from_str("application/json").unwrap()));
    req.headers_mut().set(ContentLength(json.len()));
    match req.start() {
        Ok(mut req) => {
            try!(req.write(json.to_string().into_bytes().as_slice()));
            match req.send() {
                Ok(mut resp) => {
                    match resp.read_to_string() {
                        Ok(body) => Ok(HttpResponse{code: resp.status as u16, body: body}),
                        Err(e) => Err(FromError::from_error(e))
                    }
                },
                Err(e) => Err(e)
            }
        },
        Err(e) => Err(e)
    }
}

fn make_request(req: Request<Fresh>) -> Result<HttpResponse, HttpError> {
    match req.start() {
        Ok(req) => {
            match req.send() {
                Ok(mut resp) => {
                    match resp.read_to_string() {
                        Ok(body) => Ok(HttpResponse{code: resp.status as u16, body: body}),
                        Err(e) => Err(FromError::from_error(e))
                    }
                },
                Err(e) => Err(e)
            }
        },
        Err(e) => Err(e)
    }
}
