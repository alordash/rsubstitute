use crate::generation::models::*;
use proc_macro2::Span;
use std::ops::Deref;
use syn::*;

pub(crate) fn generate(
    source_span: Span,
    mock_ident: Ident,
    target_struct: ItemStruct,
    setup_struct_ident: Ident,
    received_struct_ident: Ident,
    data_struct_ident: Ident,
) -> MockStructImpls {
    let result = MockStructImpls {
        target_mockable_impl: todo!(),
        deref_impl: todo!(),
        deref_mut: todo!(),
        mock_impl: todo!(),
    };

    return result;
}

use a::*;
mod a {
    use super::*;
    pub struct S;

    pub struct SMock {
        pub setup: SSetup,
        pub received: SReceived,
    }

    impl Deref for SMock {
        type Target = S;

        fn deref(&self) -> &Self::Target {
            &S
        }
    }

    pub struct SSetup {
        pub data: String,
    }

    impl SSetup {
        pub fn _as<T: STransform<Self> + ?Sized>(&self) -> <T as STransform<Self>>::TResult {
            T::convert(self)
        }
    }

    pub struct SReceived {
        pub data: String,
    }

    impl SReceived {
        pub fn _as<T: STransform<Self> + ?Sized>(&self) -> <T as STransform<Self>>::TResult {
            T::convert(self)
        }
    }

    impl S {
        pub fn flex(&self) {}

        pub fn mock(self) -> SMock {
            SMock {
                setup: SSetup {
                    data: "asd".to_string(),
                },
                received: todo!(),
            }
        }
    }

    fn usage() {
        let s = S;
        s.flex();
        Trait::flex(&s);
        let q: Vec<&dyn Trait> = vec![&s];
        let w = &s as &dyn Trait;
        let s_mock = s.mock();
        let s_traitSetup: S_TraitSetup = s_mock.setup._as::<dyn Trait>();
        s_mock.setup._as::<dyn Trait>().flex();

        let s_traitReceived: S_TraitReceived = s_mock.received._as::<TraitMock>();
        s_traitReceived.flex(3);

        // trait Trait2 {}
        // let a = s_mock.setup._as::<dyn Trait2>();
    }

    // global trait, defined in rsubstitute_core
    // or make it local for ability to impl it for external traits?
    pub trait STransform<TSource> {
        type TResult;

        fn convert(source: &TSource) -> Self::TResult;
    }
}

use b::*;
mod b {
    use super::*;

    pub struct S_TraitSetup {
        pub data: String,
    }
    impl S_TraitSetup {
        pub fn flex(&self) {}
    }

    pub struct S_TraitReceived {
        pub data: String,
    }
    impl S_TraitReceived {
        pub fn flex(&self, times: i32) {}
    }

    pub trait Trait {
        fn flex(&self);
    }

    impl Trait for S {
        fn flex(&self) {}
    }

    impl STransform<SSetup> for dyn Trait {
        type TResult = S_TraitSetup;

        fn convert(source: &SSetup) -> Self::TResult {
            // should I create new S_TraitSetup here each time `_as` called
            // or should I keep it as part of SSetup?
            // I CANT store S_TraitSetup in SSetup if S_TraitSetup is in another module though
            todo!()
        }
    }

    // instead of dyn Trait
    pub struct TraitMock;

    impl STransform<SReceived> for TraitMock {
        type TResult = S_TraitReceived;

        fn convert(source: &SReceived) -> Self::TResult {
            todo!()
        }
    }
}
