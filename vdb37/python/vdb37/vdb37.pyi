from typing import List

class Vector:
    def __init__(self, coordinates: List[float]): ...
    def distance(self, other: List[float]) -> float: ...
    def within(self, lower: List[float], upper: List[float]) -> bool: ...

class VectorDatabase:
    def __init__(self): ...
    def insert(self, vector: Vector): ...
    def search(self, query_vector: Vector, *, k: int) -> List[Vector]: ...
