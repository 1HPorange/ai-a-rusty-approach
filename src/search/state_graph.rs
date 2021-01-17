use std::{cmp::Ordering, collections::HashMap, hash::Hash};

pub struct StateGraph<'n, N, C> {
    _private: (),
    pub nodes: &'n [N],
    pub edges: StateGraphEdges<'n, N, C>,
}

impl<'n, N, C> StateGraph<'n, N, C> {
    pub fn new(nodes: &'n [N], edges: StateGraphEdges<'n, N, C>) -> StateGraph<'n, N, C>
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
pub struct StateGraphEdges<'n, N, C> {
    node_edges: HashMap<&'n N, HashMap<&'n N, C>>,
}

impl<'n, N, C> StateGraphEdges<'n, N, C> {
    pub fn from_bidirectional(transitions: &[(&'n N, &'n N, C)]) -> StateGraphEdges<'n, N, C>
    where
        N: Eq + Hash,
        C: PartialOrd + Copy,
    {
        let mut node_edges: HashMap<_, HashMap<_, C>> = HashMap::new();

        for &(src, dst, cost) in transitions {
            Self::insert_edge(&mut node_edges, src, dst, cost);
            Self::insert_edge(&mut node_edges, dst, src, cost);
        }

        Self { node_edges }
    }

    pub fn get_outgoing(&self, node: &'n N) -> Option<impl Iterator<Item = (&&'n N, &C)>>
    where
        N: Eq + Hash,
    {
        self.node_edges.get(node).map(|dst| dst.iter())
    }

    pub fn get_incoming(&'n self, node: &'n N) -> impl Iterator<Item = (&&'n N, &C)> + 'n
    where
        N: PartialEq,
    {
        self.node_edges.iter().flat_map(move |(src, dsts)| {
            dsts.iter()
                .filter(move |(dst, _)| dst == &&node)
                .map(move |(_, cost)| (src, cost))
        })
    }

    fn insert_edge(
        node_edges: &mut HashMap<&'n N, HashMap<&'n N, C>>,
        src: &'n N,
        dst: &'n N,
        cost: C,
    ) where
        N: Eq + Hash,
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
