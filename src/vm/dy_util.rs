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
        return &self[self.len()];
    }
}