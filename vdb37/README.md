# vdb37
Vector database. 37 is a prime number.

```python
from vdb37 import VectorDatabase, Vector

db = VectorDatabase()
db.insert(Vector([1, 2, 3]))
db.search(Vector([1, 2, 2]), k=1)
```

## Why?
This is just a simple vector database implementation. I asked AI for help as my math is undoubtedly bad.

The backend is PyO3.
