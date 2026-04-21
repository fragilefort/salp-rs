pub mod rcsb_reqwest;
pub mod search;

#[cfg(test)]
mod tests {
    use crate::rcsb_reqwest::SearchRequest;

    use super::search::Query;

    #[test]
    fn test_query_serialization() {
        let q = Query::And(vec![Query::Organism("Homo sapiens".into())]);
        let json = serde_json::to_string_pretty(&q).unwrap();

        assert!(json.contains("Homo sapiens"));
        println!("{}", json);
    }
    #[test]
    fn test_max_resolution_serialization() {
        let q = Query::MaxResolution(2.5);
        let json = serde_json::to_string_pretty(&q).unwrap();
        println!("{}", json);
    }
    #[test]
    fn test_post_request() {
        let query = Query::And(vec![
            Query::Organism("Homo sapiens".to_string()),
            Query::MaxResolution(2.5),
        ]);
        let req = SearchRequest::new(&query);
        let res = req.post_request().unwrap();
        println!("The result of the search request: {}", res)
    }
}
