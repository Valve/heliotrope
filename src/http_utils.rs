use url::Url;
use hyper::Client;
use hyper::header::{ContentType};
use hyper::status::StatusCode;
use hyper::error::Error;
use std::io::Read;


pub struct HttpResponse {
    pub status: StatusCode,
    pub body: String
}

pub fn get(url: &Url) -> Result<HttpResponse, Error> {
    let mut client = Client::new();
    let result_response = client.get(&url.to_string()).send();
    //TODO: use try! macro here
    match result_response {
        Ok(mut res) => {
            let mut body = String::new();
            let result = res.read_to_string(&mut body);
            match result {
                Ok(_) => {
                    Ok(HttpResponse{status: res.status, body: body})
                },
                //TODO: review why we use Irror::Io here
                Err(err) => {
                    Err(Error::Io(err))
                }
            }
        },
        Err(err) => Err(err)
    }
}


pub fn post_json(url: &Url, body: &str) -> Result<HttpResponse, Error> {
    let mut client = Client::new();
    let result_response = client.post(&url.to_string())
        .header(ContentType::json())
        .body(body)
        .send();
    match result_response {
        Ok(mut res) => {
            let mut body = String::new();
            let result = res.read_to_string(&mut body);
            match result {
                Ok(_) => {
                    Ok(HttpResponse{status: res.status, body: body})
                },
                Err(err) => {
                    Err(Error::Io(err))
                }
            }
        },
        Err(err) => Err(err)
    }
} 