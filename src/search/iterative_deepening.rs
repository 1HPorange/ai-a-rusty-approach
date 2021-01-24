use super::{depth_first, state_graph::StateGraph, HasZero, SearchResult};
use std::hash::Hash;
use std::ops::Add;

pub fn iterative_deepening<'n, N, C, G>(
    graph: &StateGraph<'n, N, C>,
    start: &'n N,
    depth_limit: Option<usize>,
    is_goal_state: G,
) -> Option<SearchResult<&'n N, C>>
where
    N: Eq + Hash,
    C: HasZero + Add<Output = C> + Copy,
    G: Fn(&'n N) -> bool,
{
    let mut max_depth = 0;

    while let Some(depth) = depth_limit
        .filter(|limit| max_depth <= *limit)
        .or(Some(max_depth))
    {
        match depth_first(graph, start, Some(depth), &is_goal_state) {
            result @ Some(_) => return result,
            _ => {}
        }

        max_depth += 1;
    }

    None
}
