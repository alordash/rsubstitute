use std::thread::LocalKey;

pub trait IStaticLocalKey<T> {
    fn as_static(&'static self) -> &'static T;
}

impl<T> IStaticLocalKey<T> for LocalKey<T> {
    fn as_static(&'static self) -> &'static T {
        return self.with(|x| {
            let borrow_checker_violation: &'static T = unsafe { core::mem::transmute(x) };
            return borrow_checker_violation;
        });
    }
}
