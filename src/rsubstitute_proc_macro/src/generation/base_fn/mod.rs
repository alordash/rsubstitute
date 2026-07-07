mod common {
    mod rsubstitute_self;
    
    pub(crate) use rsubstitute_self::*;
}

mod normalization;

mod generation;

pub(crate) use generation::*;

