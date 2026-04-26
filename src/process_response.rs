use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ResultResponse {
    total_count: u64,
    result_set: Vec<SearchResult>,
}

#[derive(Debug, Deserialize)]
struct SearchResult {
    identifier: String,
    #[serde(skip_deserializing)]
    _score: f64,
}

pub fn parse_ids(response: &str) -> Result<(u64, Vec<String>), serde_json::Error> {
    let response: ResultResponse = serde_json::from_str(response)?;
    let total_count = response.total_count;

    let ids = response
        .result_set
        .into_iter()
        .map(|r| r.identifier)
        .collect();
    Ok((total_count, ids))
}
