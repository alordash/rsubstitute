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
        const PASSES_MOCK_TO_CALLBACK: bool,
    >(
        &'_ mut self,
        fn_ident: &'static str,
    ) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>
    {
        let fn_data_ptr = self.map.entry(fn_ident).or_insert_with(|| {
            Box::leak(Box::new(FnData::<
                '_,
                TMock,
                HAS_RETURN_VALUE,
                SUPPORTS_BASE_CALLING,
                PASSES_MOCK_TO_CALLBACK,
            >::new(fn_ident))) as *const _ as *const ()
        });

        let fn_data_ref = Self::cast_ptr_to_ref(*fn_data_ptr);
        return fn_data_ref;
    }

    fn cast_ptr_to_ref<
        'a,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
        const PASSES_MOCK_TO_CALLBACK: bool,
    >(
        fn_data_ptr: *const (),
    ) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>
    {
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
                    PASSES_MOCK_TO_CALLBACK,
                >)
                .as_ref()
                .unwrap_unchecked()
        };

        return fn_data_ref;
    }
}

const IRRELEVANT_HAS_RETURN_VALUE: bool = false;
const IRRELEVANT_SUPPORTS_BASE_CALLING: bool = false;
const IRRELEVANT_PASSES_MOCK_TO_CALLBACK: bool = false;

impl<TMock> IMockData for MockData<TMock> {
    fn get_received_nothing_else_error_msgs<const N: usize>(
        &self,
        fn_idents: [&'static str; N],
    ) -> Vec<Vec<String>> {
        let result = fn_idents
            .iter()
            .filter_map(|x| self.map.get(x))
            .cloned()
            .map(
                Self::cast_ptr_to_ref::<
                    'static,
                    IRRELEVANT_HAS_RETURN_VALUE,
                    IRRELEVANT_SUPPORTS_BASE_CALLING,
                    IRRELEVANT_PASSES_MOCK_TO_CALLBACK,
                >,
            )
            .map(FnData::get_unexpected_calls_error_msgs)
            .collect();
        return result;
    }
}

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
                            IRRELEVANT_PASSES_MOCK_TO_CALLBACK,
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
