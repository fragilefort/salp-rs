pub mod process_response;
pub mod rcsb_reqwest;
pub mod search;
pub mod serve_pdb;

#[cfg(test)]
mod tests {
    use crate::rcsb_reqwest::SearchRequest;
    use crate::serve_pdb::{fetch_pdb, proteins_only, save_to_disk};
    use pdbtbx::*;

    use super::process_response::parse_ids;
    use super::search::Query;
    use std::path::Path;

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
        let req = SearchRequest::new(&query, 0, 100);
        let res = req.post_request().unwrap();
        println!("The result of the search request: {:?}", res)
    }
    #[test]
    fn test_parse_response() {
        let query = Query::And(vec![
            Query::Organism("Mus musculus".to_string()),
            Query::MaxResolution(3.5),
            Query::Keyword("Phosphatase".to_string()),
        ]);
        let req = SearchRequest::new(&query, 0, 100);
        match req.post_request().unwrap() {
            Some(response) => {
                let parsed = parse_ids(&response);
                println!("{:?}", parsed);
            }
            None => println!("No results found"),
        }
    }

    #[test]
    fn test_pdbtbx() {
        let (mut pdb, _error) = pdbtbx::open("example-pdbs/1UBQ.pdb").unwrap();
        pdb.remove_atoms_by(|atom| atom.element() == Some(&Element::H));

        let mut avg_b_factor = 0.0;
        for atom in pdb.atoms() {
            avg_b_factor += atom.b_factor();
        }
        avg_b_factor /= pdb.atom_count() as f64;

        println!("The average B factor of the protein is: {}", avg_b_factor);
        pdbtbx::save(
            &pdb,
            "dump/1ubq_no_hydrogens.pdb",
            pdbtbx::StrictnessLevel::Loose,
        )
        .unwrap();
    }

    #[test]
    fn test_fetch_pdb() {
        let pdb = fetch_pdb("1TUP").expect("Failed to fetch 1TUP");
        assert!(pdb.atom_count() > 0);
    }

    #[test]
    fn test_proteins_only() {
        let mut pdb = fetch_pdb("1TUP").expect("Failed to fetch 1TUP");
        let before = pdb.atom_count();
        proteins_only(&mut pdb);
        let after = pdb.atom_count();
        assert!(after <= before);
    }

    #[test]
    fn test_save_to_disk() {
        let mut pdb = fetch_pdb("1TUP").unwrap();
        proteins_only(&mut pdb);
        save_to_disk(&pdb, "/tmp/test_1TUP.pdb");
        assert!(Path::new("/tmp/test_1TUP.pdb").exists());
        assert!(Path::new("/tmp/test_1TUP.pdb").metadata().unwrap().len() > 0);
    }
}
