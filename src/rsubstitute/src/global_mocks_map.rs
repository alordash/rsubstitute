use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::LazyLock;

// Used for storing static functions' mocks instances.
#[derive(Default)]
struct GlobalMocksMap {
    pub map: RefCell<HashMap<TypeId, *const ()>>,
}

unsafe impl Send for GlobalMocksMap {}
unsafe impl Sync for GlobalMocksMap {}

impl GlobalMocksMap {
    pub fn get_specific_mock<'a, T: Default>(&'_ self) -> &'a T {
        let mut map = self.map.borrow_mut();
        let type_id = typeid::of::<T>();
        let raw_ptr = map
            .entry(type_id)
            .or_insert(Box::leak(Box::new(<T as Default>::default())) as *mut _ as *const _);

        // SAFETY: `raw_ptr` is obtained from `Box::<T>::leak`, which means that it is safe to cast
        // a pointer to `T` and treat it as reference. `as_ref` could also be replaced with `as_ref_unchecked`
        // since `Box::leak` returns a reference, which after casting to pointer can not be null.
        let mock_ref = unsafe {
            ((*raw_ptr) as *const T).as_ref().unwrap_or_else(|| {
                panic!(
                    "Pointer to global static mock of type '{}' obtained from `Box::leak` should not be null.",
                    std::any::type_name::<T>()
                )
            })
        };
        return mock_ref;
    }
}

// Located in TLS so that tests that mock same function won't overlap if run in parallel.
thread_local! {
    pub static GLOBAL_MOCKS_MAP: LazyLock<GlobalMocksMap> = LazyLock::new(Default::default);
}

pub fn get_global_mock<'a, T: Default>() -> &'a T {
    let result = GLOBAL_MOCKS_MAP.with(|this| this.get_specific_mock());
    return result;
}
