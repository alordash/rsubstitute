use crate::infrastructure::fn_data_storage::*;
use crate::infrastructure::*;
use crate::transmute_lifetime;
use std::collections::HashMap;
use std::sync::{LazyLock, RwLock, RwLockReadGuard, RwLockWriteGuard};

type InnerMap = HashMap<MockTableKey, HashMap<&'static str, *const ()>>;

#[derive(Default)]
struct MockTable {
    // Mocks -> Functions -> `FnData`s
    map: RwLock<InnerMap>,
}

// SAFETY: MockTable is static object that is stored in TLS,
// it should not be accessed from multiple threads.
unsafe impl Send for MockTable {}
// SAFETY: MockTable is static object that is stored in TLS,
// it should not be accessed from multiple threads.
unsafe impl Sync for MockTable {}

impl MockTable {
    pub fn get_fn_data<
        'a,
        TMock,
        const SUPPORTS_BASE_CALLING: bool,
        const STORES_MOCK_DATA: bool,
    >(
        &'_ self,
        mock_id: MockId,
        fn_ident: &'static str,
    ) -> &'a FnData<'static, TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA> {
        let mock_table_key = MockTableKey::new::<TMock>(mock_id);
        let raw_fn_data_ptr = self.get_existing_raw_fn_data_ptr::<TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA>(&mock_table_key, fn_ident)
                                  .unwrap_or_else(|| self.create_and_get_raw_fn_data_ptr::<TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA>(mock_table_key, fn_ident));

        // SAFETY: `raw_fn_data_ptr` is obtained from casting reference to pointer, so it can not be null.
        // Why cast reference to pointer and then back to reference? Because it's impossible to
        // store references in map directly as that would require specifying type of referenced value.
        let fn_data_ref = unsafe {
            (raw_fn_data_ptr as *const FnData<'a, TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA>)
                .as_ref()
                .unwrap_or_else(|| {
                    panic!(
                        "Pointer to data of fn '{fn_ident}' obtained from `Box::leak` should not be null.",
                    )
                })
        };
        return transmute_lifetime!(fn_data_ref);
    }

    pub fn free_mock<TMock, const SUPPORTS_BASE_CALLING: bool, const STORES_MOCK_DATA: bool>(
        &self,
        mock_id: MockId,
    ) {
        let mock_table_key = MockTableKey::new::<TMock>(mock_id);
        let mut mocks_map = self.mandatory_write();
        let Some(raw_fn_data_ptrs) = mocks_map.remove(&mock_table_key) else {
            return;
        };
        for raw_fn_data_ptr in raw_fn_data_ptrs.into_values() {
            // SAFETY: `raw_fn_data_ptr` is obtained from `Box::leak` in `get_fn_data`. Correctness
            // of generic arguments `TMock`, `SUPPORTS_BASE_CALLING` and `STORES_MOCK_DATA` is
            // guaranteed by code generation.
            unsafe {
                let boxed_fn_data = Box::from_raw(
                    raw_fn_data_ptr as *const _
                        as *mut FnData<'_, TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA>,
                );
                drop(boxed_fn_data);
            }
        }
    }
}

impl MockTable {
    fn mandatory_read(&self) -> RwLockReadGuard<'_, InnerMap> {
        self.map
            .read()
            .unwrap_or_else(|e| panic!("Unable to lock mock table for reading, error: '{e}'"))
    }

    fn mandatory_write(&self) -> RwLockWriteGuard<'_, InnerMap> {
        self.map
            .write()
            .unwrap_or_else(|e| panic!("Unable to lock mock table for writing, error: '{e}'"))
    }

    fn get_existing_raw_fn_data_ptr<
        TMock,
        const SUPPORTS_BASE_CALLING: bool,
        const STORES_MOCK_DATA: bool,
    >(
        &self,
        mock_table_key: &MockTableKey,
        fn_ident: &'static str,
    ) -> Option<*const ()> {
        let read_mocks_map = self.mandatory_read();
        let maybe_raw_fn_data_ptr = read_mocks_map
            .get(mock_table_key)
            .map(|fn_datas_map| fn_datas_map.get(fn_ident))
            .flatten()
            .cloned();
        return maybe_raw_fn_data_ptr;
    }

    fn create_and_get_raw_fn_data_ptr<
        TMock,
        const SUPPORTS_BASE_CALLING: bool,
        const STORES_MOCK_DATA: bool,
    >(
        &self,
        mock_table_key: MockTableKey,
        fn_ident: &'static str,
    ) -> *const () {
        let mut write_mocks_map = self.mandatory_write();
        let fn_datas_map = write_mocks_map
            .entry(mock_table_key)
            .or_insert_with(|| HashMap::new());
        let raw_fn_data_ptr = fn_datas_map.entry(fn_ident).or_insert_with(|| {
            Box::leak(Box::new(FnData::<
                '_,
                TMock,
                SUPPORTS_BASE_CALLING,
                STORES_MOCK_DATA,
            >::new(fn_ident))) as *mut _ as *const _
        });

        return raw_fn_data_ptr.clone();
    }
}

static MOCK_TABLE: LazyLock<MockTable> = LazyLock::new(MockTable::default);

pub fn get_fn_data<'a, TMock, const SUPPORTS_BASE_CALLING: bool, const STORES_MOCK_DATA: bool>(
    mock_id: MockId,
    fn_ident: &'static str,
) -> &'a FnData<'static, TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA> {
    let fn_data: &FnData<'_, TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA> =
        MOCK_TABLE.get_fn_data(mock_id, fn_ident);
    return fn_data;
}

pub fn get_static_fn_data<
    'a,
    TMock,
    const SUPPORTS_BASE_CALLING: bool,
    const STORES_MOCK_DATA: bool,
>(
    fn_ident: &'static str,
) -> &'a FnData<'static, TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA> {
    let fn_data: &FnData<'_, TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA> =
        MOCK_TABLE.get_fn_data(STATIC_MOCK_ID, fn_ident);
    return fn_data;
}

pub fn free_mock<TMock, const SUPPORTS_BASE_CALLING: bool, const STORES_MOCK_DATA: bool>(
    mock_id: MockId,
) {
    MOCK_TABLE.free_mock::<TMock, SUPPORTS_BASE_CALLING, STORES_MOCK_DATA>(mock_id);
}

// TODO - write test case for
// #[mock]
// unsafe impl Send/Sync for Struct {}
