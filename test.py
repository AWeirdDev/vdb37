import vdb37

db = vdb37.VectorDatabase()
db.insert(vdb37.Vector([1, 2, 3]))
db.insert(vdb37.Vector([4, 5, 6]))
print(db.search(vdb37.Vector([10, 15, 20]), k=1))
print(db)
