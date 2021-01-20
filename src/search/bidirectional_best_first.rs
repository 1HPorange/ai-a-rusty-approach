use super::{state_graph::StateGraph, HasZero, SearchResult};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    iter,
    ops::Add,
};

pub fn bidirectional_best_first<'n, N, C, F>(
    graph: &StateGraph<'n, N, C>,
    start: &'n N,
    end: &'n N,
    accept_solution: F,
) -> Option<SearchResult<'n, N, C>>
where
    N: Eq + Hash,
    C: Ord + HasZero + PartialOrd + Add<Output = C> + Copy,
    F: Fn(&SearchResult<'n, N, C>) -> bool,
{
    let mut forward_frontier = BinaryHeap::new();
    forward_frontier.push(Reverse(SearchResult {
        node: start,
        cost: C::zero(),
        path: vec![start],
    }));

    let mut forward_reached = HashMap::new();
    forward_reached.insert(start, C::zero());

    let mut backward_frontier = BinaryHeap::new();
    backward_frontier.push(Reverse(SearchResult {
        node: end,
        cost: C::zero(),
        path: vec![end],
    }));

    let mut backward_reached = HashMap::new();
    backward_reached.insert(end, C::zero());

    let mut solution = None;

    while let (Some(Reverse(forward)), Some(Reverse(backward))) =
        (forward_frontier.peek(), backward_frontier.peek())
    {
        if forward.cost < backward.cost {
            let Reverse(frontier) = forward_frontier.pop().unwrap();

            advance(
                graph,
                end,
                frontier,
                &mut forward_frontier,
                &mut forward_reached,
                &backward_frontier,
                &backward_reached,
                &mut solution,
            );
        } else {
            let Reverse(frontier) = backward_frontier.pop().unwrap();

            advance(
                graph,
                end,
                frontier,
                &mut backward_frontier,
                &mut backward_reached,
                &forward_frontier,
                &forward_reached,
                &mut solution,
            );
        };

        match solution.as_ref() {
            Some(s) if accept_solution(s) => return solution,
            _ => {}
        }
    }

    None
}

fn advance<'n, N, C>(
    graph: &StateGraph<'n, N, C>,
    end: &'n N,
    frontier: SearchResult<'n, N, C>,
    current_frontier: &mut BinaryHeap<Reverse<SearchResult<'n, N, C>>>,
    current_reached: &mut HashMap<&'n N, C>,
    other_frontier: &BinaryHeap<Reverse<SearchResult<'n, N, C>>>,
    other_reached: &HashMap<&'n N, C>,
    solution: &mut Option<SearchResult<'n, N, C>>,
) where
    N: Eq + Hash,
    C: Ord + Add<Output = C> + Copy,
{
    for (child, step_cost) in graph.edges.get_outgoing(frontier.node).unwrap() {
        let child_cost = frontier.cost + *step_cost;
        if current_reached
            .get(child)
            .map(|stored_cost| child_cost < *stored_cost)
            .unwrap_or(true)
        {
            match other_reached.get(child) {
                Some(other_cost)
                    if solution
                        .as_ref()
                        .map(|s| child_cost + *other_cost < s.cost)
                        .unwrap_or(true) =>
                {
                    *solution = Some(SearchResult {
                        node: end,
                        cost: child_cost + *other_cost,
                        path: frontier
                            .path
                            .iter()
                            .copied()
                            .chain(
                                other_frontier
                                    .iter()
                                    .find(|Reverse(f)| f.node == *child)
                                    .unwrap()
                                    .0
                                    .path
                                    .iter()
                                    .copied()
                                    .rev(),
                            )
                            .collect(),
                    })
                }
                _ => {}
            }

            current_reached.insert(child, child_cost);
            current_frontier.push(Reverse(SearchResult {
                node: child,
                cost: child_cost,
                path: frontier
                    .path
                    .iter()
                    .copied()
                    .chain(iter::once(*child))
                    .collect(),
            }));
        }
    }
}
