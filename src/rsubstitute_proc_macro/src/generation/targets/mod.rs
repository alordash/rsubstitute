pub(crate) mod models {
    mod mock_mod;
    mod mock_mod_usages;

    pub(crate) use mock_mod::*;
    pub(crate) use mock_mod_usages::*;
}

pub mod r#fn;
pub mod r#trait;
pub mod r#struct;

mod common {
    pub(crate) mod mod_usage;
}

mod mock_mod_usages;
