use super::{state_graph::StateGraph, HasZero, SearchResult};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    iter,
    ops::Add,
};

pub fn best_first<'n, N, C, G>(
    graph: &StateGraph<'n, N, C>,
    start: &'n N,
    is_goal_state: G,
) -> Option<SearchResult<'n, N, C>>
where
    N: Eq + Hash,
    C: Ord + HasZero + PartialOrd + Add<Output = C> + Copy,
    G: Fn(&'n N) -> bool,
{
    let mut frontier = BinaryHeap::new();
    frontier.push(Reverse(SearchResult {
        node: start,
        cost: C::zero(),
        path: vec![start],
    }));

    let mut reached = HashMap::new();
    reached.insert(start, C::zero());

    while let Some(Reverse(parent)) = frontier.pop() {
        if is_goal_state(parent.node) {
            return Some(parent);
        }

        for (child_node, cost) in graph.edges.get_outgoing(parent.node).unwrap() {
            let cost = *cost + reached[parent.node];

            match reached.get_mut(child_node) {
                Some(stored_cost) => {
                    if *stored_cost > cost {
                        *stored_cost = cost;
                        frontier.push(Reverse(SearchResult {
                            node: child_node,
                            cost,
                            path: parent
                                .path
                                .iter()
                                .copied()
                                .chain(iter::once(*child_node))
                                .collect(),
                        }));
                    }
                }
                None => {
                    reached.insert(child_node, cost);
                    frontier.push(Reverse(SearchResult {
                        node: child_node,
                        cost,
                        path: parent
                            .path
                            .iter()
                            .copied()
                            .chain(iter::once(*child_node))
                            .collect(),
                    }));
                }
            }
        }
    }

    None
}
