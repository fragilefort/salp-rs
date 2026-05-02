use pdbtbx::{self, Format, PDB, ReadOptions};
use reqwest::{self, blocking::get};
use std::io::BufReader;

fn format_url(pdb_id: &str) -> String {
    format!("https://files.rcsb.org/download/{}.pdb", pdb_id)
}

pub fn fetch_pdb(pdb_id: &str) -> Result<PDB, Box<dyn std::error::Error>> {
    let url = format_url(pdb_id);
    let response = get(url)?;
    let reader = BufReader::new(response);

    let (pdb, _warn) = ReadOptions::new()
        .set_format(Format::Pdb)
        .read_raw(reader)
        .map_err(|e| format!("{:?}", e))?;

    Ok(pdb)
}

pub fn proteins_only(pdb_buffer: Vec<PDB>) -> Vec<PDB> {
    pdb_buffer
        .into_iter()
        .map(|mut pdb| {
            pdb.remove_atoms_by(|atom| !atom.hetero());
            pdb
        })
        .collect()
}

// #TODO
// Decide if we will save this pdbs to the disk or not
// Write tests for this module
// Do we do docking using these pdbs?
