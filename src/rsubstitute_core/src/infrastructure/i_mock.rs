use std::ops::DerefMut;

pub(crate) struct InternalMarker;

pub trait IMock<TMock>: Sized + DerefMut<Target = TMock> {
    fn drop_boxed_mocked(&mut self) {
        let mocked = self.deref_mut();
        // SAFETY: this frees memory leaked from `IMocked::mock` method.
        unsafe {
            let _ = Box::from_raw(mocked as *mut _);
        }
    }
}

struct S<'a> {
    pub r: &'a Vec<i32>,
}

fn flex(s: S) -> (&Vec<i32>, S) {
    (s.r, s)
}

fn doo() {
    let s = S { r: &vec![1, 2, 3] };
    let (r, s) = flex(s);
    let v1 = r[1];
    drop(s);
    let v2 = r[1];
}
