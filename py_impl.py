#!/usr/bin/python3
import json
import math
import os
import struct
from typing import Optional, Tuple


class Vector:
    def __init__(self, *components, meta: Optional[str] = None):
        self.components = components
        self.meta = meta

    def dot(self, other):
        return sum(a * b for a, b in zip(self.components, other.components))

    def magnitude(self):
        return math.sqrt(sum(x**2 for x in self.components))

    def distance(self, other):
        return math.sqrt(
            sum((a - b) ** 2 for a, b in zip(self.components, other.components))
        )

    def bin(self) -> Tuple[int, bytes]:
        return (
            len(self.components),
            struct.pack(f"{len(self.components)}i", *self.components),
        )

    def __repr__(self) -> str:
        return (
            "Vector("
            + ", ".join(map(str, self.components))
            + (f", meta={self.meta!r})" if self.meta else ")")
        )


class KDNode:
    def __init__(self, vector, axis=0, left=None, right=None):
        self.vector = vector
        self.axis = axis
        self.left = left
        self.right = right


class VectorDatabase:
    def __init__(self):
        self.root = None
        self.v = []

    def add_vector(self, vector):
        self.v.append(vector)
        if not self.root:
            self.root = KDNode(vector, 0)
        else:
            self._add_vector(self.root, vector, 0)

    def _add_vector(self, node, vector, depth):
        axis = depth % len(vector.components)
        if vector.components[axis] < node.vector.components[axis]:
            if node.left:
                self._add_vector(node.left, vector, depth + 1)
            else:
                node.left = KDNode(vector, axis)
        else:
            if node.right:
                self._add_vector(node.right, vector, depth + 1)
            else:
                node.right = KDNode(vector, axis)

    def _search(self, node, vector, depth, best, best_dist):
        if not node:
            return best, best_dist

        axis = depth % len(vector.components)
        dist = vector.distance(node.vector)

        if dist < best_dist:
            best, best_dist = node.vector, dist

        diff = vector.components[axis] - node.vector.components[axis]
        close, away = (node.left, node.right) if diff < 0 else (node.right, node.left)

        best, best_dist = self._search(close, vector, depth + 1, best, best_dist)

        if diff**2 < best_dist:
            best, best_dist = self._search(away, vector, depth + 1, best, best_dist)

        return best, best_dist

    def find_nearest(self, vector):
        best, _ = self._search(self.root, vector, 0, None, float("inf"))
        return best

    def search(self, vector, top_n=1):
        def search_kdtree(node, depth=0):
            if node is None:
                return []
            axis = depth % len(vector.components)
            dist = vector.distance(node.vector)
            diff = vector.components[axis] - node.vector.components[axis]
            search_next, search_other = (
                (node.left, node.right) if diff < 0 else (node.right, node.left)
            )
            result = search_kdtree(search_next, depth + 1)
            result.append((dist, node.vector))
            result.sort(key=lambda x: x[0])
            result = result[:top_n]
            if len(result) < top_n or abs(diff) < result[-1][0]:
                result += search_kdtree(search_other, depth + 1)
                result.sort(key=lambda x: x[0])
                result = result[:top_n]
            return result

        return [v for _, v in search_kdtree(self.root)]

    def save(self, base_path: str):
        os.makedirs(base_path, exist_ok=True)
        sizes = []
        metas = []

        for i, vector in enumerate(self.v):
            vector: Vector
            size, data = vector.bin()
            with open(os.path.join(base_path, f"vector_{i}.bin"), "wb") as f:
                f.write(data)

            sizes.append(size)
            metas.append(vector.meta)

        with open(os.path.join(base_path, "metadata"), "wb") as f:
            f.write(json.dumps({"s": sizes, "m": metas}).encode("utf-8"))

    @staticmethod
    def load(base_path: str) -> "VectorDatabase":
        vdb = VectorDatabase()

        with open(os.path.join(base_path, "metadata"), "rb") as f:
            metadata = json.loads(f.read().decode("utf-8"))

        paths = os.listdir(base_path)
        paths.remove("metadata")
        for i, fn in enumerate(sorted(paths)):
            with open(os.path.join(base_path, fn), "rb") as f:
                vdb.add_vector(
                    Vector(
                        *struct.unpack(f"{metadata['s'][i]}i", f.read()),
                        meta=metadata["m"][i],
                    )
                )

        return vdb

        diff = vector.components[axis] - node.vector.components[axis]
        close, away = (node.left, node.right) if diff < 0 else (node.right, node.left)

        best, best_dist = self._search(close, vector, depth + 1, best, best_dist)

        if diff**2 < best_dist:
            best, best_dist = self._search(away, vector, depth + 1, best, best_dist)

        return best, best_dist

    def find_nearest(self, vector):
        best, _ = self._search(self.root, vector, 0, None, float("inf"))
        return best

    def search(self, vector, top_n=1):
        def search_kdtree(node, depth=0):
            if node is None:
                return []
            axis = depth % len(vector.components)
            dist = vector.distance(node.vector)
            diff = vector.components[axis] - node.vector.components[axis]
            search_next, search_other = (
                (node.left, node.right) if diff < 0 else (node.right, node.left)
            )
            result = search_kdtree(search_next, depth + 1)
            result.append((dist, node.vector))
            result.sort(key=lambda x: x[0])
            result = result[:top_n]
            if len(result) < top_n or abs(diff) < result[-1][0]:
                result += search_kdtree(search_other, depth + 1)
                result.sort(key=lambda x: x[0])
                result = result[:top_n]
            return result

        return [v for _, v in search_kdtree(self.root)]


# v = Vector(-1, 2, 3, 4, 5, 6, meta="Yo!")
# t = struct.pack(f"{len(v.components)}i", *v.components)
# print(struct.unpack(f"{len(v.components)}i", t))
