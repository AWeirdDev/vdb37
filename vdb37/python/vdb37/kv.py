import os
from typing import Dict

from .vdb37 import VectorDatabase


class KV:
    """Represents a KV-paired vector database."""

    kv: Dict[str, VectorDatabase]

    def __init__(self):
        self.kv = {}

    def get(self, key: str) -> VectorDatabase:
        return self.kv[key]

    def update(self, key: str, db: VectorDatabase) -> None:
        self.kv[key] = db

    def save(self, base_path: str = "kv") -> None:
        for key, db in self.kv.items():
            db.save(os.path.join(base_path, key))

    @staticmethod
    def load(base_path: str = "kv") -> "KV":
        kv = KV()

        for key in sorted(os.listdir(base_path)):
            kv.kv[key] = VectorDatabase.load(os.path.join(base_path, key))

        return kv

    def __repr__(self) -> str:
        return f"<KV {self.kv}>"
