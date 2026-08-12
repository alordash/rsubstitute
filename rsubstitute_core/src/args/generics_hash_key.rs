use std::hash::Hash;

#[derive(Eq, PartialEq, Hash)]
pub struct GenericsHashKey(pub(crate) u64);
