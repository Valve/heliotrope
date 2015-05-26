use url::Url;
use hyper::Client;
use hyper::header::{ContentType, ContentLength};
use hyper::net::Fresh;
use hyper::client::Request;
use hyper::status::StatusCode;


pub struct HttpResponse {
    pub status: StatusCode,
    pub body: String
}

pub fn get<E>(url: &Url) -> Result<HttpResponse, E> {
    let mut client = Client::new();
    let req = client.get(&url.to_string())
        .header(ContentType("application/json".parse().unwrap()))
        .header(ContentLength(0));
    let res = try!(req.send());
}

//pub fn post<E>(url: &Url) -> Result<HttpResponse, E> {
    //let mut client = Client::new();
    //client.post(url.to_string().as_slice()).header(ContentType("application/json".parse().unwrap()));
    //let mut req = Request::post(url.clone()).unwrap();
    //req.headers_mut().set(ContentType(from_str("application/json").unwrap()));
    //req.headers_mut().set(ContentLength(0));
    //make_request(req)
//}

//pub fn post_json<E>(url: &Url, json: &str) -> Result<HttpResponse, E> {
    //let mut client = Client::new();
    //client.post(url.to_string().as_slice()).header(ContentType("application/json".parse().unwrap()));
    //let mut req = Request::post(url.clone()).unwrap();
    //req.headers_mut().set(ContentType(from_str("application/json").unwrap()));
    //req.headers_mut().set(ContentLength(json.len()));
    //match req.start() {
        //Ok(mut req) => {
            //try!(req.write(json.to_string().into_bytes().as_slice()));
            //match req.send() {
                //Ok(mut resp) => {
                    //match resp.read_to_string() {
                        //Ok(body) => Ok(HttpResponse{code: resp.status as u16, body: body}),
                        //Err(e) => Err(FromError::from_error(e))
                    //}
                //},
                //Err(e) => Err(e)
            //}
        //},
        //Err(e) => Err(e)
    //}
//}

//fn make_request<C, E>(client: &mut Client<C>) -> Result<HttpResponse, E> {

    //match req.start() {
        //Ok(req) => {
            //match req.send() {
                //Ok(mut resp) => {
                    //match resp.read_to_string() {
                        //Ok(body) => Ok(HttpResponse{code: resp.status as u16, body: body}),
                        //Err(e) => Err(FromError::from_error(e))
                    //}
                //},
                //Err(e) => Err(e)
            //}
        //},
        //Err(e) => Err(e)
    //}
//}
