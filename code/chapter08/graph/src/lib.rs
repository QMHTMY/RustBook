use std::hash::Hash;
use std::ops::{Add, Sub};
pub mod adjacency_list;

// 点特性
pub trait Vertex<K, V>
    where K: Hash + Eq + Clone,
{
    fn key(&self) -> K;
    fn get_value(&self) -> &V;
    fn get_mut_value(&mut self) -> &mut V;
}

// 边特性
pub trait Edge<K, W, C>
    where K: Hash + Eq + Clone,
          C: Hash + Eq + Clone,
          W: Add + Sub + Eq + Ord + Copy
{
    fn set_weight(&mut self, weight: W);
    fn get_weight(&self) -> W;
    fn from(&self) -> &K;
    fn to(&self) -> &K;
    fn get_pair(&self) -> (&K, &K);
    fn generate_key(pair: (&K, &K)) -> C;
    fn key(&self) -> C;
}

// 边处理错误枚举
#[derive(Debug, Eq, PartialEq)]
pub enum EdgeError {
    FromErr,
    ToErr,
    BothErr
}

// 图特性
pub trait DirectedGraph<T, E, K, V, W, C>
    where K: Hash + Eq + Clone,
          C: Hash + Eq + Clone,
          W: Add + Sub + Eq + Ord + Copy,
          T: Vertex<K, V>,
          E: Edge<K, W, C> {
    fn adjacent(&self, from: &K, to: &K) -> bool;
    fn neighbors(&self, from: &K) -> Vec<&K>;
    fn leading_to(&self, to: &K) -> Vec<&K>;
    fn get_all_keys(&self) -> Vec<&K>;
    fn get_all_pairs(&self) -> Vec<(&K, &K)>;
    fn get_vertex(&self, key: &K) -> Option<&T>;
    fn get_mut_vertex(&mut self, key: &K) -> Option<&mut T>;
    fn get_edge(&self, pair: (&K, &K)) -> Option<&E>;
    fn get_mut_edge(&mut self, pair: (&K, &K)) -> Option<&mut E>;
}

// 点特性
pub trait VariableVertexes<T, E, K, V, W, C>: DirectedGraph<T, E, K, V, W, C>
    where K: Hash + Eq + Clone,
          C: Hash + Eq + Clone,
          W: Add + Sub + Eq + Ord + Copy,
          T: Vertex<K, V>,
          E: Edge<K, W, C> {
    fn add_vertex(&mut self, vertex: T) -> Option<T>;
    fn remove_vertex(&mut self, key: K) -> Option<T>;
}

pub trait VariableEdges<T, E, K, V, W, C>: DirectedGraph<T, E, K, V, W, C>
    where K: Hash + Eq + Clone,
          C: Hash + Eq + Clone,
          W: Add + Sub + Eq + Ord + Copy,
          T: Vertex<K, V>,
          E: Edge<K, W, C> {
    fn add_edge(&mut self, edge: E) -> Result<Option<E>, EdgeError>;
    fn remove_edge(&mut self, pair: (&K, &K)) -> Option<E>;
}

pub trait Graph<T, E, K, V, W, C>: DirectedGraph<T, E, K, V, W, C>
    where K: Hash + Eq + Clone,
          C: Hash + Eq + Clone,
          W: Add + Sub + Eq + Ord + Copy,
          T: Vertex<K, V>,
          E: Edge<K, W, C> {
}

#[derive(Debug, Eq, PartialEq)]
pub struct SimpleVertex<K: Hash + Eq + Clone, V> {
    key: K,
    value: V,
}

impl<K: Hash + Eq + Clone, V> SimpleVertex<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

impl<K: Hash + Eq + Clone, V> Vertex<K, V> for SimpleVertex<K, V> {
    fn get_value(&self) -> &V {
        &(self.value)
    }

    fn get_mut_value(&mut self) -> &mut V {
        &mut (self.value)
    }

    fn key(&self) -> K {
        self.key.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_vertex_construction() {
        let vertex = SimpleVertex::new(1,0);

        assert_eq!(vertex, SimpleVertex {key: 1, value: 0});
    }

    #[test]
    fn simple_vertex_getters() {
        let mut vertex = SimpleVertex::new(1,0);

        assert_eq!(vertex.get_value(), &0);
        assert_eq!(vertex.key(), 1);

        *vertex.get_mut_value() += 3;
        assert_eq!(vertex.get_value(), &3);
    }
}
