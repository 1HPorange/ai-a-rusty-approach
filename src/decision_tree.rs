pub struct Extractor<E> {
    pub name: &'static str,
    pub extract: E,
}

pub struct Comparator<C> {
    pub name: &'static str,
    pub compare: C,
}

pub trait SplitNode<D, L> {
    fn resolve(&self, data: &D) -> &L;
}

pub enum DTreeNode<D, L> {
    Split(Box<dyn SplitNode<D, L>>),
    Leaf(L),
}

pub struct DTreeSplit<D, L, E, C, T> {
    pub extractor: Extractor<E>,
    pub comparator: Comparator<C>,
    pub value: T,
    pub left: DTreeNode<D, L>,
    pub right: DTreeNode<D, L>,
}

impl<D, L, E, C, T> SplitNode<D, L> for DTreeSplit<D, L, E, C, T>
where
    E: Fn(&D) -> &T,
    C: Fn(&T, &T) -> bool,
{
    fn resolve(&self, data: &D) -> &L {
        let other = (self.extractor.extract)(data);

        if (self.comparator.compare)(&self.value, other) {
            self.right.resolve(data)
        } else {
            self.left.resolve(data)
        }
    }
}

impl<D, L> SplitNode<D, L> for DTreeNode<D, L> {
    fn resolve(&self, data: &D) -> &L {
        match self {
            DTreeNode::Split(s) => s.resolve(data),
            DTreeNode::Leaf(l) => l,
        }
    }
}
