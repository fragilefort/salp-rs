# salp

A Python library for querying and fetching protein structures from the RCSB PDB, written in Rust.

## Installation

```bash
pip install salp-rs
```

With uv:

```bash
uv add salp-rs
```

## Example usage

```python
from salp import Query, search, fetch_and_save
# btw these are the only 2 functions you got

query = PyQuery.and_([
    PyQuery.organism("Homo sapiens"),
    PyQuery.max_resolution(2.5),
    PyQuery.keyword("kinase"),
])

# This returns the total count and list of PDB IDs
# RCSB returns up to 10 results by default. Control this with `start` and `rows`:
total, ids = search(query, start=0, rows=50)
print(f"Found {total} structures, fetching first {len(ids)}")

# Fetch and save, filter_proteins removes non-protein atoms
fetch_and_save(ids, filter_proteins=True, output_dir="./structures")
```
## Contributing

Issues and PRs welcome on [GitHub](https://github.com/fragilefort/salp-rs).
