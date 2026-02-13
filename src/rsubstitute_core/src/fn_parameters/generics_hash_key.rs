use std::any::TypeId;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Eq, PartialEq, Hash)]
pub struct GenericsHashKey(u64);

pub trait IGenericsHashKeyProvider {
    fn get_generics_type_ids(&self) -> Vec<TypeId>;
    
    fn get_generics_hash_key(&self) -> GenericsHashKey {
        let mut hasher = DefaultHasher::new();
        let generics_type_ids = self.get_generics_type_ids();
        generics_type_ids.hash(&mut hasher);
        let generics_hash_key = GenericsHashKey(hasher.finish());
        return generics_hash_key;
    }
}