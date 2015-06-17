extern crate rustc_serialize;
extern crate heliotrope;
extern crate url;
extern crate hyper;
extern crate time;

use rustc_serialize::json::Json;
use heliotrope::{HttpResponse, get, post_json, Solr, SolrDocument, SolrQuery, SolrValue, SolrField};
use url::Url;
use hyper::status::StatusCode;

#[test]
fn test_ping() {
    let ping_url = "http://localhost:8983/solr/test/admin/ping?wt=json";
    let url: Url = Url::parse(ping_url).unwrap();

    let res = get(&url).unwrap();
    assert_eq!(StatusCode::Ok, res.status);
    let row_json = Json::from_str(&res.body).unwrap();
    assert_eq!("OK".to_string(),
        row_json.as_object().unwrap().get("status").unwrap().as_string().unwrap());
}

#[test]
fn test_get_documents() {
    let docs = get_all_docs();
    assert!(docs.len() >= 0);
}

#[test]
fn test_create_document() {
    delete_all();

    let docs = get_all_docs();  
    assert_eq!(0, docs.len());

    let update_url = "http://localhost:8983/solr/test/update?&wt=json&commit=true";
    let url: Url = Url::parse(update_url).unwrap();
    let res = post_json(&url, &format!("{{add: {{doc: {{id: \"999\", time: \"{t}\"}}}}}}", t=time::now().rfc822())).unwrap();
    assert_eq!(StatusCode::Ok, res.status);
    
    let docs = get_all_docs();  
    assert_eq!(1, docs.len());
    let doc = &docs[0];
    assert_eq!("999", doc.as_object().unwrap().get("id").unwrap().as_string().unwrap());
}

fn delete_all() {
    let delete_all_url ="http://localhost:8983/solr/test/update?q=*:*&wt=json&commit=true";
    let delete_url = Url::parse(delete_all_url).unwrap();
    post_json(&delete_url, "{delete: {query: \"*:*\"}}").unwrap();
}

fn get_all_docs() -> Vec<Json> {
    let query_url = "http://localhost:8983/solr/test/select?q=*:*&wt=json";

    let url: Url = Url::parse(query_url).unwrap();
    let res = get(&url).unwrap();
    assert_eq!(StatusCode::Ok, res.status);

    let row_json = Json::from_str(&res.body).unwrap();
    let responce_field = row_json.as_object().unwrap().get("response")
        .unwrap().as_object().unwrap();
    let docs = responce_field.get("docs").unwrap().as_array().unwrap();
    docs.clone()
}

#[test]
fn add_and_commit() {
    delete_all();
    let base_url = "http://localhost:8983/solr/test/";
    let url: Url = Url::parse(base_url).unwrap();
    let client = Solr::new(&url);

    let mut doc = SolrDocument::new();
    doc.add_field("id", "00");
    doc.add_field("time", &format!("{t}", t=time::now().rfc822()));

    let result = client.add_and_commit(&doc);

    match result {
        Ok(resp) => {
            assert_eq!(0, resp.status);
            let docs = get_all_docs();
            assert_eq!(1, docs.len());
            assert_eq!("00", docs[0].as_object().unwrap().get("id").unwrap().as_string().unwrap());
        },
        Err(e) => panic!(e.message)
    }

}


#[test]
fn commit(){
    delete_all();
    let docs = get_all_docs();
    assert_eq!(0, docs.len());

    let mut doc1 = SolrDocument::new();
    doc1.add_field("id", "0");
    let mut doc2 = SolrDocument::new();
    doc2.add_field("id", "1");

    let base_url = "http://localhost:8983/solr/test/";
    let url: Url = Url::parse(base_url).unwrap();
    let client = Solr::new(&url);

    client.add_many(&[&doc1, &doc2]);
    let docs = get_all_docs();
    assert_eq!(0, docs.len());

    client.commit();
    let docs = get_all_docs();
    assert_eq!(2, docs.len());

}

#[test]
fn delete() {
    delete_all();
    let docs = get_all_docs();
    assert_eq!(0, docs.len());

    let mut doc1 = SolrDocument::new();
    doc1.add_field("id", "0");
    let mut doc2 = SolrDocument::new();
    doc2.add_field("id", "1");

    let base_url = "http://localhost:8983/solr/test/";
    let url: Url = Url::parse(base_url).unwrap();
    let client = Solr::new(&url);

    client.add_many_and_commit(&[&doc1, &doc2]);

    let docs = get_all_docs();
    assert_eq!(2, docs.len());

    client.delete_by_id("0");

    let docs = get_all_docs();
    
    assert_eq!(1, docs.len());    
    assert_eq!("1", docs[0].as_object().unwrap().get("id").unwrap().as_string().unwrap());
}

#[test]
fn query() {
    delete_all();

    let base_url = "http://localhost:8983/solr/test/";
    let url: Url = Url::parse(base_url).unwrap();
    let client = Solr::new(&url);

    let query = SolrQuery::new("*:*");

    let results = client.query(&query);

    match results {
        Ok(resp) => {
            assert_eq!(0,resp.total);
            assert_eq!(0, resp.items.len());
        },
        Err(err) => panic!("Error solr query")
    }
}

#[test]
fn query_after_create() {
     delete_all();

    let base_url = "http://localhost:8983/solr/test/";
    let url: Url = Url::parse(base_url).unwrap();
    let client = Solr::new(&url);

    let mut doc1 = SolrDocument::new();
    doc1.add_field("id", "1");

    let mut doc2 = SolrDocument::new();
    doc2.add_field("id", "2");

    let mut doc3 = SolrDocument::new();
    doc3.add_field("id", "3");

    client.add_many_and_commit(&[&doc1, &doc2, &doc3]);


    let query_all = SolrQuery::new("*:*");
    let results = client.query(&query_all);

    match results {
        Ok(resp) => {
            assert_eq!(3,resp.total);
            assert_eq!(3, resp.items.len());
        },
        Err(err) => panic!("Error solr query *:*")
    }

     let query_first = SolrQuery::new("id:1");

    let results = client.query(&query_first);

    match results {
        Ok(resp) => {
            assert_eq!(1,resp.total);
            assert_eq!(1, resp.items.len());
            let doc = &resp.items[0];
            assert_eq!(2, doc.fields.len());
            for f in &doc.fields {
                match f.name.as_ref() {
                    "id" => assert_eq!(SolrValue::String("1".to_string()), f.value),
                    "_version_" => (),
                    _ => panic!("unknown field name in the doc")
                }
            }
        },
        Err(err) => panic!("Error solr query for id:1")
    }
}

#[test]
fn delete_by_ids() {
    delete_all();

    let base_url = "http://localhost:8983/solr/test/";
    let url: Url = Url::parse(base_url).unwrap();
    let client = Solr::new(&url);


    let mut doc1 = SolrDocument::new();
    doc1.add_field("id", "1");

    let mut doc2 = SolrDocument::new();
    doc2.add_field("id", "2");

    let mut doc3 = SolrDocument::new();
    doc3.add_field("id", "3");

    client.add_many_and_commit(&[&doc1, &doc2, &doc3]);

    client.delete_by_ids(&vec!["1".to_string(), "2".to_string()]);

    let query_all = SolrQuery::new("*:*");
    let results = client.query(&query_all);

    match results {
        Ok(resp) => {
            assert_eq!(1,resp.total);
            assert_eq!(1, resp.items.len());
            let doc = &resp.items[0];
        },
        Err(err) => panic!("Error delete documents by many ids")
    }
}


#[test]
fn delete_by_query() {
    delete_all();

    let base_url = "http://localhost:8983/solr/test/";
    let url: Url = Url::parse(base_url).unwrap();
    let client = Solr::new(&url);


    let mut doc1 = SolrDocument::new();
    doc1.add_field("id", "1");
    doc1.add_field("city", "London");

    let mut doc2 = SolrDocument::new();
    doc2.add_field("id", "2");
    doc2.add_field("city", "NY");

    let mut doc3 = SolrDocument::new();
    doc3.add_field("id", "3");
    doc3.add_field("city", "NY");

    client.add_many_and_commit(&[&doc1, &doc2, &doc3]);

    client.delete_by_query("city:NY");

    let query_all = SolrQuery::new("*:*");
    let results = client.query(&query_all);

    match results {
        Ok(resp) => {
            assert_eq!(1, resp.total);
            assert_eq!(1, resp.items.len());
            let doc = &resp.items[0];
            for f in &doc.fields {
                match f.name.as_ref() {
                    "id" => assert_eq!(SolrValue::String("1".to_string()), f.value),
                    "city" => (),
                    "_version_" => (),
                    _ => panic!("unknown field name in the doc")
                }
            }
        },
        Err(err) => panic!("Error delete documents by query")
    }
}

#[test]
fn rollback() {
    delete_all();
    let docs = get_all_docs();
    assert_eq!(0, docs.len());

    let mut doc1 = SolrDocument::new();
    doc1.add_field("id", "0");
    let mut doc2 = SolrDocument::new();
    doc2.add_field("id", "1");

    let base_url = "http://localhost:8983/solr/test/";
    let url: Url = Url::parse(base_url).unwrap();
    let client = Solr::new(&url);

    client.add_many(&[&doc1, &doc2]);
    let docs = get_all_docs();
    assert_eq!(0, docs.len());

    client.rollback();
    client.commit();
    let docs = get_all_docs();
    assert_eq!(0, docs.len());
}

#[test]
fn optimize() {
    let base_url = "http://localhost:8983/solr/test/";
    let url: Url = Url::parse(base_url).unwrap();
    let client = Solr::new(&url);

    let res = client.optimize();
    
    match res {
        Ok(r) => assert_eq!(0, r.status),
        Err(e) => panic!("Error test optimize")
    }
}