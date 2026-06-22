use crate::infrastructure::fn_data_storage::*;
use std::any::TypeId;

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct MockTableKey {
    pub type_id: TypeId,
    pub mock_id: MockId,
}

impl MockTableKey {
    pub fn new<TMock>(mock_id: MockId) -> MockTableKey {
        MockTableKey {
            type_id: typeid::of::<TMock>(),
            mock_id,
        }
    }
}
