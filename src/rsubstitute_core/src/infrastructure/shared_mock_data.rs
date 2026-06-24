use crate::infrastructure::*;
use std::cell::RefCell;
use std::rc::Rc;

pub trait ISharedMockData<TMock, const SUPPORTS_BASE_CALLING: bool, const STORES_MOCK_DATA: bool> {
    fn get_shared_fn_data<'a>(
        &'_ self,
        fn_ident: &'static str,
    ) -> &'a FnData<'static, TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA>;
}

pub type SharedMockData<TMock, const SUPPORTS_BASE_CALLING: bool, const STORES_MOCK_DATA: bool> =
    Rc<RefCell<MockData<TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA>>>;

impl<TMock, const SUPPORTS_BASE_CALLING: bool, const STORES_MOCK_DATA: bool>
    ISharedMockData<TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA>
    for SharedMockData<TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA>
{
    fn get_shared_fn_data<'a>(
        &'_ self,
        fn_ident: &'static str,
    ) -> &'a FnData<'static, TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA> {
        self.borrow_mut().get_fn_data(fn_ident)
    }
}
