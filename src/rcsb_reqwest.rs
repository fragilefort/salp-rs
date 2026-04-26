use super::search::Query;
use reqwest;
use serde::Serialize;

#[derive(Serialize)]
pub struct SearchRequest<'a> {
    query: &'a Query,
    return_type: &'a str,
}

impl<'a> SearchRequest<'a> {
    pub fn new(query: &'a Query) -> Self {
        SearchRequest {
            query,
            return_type: "entry",
        }
    }
    pub fn post_request(&self) -> Result<Option<String>, reqwest::Error> {
        let client = reqwest::blocking::Client::new();
        let res = client
            .post("https://search.rcsb.org/rcsbsearch/v2/query")
            .json(&self)
            .send()?;
        if res.status() == 204 {
            return Ok(None);
        }
        Ok(Some(res.text()?))
    }
}
