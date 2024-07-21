from typing import List, Optional

class Vector:
    """Represents a vector.

    Args:
        coordinates (list[float]): The coordinates of the vector.
        meta (str, optional): The metadata of the vector, if provided.
    """
    def __init__(self, coordinates: List[float], *, meta: Optional[str] = None): ...

    # ~

    def distance(self, other: List[float]) -> float:
        """Calculates the distance between two vectors.

        Args:
            other: The other vector.

        Returns:
            float: The distance between the two vectors.
        """

    def within(self, lower: List[float], upper: List[float]) -> bool:
        """Checks if the vector is within the given bounds.

        Args:
            lower: The lower bound.
            upper: The upper bound.

        Returns:
            bool: True if the vector is within the bounds, False otherwise.
        """

    @property
    def coordinates(self) -> List[float]:
        """The coordinates of the vector."""

class VectorDatabase:
    """Represents the VDB37 vector database."""
    def __init__(self): ...
    def insert(self, vector: Vector):
        """Inserts a vector into the database.

        The vector will no longer be editable.

        TODO: deletion support

        Args:
            vector: The vector to insert.
        """

    def search(self, query_vector: Vector, *, k: int) -> List[Vector]:
        """Searches the database for nearest neighbors.

        Args:
            query_vector: The vector to search for.
            k: The number of nearest neighbors to return.

        Returns:
            List[Vector]: The nearest neighbors.
        """

def load_bin(filename: str) -> Vector:
    """Loads a vector from a binary file.

    Args:
        filename: The filename of the binary file.

    Returns:
        Vector: The loaded vector.
    """

def create_bin(filename: str, vector: Vector) -> None:
    """Creates a vector in a binary file.

    Args:
        filename: The filename of the binary file.
        vector: The vector to create.
    """
