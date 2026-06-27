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

        pub trait IMockable {
            type Mock;
            fn mock(self) -> Self::Mock;
        }

        impl<S1> IMockable for Struct<S1> {
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
            pub data: SharedMockData<StructMock<S1>>,
            mockable: Box<Struct<S1>>,
        }

        // TODO - create only if there are static fns
        pub trait IStaticSetup {
            type Setup;
            fn setup_static() -> Self::Setup;
        }

        impl<S1> IStaticSetup for Struct<S1> {
            type Setup = StructStaticSetup<S1>;
            fn setup_static() -> Self::Setup {
                StructStaticSetup {
                    _generics: PhantomData,
                }
            }
        }

        pub struct StructSetup<S1> {
            #[doc(hidden)]
            pub data: SharedMockData<StructMock<S1>>,
            _generics: PhantomData<(S1,)>,
        }
        pub struct StructStaticSetup<S1> {
            _generics: PhantomData<(S1,)>,
        }

        impl<S1> StructMock<S1> {
            pub fn setup(&mut self) -> StructSetup<S1> {
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
            pub fn f<'__rsa, S2>(
                &mut self,
                _: i32,
            ) -> FnConfigurator<
                '__rsa,
                StructMock<S1>,
                StructSetup<S1>,
                (),
                (),
                &mut Struct<S1>,
                false,
                true,
                true,
            > {
                let fn_data: &FnData<StructMock<S1>, false, true, false> =
                    self.data.get_shared_fn_data("f");
                todo!()
            }
        }

        impl<S1> StructStaticSetup<S1> {
            pub fn g<'__rsa, S3>(
                &self,
                _: i32,
            ) -> FnConfigurator<
                '__rsa,
                StructMock<S1>,
                StructStaticSetup<S1>,
                (),
                (),
                (),
                false,
                true,
                false,
            > {
                let fn_data: &FnData<StructMock<S1>, false, true, false> = get_static_fn_data("g");
                todo!()
            }
        }

        // Applying impl to StructMock because that's where fn config is used
        // (and optionally base impl is called)
        impl<S1> StructMock<S1> {
            pub fn f<S2>(&self) {
                let fn_data: &FnData<StructMock<S1>, false, true, false> =
                    self.data.get_shared_fn_data("f");
            }
            pub fn g<S3>() {
                let fn_data: &FnData<StructMock<S1>, false, true, false> = get_static_fn_data("g");
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
            pub fn sself<'__rsa, S4>(
                &self,
                _: i32,
            ) -> FnConfigurator<
                '__rsa,
                StructMock<i8>,
                StructSetup<i8>,
                (),
                (),
                &mut Struct<i8>,
                false,
                true,
                true,
            > {
                let fn_data: &FnData<StructMock<i8>, false, true, false> =
                    self.data.get_shared_fn_data("sself");
                todo!()
            }
        }

        impl StructStaticSetup<i8> {
            pub fn sstatic<'__rsa, S5>(
                &self,
                _: i32,
            ) -> FnConfigurator<
                '__rsa,
                StructMock<i8>,
                StructStaticSetup<i8>,
                (),
                (),
                (),
                false,
                true,
                false,
            > {
                let fn_data: &FnData<StructMock<i8>, false, true, false> =
                    get_static_fn_data("sstatic");
                todo!()
            }
        }

        impl<'__rs> StructMock<i8> {
            pub fn sself<S4>(&self) {
                let fn_data: &FnData<StructMock<i8>, false, true, false> =
                    self.data.get_shared_fn_data("sself");
            }
            pub fn sstatic<S5>() {
                let fn_data: &FnData<StructMock<i8>, false, true, false> =
                    get_static_fn_data("sstatic");
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
            pub data: SharedMockData<StructMock<i16>>,
            _generics: PhantomData<(T1,)>,
        }
        pub struct StructTraitStaticSetup<T1> {
            _generics: PhantomData<(T1,)>,
        }

        impl<T1> StructTraitSetup<T1> {
            pub fn f<'__rsa, T2>(
                &self,
                _: i32,
            ) -> FnConfigurator<
                '__rsa,
                StructMock<i16>,
                StructTraitSetup<i16>,
                (),
                T1,
                &mut Struct<i16>,
                true,
                true,
                true,
            > {
                let fn_data: &FnData<StructMock<i16>, true, true, false> =
                    self.data.get_shared_fn_data("Trait::f");
                todo!()
            }
            pub fn tself<'__rsa, T4>(
                &self,
                _: i32,
            ) -> FnConfigurator<
                '__rsa,
                StructMock<i16>,
                StructTraitSetup<i16>,
                (),
                (),
                &mut Struct<i16>,
                false,
                true,
                true,
            > {
                let fn_data: &FnData<StructMock<i16>, false, true, false> =
                    self.data.get_shared_fn_data("Trait::tself");
                todo!()
            }
        }

        impl<T1> StructTraitStaticSetup<T1> {
            pub fn g<'__rsa, T4>(
                &self,
                _: i32,
            ) -> FnConfigurator<
                '__rsa,
                StructMock<i16>,
                StructTraitStaticSetup<i16>,
                (),
                (),
                (),
                false,
                true,
                false,
            > {
                let fn_data: &FnData<StructMock<i16>, false, true, false> =
                    get_static_fn_data("Trait::g");
                todo!()
            }
            pub fn tstatic<'__rsa, T5>(
                &self,
                _: i32,
            ) -> FnConfigurator<
                '__rsa,
                StructMock<i16>,
                StructTraitStaticSetup<i16>,
                (),
                (),
                (),
                false,
                true,
                false,
            > {
                let fn_data: &FnData<StructMock<i16>, false, true, false> =
                    get_static_fn_data("Trait::tstatic");
                todo!()
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
                let fn_data: &FnData<StructMock<i16>, false, true, false> =
                    self.data.get_shared_fn_data("Trait::f");
                todo!()
            }
            fn g<T3>() {
                let fn_data: &FnData<StructMock<i16>, false, true, false> =
                    get_static_fn_data("Trait::g");
            }
            fn tself<T4>(&self) {
                let fn_data: &FnData<StructMock<i16>, false, true, false> =
                    self.data.get_shared_fn_data("Trait::tself");
            }
            fn tstatic<T5>() {
                let fn_data: &FnData<StructMock<i16>, false, true, false> =
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
            pub data: SharedMockData<StructMock<i128>>,
            _generics: PhantomData<()>,
        }
        pub struct StructTraitStaticSetup {
            _generics: PhantomData<()>,
        }

        impl StructTraitSetup {
            pub fn f<'__rsa, T2>(
                &self,
                _: i32,
            ) -> FnConfigurator<
                '__rsa,
                StructMock<i16>,
                crate::struct_trait_mock::result::struct_impl_line34_col4::StructTraitSetup<i16>,
                (),
                (),
                &mut Struct<i16>,
                false,
                true,
                true,
            > {
                let fn_data: &FnData<StructMock<i128>, false, true, false> =
                    self.data.get_shared_fn_data("Trait::f");
                todo!()
            }
            pub fn tself<'__rsa, T4>(
                &self,
                _: i32,
            ) -> FnConfigurator<
                '__rsa,
                StructMock<i16>,
                crate::struct_trait_mock::result::struct_impl_line34_col4::StructTraitSetup<i16>,
                (),
                (),
                &mut Struct<i16>,
                false,
                true,
                true,
            > {
                let fn_data: &FnData<StructMock<i128>, false, true, false> =
                    self.data.get_shared_fn_data("Trait::tself");
                todo!()
            }
        }

        impl StructTraitStaticSetup {
            pub fn g<'__rsa, T4>(
                &self,
                _: i32,
            ) -> FnConfigurator<
                '__rsa,
                StructMock<i16>,
                crate::struct_trait_mock::result::struct_impl_line34_col4::StructTraitStaticSetup<
                    i16,
                >,
                (),
                (),
                (),
                false,
                true,
                false,
            > {
                let fn_data: &FnData<StructMock<i128>, false, true, false> =
                    get_static_fn_data("Trait::g");
                todo!()
            }
            pub fn tstatic<'__rsa, T5>(
                &self,
                _: i32,
            ) -> FnConfigurator<
                '__rsa,
                StructMock<i16>,
                crate::struct_trait_mock::result::struct_impl_line34_col4::StructTraitStaticSetup<
                    i16,
                >,
                (),
                (),
                (),
                false,
                true,
                false,
            > {
                let fn_data: &FnData<StructMock<i128>, false, true, false> =
                    get_static_fn_data("Trait::tstatic");
                todo!()
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
                let fn_data: &FnData<StructMock<i128>, false, true, false> =
                    self.data.get_shared_fn_data("Trait::f");
                todo!()
            }
            fn g<T3>() {
                let fn_data: &FnData<StructMock<i128>, false, true, false> =
                    get_static_fn_data("Trait::g");
            }
            fn tself<T4>(&self) {
                let fn_data: &FnData<StructMock<i128>, false, true, false> =
                    self.data.get_shared_fn_data("Trait::tself");
            }
            fn tstatic<T5>() {
                let fn_data: &FnData<StructMock<i128>, false, true, false> =
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
            pub data: SharedMockData<StructMock<S1>>,
            _generics: PhantomData<(G1, S1)>,
        }
        pub struct StructGenStaticSetup<G1, S1> {
            _generics: PhantomData<(G1, S1)>,
        }

        impl<G1, S1> StructGenSetup<G1, S1> {
            pub fn f<'__rsa, T2>(
                &self,
                _: i32,
            ) -> FnConfigurator<
                '__rsa,
                StructMock<S1>,
                StructGenSetup<G1, S1>,
                (),
                (),
                &mut Struct<S1>,
                false,
                true,
                true,
            > {
                let fn_data: &FnData<StructMock<S1>, false, true, false> =
                    self.data.get_shared_fn_data("Gen::f");
                todo!()
            }
            pub fn gself<'__rsa, T4>(
                &self,
                _: i32,
            ) -> FnConfigurator<
                '__rsa,
                StructMock<S1>,
                StructGenSetup<G1, S1>,
                (),
                (),
                &mut Struct<S1>,
                false,
                true,
                true,
            > {
                let fn_data: &FnData<StructMock<S1>, false, true, false> =
                    self.data.get_shared_fn_data("Gen::tself");
                todo!()
            }
        }

        impl<G1, S1> StructGenStaticSetup<G1, S1> {
            pub fn g<'__rsa, T4>(
                &self,
                _: i32,
            ) -> FnConfigurator<
                '__rsa,
                StructMock<S1>,
                StructGenStaticSetup<G1, S1>,
                (),
                (),
                (),
                false,
                true,
                false,
            > {
                let fn_data: &FnData<StructMock<S1>, false, true, false> =
                    get_static_fn_data("Gen::g");
                todo!()
            }
            pub fn gstatic<'__rsa, T5>(
                &self,
                _: i32,
            ) -> FnConfigurator<
                '__rsa,
                StructMock<S1>,
                StructGenStaticSetup<G1, S1>,
                (),
                (),
                (),
                false,
                true,
                false,
            > {
                let fn_data: &FnData<StructMock<S1>, false, true, false> =
                    get_static_fn_data("Gen::tstatic");
                todo!()
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
                let fn_data: &FnData<StructMock<S1>, false, true, false> =
                    self.data.get_shared_fn_data("Gen::f");
            }
            fn g<T3>() {
                let fn_data: &FnData<StructMock<S1>, false, true, false> =
                    get_static_fn_data("Gen::g");
            }
            fn gself<T4>(&self) {
                let fn_data: &FnData<StructMock<S1>, false, true, false> =
                    self.data.get_shared_fn_data("Gen::tself");
            }
            fn gstatic<T5>() {
                let fn_data: &FnData<StructMock<i16>, false, true, false> =
                    get_static_fn_data("Gen::tstatic");
            }
        }
    }

    fn usage() {
        {
            let s = Struct(-1i64);
            // s.sself::<i32>();
            let mut mock = s.mock();
            mock.setup().f::<i32>(1);
            mock.setup().f::<[u8; 1]>(1);
            // mock.setup.sself::<i32>(1);
            mock.f::<i32>();
            // mock.sself();

            Struct::<i32>::setup_static().g::<i64>(2);
            Struct::<[u8; 1]>::setup_static().g::<[u8; 2]>(2);
            // Struct::<i32>::setup.sstatic::<i32>(2);
            Struct::<i32>::g::<i32>();
            Struct::<[u8; 1]>::g::<[u8; 2]>();
            // Struct::<i32>::sstatic();
        }
        {
            let s = Struct(-1i8);
            s.sself::<i32>();
            let mut mock = s.mock();
            mock.setup().f::<i32>(1);
            mock.setup().f::<[u8; 1]>(1);
            mock.setup().sself::<i32>(1);
            mock.f::<i32>();
            mock.sself::<i32>();

            Struct::<i32>::setup_static().g::<i64>(2);
            Struct::<[u8; 1]>::setup_static().g::<[u8; 2]>(2);
            Struct::<i8>::setup_static().sstatic::<i32>(2);
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
            let mut mock = s.mock();
            <Struct<i16> as Trait<[u8; 1]>>::tself::<[u8; 1]>(&mock);
            mock.setup()
                .as_Trait::<[u8; 1]>()
                .tself::<[u8; 2]>(1)
                .call_base()
                .f::<Vec<u8>>(3222)
                .returns_with(|_| 101)
                .and_does(|s, _| println!("{}", s.0))
                .f::<String>(111)
                .call_base()
                .and_does(|s, _| println!("called base after returning!"));
            Struct::<i16>::setup_static()
                .as_Trait::<[u8; 1]>()
                .tstatic::<[u8; 2]>(2);
        }
        {
            let s = Struct(-1i128);
            <Struct<i128> as Trait<i64>>::f::<[u8; 2]>(&s);
            <Struct<i128> as Trait<i64>>::g::<[u8; 2]>();
            <Struct<i128> as Trait<i64>>::tself::<[u8; 2]>(&s);
            <Struct<i128> as Trait<i64>>::tstatic::<[u8; 2]>();
            let mut mock = s.mock();
            <Struct<i128> as Trait<i64>>::tself::<[u8; 1]>(&mock);
            mock.setup().as_Trait().tself::<[u8; 2]>(1);
            Struct::<i128>::setup_static()
                .as_Trait()
                .tstatic::<[u8; 2]>(2);
        }
        {
            let s = Struct(-1i32);
            <Struct<i32> as Gen<[u8; 1]>>::f::<[u8; 2]>(&s);
            <Struct<i32> as Gen<[u8; 1]>>::g::<[u8; 2]>();
            <Struct<i32> as Gen<[u8; 1]>>::gself::<[u8; 2]>(&s);
            <Struct<i32> as Gen<[u8; 1]>>::gstatic::<[u8; 2]>();
            let mut mock = s.mock();
            <Struct<i32> as Gen<i64>>::gself::<[u8; 1]>(&mock);
            mock.setup().as_Gen::<[u8; 24]>().gself::<[u8; 3]>(1);
            Struct::<i32>::setup_static()
                .as_Gen::<[u8; 2]>()
                .gstatic::<[u8; 3]>(3)
                .call_base()
                .g::<[u8; 111]>(21)
                .does(|_| {});
        }
    }
}
