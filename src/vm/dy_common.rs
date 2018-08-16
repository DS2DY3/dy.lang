use std::ops::{Index, IndexMut};


pub struct DyNode<T> {
    data: T,
    idx: usize,
    parent: Option<usize>,
    children: Vec<usize>,
}

pub struct DyTree<T> {
    nodes: Vec<DyNode<T>>,
    root: usize,
}


impl<T> Index<NodeId> for DyTree<T> {
    type Output = DyNode<T>;

    fn index(&self, index: usize) -> &DyNode<T> {
        &self.nodes[index]
    }
}

impl<T> IndexMut<NodeId> for DyTree<T> {
    fn index_mut(&mut self, index: usize) -> &mut DyNode<T> {
        &mut self.nodes[index]
    }
}

impl<T> DyTree<T> {

    
    pub fn get(&self, index: usize) -> Option<&DyNode<T>> {
        self.nodes.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut DyNode<T>> {
        self.nodes.get_mut(index)
    }


}
