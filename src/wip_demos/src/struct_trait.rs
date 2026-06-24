// LIMITATION: if struct is defined in `a` mod and impl is in `b`, then in order to mock impl mod `b`
// must import whole `a`, not just `a::Struct`, because in this case `a::StructSetup` won't be visible
trait Trait<T1> {
    fn f<T2>(&self) -> T1;
    fn g<T3>();
    fn tself<T4>(&self);
    fn tstatic<T5>();
}
trait Gen<G1> {
    fn f<G2>(&self);
    fn g<G3>();
    fn gself<G4>(&self);
    fn gstatic<G5>();
}

mod source {
    use super::*;

    // #[mock]
    struct Struct<S1>(pub S1);
    // #[mock]
    impl<S1> Struct<S1> {
        pub fn f<S2>(&self) {}
        pub fn g<S3>() {}
    }
    // #[mock]
    impl Struct<i8> {
        pub fn sself<S4>(&self) {}
        pub fn sstatic<S5>() {}
    }

    // this `<T1>` in `impl<T1>` dictates which generics should be in `as_Trait<T1>()`
    // #[mock]
    impl<T1> Trait<T1> for Struct<i16> {
        fn f<T2>(&self) -> T1 {
            todo!()
        }
        fn g<T3>() {}
        fn tself<T4>(&self) {}
        fn tstatic<T5>() {}
    }
    // #[mock]
    impl Trait<i64> for Struct<i128> {
        fn f<T2>(&self) -> i64 {
            todo!()
        }
        fn g<T3>() {}
        fn tself<T4>(&self) {}
        fn tstatic<T5>() {}
    }
    // #[mock]
    impl<G1, S1> Gen<G1> for Struct<S1> {
        fn f<G2>(&self) {}
        fn g<G3>() {}
        fn gself<G4>(&self) {}
        fn gstatic<G5>() {}
    }
}

mod result {
    use super::*;
    // TODO - replace this `use` with `use rsubstitute::prelude::*`
    use rsubstitute_core::infrastructure::*;

    // mod visibility is same as targets or public if target doesn't have visibility (like `impl`)
    use struct_mock::*;
    mod struct_mock {
        use super::*;
        use std::marker::PhantomData;
        use std::ops::{Deref, DerefMut};

        // source
        pub struct Struct<S1>(pub S1);

        pub trait IMockable<S1> {
            type Mock;
            fn mock(self) -> Self::Mock;
        }

        impl<S1> IMockable<S1> for Struct<S1> {
            type Mock = StructMock<S1>;
            fn mock(self) -> StructMock<S1> {
                StructMock {
                    data: Default::default(),
                    mockable: Box::new(self),
                }
            }
        }

        pub struct StructMock<S1> {
            #[doc(hidden)]
            pub data: SharedMockData<StructMock<S1>, true, false>,
            mockable: Box<Struct<S1>>,
        }

        // TODO - create only if there are static fns
        pub trait IStaticSetup {
            type Setup;
            fn setup() -> Self::Setup;
        }

        impl<S1> IStaticSetup for Struct<S1> {
            type Setup = StructStaticSetup<S1>;
            fn setup() -> Self::Setup {
                StructStaticSetup {
                    _generics: PhantomData,
                }
            }
        }

        pub struct StructSetup<S1> {
            #[doc(hidden)]
            pub data: SharedMockData<StructMock<S1>, true, false>,
            _generics: PhantomData<(S1,)>,
        }
        pub struct StructStaticSetup<S1> {
            _generics: PhantomData<(S1,)>,
        }

        impl<S1> StructMock<S1> {
            pub fn setup(&self) -> StructSetup<S1> {
                StructSetup {
                    data: self.data.clone(),
                    _generics: PhantomData,
                }
            }
        }

        impl<S1> Deref for StructMock<S1> {
            type Target = Struct<S1>;

            fn deref(&self) -> &Self::Target {
                self.mockable.as_ref()
            }
        }

        impl<S1> DerefMut for StructMock<S1> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.mockable.as_mut()
            }
        }
    }

    mod struct_impl_line22_col4 {
        use super::*;

        // source
        impl<S1> Struct<S1> {
            pub fn f<S2>(&self) {}
            pub fn g<S3>() {}
        }

        impl<S1> StructSetup<S1> {
            pub fn f<S2>(&mut self, _: i32) {
                let fn_data: &FnData<StructMock<S1>, true, false> =
                    self.data.borrow_mut().get_fn_data("f");
            }
        }

        impl<S1> StructStaticSetup<S1> {
            pub fn g<S3>(&self, _: i32) {
                let fn_data: &FnData<StructMock<S1>, true, false> = get_static_fn_data("g");
            }
        }

        // Applying impl to StructMock because that's where fn config is used
        // (and optionally base impl is called)
        impl<S1> StructMock<S1> {
            pub fn f<S2>(&self) {
                let fn_data: &FnData<StructMock<S1>, true, false> =
                    self.data.borrow_mut().get_fn_data("f");
            }
            pub fn g<S3>() {
                let fn_data: &FnData<StructMock<S1>, true, false> = get_static_fn_data("g");
            }
        }
    }

    mod struct_impl_line27_col4 {
        use super::*;

        // source
        impl Struct<i8> {
            pub fn sself<S4>(&self) {}
            pub fn sstatic<S5>() {}
        }

        impl StructSetup<i8> {
            pub fn sself<S4>(&self, _: i32) {
                let fn_data: &FnData<StructMock<i8>, true, false> =
                    self.data.borrow_mut().get_fn_data("sself");
            }
        }

        impl StructStaticSetup<i8> {
            pub fn sstatic<S5>(&self, _: i32) {
                let fn_data: &FnData<StructMock<i8>, true, false> = get_static_fn_data("sstatic");
            }
        }

        impl<'__rs> StructMock<i8> {
            pub fn sself<S4>(&self) {
                let fn_data: &FnData<StructMock<i8>, true, false> =
                    self.data.borrow_mut().get_fn_data("sself");
            }
            pub fn sstatic<S5>() {
                let fn_data: &FnData<StructMock<i8>, true, false> = get_static_fn_data("sstatic");
            }
        }
    }

    use struct_impl_line34_col4::*;
    mod struct_impl_line34_col4 {
        use super::*;
        use std::marker::PhantomData;

        // source
        impl<T1> Trait<T1> for Struct<i16> {
            fn f<T2>(&self) -> T1 {
                todo!()
            }
            fn g<T3>() {}
            fn tself<T4>(&self) {}
            fn tstatic<T5>() {}
        }

        pub struct StructTraitSetup<T1> {
            #[doc(hidden)]
            pub data: SharedMockData<StructMock<i16>, true, false>,
            _generics: PhantomData<(T1,)>,
        }
        pub struct StructTraitStaticSetup<T1> {
            _generics: PhantomData<(T1,)>,
        }

        impl<T1> StructTraitSetup<T1> {
            pub fn f<T2>(&self, _: i32) {
                let fn_data: &FnData<StructMock<i16>, true, false> =
                    self.data.borrow_mut().get_fn_data("Trait::f");
            }
            pub fn tself<T4>(&self, _: i32) {
                let fn_data: &FnData<StructMock<i16>, true, false> =
                    self.data.borrow_mut().get_fn_data("Trait::tself");
            }
        }

        impl<T1> StructTraitStaticSetup<T1> {
            pub fn g<T4>(&self, _: i32) {
                let fn_data: &FnData<StructMock<i16>, true, false> = get_static_fn_data("Trait::g");
            }
            pub fn tstatic<T5>(&self, _: i32) {
                let fn_data: &FnData<StructMock<i16>, true, false> =
                    get_static_fn_data("Trait::tstatic");
            }
        }

        impl StructSetup<i16> {
            pub fn as_Trait<T1>(&self) -> StructTraitSetup<T1> {
                StructTraitSetup {
                    data: self.data.clone(),
                    _generics: PhantomData,
                }
            }
        }

        impl StructStaticSetup<i16> {
            pub fn as_Trait<T1>(&self) -> StructTraitStaticSetup<T1> {
                StructTraitStaticSetup {
                    _generics: PhantomData,
                }
            }
        }

        impl<T1> Trait<T1> for StructMock<i16> {
            fn f<T2>(&self) -> T1 {
                let fn_data: &FnData<StructMock<i16>, true, false> =
                    self.data.borrow_mut().get_fn_data("Trait::f");
                todo!()
            }
            fn g<T3>() {
                let fn_data: &FnData<StructMock<i16>, true, false> = get_static_fn_data("Trait::g");
            }
            fn tself<T4>(&self) {
                let fn_data: &FnData<StructMock<i16>, true, false> =
                    self.data.borrow_mut().get_fn_data("Trait::tself");
            }
            fn tstatic<T5>() {
                let fn_data: &FnData<StructMock<i16>, true, false> =
                    get_static_fn_data("Trait::tstatic");
            }
        }
    }

    use struct_impl_line41_col4::*;
    mod struct_impl_line41_col4 {
        use super::*;
        use std::marker::PhantomData;

        // source
        impl Trait<i64> for Struct<i128> {
            fn f<T2>(&self) -> i64 {
                todo!()
            }
            fn g<T3>() {}
            fn tself<T4>(&self) {}
            fn tstatic<T5>() {}
        }

        pub struct StructTraitSetup {
            #[doc(hidden)]
            pub data: SharedMockData<StructMock<i128>, true, false>,
            _generics: PhantomData<()>,
        }
        pub struct StructTraitStaticSetup {
            _generics: PhantomData<()>,
        }

        impl StructTraitSetup {
            pub fn f<T2>(&self, _: i32) {
                let fn_data: &FnData<StructMock<i128>, true, false> =
                    self.data.borrow_mut().get_fn_data("Trait::f");
            }
            pub fn tself<T4>(&self, _: i32) {
                let fn_data: &FnData<StructMock<i128>, true, false> =
                    self.data.borrow_mut().get_fn_data("Trait::tself");
            }
        }

        impl StructTraitStaticSetup {
            pub fn g<T4>(&self, _: i32) {
                let fn_data: &FnData<StructMock<i128>, true, false> =
                    get_static_fn_data("Trait::g");
            }
            pub fn tstatic<T5>(&self, _: i32) {
                let fn_data: &FnData<StructMock<i128>, true, false> =
                    get_static_fn_data("Trait::tstatic");
            }
        }

        impl StructSetup<i128> {
            pub fn as_Trait(&self) -> StructTraitSetup {
                StructTraitSetup {
                    data: self.data.clone(),
                    _generics: PhantomData,
                }
            }
        }

        impl StructStaticSetup<i128> {
            pub fn as_Trait(&self) -> StructTraitStaticSetup {
                StructTraitStaticSetup {
                    _generics: PhantomData,
                }
            }
        }

        impl<'__rs> Trait<i64> for StructMock<i128> {
            fn f<T2>(&self) -> i64 {
                let fn_data: &FnData<StructMock<i128>, true, false> =
                    self.data.borrow_mut().get_fn_data("Trait::f");
                todo!()
            }
            fn g<T3>() {
                let fn_data: &FnData<StructMock<i128>, true, false> =
                    get_static_fn_data("Trait::g");
            }
            fn tself<T4>(&self) {
                let fn_data: &FnData<StructMock<i128>, true, false> =
                    self.data.borrow_mut().get_fn_data("Trait::tself");
            }
            fn tstatic<T5>() {
                let fn_data: &FnData<StructMock<i128>, true, false> =
                    get_static_fn_data("Trait::tstatic");
            }
        }
    }

    use struct_impl_line48_col4::*;
    mod struct_impl_line48_col4 {
        use super::*;
        use std::marker::PhantomData;

        // source
        impl<G1, S1> Gen<G1> for Struct<S1> {
            fn f<T2>(&self) {}
            fn g<T3>() {}
            fn gself<T4>(&self) {}
            fn gstatic<T5>() {}
        }

        pub struct StructGenSetup<G1, S1> {
            #[doc(hidden)]
            pub data: SharedMockData<StructMock<S1>, true, false>,
            _generics: PhantomData<(G1, S1)>,
        }
        pub struct StructGenStaticSetup<G1, S1> {
            _generics: PhantomData<(G1, S1)>,
        }

        impl<G1, S1> StructGenSetup<G1, S1> {
            pub fn f<T2>(&self, _: i32) {
                let fn_data: &FnData<StructMock<S1>, true, false> =
                    self.data.borrow_mut().get_fn_data("Gen::f");
            }
            pub fn gself<T4>(&self, _: i32) {
                let fn_data: &FnData<StructMock<S1>, true, false> =
                    self.data.borrow_mut().get_fn_data("Gen::tself");
            }
        }

        impl<G1, S1> StructGenStaticSetup<G1, S1> {
            pub fn g<T4>(&self, _: i32) {
                let fn_data: &FnData<StructMock<S1>, true, false> = get_static_fn_data("Gen::g");
            }
            pub fn gstatic<T5>(&self, _: i32) {
                let fn_data: &FnData<StructMock<S1>, true, false> =
                    get_static_fn_data("Gen::tstatic");
            }
        }

        impl<S1> StructSetup<S1> {
            pub fn as_Gen<G1>(&self) -> StructGenSetup<G1, S1> {
                StructGenSetup {
                    data: self.data.clone(),
                    _generics: PhantomData,
                }
            }
        }

        impl<S1> StructStaticSetup<S1> {
            pub fn as_Gen<G1>(&self) -> StructGenStaticSetup<G1, S1> {
                StructGenStaticSetup {
                    _generics: PhantomData,
                }
            }
        }

        impl<G1, S1> Gen<G1> for StructMock<S1> {
            fn f<T2>(&self) {
                let fn_data: &FnData<StructMock<S1>, true, false> =
                    self.data.borrow_mut().get_fn_data("Gen::f");
            }
            fn g<T3>() {
                let fn_data: &FnData<StructMock<S1>, true, false> = get_static_fn_data("Gen::g");
            }
            fn gself<T4>(&self) {
                let fn_data: &FnData<StructMock<S1>, true, false> =
                    self.data.borrow_mut().get_fn_data("Gen::tself");
            }
            fn gstatic<T5>() {
                let fn_data: &FnData<StructMock<i16>, true, false> =
                    get_static_fn_data("Gen::tstatic");
            }
        }
    }

    fn usage() {
        {
            let s = Struct(-1i64);
            // s.sself::<i32>();
            let mock = s.mock();
            mock.setup().f::<i32>(1);
            mock.setup().f::<[u8; 1]>(1);
            // mock.setup.sself::<i32>(1);
            mock.f::<i32>();
            // mock.sself();

            Struct::<i32>::setup().g::<i64>(2);
            Struct::<[u8; 1]>::setup().g::<[u8; 2]>(2);
            // Struct::<i32>::setup.sstatic::<i32>(2);
            Struct::<i32>::g::<i32>();
            Struct::<[u8; 1]>::g::<[u8; 2]>();
            // Struct::<i32>::sstatic();
        }
        {
            let s = Struct(-1i8);
            s.sself::<i32>();
            let mock = s.mock();
            mock.setup().f::<i32>(1);
            mock.setup().f::<[u8; 1]>(1);
            mock.setup().sself::<i32>(1);
            mock.f::<i32>();
            mock.sself::<i32>();

            Struct::<i32>::setup().g::<i64>(2);
            Struct::<[u8; 1]>::setup().g::<[u8; 2]>(2);
            Struct::<i8>::setup().sstatic::<i32>(2);
            Struct::<i32>::g::<i32>();
            Struct::<[u8; 1]>::g::<[u8; 2]>();
            Struct::<i8>::sstatic::<i32>();
        }
        {
            let s = Struct(-1i16);
            <Struct<i16> as Trait<[u8; 1]>>::f::<[u8; 2]>(&s);
            <Struct<i16> as Trait<[u8; 1]>>::g::<[u8; 2]>();
            <Struct<i16> as Trait<[u8; 1]>>::tself::<[u8; 2]>(&s);
            <Struct<i16> as Trait<[u8; 1]>>::tstatic::<[u8; 2]>();
            let mock = s.mock();
            <Struct<i16> as Trait<[u8; 1]>>::tself::<[u8; 1]>(&mock);
            mock.setup().as_Trait::<[u8; 1]>().tself::<[u8; 2]>(1);
            Struct::<i16>::setup()
                .as_Trait::<[u8; 1]>()
                .tstatic::<[u8; 2]>(2);
        }
        {
            let s = Struct(-1i128);
            <Struct<i128> as Trait<i64>>::f::<[u8; 2]>(&s);
            <Struct<i128> as Trait<i64>>::g::<[u8; 2]>();
            <Struct<i128> as Trait<i64>>::tself::<[u8; 2]>(&s);
            <Struct<i128> as Trait<i64>>::tstatic::<[u8; 2]>();
            let mock = s.mock();
            <Struct<i128> as Trait<i64>>::tself::<[u8; 1]>(&mock);
            mock.setup().as_Trait().tself::<[u8; 2]>(1);
            Struct::<i128>::setup().as_Trait().tstatic::<[u8; 2]>(2);
        }
        {
            let s = Struct(-1i32);
            <Struct<i32> as Gen<[u8; 1]>>::f::<[u8; 2]>(&s);
            <Struct<i32> as Gen<[u8; 1]>>::g::<[u8; 2]>();
            <Struct<i32> as Gen<[u8; 1]>>::gself::<[u8; 2]>(&s);
            <Struct<i32> as Gen<[u8; 1]>>::gstatic::<[u8; 2]>();
            let mock = s.mock();
            <Struct<i32> as Gen<i64>>::gself::<[u8; 1]>(&mock);
            mock.setup().as_Gen::<[u8; 24]>().gself::<[u8; 3]>(1);
            Struct::<i32>::setup()
                .as_Gen::<[u8; 2]>()
                .gstatic::<[u8; 3]>(3);
        }
    }
}
