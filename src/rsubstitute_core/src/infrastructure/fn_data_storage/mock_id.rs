use core::sync::atomic::{AtomicU32, Ordering};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct MockId(u32);

impl MockId {
    pub fn next() -> MockId {
        MockId(VACANT_MOCK_ID.fetch_add(1, Ordering::AcqRel))
    }
}
pub(crate) const STATIC_MOCK_ID: MockId = MockId(u32::MAX);

static VACANT_MOCK_ID: AtomicU32 = AtomicU32::new(0);
