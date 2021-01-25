use super::{state_graph::StateGraph, HasZero, SearchResult};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    iter,
    ops::Add,
};

pub fn best_first<N, C, G, H>(
    graph: &StateGraph<N, C>,
    start: N,
    is_goal_state: G,
    heuristic: H,
) -> Option<SearchResult<N, C>>
where
    N: Eq + Hash + Copy,
    C: Ord + HasZero + PartialOrd + Add<Output = C> + Copy,
    G: Fn(N) -> bool,
    H: Fn(&SearchResult<N, C>) -> isize,
{
    let mut frontier = BinaryHeap::new();
    let first = SearchResult {
        node: start,
        cost: C::zero(),
        path: vec![start],
    };
    frontier.push((Reverse(heuristic(&first)), first));

    let mut reached = HashMap::new();
    reached.insert(start, C::zero());

    while let Some((_, parent)) = frontier.pop() {
        if is_goal_state(parent.node) {
            return Some(parent);
        }

        for (child_node, cost) in graph.edges.get_outgoing(parent.node).unwrap() {
            let cost = *cost + reached[&parent.node];

            match reached.get_mut(child_node) {
                Some(stored_cost) => {
                    if *stored_cost > cost {
                        *stored_cost = cost;
                        let candidate = SearchResult {
                            node: *child_node,
                            cost,
                            path: parent
                                .path
                                .iter()
                                .copied()
                                .chain(iter::once(*child_node))
                                .collect(),
                        };
                        frontier.push((Reverse(heuristic(&candidate)), candidate));
                    }
                }
                None => {
                    reached.insert(*child_node, cost);
                    let candidate = SearchResult {
                        node: *child_node,
                        cost,
                        path: parent
                            .path
                            .iter()
                            .copied()
                            .chain(iter::once(*child_node))
                            .collect(),
                    };
                    frontier.push((Reverse(heuristic(&candidate)), candidate));
                }
            }
        }
    }

    None
}
