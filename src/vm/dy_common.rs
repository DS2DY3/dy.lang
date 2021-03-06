use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref, RefMut};
use std::fmt;
use std::ops::{Deref, DerefMut};
use std::default::Default;

// copy from: https://github.com/RazrFalcon/rctree/
// DyRef 最多有一个parent节点

type WeakRef<T> = Weak<RefCell<DyNode<T>>>;
type RcRef<T> = Rc<RefCell<DyNode<T>>>;

pub struct DyNode<T> {
    data: T,
    parent: WeakRef<T>,
    first_child: Option<RcRef<T>>,
    last_child: WeakRef<T>,  // remove?
    next_sibling: Option<RcRef<T>>,
    pre_sibling: WeakRef<T>,
}

impl<T> Deref for DyNode<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        return &self.data;
    }
}

// 解引用
impl<T> DerefMut for DyNode<T> {
    fn deref_mut(&mut self) -> &mut T {
        return &mut self.data;
    }
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


impl<T: fmt::Debug> fmt::Debug for DyRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&*self.borrow(), f)
    }
}

impl<T: fmt::Display> fmt::Display for DyRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&*self.borrow(), f)
    }
}


impl<T: Default> Default for DyRef<T> {
    fn default() -> DyRef<T> {
        DyRef::new(T::default())
    }
}

impl<T> DyRef<T> {
    pub fn new(data: T) ->DyRef<T> {
        DyRef(Rc::new(RefCell::new(DyNode{
            data,
            parent: Weak::new(),
            first_child: None,
            last_child: Weak::new(),
            next_sibling: None,
            pre_sibling: Weak::new(),
        })))
    }


//    pub fn replace(&mut self, data: T) {
//        self.0.
//    }


    pub fn borrow(&self) -> Ref<T> {
        // 不导入 Borrow 的时候是 RefCell 的，导入 Borrow 就是 Rc as Borrow
        // self.0.borrow()
        Ref::map(self.0.borrow(), |v| &v.data)
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        RefMut::map(self.0.borrow_mut(), |v| &mut v.data)
    }

    // copy link
    pub fn deep_copy(&self) -> DyRef<T> where T: Clone{
        let mut root = self.make_copy();
        DyRef::_deep_copy(&mut root, self);
        root
    }

    fn _deep_copy(parent: &mut DyRef<T>, node: &DyRef<T>) where T: Clone {
        for child in node.children() {
            let mut new_node = child.make_copy();
            parent.append(&new_node);

            if child.has_children() {
                DyRef::_deep_copy(&mut new_node, &child);
            }
        }
    }

    // only copy data
    pub fn make_copy(&self) -> DyRef<T> where T: Clone {
        DyRef::new(self.borrow().clone())
    }

    // 访问节点

    pub fn root(&self) -> DyRef<T> {
        match self.0.borrow().parent.upgrade() {
            None => self.clone(),
            Some(ref parent) => DyRef(Rc::clone(parent)).root(),
        }
    }

    pub fn parent(&self) -> Option<DyRef<T>> {
        let op_rc = self.0.borrow().parent.upgrade();
        match op_rc {
            None => None,
            Some(ref value) => Some(DyRef(Rc::clone(value))),
        }
    }

    pub fn first_child(&self) -> Option<DyRef<T>> {
        let op_rc = &self.0.borrow().first_child;
        match op_rc {
            None => None,
            Some(ref value) => Some(DyRef(Rc::clone(value))),
        }
    }

    pub fn last_child(&self) -> Option<DyRef<T>> {
        let op_rc = self.0.borrow().last_child.upgrade();
        match op_rc {
            None => None,
            Some(ref value) => Some(DyRef(Rc::clone(value))),
        }
    }

    pub fn next_sibling(&self) -> Option<DyRef<T>> {
        let op_rc = &self.0.borrow().next_sibling;
        match op_rc {
            None => None,
            Some(ref value) => Some(DyRef(Rc::clone(value))),
        }
    }

    pub fn pre_sibling(&self) -> Option<DyRef<T>> {
        let op_rc = self.0.borrow().pre_sibling.upgrade();
        match op_rc {
            None => None,
            Some(ref value) => Some(DyRef(Rc::clone(value))),
        }
    }

    pub fn has_children(&self) -> bool {
        self.first_child().is_some()
    }


    // 插入操作

    pub fn append(&self, child: &DyRef<T>) {
        assert!(*self != *child, "can't append to self");
        child.detach();
        let mut self_node = self.0.borrow_mut();
        let mut child_borrow = child.0.borrow_mut();
        if self_node.last_child.upgrade().is_some() {
            let last_child_rc = self_node.last_child.upgrade().unwrap();
            last_child_rc.borrow_mut().next_sibling = Some(Rc::clone(&child.0));
            child_borrow.pre_sibling = Rc::downgrade(&last_child_rc);
        }
        else {
            self_node.first_child = Some(Rc::clone(&child.0));
        }
        self_node.last_child = Rc::downgrade(&child.0);
        child_borrow.parent = Rc::downgrade(&self.0);
    }

    pub fn prepend(&self, child: &DyRef<T>) {
        assert!(*self != *child, "can't prepend to self");
        child.detach();
        let mut self_node = self.0.borrow_mut();
        let mut child_borrow = child.0.borrow_mut();
        // 第1版本
        // let mut first_child_rc_op = self_node.first_child;
        //if let Some(ref mut first_child_rc) = first_child_rc_op {
        // 第2版本
        // if let Some(ref mut first_child_rc) = self_node.first_child.as_mut() {
        
        // 第3版本
        // let first_child_rc_op = &mut self_node.first_child;
        // if let Some(ref mut first_child_rc) = first_child_rc_op {

        // 第4版本
        // 总结：自身是引用成员变量也要借用访问
        if self_node.first_child.is_some() {
            let first_child_rc = self_node.first_child.as_mut().unwrap();
            first_child_rc.borrow_mut().pre_sibling = Rc::downgrade(&child.0);
            child_borrow.next_sibling = Some(Rc::clone(first_child_rc));
        }
        else {
            self_node.last_child = Rc::downgrade(&child.0);
        }
        self_node.first_child = Some(Rc::clone(&child.0));
        child_borrow.parent = Rc::downgrade(&self.0);
    }

    pub fn insert_after(&self, sibling: &DyRef<T>) {
        assert!(*self != *sibling, "can't insert to self");
        sibling.detach();
        let mut self_node = self.0.borrow_mut();
        let mut parent_rc_op = self_node.parent.upgrade();
        if let Some(ref mut parent_rc) = parent_rc_op {
            let mut parent_mut = parent_rc.borrow_mut();
            if Rc::ptr_eq(&parent_mut.last_child.upgrade().unwrap(), &self.0) {
                parent_mut.last_child = Rc::downgrade(&sibling.0);
            }
            sibling.0.borrow_mut().parent = Rc::downgrade(parent_rc);
        }
        self_node.next_sibling = Some(Rc::clone(&sibling.0));
        sibling.0.borrow_mut().pre_sibling = Rc::downgrade(&self.0)
    }

    pub fn insert_before(&self, sibling: &DyRef<T>) {
        assert!(*self != *sibling, "can't insert to self");
        sibling.detach();
        let mut self_node = self.0.borrow_mut();
        let mut parent_rc_op = self_node.parent.upgrade();
        if let Some(ref mut parent_rc) = parent_rc_op {
            let mut parent_mut = parent_rc.borrow_mut();
            if Rc::ptr_eq(parent_mut.first_child.as_mut().unwrap(), &self.0) {
                parent_mut.first_child = Some(Rc::clone(&sibling.0));
            }
            sibling.0.borrow_mut().parent = Rc::downgrade(parent_rc);
        }
        self_node.pre_sibling = Rc::downgrade(&sibling.0);
        sibling.0.borrow_mut().next_sibling = Some(Rc::clone(&self.0));
    }

    // remove from parent and reconnect sibling
    pub fn detach(&self) {
        let mut self_node = self.0.borrow_mut();
        let mut parent_rc_op = self_node.parent.upgrade();
        self_node.parent = Weak::new();
        let mut pre_sibling_rc_op = &mut self_node.pre_sibling.upgrade();

        // remove frome parent
        if let Some(ref mut parent_rc) = parent_rc_op {
            if self_node.next_sibling.is_none() {
                if pre_sibling_rc_op.is_some() {
                    parent_rc.borrow_mut().last_child = self_node.pre_sibling.clone();
                }
                else {
                    parent_rc.borrow_mut().last_child = Weak::new();
                }
            }
            if pre_sibling_rc_op.is_none() {
                if self_node.next_sibling.is_some() {
                    let next_sibling_rc = self_node.next_sibling.as_mut().unwrap();
                    parent_rc.borrow_mut().first_child = Some(Rc::clone(next_sibling_rc));
                }
                else {
                    parent_rc.borrow_mut().first_child = None;
                }
            }
        }
        // reconnect sibling
        if self_node.next_sibling.is_some() {
            let next_sibling_rc = self_node.next_sibling.as_mut().unwrap();
            if let Some(ref mut pre_sibling_rc) = pre_sibling_rc_op {
                next_sibling_rc.borrow_mut().pre_sibling = Rc::downgrade(pre_sibling_rc);
                pre_sibling_rc.borrow_mut().next_sibling = Some(Rc::clone(next_sibling_rc));
            }
            else {
                next_sibling_rc.borrow_mut().pre_sibling = Weak::new();
            }
        }
        else if let Some(ref mut pre_sibling_rc) = pre_sibling_rc_op {
            pre_sibling_rc.borrow_mut().next_sibling = None;
        }

        self_node.pre_sibling = Weak::new();
        self_node.next_sibling = None;
    }

    // 迭代器
    pub fn acestors(&self) -> Ancestors<T> {
        Ancestors(Some(self.clone()))
    }

    // Returns an iterator of nodes to this node and the siblings before it.
    ///
    /// Includes the current node.
    pub fn preceding_siblings(&self) -> PrecedingSiblings<T> {
        PrecedingSiblings(Some(self.clone()))
    }

    /// Returns an iterator of nodes to this node and the siblings after it.
    ///
    /// Includes the current node.
    pub fn following_siblings(&self) -> FollowingSiblings<T> {
        FollowingSiblings(Some(self.clone()))
    }

    /// Returns an iterator of nodes to this node's children.
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutability borrowed.
    pub fn children(&self) -> Children<T> {
        Children(self.first_child())
    }

    /// Returns an iterator of nodes to this node's children, in reverse order.
    ///
    /// # Panics
    ///
    /// Panics if the node is currently mutability borrowed.
    pub fn reverse_children(&self) -> ReverseChildren<T> {
        ReverseChildren(self.last_child())
    }


    pub fn traverse(&self, is_depth_first: bool, is_reverse: bool) {
        Traverse::new(self.clone(), is_depth_first, is_reverse);
    }

}

pub mod iterator {
    pub use super::Ancestors;
    pub use super::PrecedingSiblings;
    pub use super::FollowingSiblings;
    pub use super::Children;
    pub use super::ReverseChildren;
    pub use super::Traverse;
}

macro_rules! impl_node_iterator {
    ($name: ident, $next: expr) => {
        impl<T> Iterator for $name<T> {
            type Item = DyRef<T>;

            /// # Panics
            ///
            /// Panics if the node about to be yielded is currently mutability borrowed.
            fn next(&mut self) -> Option<Self::Item> {
                match self.0.take() {
                    Some(node) => {
                        self.0 = $next(&node);
                        Some(node)
                    }
                    None => None
                }
            }
        }
    }
}

/// An iterator of nodes to the ancestors a given node.
pub struct Ancestors<T>(Option<DyRef<T>>);
impl_node_iterator!(Ancestors, |node: &DyRef<T>| node.parent());

/// An iterator of nodes to the siblings before a given node.
pub struct PrecedingSiblings<T>(Option<DyRef<T>>);
impl_node_iterator!(PrecedingSiblings, |node: &DyRef<T>| node.pre_sibling());

/// An iterator of nodes to the siblings after a given node.
pub struct FollowingSiblings<T>(Option<DyRef<T>>);
impl_node_iterator!(FollowingSiblings, |node: &DyRef<T>| node.next_sibling());

/// An iterator of nodes to the children of a given node.
pub struct Children<T>(Option<DyRef<T>>);
impl_node_iterator!(Children, |node: &DyRef<T>| node.next_sibling());

/// An iterator of nodes to the children of a given node, in reverse order.
pub struct ReverseChildren<T>(Option<DyRef<T>>);
impl_node_iterator!(ReverseChildren, |node: &DyRef<T>| node.pre_sibling());



pub struct Traverse<T> {
    pub is_reverse: bool,
    pub is_depth_first: bool,
    root: DyRef<T>,
    cur: Option<DyRef<T>>,
}

impl<T> Traverse<T> {

    fn new(root: DyRef<T>, is_depth_first: bool, is_reverse: bool) -> Traverse<T> {
        let cur = Some(root.clone());
        let traverse = Traverse {
            is_depth_first,
            is_reverse,
            root,
            cur,
        };
        return traverse;
    }

    fn get_next(&mut self, cur_node: DyRef<T>) -> Option<DyRef<T>> {
        let mut parent = cur_node;
        loop {
            let mut first_child: Option<DyRef<T>>;
            if self.is_depth_first && self.is_reverse {
                first_child = parent.last_child();
            }
            else if self.is_depth_first && !self.is_reverse {
                first_child = parent.first_child();
            }
            else if self.is_reverse {
                if parent == self.root {
                    return None;
                }
                first_child = parent.pre_sibling();
            }
            else {
                if parent == self.root {
                    return None;
                }
                first_child = parent.next_sibling();
            }
            if first_child.is_none() {
                let mut next_sibling: Option<DyRef<T>>;
                if self.is_depth_first && self.is_reverse {
                    if parent == self.root {
                        return None;
                    }
                    next_sibling = parent.pre_sibling();
                }
                else if self.is_depth_first && !self.is_reverse {
                    if parent == self.root {
                        return None;
                    }
                    next_sibling = parent.next_sibling();
                }
                else if self.is_reverse {
                    next_sibling = parent.last_child();
                }
                else {
                    next_sibling = parent.first_child();
                }
                if next_sibling.is_none() {
                    parent = parent.parent().unwrap();
                    if parent == self.root {
                        return None;
                    }
                } else {
                    return next_sibling;
                }
            } else {
                return first_child;
            }
        }
        return None;
    }
}

impl<T> Iterator for Traverse<T> {
    type Item=DyRef<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cur.take() {
            Some(node) => {
                self.cur = self.get_next(node.clone());
                Some(node)
            }
            None => None
        }
    }
}


// ------------------------ test --------------------------

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parent() {
        let mut parent = DyRef::new(1);
        let child = DyRef::new(2);
        parent.append(&child);
        assert_eq!(child, parent.first_child().unwrap());
        assert_eq!(child.parent().unwrap(), parent);
        assert_eq!(parent.parent(), None);
    }

    #[test]
    fn test_root() {
        let mut parent = DyRef::new(1);
        let child = DyRef::new(2);
        parent.append(&child);
        assert_eq!(child.root(), parent);
        assert_eq!(parent.root(), parent);
    }

    #[test]
    fn test_first_child() {
        let mut parent = DyRef::new(1);
        let child = DyRef::new(2);
        parent.append(&child);
        assert_eq!(child, parent.first_child().unwrap());
        parent.prepend(&child);
        assert_eq!(child, parent.first_child().unwrap());
    }

    #[test]
    fn test_last_child() {
        let mut parent = DyRef::new(1);
        let child = DyRef::new(2);
        parent.append(&child);
        assert_eq!(child, parent.last_child().unwrap());
        parent.prepend(&child);
        assert_eq!(child, parent.last_child().unwrap());
    }

    #[test]
    fn test_next_sibling() {
        let mut parent = DyRef::new(1);
        let child = DyRef::new(2);
        parent.insert_after(&child);
        assert_eq!(child, parent.next_sibling().unwrap());
        assert_eq!(child.pre_sibling().unwrap(), parent);
        child.detach();
        assert_eq!(parent.next_sibling(), None);
        assert_eq!(child.pre_sibling(), None);
    }

    #[test]
    fn test_pre_sibling() {
        let mut parent = DyRef::new(1);
        let child = DyRef::new(2);
        parent.insert_before(&child);
        assert_eq!(child, parent.pre_sibling().unwrap());
        assert_eq!(child.next_sibling().unwrap(), parent);
        child.detach();
        assert_eq!(parent.pre_sibling(), None);
        assert_eq!(child.next_sibling(), None);
    }

    #[test]
    fn test_detach() {
        let mut parent = DyRef::new(1);
        let child = DyRef::new(2);
        parent.append(&child);
        let pre = DyRef::new(3);
        let next = DyRef::new(4);
        child.insert_before(&pre);
        child.insert_after(&next);
        child.detach();
        assert_eq!(child.parent().is_none(), true);
        assert_eq!(pre.next_sibling().unwrap(), next);
        assert_eq!(next.pre_sibling().unwrap(), pre);
    }

    #[test]
    fn test_copy() {
        let mut parent = DyRef::new(1);
        let child = DyRef::new(2);
        parent.append(&child);
        let copy_parent = parent.deep_copy();
        assert_eq!(copy_parent.has_children(), true);
        assert_ne!(copy_parent.first_child().unwrap(), child);
        let copy_child = copy_parent.first_child().unwrap();
        assert_eq!(*child.borrow_mut(), *copy_child.borrow_mut());
    }

    #[test]
    fn test_clone() {
        let mut parent = DyRef::new(1);
        let clone_parent = parent.clone();
        let child = DyRef::new(2);
        parent.append(&child);
        assert_eq!(clone_parent, parent);
        assert_eq!(clone_parent.first_child().unwrap(), child);
    }

    #[test]
    fn test_iter() {
        let mut parent = DyRef::new(1);
        let child = DyRef::new(2);
        parent.append(&child);
        parent.append(&DyRef::new(3));
        child.append(&DyRef::new(4));
        let next_child = child.next_sibling().unwrap();
        next_child.prepend(&DyRef::new(5));
        next_child.insert_before(&DyRef::new(6));
        next_child.insert_after(&DyRef::new(7));

        let children = parent.children();
        assert_eq!(children.next().unwrap(), child);
        assert_eq!(children.next().unwrap().borrow(), &6);
        assert_eq!(children.next().unwrap().borrow(), &3);
        assert_eq!(children.next().unwrap().borrow(), &7);

    }
}

