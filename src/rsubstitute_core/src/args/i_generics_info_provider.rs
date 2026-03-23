use crate::args::{GenericParameterInfo, GenericsHashKey};
use std::any::TypeId;
use std::hash::{DefaultHasher, Hash, Hasher};

pub type GenericsHasher = DefaultHasher;

// TODO - add #[allow(unused_var)] (warning triggers on hasher arg)
pub trait IGenericsInfoProvider {
    fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo>;

    // TODO - add #[allow(unused)] on hasher in generated impl
    fn hash_generics_type_ids(&self, #[allow(unused)] hasher: &mut GenericsHasher);

    // TODO - add #[allow(unused_args)] on hasher in generated impl
    fn hash_const_values(&self, #[allow(unused)] hasher: &mut GenericsHasher);

    fn get_generics_hash_key(&self) -> GenericsHashKey {
        let mut hasher = GenericsHasher::new();
        self.hash_generics_type_ids(&mut hasher);
        self.hash_const_values(&mut hasher);
        let generics_hash_key = GenericsHashKey(hasher.finish());
        return generics_hash_key;
    }
}

// Helper method for clearer `IGenericsInfoProvider::hash_generics_type_ids` auto-generated implementation.
pub fn tid<T>() -> TypeId {
    typeid::of::<T>()
}

// Helper method for calculating hash in `IGenericsInfoProvider::hash_consts_values` of any sized
// const value (passed as const parameter) by using types raw bytes. Not calling `t.hash` because
// `T` is not guaranteed to implement `Hash`.
// This approach anticipates adt_const_params feature:
// https://doc.rust-lang.org/beta/unstable-book/language-features/adt-const-params.html
pub fn const_hash<T: Sized + 'static>(t: &T, hasher: &mut GenericsHasher) {
    let t_size = size_of::<T>();
    let t_ptr = t as *const _ as *const u8;
    unsafe {
        let t_slice = std::slice::from_raw_parts(t_ptr, t_size);
        t_slice.hash(hasher);
    }
}
