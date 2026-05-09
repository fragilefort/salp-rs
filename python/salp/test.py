from salp import Query, search, fetch_and_save
import os

query = Query.and_([
    Query.organism("Homo sapiens"),
    Query.max_resolution(2.5),
    Query.keyword("kinase"),
])

total, ids = search(query, 0, 10)
print(f"Total results: {total}")
print(f"First 5 IDs: {ids[:5]}")
assert total > 0, "Search returned no results"
assert len(ids) == 10, "Expected 10 IDs"

os.makedirs("./test_structures", exist_ok=True)
fetch_and_save(ids[:3], True, "./test_structures")
for id in ids[:3]:
    path = f"./test_structures/{id}.pdb"
    assert os.path.exists(path), f"File {path} not found"
    assert os.path.getsize(path) > 0, f"File {path} is empty"

print("\n tests passed!")

