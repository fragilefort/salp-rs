use super::search::Query;
use reqwest;
use serde::Serialize;

#[derive(Serialize)]
pub struct SearchRequest<'a> {
    query: &'a Query,
    return_type: &'a str,
    request_options: RequestOptions,
}

#[derive(Serialize)]
struct Pagination {
    start: u32,
    rows: u32,
}

#[derive(Serialize)]
struct RequestOptions {
    paginate: Pagination,
}

impl<'a> SearchRequest<'a> {
    pub fn new(query: &'a Query, start: u32, rows: u32) -> Self {
        SearchRequest {
            query,
            return_type: "entry",
            request_options: RequestOptions {
                paginate: Pagination { start, rows },
            },
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
