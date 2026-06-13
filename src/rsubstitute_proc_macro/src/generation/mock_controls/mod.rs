pub mod models {
    mod mock_data_struct;
    
    pub use mock_data_struct::*;
}

mod mock_data_generation;
mod mock_type_generation;

pub use mock_data_generation::*;
pub use mock_type_generation::*;