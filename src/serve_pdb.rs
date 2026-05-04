use pdbtbx::{self, Format, PDB, ReadOptions, StrictnessLevel, save};
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

pub fn proteins_only(pdb_buffer: &mut PDB) {
    pdb_buffer.remove_atoms_by(|atom| !atom.hetero());
}

pub fn save_to_disk(pdb: &PDB, filename: &str) {
    save(pdb, filename, StrictnessLevel::Medium).expect("Failed to save to disk")
}
