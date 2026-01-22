use crate::for_generated::IStaticLocalKey;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Default)]
struct GlobalMocksMap {
    pub map: RefCell<HashMap<TypeId, *const ()>>,
}

unsafe impl Send for GlobalMocksMap {}
unsafe impl Sync for GlobalMocksMap {}

thread_local! {
    pub static GLOBAL_MOCKS_MAP: LazyLock<GlobalMocksMap> = LazyLock::new(Default::default);
}

pub fn get_mock<'a, T: Default + ?Sized>() -> &'a T {
    let ref_map = &GLOBAL_MOCKS_MAP.as_static().map;
    let mut map = ref_map.borrow_mut();

    let type_id = typeid::of::<T>();
    let raw_ptr = map
        .entry(type_id)
        .or_insert(Box::leak(Box::new(<T as Default>::default())) as *mut _ as *const _);
    let mock_ptr = (*raw_ptr) as *const T;
    let mock_ref = unsafe {
        &*mock_ptr.as_ref().expect(&format!(
            "Pointer to global static mock of type '{}' should not be null.",
            std::any::type_name::<T>()
        ))
    };
    return mock_ref;
}
