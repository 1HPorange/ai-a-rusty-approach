use super::{state_graph::StateGraph, HasZero, SearchResult};
use std::{hash::Hash, iter, ops::Add};

pub fn depth_first<N, C, G>(
    graph: &StateGraph<N, C>,
    start: N,
    depth_limit: Option<usize>,
    is_goal_state: G,
) -> Option<SearchResult<N, C>>
where
    N: Eq + Hash + Copy,
    C: HasZero + Add<Output = C> + Copy,
    G: Fn(N) -> bool,
{
    let mut frontier = vec![SearchResult {
        node: start,
        cost: C::zero(),
        path: vec![start],
    }];

    while let Some(parent) = frontier.pop() {
        if is_goal_state(parent.node) {
            return Some(parent);
        }

        if depth_limit
            .map(|limit| parent.path.len() >= limit)
            .unwrap_or(false)
        {
            continue;
        }

        frontier.extend(
            graph
                .edges
                .get_outgoing(parent.node)
                .unwrap()
                .filter(|(dst, _)| !parent.path.contains(dst))
                .map(|(dst, cost)| SearchResult {
                    node: *dst,
                    cost: parent.cost + *cost,
                    path: parent
                        .path
                        .iter()
                        .copied()
                        .chain(iter::once(*dst))
                        .collect(),
                }),
        );
    }

    None
}
