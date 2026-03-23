use rsubstitute::macros::mock;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

#[rustfmt::skip]
#[allow(unused)]
mod consts {
    pub const BY_VALUE_VALUE:                       i32 = 1 ;
    pub const BY_VALUE_COLON_VALUE:                 i32 = 2 ;
    pub const BY_MUT_VALUE_VALUE:                   i32 = 3 ;
    pub const BY_MUT_VALUE_COLON_VALUE:             i32 = 4 ;
    pub const BY_REF_VALUE:                         i32 = 5 ;
    pub const BY_REF_COLON_VALUE:                   i32 = 6 ;
    pub const BY_REF_WITH_LIFETIME_VALUE:           i32 = 7 ;
    pub const BY_REF_COLON_WITH_LIFETIME_VALUE:     i32 = 8 ;
    pub const BY_REF_MUT_VALUE:                     i32 = 9 ;
    pub const BY_REF_MUT_COLON_VALUE:               i32 = 10;
    pub const BY_REF_MUT_WITH_LIFETIME_VALUE:       i32 = 11;
    pub const BY_REF_MUT_COLON_WITH_LIFETIME_VALUE: i32 = 12;
    pub const BY_BOX_VALUE:                         i32 = 13;
    pub const BY_RC_VALUE:                          i32 = 14;
    pub const BY_ARC_VALUE:                         i32 = 15;
    pub const BY_PIN_VALUE:                         i32 = 16;
    pub const NESTED_VALUE:                         i32 = 17;
}
use consts::*;

#[mock]
trait Trait: Sized {
    fn by_value(self) {}
    fn by_value_colon(self: Self) {}
    fn by_mut_value(#[allow(unused_mut)] mut self) {}
    fn by_mut_value_colon(#[allow(unused_mut)] mut self: Self) {}

    fn by_ref(&self) {}
    fn by_ref_colon(self: &Self) {}
    fn by_ref_with_lifetime<'a>(&'a self) {}
    fn by_ref_colon_with_lifetime<'a>(self: &'a Self) {}
    fn by_ref_mut(&mut self) {}
    fn by_ref_mut_colon(self: &mut Self) {}
    fn by_ref_mut_with_lifetime<'a>(&'a mut self) {}
    fn by_ref_mut_colon_with_lifetime<'a>(self: &'a mut Self) {}

    fn by_box(self: Box<Self>) {}
    fn by_rc(self: Rc<Self>) {}
    fn by_arc(self: Arc<Self>) {}
    fn by_pin(self: Pin<&Self>) {}

    fn nested<'a>(self: &'a Box<Pin<Rc<Box<Arc<Pin<Rc<Self>>>>>>>) {}

    fn return_by_value(self) -> i32 {
        BY_VALUE_VALUE
    }
    fn return_by_value_colon(self: Self) -> i32 {
        BY_VALUE_COLON_VALUE
    }
    fn return_by_mut_value(#[allow(unused_mut)] mut self) -> i32 {
        BY_MUT_VALUE_VALUE
    }
    fn return_by_mut_value_colon(#[allow(unused_mut)] mut self: Self) -> i32 {
        BY_MUT_VALUE_COLON_VALUE
    }

    fn return_by_ref(&self) -> i32 {
        BY_REF_VALUE
    }
    fn return_by_ref_colon(self: &Self) -> i32 {
        BY_REF_COLON_VALUE
    }
    fn return_by_ref_with_lifetime<'a>(&'a self) -> i32 {
        BY_REF_WITH_LIFETIME_VALUE
    }
    fn return_by_ref_colon_with_lifetime<'a>(self: &'a Self) -> i32 {
        BY_REF_COLON_WITH_LIFETIME_VALUE
    }
    fn return_by_ref_mut(&mut self) -> i32 {
        BY_REF_MUT_VALUE
    }
    fn return_by_ref_mut_colon(self: &mut Self) -> i32 {
        BY_REF_MUT_COLON_VALUE
    }
    fn return_by_ref_mut_with_lifetime<'a>(&'a mut self) -> i32 {
        BY_REF_MUT_WITH_LIFETIME_VALUE
    }
    fn return_by_ref_mut_colon_with_lifetime<'a>(self: &'a mut Self) -> i32 {
        BY_REF_MUT_COLON_WITH_LIFETIME_VALUE
    }

    fn return_by_box(self: Box<Self>) -> i32 {
        BY_BOX_VALUE
    }
    fn return_by_rc(self: Rc<Self>) -> i32 {
        BY_RC_VALUE
    }
    fn return_by_arc(self: Arc<Self>) -> i32 {
        BY_ARC_VALUE
    }
    fn return_by_pin(self: Pin<&Self>) -> i32 {
        BY_PIN_VALUE
    }

    fn return_nested<'a>(self: &'a Box<Pin<Rc<Box<Arc<Pin<Rc<Self>>>>>>>) -> i32 {
        NESTED_VALUE
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    #![allow(unused_imports)]

    use super::*;
    use not_enough_asserts::panics::*;
    use rsubstitute::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn compiles() {
        let mock = TraitMock::new();
    }
}
