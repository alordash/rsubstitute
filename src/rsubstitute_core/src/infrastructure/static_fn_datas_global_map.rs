use crate::infrastructure::FnData;
use std::any::TypeId;
use std::cell::UnsafeCell;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
struct StaticFnDataKey(TypeId, &'static str);

// Used for storing static functions' mock data.
#[derive(Default)]
struct StaticFnDatasGlobalMap {
    pub map: UnsafeCell<HashMap<TypeId, HashMap<&'static str, *const ()>>>,
}

impl StaticFnDatasGlobalMap {
    pub fn get_specific_fn_data<
        'a,
        TMock,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
    >(
        &'_ self,
        fn_ident: &'static str,
    ) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, false> {
        // SAFETY: static functions data is stored in global TLS, which guarantees that there can't
        // be more than one mutable reference to given static function data at the same time.
        // This is why `UnsafeCell` can be safely used here.  
        let maybe_map = unsafe { self.map.get().as_mut() };
        // SAFETY: `UnsafeCell::get` can not return null pointer.
        let map = unsafe { maybe_map.unwrap_unchecked() };
        let type_id = typeid::of::<TMock>();
        let key = StaticFnDataKey(type_id, fn_ident);
        let raw_ptr = map.entry(key).or_insert(Box::leak(Box::new(FnData::<
            TMock,
            HAS_RETURN_VALUE,
            SUPPORTS_BASE_CALLING,
            false,
        >::new(fn_ident))) as *mut _
            as *const _);

        // SAFETY: `raw_ptr` is obtained from `Box::<T>::leak`, which means that it is safe to cast
        // a pointer to `T` and treat it as reference. `as_ref` could also be replaced with `as_ref_unchecked`
        // since `Box::leak` returns a reference, which after casting to pointer can not be null.
        let fn_data_ref = unsafe {
            ((*raw_ptr) as *const FnData<
                TMock,
                HAS_RETURN_VALUE,
                SUPPORTS_BASE_CALLING,
                false,
            >).as_ref().unwrap_or_else(|| {
                panic!(
                    "Pointer to global static mock of type '{}' obtained from `Box::leak` should not be null.",
                    std::any::type_name::<TMock>()
                )
            })
        };
        return fn_data_ref;
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

pub fn clear_static_fn_data<TMock>(fn_iden)