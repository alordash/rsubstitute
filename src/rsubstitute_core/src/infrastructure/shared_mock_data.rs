use crate::args::*;
use crate::infrastructure::*;
use std::cell::RefCell;
use std::rc::Rc;

pub trait ISharedMockData {
    fn get_shared_fn_data<
        'a,
        TMock,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
        const PASSES_MOCK_TO_CALLBACK: bool,
    >(
        &'_ self,
        fn_ident: &'static str,
        generics_hash_key: GenericsHashKey,
    ) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>;

    fn get_shared_fn_data_for_struct<
        'a,
        TMock,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
        const PASSES_MOCK_TO_CALLBACK: bool,
    >(
        &'_ self,
        fn_ident: &'static str,
        generics_hash_key: GenericsHashKey,
    ) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>;
}

pub type SharedMockData = Rc<RefCell<MockData>>;

impl ISharedMockData for SharedMockData {
    fn get_shared_fn_data<
        'a,
        TMock,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
        const PASSES_MOCK_TO_CALLBACK: bool,
    >(
        &'_ self,
        fn_ident: &'static str,
        generics_hash_key: GenericsHashKey,
    ) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>
    {
        self.borrow_mut()
            .get_fn_data(fn_ident, generics_hash_key, false)
    }

    fn get_shared_fn_data_for_struct<
        'a,
        TMock,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
        const PASSES_MOCK_TO_CALLBACK: bool,
    >(
        &'_ self,
        fn_ident: &'static str,
        generics_hash_key: GenericsHashKey,
    ) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>
    {
        self.borrow_mut()
            .get_fn_data(fn_ident, generics_hash_key, true)
    }
}

impl IMockData for SharedMockData {
    fn get_received_nothing_else_error_msgs(&self) -> Vec<Vec<String>> {
        self.borrow().get_received_nothing_else_error_msgs()
    }
}
