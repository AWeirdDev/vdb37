import vdb37

db = vdb37.VectorDatabase()
db.insert(vdb37.Vector([1, 2, 3]))
print(db.search(vdb37.Vector([1, 2, 3]), k=5))