use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Add, Sub};

pub mod directedge;
use directedge::*;
use crate::{Vertex, EdgeError};
use super::{DirectedGraph, VariableEdges, VariableVertexes, SimpleVertex, Edge};

struct AdjacencyList<K, V, W>
    where K: Hash + Eq + Clone,
          W: Add + Sub + Eq + Ord + Copy
{
    vertex: DirectedVertex<K, V>,
    list: HashMap<CompoundKey<K>, DirectedEdge<K, W>>,
}

pub struct AdjacencyGraph<K, V, W>
    where K: Hash + Eq + Clone,
          W: Add + Sub + Eq + Ord + Copy
{
    vertexes: HashMap<K, AdjacencyList<K, V, W>>,
}

impl<K, V, W> AdjacencyGraph<K, V, W>
    where K: Hash + Eq + Clone,
          W: Add + Sub + Eq + Ord + Copy
{
    pub fn new() -> AdjacencyGraph<K, V, W> {
        AdjacencyGraph {
            vertexes: HashMap::new()
        }
    }
}

impl<K, V, W> DirectedGraph<
    DirectedVertex<K, V>,
    DirectedEdge<K, W>,
    K, V, W,
    CompoundKey<K>>
    for AdjacencyGraph<K, V, W>
    where K: Hash + Eq + Clone,
          W: Add + Sub + Eq + Ord + Copy
{
    fn adjacent(&self, from: &K, to: &K) -> bool {
        if let Some(adjacency) = self.vertexes.get(from) {
            if let Some(_) = adjacency.list
                                      .get(&DirectedEdge::<K,W>::generate_key((from,to))) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn neighbors(&self, from: &K) -> Vec<&K> {
        let mut neighbors = Vec::new();

        if let Some(adjacency) = self.vertexes.get(from) {
            for (_, edge) in &adjacency.list {
                neighbors.push(edge.to());
            }
        }

        neighbors
    }

    fn leading_to(&self, to: &K) -> Vec<&K> {
        let mut leading = Vec::new();

        for (from, adjacency) in &self.vertexes {
            if let Some(_) = adjacency.list
                                      .get(&DirectedEdge::<K,W>::generate_key((from, &to))){
                leading.push(from);
            }
        }

        leading
    }

    fn get_all_keys(&self) -> Vec<&K> {
        let mut vertexes = Vec::new();

        for (key, _) in &self.vertexes {
            vertexes.push(key);
        }

        vertexes
    }

    fn get_all_pairs(&self) -> Vec<(&K, &K)> {
        let mut pairs = Vec::new();

        for (_, adjacency) in &self.vertexes {
            for (_, edge) in &adjacency.list {
                pairs.push(edge.get_pair());
            }
        }

        pairs
    }

    fn get_vertex(&self, key: &K) -> Option<&SimpleVertex<K, V>> {
        if let Some(adjacency) = self.vertexes.get(&key) {
            Some(&adjacency.vertex)
        } else {
            None
        }
    }

    fn get_mut_vertex(&mut self, key: &K) -> Option<&mut SimpleVertex<K, V>> {
        if let Some(adjacency) = self.vertexes.get_mut(&key) {
            Some(&mut adjacency.vertex)
        } else {
            None
        }
    }

    fn get_edge(&self, pair: (&K, &K)) -> Option<&DirectedEdge<K, W>> {
        if let Some(adjacency) = self.vertexes.get(pair.0) {
            adjacency.list.get(&DirectedEdge::<K, W>::generate_key(pair))
        } else {
            None
        }
    }

    fn get_mut_edge(&mut self, pair: (&K, &K)) -> Option<&mut DirectedEdge<K, W>> {
        if let Some(adjacency) = self.vertexes.get_mut(pair.0) {
            adjacency.list.get_mut(&DirectedEdge::<K, W>::generate_key(pair))
        } else {
            None
        }
    }
}

impl<K, V, W> VariableVertexes<
    DirectedVertex<K, V>,
    DirectedEdge<K, W>,
    K, V, W,
    CompoundKey<K>>
    for AdjacencyGraph<K, V, W>
    where K: Hash + Eq + Clone,
          W: Add + Sub + Eq + Ord + Copy,
{
    fn add_vertex(&mut self, vertex: SimpleVertex<K, V>) -> Option<SimpleVertex<K, V>> {
        if let Some(
            AdjacencyList{
                vertex: old_vertex,
                list
            }) = self.vertexes.remove(&vertex.key()) {
            self.vertexes.insert(
                vertex.key(),
                AdjacencyList {
                    vertex,
                    list
                });
            Some(old_vertex)
        } else {
            self.vertexes.insert(
                vertex.key(),
                AdjacencyList {
                    vertex,
                    list: HashMap::new()
                });
            None
        }
    }

    fn remove_vertex(&mut self, key: K) -> Option<SimpleVertex<K, V>> {
        for (from, adjacency) in &mut self.vertexes {
            adjacency.list.remove(&DirectedEdge::<K, W>::generate_key((from, &key)));
        }

        if let Some(
            AdjacencyList{
                vertex: old_vertex,
                list: _
            }) = self.vertexes.remove(&key) {
            Some(old_vertex)
        } else {
            None
        }
    }
}

impl<K, V, W> VariableEdges<
    DirectedVertex<K, V>,
    DirectedEdge<K, W>,
    K, V, W,
    CompoundKey<K>>
    for AdjacencyGraph<K, V, W>
    where K: Hash + Eq + Clone,
          W: Add + Sub + Eq + Ord + Copy
{
    fn add_edge(&mut self, edge: DirectedEdge<K, W>)
        -> Result<Option<DirectedEdge<K, W>>, EdgeError> {
        if let Some(_) =  self.vertexes.get(&edge.to()) {
            if let Some(AdjacencyList {
                        vertex: _, list
                       }) = self.vertexes.get_mut(&edge.from()) {

                Ok(if let Some(old_edge) = list.remove(&edge.key()) {
                    list.insert(edge.key(), edge);
                    Some(old_edge)
                } else {
                    list.insert(edge.key(), edge);
                    None
                })

            } else {
                Err(EdgeError::FromErr)
            }
        } else {
            if let Some(_) =  self.vertexes.get(&edge.from()){
                Err(EdgeError::ToErr)
            } else {
                Err(EdgeError::BothErr)
            }
        }
    }

    fn remove_edge(&mut self, pair: (&K, &K)) -> Option<DirectedEdge<K, W>> {
        if let Some(adjacency) = self.vertexes.get_mut(pair.0) {
            adjacency.list.remove(&DirectedEdge::<K, W>::generate_key(pair))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EdgeError::*;

    #[test]
    fn add_remove_edge_all_cases() {
        let mut graph = AdjacencyGraph::new();
        graph.add_vertex(SimpleVertex::new(1, "a"));
        graph.add_vertex(SimpleVertex::new(2, "b"));
        graph.add_vertex(SimpleVertex::new(3, "c"));
        graph.add_vertex(SimpleVertex::new(4, "d"));

        assert_eq!(Ok(None), graph.add_edge(DirectedEdge::new(1, 2, 3)));
        assert_eq!(Ok(None), graph.add_edge(DirectedEdge::new(2, 1, 3)));
        assert_eq!(
            Ok(Some(DirectedEdge::new(2,1, 3))),
            graph.add_edge(DirectedEdge::new(2,1, 4))
        );
        assert_eq!(Err(FromErr), graph.add_edge(DirectedEdge::new(150, 2, 1)));
        assert_eq!(Err(ToErr), graph.add_edge(DirectedEdge::new(3, 2000, 1)));
        assert_eq!(Err(BothErr), graph.add_edge(DirectedEdge::new(48, 56, 1)));
    }

    #[test]
    fn adjacency_neighboring() {
        let mut graph = AdjacencyGraph::new();
        graph.add_vertex(SimpleVertex::new(1, "a"));
        graph.add_vertex(SimpleVertex::new(2, "b"));
        graph.add_vertex(SimpleVertex::new(3, "c"));
        graph.add_vertex(SimpleVertex::new(4, "d"));
        graph.add_vertex(SimpleVertex::new(5, "e"));
        graph.add_edge(DirectedEdge::new(1, 2, 3)).expect("Won't fail");
        graph.add_edge(DirectedEdge::new(2, 1, 3)).expect("Won't fail");
        graph.add_edge(DirectedEdge::new(2, 3, 3)).expect("Won't fail");
        graph.add_edge(DirectedEdge::new(1, 3, 3)).expect("Won't fail");
        graph.add_edge(DirectedEdge::new(4, 2, 3)).expect("Won't fail");

        assert_eq!(true, graph.adjacent(&1, &3));
        assert_eq!(false, graph.adjacent(&3, &1));
        assert_eq!(false, graph.adjacent(&200, &300));
        assert_eq!(Vec::<&i32>::new(), graph.neighbors(&5));
        assert_eq!(Vec::<&i32>::new(), graph.leading_to(&5));
        assert_eq!(Vec::<&i32>::new(), graph.neighbors(&6));
        assert_eq!(Vec::<&i32>::new(), graph.leading_to(&6));

        let mut neighbors = graph.neighbors(&2);
        let mut leading_to = graph.leading_to(&2);
        neighbors.sort();
        leading_to.sort();
        assert_eq!(neighbors, vec![&1,&3]);
        assert_eq!(leading_to, vec![&1,&4]);

        graph.add_vertex(SimpleVertex::new(2, "f"));
        let mut neighbors = graph.neighbors(&2);
        let mut leading_to = graph.leading_to(&2);
        neighbors.sort();
        leading_to.sort();
        assert_eq!(neighbors, vec![&1,&3]);
        assert_eq!(leading_to, vec![&1,&4]);
        assert_eq!(Some(&SimpleVertex::new(2, "f")), graph.get_vertex(&2));
    }

    #[test]
    fn mutable_getters() {
        let mut graph = AdjacencyGraph::new();
        graph.add_vertex(SimpleVertex::new(1, 4));
        graph.add_vertex(SimpleVertex::new(2, 5));
        graph.add_vertex(SimpleVertex::new(3, 6));
        graph.add_edge(DirectedEdge::new(1, 2, 3)).expect("Won't fail");
        graph.add_edge(DirectedEdge::new(2, 1, 3)).expect("Won't fail");

        assert_eq!(None, graph.get_mut_vertex(&4));
        assert_eq!(None, graph.get_mut_edge((&4, &4)));
        assert_eq!(None, graph.get_mut_edge((&2, &3)));

        let vertex = graph.get_mut_vertex(&2);
        assert_eq!(Some(&mut SimpleVertex::new(2, 5)), vertex);
        let vertex = vertex.unwrap();
        *vertex.get_mut_value() += 1;
        assert_eq!(Some(&SimpleVertex::new(2, 6)), graph.get_vertex(&2));

        let edge = graph.get_mut_edge((&1, &2));
        assert_eq!(Some(&mut DirectedEdge::new(1,2, 3)), edge);
        let edge = edge.unwrap();
        edge.set_weight(12);
        assert_eq!(Some(&DirectedEdge::new(1,2, 12)), graph.get_edge((&1, &2)));
    }
}
