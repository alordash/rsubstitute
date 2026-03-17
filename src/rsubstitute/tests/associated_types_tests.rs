#![feature(associated_type_defaults)]

// #[mock]
// trait Trait {
//     type MyType<TT>: Clone
//     where
//         Self: Sized,
//         TT: Clone;
// }

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    #[test]
    fn compile() {}
}

trait Trait {
    type MyType<TT>: Clone
    where
        Self: Sized,
        TT: Clone;
}
#[cfg(test)]
pub use __rsubstitute_generated_Trait::*;
#[cfg(test)]
#[allow(unused_parens)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(mismatched_lifetime_syntaxes)]
mod __rsubstitute_generated_Trait {
    use super::*;
    use rsubstitute::for_generated::*;
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct TraitMockData<'__rs, MyType: Clone> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom___rs: PhantomData<&'__rs ()>,
        _phantom_MyType: PhantomData<MyType>,
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct TraitMockSetup<'__rs, MyType: Clone> {
        data: Arc<TraitMockData<'__rs, MyType>>,
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct TraitMockReceived<'__rs, MyType: Clone> {
        data: Arc<TraitMockData<'__rs, MyType>>,
    }
    #[derive(CloneForRSubstitute)]
    pub struct TraitMock<'__rs, MyType: Clone> {
        pub setup: TraitMockSetup<'__rs, MyType>,
        pub received: TraitMockReceived<'__rs, MyType>,
        pub data: Arc<TraitMockData<'__rs, MyType>>,
    }
    impl<'__rs, MyType: Clone> Trait for TraitMock<'__rs, MyType> {
        type MyType<TT>
        where
            Self: Sized,
            TT: Clone
        = MyType;
    }
    impl<'__rs, MyType: Clone> TraitMock<'__rs, MyType> {
        pub fn new() -> Self {
            let data = Arc::new(TraitMockData {
                _phantom_lifetime: PhantomData,
                _phantom___rs: PhantomData,
                _phantom_MyType: PhantomData,
            });
            return TraitMock {
                setup: TraitMockSetup { data: data.clone() },
                received: TraitMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'__rs, MyType: Clone> TraitMockSetup<'__rs, MyType> {}
    impl<'__rs, MyType: Clone> TraitMockReceived<'__rs, MyType> {
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
}
