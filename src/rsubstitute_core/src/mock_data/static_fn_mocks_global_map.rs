use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::LazyLock;

// Used for storing static functions' mocks instances.
#[derive(Default)]
struct StaticFnMocksGlobalMap {
    pub map: RefCell<HashMap<TypeId, *const ()>>,
}

unsafe impl Send for StaticFnMocksGlobalMap {}
unsafe impl Sync for StaticFnMocksGlobalMap {}

impl StaticFnMocksGlobalMap {
    pub fn get_specific_mock<'a, TMock: Default>(&'_ self) -> &'a TMock {
        let mut map = self.map.borrow_mut();
        let type_id = typeid::of::<TMock>();
        let raw_ptr = map
            .entry(type_id)
            .or_insert(Box::leak(Box::new(<TMock as Default>::default())) as *mut _ as *const _);

        // SAFETY: `raw_ptr` is obtained from `Box::<T>::leak`, which means that it is safe to cast
        // a pointer to `T` and treat it as reference. `as_ref` could also be replaced with `as_ref_unchecked`
        // since `Box::leak` returns a reference, which after casting to pointer can not be null.
        let mock_ref = unsafe {
            ((*raw_ptr) as *const TMock).as_ref().unwrap_or_else(|| {
                panic!(
                    "Pointer to global static mock of type '{}' obtained from `Box::leak` should not be null.",
                    std::any::type_name::<TMock>()
                )
            })
        };
        return mock_ref;
    }
}

// Located in TLS so that tests that mock same function won't overlap if run in parallel.
thread_local! {
    pub static STATIC_FN_MOCKS_GLOBAL_MAP: LazyLock<StaticFnMocksGlobalMap> = LazyLock::new(Default::default);
}

pub fn get_static_fn_global_mock<'a, TMock: Default>() -> &'a TMock {
    let result = STATIC_FN_MOCKS_GLOBAL_MAP.with(|this| this.get_specific_mock());
    return result;
}
