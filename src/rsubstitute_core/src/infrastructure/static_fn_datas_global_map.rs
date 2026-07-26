use crate::infrastructure::{FnData, IMockData};
use std::any::TypeId;
use std::cell::UnsafeCell;
use std::collections::HashMap;

type Map = HashMap<TypeId, HashMap<&'static str, *const ()>>;

// Used for storing static functions' mock data.
#[derive(Default)]
struct StaticFnDatasGlobalMap {
    pub map: UnsafeCell<Map>,
}

impl StaticFnDatasGlobalMap {
    fn get_mut_map(&self) -> &mut Map {
        // SAFETY: static functions data is stored in global TLS, which guarantees that there can't
        // be more than one mutable reference to given static function data at the same time.
        // This is why `UnsafeCell` can be safely used here.
        let maybe_map = unsafe { self.map.get().as_mut() };
        // SAFETY: `UnsafeCell::get` can not return null pointer.
        let map = unsafe { maybe_map.unwrap_unchecked() };
        return map;
    }

    fn get_fn_data_ref<
        'a,
        TMock,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
    >(
        fn_data_raw_ptr: *const (),
    ) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, false> {
        // SAFETY: `raw_ptr` is obtained from `Box::<T>::leak`, which means that it is safe to cast
        // the pointer to `T` and treat it as reference.
        let result = unsafe {
            (fn_data_raw_ptr
                as *const FnData<TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, false>)
                .as_ref_unchecked()
        };
        return result;
    }

    pub fn get_specific_fn_data<
        'a,
        TMock,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
    >(
        &'_ self,
        fn_ident: &'static str,
    ) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, false> {
        let type_id = typeid::of::<TMock>();
        let map = self.get_mut_map();
        let raw_ptr = map
            .entry(type_id)
            .or_insert_with(|| HashMap::new())
            .entry(fn_ident)
            .or_insert_with(|| {
                Box::leak(Box::new(FnData::<
                    TMock,
                    HAS_RETURN_VALUE,
                    SUPPORTS_BASE_CALLING,
                    false,
                >::new(fn_ident))) as *mut _ as *const _
            });

        let fn_data_ref =
            Self::get_fn_data_ref::<'a, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING>(*raw_ptr);
        return fn_data_ref;
    }

    pub fn clear_mock_fn_datas<TMock>(&self) {
        let type_id = typeid::of::<TMock>();
        let map = self.get_mut_map();
        map.remove_entry(&type_id);
    }

    pub fn verify_received_nothing_else<TMock>(&self) {
        let type_id = typeid::of::<TMock>();
        let map = self.get_mut_map();
        let Some(fn_datas_raw_ptrs) = map.get(&type_id) else {
            return;
        };
        for fn_data_raw_ptr in fn_datas_raw_ptrs.values() {
            const IRRELEVANT_HAS_RETURN_VALUE: bool = false;
            const IRRELEVANT_SUPPORTS_BASE_CALLING: bool = false;
            let fn_data = Self::get_fn_data_ref::<
                TMock,
                IRRELEVANT_HAS_RETURN_VALUE,
                IRRELEVANT_SUPPORTS_BASE_CALLING,
            >(*fn_data_raw_ptr);
            fn_data.verify_received_nothing_else();
        }
    }
}

// Located in TLS so that tests that mock same function won't overlap if run in parallel.
thread_local! {
    pub static STATIC_FN_DATAS_GLOBAL_MAP: StaticFnDatasGlobalMap = StaticFnDatasGlobalMap {
        map: Default::default(),
    };
}

pub fn get_static_fn_data<
    'a,
    TMock,
    const HAS_RETURN_VALUE: bool,
    const SUPPORTS_BASE_CALLING: bool,
>(
    fn_ident: &'static str,
) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, false> {
    let result = STATIC_FN_DATAS_GLOBAL_MAP.with(|this| this.get_specific_fn_data(fn_ident));
    return result;
}

pub fn clear_static_fn_data<TMock>() {
    STATIC_FN_DATAS_GLOBAL_MAP.with(|this| this.clear_mock_fn_datas::<TMock>());
}

pub fn get_clean_static_fn_data<
    'a,
    TMock,
    const HAS_RETURN_VALUE: bool,
    const SUPPORTS_BASE_CALLING: bool,
>(
    fn_ident: &'static str,
) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, false> {
    clear_static_fn_data::<TMock>();
    return get_static_fn_data(fn_ident);
}

pub fn verify_static_fn_received_nothing_else<TMock>() {
    STATIC_FN_DATAS_GLOBAL_MAP.with(|this| this.verify_received_nothing_else::<TMock>());
}
