use crate::infrastructure::*;
use crate::transmute_lifetime;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;

pub struct MockData<TMock, const SUPPORTS_BASE_CALLING: bool, const STORES_MOCK_DATA: bool> {
    map: HashMap<&'static str, FnData<'static, TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA>>,
    _mock: PhantomData<TMock>,
}

pub type SharedMockData<TMock, const SUPPORTS_BASE_CALLING: bool, const STORES_MOCK_DATA: bool> =
    Rc<RefCell<MockData<TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA>>>;

impl<TMock, const SUPPORTS_BASE_CALLING: bool, const STORES_MOCK_DATA: bool> Default
    for MockData<TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA>
{
    fn default() -> Self {
        Self {
            map: Default::default(),
            _mock: Default::default(),
        }
    }
}

impl<TMock, const SUPPORTS_BASE_CALLING: bool, const STORES_MOCK_DATA: bool>
    MockData<TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA>
{
    pub fn get_fn_data<'a>(
        &'_ mut self,
        fn_ident: &'static str,
    ) -> &'a FnData<'static, TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA> {
        let fn_data = self.map.entry(fn_ident).or_insert_with(|| {
            FnData::<'_, TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA>::new(fn_ident)
        });

        return transmute_lifetime!(fn_data);
    }
}

// TODO - write test case for:
// #[mock]
// unsafe impl Send/Sync for Struct {}
// TODO - and write that it's not supported
