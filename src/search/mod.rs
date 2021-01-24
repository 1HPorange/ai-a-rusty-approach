pub mod examples;
pub mod state_graph;

mod best_first;
mod bidirectional_best_first;
mod breadth_first;
mod depth_first;
mod iterative_deepening;

pub use best_first::*;
pub use bidirectional_best_first::*;
pub use breadth_first::*;
pub use depth_first::*;
pub use iterative_deepening::*;

pub trait HasZero {
    fn zero() -> Self;
}

impl HasZero for u16 {
    fn zero() -> Self {
        0
    }
}

pub struct SearchResult<N, C> {
    pub node: N,
    pub cost: C,
    pub path: Vec<N>,
}

impl<N, C> PartialEq for SearchResult<N, C>
where
    C: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl<N, C> Eq for SearchResult<N, C> where C: Eq {}

impl<N, C> PartialOrd for SearchResult<N, C>
where
    C: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl<N, C> Ord for SearchResult<N, C>
where
    C: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}
