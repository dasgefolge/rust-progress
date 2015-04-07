use std::collections::hash_map::{self, HashMap};
use std::default::Default;
use std::hash::Hash;

use rustc_serialize::{Encodable, Encoder};

pub struct Graph<K: Eq + Hash, N, E> {
    nodes: HashMap<K, N>,
    edges: HashMap<K, HashMap<K, E>>
}

impl<K: Eq + Hash, N, E> Graph<K, N, E> {
    pub fn edges_from(&self, from_key: &K) -> hash_map::Iter<K, E> {
        self.edges[from_key].iter()
    }

    pub fn nodes(&self) -> hash_map::Iter<K, N> {
        self.nodes.iter()
    }
}

impl<K: Eq + Hash, N, E> Default for Graph<K, N, E> {
    fn default() -> Self {
        Graph { nodes: HashMap::new(), edges: HashMap::new() }
    }
}

impl<N: Encodable, E: Encodable> Encodable for Graph<String, N, E> {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_struct("Graph", 0, |encoder| {
            for (i, (key, node)) in self.nodes().enumerate() {
                try!(encoder.emit_struct_field(key, i, |encoder| {
                    encoder.emit_struct("Node", 0, |encoder| {
                        try!(encoder.emit_struct_field("data", 0, |encoder| node.encode(encoder)));
                        try!(encoder.emit_struct_field("edges", 1, |encoder| {
                            encoder.emit_struct("Edges", 0, |encoder| {
                                for (i, (target_key, edge)) in self.edges_from(key).enumerate() {
                                    try!(encoder.emit_struct_field(target_key, i, |encoder| edge.encode(encoder)));
                                }
                                Ok(())
                            })
                        }));
                        Ok(())
                    })
                }));
            }
            Ok(())
        })
    }
}
