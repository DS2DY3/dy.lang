use std::rc::Rc;
use std::rc::Weak;

fn weak_ref<T>(target: T) -> Weak<T> {
    let rc = Rc::new(target);
    return Rc::downgrade(&rc);
}
