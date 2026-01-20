use std::any::TypeId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::LazyLock;
use crate::for_generated::IStaticLocalKey;

#[derive(Default)]
struct MocksMap {
    pub map: RefCell<HashMap<TypeId, *const ()>>,
}

unsafe impl Send for MocksMap {}
unsafe impl Sync for MocksMap {}

thread_local! {
    pub static MOCKS_MAP: LazyLock<MocksMap> = LazyLock::new(Default::default);
}

pub fn get_mock<T: Default>() -> *const T {
    let ref_map = &MOCKS_MAP.as_static().map;
    let mut map = ref_map.borrow_mut();

    let type_id = typeid::of::<T>();
    let raw_ptr = map
        .entry(type_id)
        .or_insert(Box::leak(Box::new(Default::default())) as *const _ as *const ());
    let mock_ptr = *raw_ptr as *const T;
    return mock_ptr;
}