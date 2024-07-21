from vdb37 import VectorDatabase, Vector

vector = Vector([1.0, 2.0, 3.0], meta="worst mood")
assert vector.within(Vector([0.0, 0.0, 0.0]), Vector([10.0, 10.0, 10.0]))

db = VectorDatabase()
db.insert(vector)
print(db.search(Vector([1, 2, 2]), k=1)[0].meta)
