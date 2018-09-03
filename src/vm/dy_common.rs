use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref, RefMut};
use std::fmt;
use std::ops::{Deref, DerefMut};
//use std::borrow::{Borrow, BorrowMut};

// copy from: https://github.com/RazrFalcon/rctree/


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

    pub fn borrow(&self) -> Ref<T> {
        // 不导入 Borrow 的时候是 RefCell 的，导入 Borrow 就是 Rc as Borrow
        // self.0.borrow()
        Ref::map(self.0.borrow(), |v| &v.data)
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        RefMut::map(self.0.borrow_mut(), |v| &mut v.data)
    }

    // copy link
//    pub fn deep_copy(&self) -> DyRef<T> where T: Clone{
//        let mut root = self.make_copy();
//        DyRef::_deep_copy(&mut node, self);
//        root
//    }

//    fn _deep_copy(parent: &mut DyRef<T>, node: &DyRef<T>) where T: clone {
//        for mut child in node.children() {
//            let mut new_node = child.make_copy();
//            parent.append(new_node.clone());
//
//            if child.has_children() {
//                Node::_deep_copy(&mut new_node, &child);
//            }
//        }
//    }

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

    pub fn contains(&self, child: &DyRef<T>) -> bool {
        false
    }

    pub fn includes(&self, child: &DyRef<T>) -> bool {
        false
    }

    // 插入操作

    pub fn append(&self, child: &DyRef<T>) {
        assert!(*self != *child, "can't append to self");
        child.detach();
        let mut self_node = self.0.borrow_mut();
        let mut child_borrow = child.0.borrow_mut();
        let mut last_child_rc_op = self_node.last_child.upgrade();
        let mut first_child_rc_op = self_node.first_child;
        if let Some(ref mut last_child_rc) = last_child_rc_op {
            last_child_rc.borrow_mut().next_sibling = Some(Rc::clone(&child.0));
            child_borrow.pre_sibling = Rc::downgrade(last_child_rc);
        }
        else {
            self_node.first_child = Some(Rc::clone(&child.0));
        }
        self_node.last_child = Rc::downgrade(&child.0);
        let parent_rc_op = self_node.parent.upgrade();
        if let Some(ref parent_rc) = parent_rc_op {
            child_borrow.parent = Rc::downgrade(parent_rc)
        }

    }

    pub fn prepend(&self, child: &DyRef<T>) {
        assert!(*self != *child, "can't prepend to self");
        child.detach();
        let mut self_node = self.0.borrow_mut();
        let mut child_borrow = child.0.borrow_mut();
        let mut last_child_rc_op = self_node.last_child.upgrade();
        let mut first_child_rc_op = self_node.first_child;
        if let Some(ref mut first_child_rc) = first_child_rc_op {
            first_child_rc.borrow_mut().pre_sibling = Rc::downgrade(&child.0);
            child_borrow.next_sibling = Some(Rc::clone(first_child_rc));
        }
        else {
            self_node.last_child = Rc::downgrade(&child.0);
        }
        self_node.first_child = Some(Rc::clone(&child.0));
        let parent_rc_op = self_node.parent.upgrade();
        if let Some(ref parent_rc) = parent_rc_op {
            child_borrow.parent = Rc::downgrade(parent_rc)
        }
    }

    pub fn insert_after(&self, sibling: &DyRef<T>) {
        assert!(*self != *sibling, "can't insert to self");
        sibling.detach();
        let mut self_node = self.0.borrow_mut();
        let mut parent_rc_op = self_node.parent.upgrade();
        if let Some(ref mut parent_rc) = parent_rc_op {
            let mut parent_mut = parent_rc.borrow_mut();
            if Rc::ptr_eq(&parent_mut.last_child.upgrade().unwrap(), &self.0) {
                parent_rc.borrow_mut().last_child = Rc::downgrade(&sibling.0);
            }
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
            if Rc::ptr_eq(&parent_mut.first_child.unwrap(), &self.0) {
                parent_mut.first_child = Some(Rc::clone(&sibling.0));
            }
        }
        self_node.pre_sibling = Rc::downgrade(&sibling.0);
        sibling.0.borrow_mut().next_sibling = Some(Rc::clone(&self.0));
    }

    // remove from parent and reconnect sibling
    pub fn detach(&self) {
        let mut self_node = self.0.borrow_mut();
        let mut parent_rc_op = self_node.parent.upgrade();
        self_node.parent = Weak::new();
        let mut next_sibling_rc_op = self_node.next_sibling;
        self_node.next_sibling = None;
        let mut pre_sibling_rc_op = self_node.pre_sibling.upgrade();
        self_node.pre_sibling = Weak::new();

        // remove frome parent
        if let Some(ref mut parent_rc) = parent_rc_op {
            if next_sibling_rc_op.is_none() {
                if let Some(ref pre_sibling_rc) = pre_sibling_rc_op {
                    parent_rc.borrow_mut().last_child = Rc::downgrade(pre_sibling_rc);
                }
                else {
                    parent_rc.borrow_mut().last_child = Weak::new();
                }
            }
            if pre_sibling_rc_op.is_none() {
                if let Some(ref next_sibling_rc) = next_sibling_rc_op {
                    parent_rc.borrow_mut().first_child = Some(Rc::clone(next_sibling_rc));
                }
                else {
                    parent_rc.borrow_mut().first_child = None;
                }
            }
        }
        // reconnect sibling
        if let Some(ref mut next_sibling_rc ) = next_sibling_rc_op {
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

    }

    // 迭代器

}

