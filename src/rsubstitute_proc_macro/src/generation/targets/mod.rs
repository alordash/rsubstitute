pub(crate) mod models {
    mod mock_mod;
    mod mock_mod_usages;

    pub(crate) use mock_mod::*;
    pub(crate) use mock_mod_usages::*;
}

pub mod r#fn;

mod mock_mod_usages;

// #[derive(Clone)]
struct S {
    pub number: i32,
}

impl ::core::clone::Clone for S {
    #[inline]
    fn clone(&self) -> S {
        S {
            number: ::core::clone::Clone::clone(&self.number),
        }
    }
}
