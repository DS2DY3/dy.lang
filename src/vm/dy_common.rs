use std::ops::{Index, IndexMut};
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;


pub struct DyTree<T> {
    nodes: Vec<DyNode<T>>
}

pub struct DyNode<T> {
    pub data: T,
    pub index: usize,
}

pub struct DyRef<T> {
    index: usize,
    parent: Weak<DyRef<T>>,
    first_child: Rc<DyRef<T>>,
    next_sibling: Rc<DyRef<T>>,
    tree: Weak<DyNodeTree<T>>,
}