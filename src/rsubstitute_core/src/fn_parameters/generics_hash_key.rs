#[derive(Eq, PartialEq, Hash)]
pub struct GenericsHashKey(u64);

pub trait IGenericsHashKeyProvider {
    fn get_generics_hash_key(&self) -> GenericsHashKey;
}