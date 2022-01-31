use std::hash::Hash;
use std::ops::{Add, Sub};
use crate::Edge;
pub use crate::SimpleVertex as DirectedVertex;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct CompoundKey<K: Hash + Eq + Clone> {
    from: K,
    to: K,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct DirectedEdge<K, W>
    where K: Hash + Eq + Clone,
          W: Add + Sub + Eq + Ord + Copy,
{
    from: K,
    to: K,
    weight: W,
}

impl<K, W> DirectedEdge<K, W>
    where K: Hash + Eq + Clone,
          W: Add + Sub + Eq + Ord + Copy,
{
    pub fn new(from: K, to: K, weight: W) -> DirectedEdge<K, W> {
        DirectedEdge {
            from,
            to,
            weight,
        }
    }
}

impl<K, W> Edge<K, W, CompoundKey<K>> for DirectedEdge<K, W>
    where K: Hash + Eq + Clone,
          W: Add + Sub + Eq + Ord + Copy,
{
    fn get_weight(&self) -> W {
        self.weight
    }

    fn set_weight(&mut self, weight: W) {
        self.weight = weight;
    }

    fn from(&self) -> &K {
        &self.from
    }

    fn to(&self) -> &K {
        &self.to
    }

    fn get_pair(&self) -> (&K, &K) {
        (&self.from, &self.to)
    }

    fn generate_key(pair: (&K, &K)) -> CompoundKey<K> {
        CompoundKey {
            from: pair.0.clone(),
            to: pair.1.clone()
        }
    }

    fn key(&self) -> CompoundKey<K> {
        CompoundKey {
            from: self.from.clone(),
            to: self.to.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_construction() {
        let edge = DirectedEdge::new(1, 2, 3);

        assert_eq!(edge,
        DirectedEdge{ from: 1, to: 2, weight: 3});
    }

    #[test]
    fn edge_getters_setters() {
        let mut edge = DirectedEdge::new(1, 2, 3);
        assert_eq!((&1, &2, 3),
                   (edge.from(), edge.to(), edge.get_weight()));
        assert_eq!((&1, &2), edge.get_pair());

        edge.set_weight(4);
        assert_eq!(4, edge.get_weight());
    }

    #[test]
    fn key_generation() {
        let edge = DirectedEdge::new(1, 2, 3);
        let key = CompoundKey{from: 1, to: 2};

        assert_eq!(key, edge.key());
        assert_eq!(key, DirectedEdge::<i32, i32>::generate_key((&1, &2)));
    }

    #[test]
    fn key_equality() {
        let key1 = CompoundKey{from: 1, to: 2};
        let key2 = CompoundKey{from: 1, to: 2};
        let key3 = CompoundKey{from: 2, to: 1};

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }
}
