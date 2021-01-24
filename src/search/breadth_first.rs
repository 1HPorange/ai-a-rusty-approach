use super::{state_graph::StateGraph, HasZero, SearchResult};
use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
    ops::Add,
};

pub fn breadth_first<'n, N, C, G>(
    graph: &StateGraph<'n, N, C>,
    start: &'n N,
    is_goal_state: G,
) -> Option<SearchResult<&'n N, C>>
where
    N: Eq + Hash,
    C: HasZero + Add<Output = C> + Copy,
    G: Fn(&'n N) -> bool,
{
    if is_goal_state(start) {
        return Some(SearchResult {
            node: start,
            cost: C::zero(),
            path: vec![start],
        });
    }

    let mut reached = graph
        .nodes
        .iter()
        .map(|node| {
            (
                node,
                SearchResult {
                    node,
                    cost: C::zero(),
                    path: vec![node],
                },
            )
        })
        .collect::<HashMap<_, _>>();

    let mut node_queue = VecDeque::new();
    node_queue.push_back(start);

    while let Some(parent) = node_queue.pop_front() {
        let parent_state = &reached[parent];
        let parent_cost = parent_state.cost;
        let parent_path = parent_state.path.clone();

        for (child, cost) in graph.edges.get_outgoing(parent).unwrap() {
            let child_state = SearchResult {
                node: *child,
                cost: parent_cost + *cost,
                path: {
                    let mut new_path = parent_path.clone();
                    new_path.push(child);
                    new_path
                },
            };

            if is_goal_state(child) {
                return Some(child_state);
            }

            *reached.get_mut(child).unwrap() = child_state;

            node_queue.push_back(child);
        }
    }

    None
}
