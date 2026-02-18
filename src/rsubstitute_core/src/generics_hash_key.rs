use std::any::TypeId;
use std::hash::{DefaultHasher, Hash, Hasher};

pub type GenericsHasher = DefaultHasher;

#[derive(Eq, PartialEq, Hash)]
pub struct GenericsHashKey(u64);

pub trait IGenericsHashKeyProvider {
    fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher);

    fn hash_const_values(&self, hasher: &mut GenericsHasher);

    fn get_generics_hash_key(&self) -> GenericsHashKey {
        let mut hasher = GenericsHasher::new();
        self.hash_generics_type_ids(&mut hasher);
        self.hash_const_values(&mut hasher);
        let generics_hash_key = GenericsHashKey(hasher.finish());
        return generics_hash_key;
    }
}

// Helper method for clearer `IGenericsHashKeyProvider::hash_generics_type_ids`
// auto-generated implementation.
pub fn tid<T>() -> TypeId {
    typeid::of::<T>()
}

// Helper method for calculating hash in `IGenericsHashKeyProvider::hash_consts_values`
// of any sized const value (passed as const parameter). Not calling `t.hash` because
// `T` is not guaranteed to implement `Hash`. This approach anticipates adt_const_params feature:
// https://doc.rust-lang.org/beta/unstable-book/language-features/adt-const-params.html
pub fn const_hash<T: Sized + 'static>(t: &T, hasher: &mut GenericsHasher) {
    let t_size = size_of::<T>();
    let t_ptr = t as *const _ as *const u8;
    unsafe {
        let t_slice = std::slice::from_raw_parts(t_ptr, t_size);
        t_slice.hash(hasher);
    }
}
