use std::cmp::Ordering;

use crate::vector::Vector;

pub struct KDNode {
    pub vector: Vector,
    left: Option<Box<KDNode>>,
    right: Option<Box<KDNode>>,
}

pub struct KDTree {
    pub root: Option<Box<KDNode>>,
}

impl KDTree {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, vector: Vector) {
        self.root = Self::insert_rec(self.root.take(), vector, 0);
    }

    fn insert_rec(node: Option<Box<KDNode>>, vector: Vector, depth: usize) -> Option<Box<KDNode>> {
        match node {
            None => Some(Box::new(KDNode {
                vector,
                left: None,
                right: None,
            })),
            Some(mut n) => {
                let k = n.vector.coordinates.len();
                let axis = depth % k;

                if vector.coordinates[axis] < n.vector.coordinates[axis] {
                    n.left = Self::insert_rec(n.left.take(), vector, depth + 1);
                } else {
                    n.right = Self::insert_rec(n.right.take(), vector, depth + 1);
                }
                Some(n)
            }
        }
    }

    pub fn nearest(&self, query: &Vector, k: usize) -> Vec<Vector> {
        let mut best = vec![];
        Self::nearest_rec(&self.root, query, k, 0, &mut best);
        best.into_iter().map(|(vec, _)| vec).collect()
    }

    fn nearest_rec(
        node: &Option<Box<KDNode>>,
        query: &Vector,
        k: usize,
        depth: usize,
        best: &mut Vec<(Vector, f64)>,
    ) {
        if let Some(n) = node {
            let k_dim = n.vector.coordinates.len();
            let axis = depth % k_dim;

            let dist = query.distance(&n.vector);
            if best.len() < k {
                best.push((n.vector.clone(), dist));
                best.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
            } else if dist < best[0].1 {
                best[0] = (n.vector.clone(), dist);
                best.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
            }

            let diff = query.coordinates[axis] - n.vector.coordinates[axis];
            let (close, away) = if diff < 0.0 {
                (&n.left, &n.right)
            } else {
                (&n.right, &n.left)
            };

            Self::nearest_rec(close, query, k, depth + 1, best);
            if best.len() < k || diff.abs() < best[0].1 {
                Self::nearest_rec(away, query, k, depth + 1, best);
            }
        }
    }
}
