use crate::args::*;
use crate::infrastructure::*;
use std::sync::{Arc, RwLock};

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

// TODO - maybe add some multithreaded tests?
pub type SharedMockData = Arc<RwLock<MockData>>;

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
        self.write()
            .expect(UNABLE_TO_LOCK_FOR_WRITING_ERROR)
            .get_or_create_fn_data(fn_ident, generics_hash_key, false)
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
        self.write()
            .expect(UNABLE_TO_LOCK_FOR_WRITING_ERROR)
            .get_or_create_fn_data(fn_ident, generics_hash_key, true)
    }
}

impl IMockData for SharedMockData {
    fn get_received_nothing_else_error_msgs(&self) -> Vec<Vec<String>> {
        self.read()
            .expect(UNABLE_TO_LOCK_FOR_READING_ERROR)
            .get_received_nothing_else_error_msgs()
    }
}

const UNABLE_TO_LOCK_FOR_WRITING_ERROR: &'static str = "Unable to lock SharedMockData for writing.";
const UNABLE_TO_LOCK_FOR_READING_ERROR: &'static str = "Unable to lock SharedMockData for reading.";
