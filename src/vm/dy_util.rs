use std::rc::Rc;
use std::rc::Weak;
//use std::ops::Index;

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

pub trait WeakExtend {
	// type ItemType;
	// borrow_mut 已经有实现了
	// fn get_mut(&mut self) -> Option<&mut Self::ItemType>;
	fn is_none(&self) -> bool;
	fn is_some(&self) -> bool;
}

impl<T> WeakExtend for Weak<T> {
	// type ItemType = T;
	// fn get_mut(&mut self) -> Option<&mut T> {
	//    // let rc_op = self.upgrade();
	//    	if self.is_some() {
	//        return Rc::get_mut(self.upgrade().as_mut().unwrap());
	//    	}
	//    	return None;
	// }

	fn is_none(&self) -> bool {
		self.upgrade().is_none()
	}

	fn is_some(&self) -> bool {
		self.upgrade().is_some()
	}
}