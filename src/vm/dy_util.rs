use std::rc::Rc;
use std::rc::Weak;
use std::ops::Index;

//pub fn weak_ref<T>(target: &T) -> Weak<T> {
//    let rc = Rc::new(target);
//    return Rc::downgrade(&rc);
//}

pub trait VecExtend {
    type Output;
    fn put(&mut self, item: Self::Output) -> &Self::Output;
}

impl<T> VecExtend for Vec<T> {
    type Output = T;
    fn put(&mut self, item: T) -> &T {
        self.push(item);
        return &self[self.len()-1];
    }
}

//pub trait WeakExtend {
//    type ItemType;
//    fn get_mut(&mut self) -> Option<&mut Self::ItemType>;
//}
//
//impl <T> WeakExtend for Weak<T> {
//    type ItemType = T;
//    fn get_mut(&mut self) -> Option<&mut T> {
//        let rc_op = self.upgrade();
//        match rc_op {
//            None => None,
//            Some(ref mut rc_obj) => Rc::get_mut(&mut rc_obj)
//        }
//    }
//}