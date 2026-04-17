pub mod search;

#[cfg(test)]
mod tests {
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
}
