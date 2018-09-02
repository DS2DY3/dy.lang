use std::rc::{Rc, Weak};
use std::cell::{RefCell, Ref, RefMut};
use std::fmt;
use std::ops::{Deref, DerefMut};


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

    pub fn root(&self) -> DyRef<T> {
        match self.0.borrow().root.upgrade() {
            None => self.clone(),
            Some(root) => DyRef(root),
        }
    }

    pub fn parent(&self) -> Option<DyRef<T>> {
        let borrowed = self.0.borrow();
        match borrowed.parent.upgrade() {
            None => None,
            Some(ref parent) => Some(DyRef(Rc::clone(parent))),
        }
    }

    pub fn borrow(&self) -> Ref<T> {
        Ref::map(self.0.borrow(), |v| &v.data)
    }

    pub fn borrow_mut(&mut self) -> RefMut<T> {
        RefMut::map(self.0.borrow_mut(), |v| &mut v.data)
    }

//    pub fn first_child(&self) -> Option<DyRef<T>> {
//        let borrowed = self.0.borrow();
//        match borrowed.parent {
//            None => None,
//            Some(parent) => DyRef(Rc::clone(parent)),
//        }
//    }
//
//    pub fn last_child(&self) -> Option<DyRef<T>> {
//        let borrowed = self.0.borrow();
//        match borrowed.parent.upgrade() {
//            None => None,
//            Some(parent) => DyRef(Rc::clone(parent)),
//        }
//    }
//
//    pub fn next_sibling(&self) -> Option<DyRef<T>> {
//        let borrowed = self.0.borrow();
//        match borrowed.parent.upgrade() {
//            None => None,
//            Some(parent) => DyRef(Rc::clone(parent)),
//        }
//    }
//
//    pub fn pre_sibling(&self) -> Option<DyRef<T>> {
//        let borrowed = self.0.borrow();
//        match borrowed.parent.upgrade() {
//            None => None,
//            Some(parent) => DyRef(Rc::clone(parent)),
//        }
//    }

}






//impl<T: fmt::Debug> fmt::Debug for DyRef<T> {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        fmt::Debug::fmt(&*self.borrow(), f)
//    }
//}
//
//impl<T: fmt::Display> fmt::Display for DyRef<T> {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        fmt::Display::fmt(&*self.borrow(), f)
//    }
//}
