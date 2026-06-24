use crate::infrastructure::*;
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct MockData<TMock> {
    map: HashMap<&'static str, *const ()>,
    _mock: PhantomData<TMock>,
}

impl<TMock> Default for MockData<TMock> {
    fn default() -> Self {
        Self {
            map: Default::default(),
            _mock: PhantomData,
        }
    }
}

impl<TMock> MockData<TMock> {
    pub(crate) fn get_fn_data<
        'a,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
        const STORES_MOCK_DATA: bool,
    >(
        &'_ mut self,
        fn_ident: &'static str,
    ) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA> {
        let fn_data_ptr = self.map.entry(fn_ident).or_insert_with(|| {
            Box::leak(Box::new(FnData::<
                '_,
                TMock,
                HAS_RETURN_VALUE,
                SUPPORTS_BASE_CALLING,
                STORES_MOCK_DATA,
            >::new(fn_ident))) as *const _ as *const ()
        });

        // SAFETY:
        // `as_ref` - ptr is aligned since it was cast from reference returned from `Box::leak`.
        // Pointed value is a newly created valid `FnData`.
        // Function data is stored behind `Rc<RefCell>` which ensures aliasing rules.
        //
        // `unwrap_unchecked` - pointer was obtained from reference returned from `Box::leak`.
        let fn_data_ref = unsafe {
            (fn_data_ptr as *const _
                as *const FnData<
                    'static,
                    TMock,
                    HAS_RETURN_VALUE,
                    SUPPORTS_BASE_CALLING,
                    STORES_MOCK_DATA,
                >)
                .as_ref()
                .unwrap_unchecked()
        };

        return fn_data_ref;
    }
}

const IRRELEVANT_HAS_RETURN_VALUE: bool = false;
const IRRELEVANT_SUPPORTS_BASE_CALLING: bool = false;
const IRRELEVANT_STORES_MOCK_DATA: bool = false;

impl<TMock> Drop for MockData<TMock> {
    fn drop(&mut self) {
        for fn_data_ptr in self.map.values() {
            let boxed_fn_data = unsafe {
                Box::from_raw(
                    (*fn_data_ptr) as *const _
                        as *mut FnData<
                            'static,
                            TMock,
                            IRRELEVANT_HAS_RETURN_VALUE,
                            IRRELEVANT_SUPPORTS_BASE_CALLING,
                            IRRELEVANT_STORES_MOCK_DATA,
                        >,
                )
            };
            drop(boxed_fn_data);
        }
    }
}

// TODO - write test case for:
// #[mock]
// unsafe impl Send/Sync for Struct {}
// TODO - and write that it's not supported
