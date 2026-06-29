use crate::infrastructure::*;
use std::cell::RefCell;
use std::rc::Rc;

pub trait ISharedMockData<TMock> {
    fn get_shared_fn_data<
        'a,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
        const PASSES_MOCK_TO_CALLBACK: bool,
    >(
        &'_ self,
        fn_ident: &'static str,
    ) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>;
}

pub type SharedMockData<TMock> = Rc<RefCell<MockData<TMock>>>;

impl<TMock> ISharedMockData<TMock> for SharedMockData<TMock> {
    fn get_shared_fn_data<
        'a,
        const HAS_RETURN_VALUE: bool,
        const SUPPORTS_BASE_CALLING: bool,
        const PASSES_MOCK_TO_CALLBACK: bool,
    >(
        &'_ self,
        fn_ident: &'static str,
    ) -> &'a FnData<'static, TMock, HAS_RETURN_VALUE, SUPPORTS_BASE_CALLING, PASSES_MOCK_TO_CALLBACK>
    {
        self.borrow_mut().get_fn_data(fn_ident)
    }
}

impl<TMock> IMockData for SharedMockData<TMock> {
    fn get_received_nothing_else_error_msgs<const N: usize>(
        &self,
        fn_idents: [&'static str; N],
    ) -> Vec<Vec<String>> {
        self.borrow()
            .get_received_nothing_else_error_msgs(fn_idents)
    }
}
