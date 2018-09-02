use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref, RefMut};
use std::fmt;
use std::ops::{Deref, DerefMut};
//use std::borrow::{Borrow, BorrowMut};


type WeakRef<T> = Weak<RefCell<DyNode<T>>>;
type RcRef<T> = Rc<RefCell<DyNode<T>>>;

pub struct DyNode<T> {
    data: T,
    parent: WeakRef<T>,
    first_child: Option<RcRef<T>>,
    last_child: WeakRef<T>,
    next_sibling: Option<RcRef<T>>,
    pre_sibling: WeakRef<T>,
    root: WeakRef<T>,
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
            root: Weak::new(),
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

    // 访问节点

    pub fn root(&self) -> DyRef<T> {
        match self.0.borrow().root.upgrade() {
            None => self.clone(),
            Some(root) => DyRef(root),
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

    }

    pub fn prepend(&self, child: &DyRef<T>) {

    }

    pub fn insert_after(&self, sibling: &DyRef<T>) {

    }

    pub fn insert_before(&self, sibling: &DyRef<T>) {

    }

    // 迭代器

}

