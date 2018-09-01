use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref, RefMut};
use std::fmt;


type WeakRef<T> = Weak<RefCell<DyNode<T>>>;
type RcRef<T> = Rc<RefCell<DyNode<T>>>;

pub struct DyNode<T> {
    data: T,
    parent: WeakRef<T>,
    first_child: RcRef<T>,
    next_sibling: RcRef<T>,
    root: WeakRef<T>,
}

pub struct DyRef<T> (RcRef<T>);

impl<T> Clone for DyRef<T> {
    fn clone(&self) -> Self {
        DyRef(Rc::clone(&self.0))
    }
}

impl<T> PartialEq for DyRef<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

