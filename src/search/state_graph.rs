use std::{cmp::Ordering, collections::HashMap, hash::Hash};

pub struct StateGraph<N, C> {
    _private: (),
    pub nodes: Vec<N>,
    pub edges: StateGraphEdges<N, C>,
}

impl<N, C> StateGraph<N, C> {
    pub fn new(nodes: Vec<N>, edges: StateGraphEdges<N, C>) -> StateGraph<N, C>
    where
        N: PartialEq,
    {
        // Assert edge source nodes are all in `nodes`
        assert!(
            edges.node_edges.keys().all(|src| nodes.contains(src)),
            "nodes/edges mismatch"
        );

        // Assert edge target nodes are all in `nodes`
        assert!(
            edges
                .node_edges
                .values()
                .flat_map(|dst| dst.keys())
                .all(|dst| nodes.contains(dst)),
            "nodes/edges mismatch"
        );

        Self {
            _private: (),
            nodes,
            edges,
        }
    }

    pub fn nodes(&self) -> &[N] {
        &self.nodes
    }
}

#[derive(Debug)]
pub struct StateGraphEdges<N, C> {
    node_edges: HashMap<N, HashMap<N, C>>,
}

impl<N, C> StateGraphEdges<N, C> {
    pub fn from_bidirectional(transitions: &[(N, N, C)]) -> StateGraphEdges<N, C>
    where
        N: Eq + Hash + Copy,
        C: PartialOrd + Copy,
    {
        let mut node_edges: HashMap<_, HashMap<_, C>> = HashMap::new();

        for &(src, dst, cost) in transitions {
            Self::insert_edge(&mut node_edges, src, dst, cost);
            Self::insert_edge(&mut node_edges, dst, src, cost);
        }

        Self { node_edges }
    }

    pub fn get_outgoing(&self, node: N) -> Option<impl Iterator<Item = (&N, &C)>>
    where
        N: Eq + Hash,
    {
        self.node_edges.get(&node).map(|dst| dst.iter())
    }

    pub fn get_incoming(&self, node: N) -> impl Iterator<Item = (&N, &C)>
    where
        N: PartialEq + Copy,
    {
        self.node_edges.iter().flat_map(move |(src, dsts)| {
            dsts.iter()
                .filter(move |(dst, _)| dst == &&node)
                .map(move |(_, cost)| (src, cost))
        })
    }

    fn insert_edge(node_edges: &mut HashMap<N, HashMap<N, C>>, src: N, dst: N, cost: C)
    where
        N: Eq + Hash + Copy,
        C: PartialOrd + Copy,
    {
        node_edges
            .entry(src)
            .and_modify(|destinations| {
                destinations
                    .entry(dst)
                    .and_modify(|stored_cost| match stored_cost.partial_cmp(&stored_cost) {
                        Some(Ordering::Greater) => *stored_cost = cost,
                        _ => {}
                    })
                    .or_insert(cost);
            })
            .or_insert_with(|| {
                let mut destinations = HashMap::new();
                destinations.insert(dst, cost);
                destinations
            });
    }
}
