// LIMITATION: if struct is defined in `a` mod and impl is in `b`, then in order to mock impl mod `b`
// must import whole `a`, not just `a::Struct`, because in this case `a::StructSetup` won't be visible
trait Trait<T1> {
    fn f<T2>(&self);
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
        fn f<T2>(&self) {}
        fn g<T3>() {}
        fn tself<T4>(&self) {}
        fn tstatic<T5>() {}
    }
    // #[mock]
    impl Trait<i64> for Struct<i128> {
        fn f<T2>(&self) {}
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
    use rsubstitute_core::*;

    // mod visibility is same as targets or public if target doesn't have visibility (like `impl`)
    use struct_mock::*;
    mod struct_mock {
        use super::*;
        use std::marker::PhantomData;
        use std::ops::{Deref, DerefMut};

        pub struct Struct<S1>(pub S1);

        impl<'__rs, S1> IMockable<StructMock<'__rs, S1>> for Struct<S1> {
            fn mock_from_ref(&mut self) -> StructMock<'__rs, S1> {
                StructMock {
                    setup: todo!(),
                    mockable: transmute_lifetime!(self),
                }
            }
        }

        pub struct StructSetup<S1>(String, PhantomData<S1>);
        pub struct StructStaticSetup<S1>(PhantomData<S1>);

        pub struct StructMock<'__rs, S1> {
            pub setup: StructSetup<S1>,
            mockable: &'__rs mut Struct<S1>,
        }

        impl<'__rs, S1> Deref for StructMock<'__rs, S1> {
            type Target = Struct<S1>;

            fn deref(&self) -> &Self::Target {
                self.mockable
            }
        }

        impl<'__rs, S1> DerefMut for StructMock<'__rs, S1> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.mockable
            }
        }

        impl<'__rs, S1> IMock<Struct<S1>> for StructMock<'__rs, S1> {}

        impl<'__rs, S1> Drop for StructMock<'__rs, S1> {
            fn drop(&mut self) {
                self.drop_boxed_mockable()
            }
        }

        impl<S1> Struct<S1> {
            #[allow(non_upper_case_globals)]
            pub const setup: StructStaticSetup<S1> = StructStaticSetup(todo!());
        }
    }

    use struct_impl_line16_col4::*;
    mod struct_impl_line16_col4 {
        use super::*;

        impl<S1> Struct<S1> {
            pub fn f<S2>(&self) {}
            pub fn g<S3>() {}
        }

        impl<S1> StructSetup<S1> {
            pub fn f<S2>(&self, _: i32) {}
        }

        impl<S1> StructStaticSetup<S1> {
            pub fn g<S3>(&self, _: i32) {}
        }

        // Applying impl to StructMock because that's where fn config is used
        // (and optionally base impl is called)
        impl<'__rs, S1> StructMock<'__rs, S1> {
            pub fn f<S2>(&self) {}
            pub fn g<S3>() {}
        }
    }

    use struct_impl_line21_col4::*;
    mod struct_impl_line21_col4 {
        use super::*;

        impl Struct<i8> {
            pub fn sself<S4>(&self) {}
            pub fn sstatic<S5>() {}
        }

        impl StructSetup<i8> {
            pub fn sself<S4>(&self, _: i32) {}
        }

        impl StructStaticSetup<i8> {
            pub fn sstatic<S5>(&self, _: i32) {}
        }

        impl<'__rs> StructMock<'__rs, i8> {
            pub fn sself<S4>(&self) {}
            pub fn sstatic<S5>() {}
        }
    }

    use struct_impl_line34_col4::*;
    mod struct_impl_line34_col4 {
        use super::*;

        impl<T1> Trait<T1> for Struct<i16> {
            fn f<T2>(&self) {}
            fn g<T3>() {}
            fn tself<T4>(&self) {}
            fn tstatic<T5>() {}
        }

        pub struct StructTraitSetup(String);
        pub struct StructTraitStaticSetup;

        impl StructTraitSetup {
            pub fn f<T2>(&self, _: i32) {}
            pub fn tself<T4>(&self, _: i32) {}
        }

        impl StructTraitStaticSetup {
            pub fn g<T4>(&self, _: i32) {}
            pub fn tstatic<T5>(&self, _: i32) {}
        }

        impl StructSetup<i16> {
            pub fn as_Trait<T1>(&self) -> StructTraitSetup {
                StructTraitSetup(todo!())
            }
        }

        impl StructStaticSetup<i16> {
            pub fn as_Trait<T1>(&self) -> StructTraitStaticSetup {
                StructTraitStaticSetup
            }
        }

        impl<'__rs, T1> Trait<T1> for StructMock<'__rs, i16> {
            fn f<T2>(&self) {}
            fn g<T3>() {}
            fn tself<T4>(&self) {}
            fn tstatic<T5>() {}
        }
    }

    use struct_impl_line41_col4::*;
    mod struct_impl_line41_col4 {
        use super::*;

        impl Trait<i64> for Struct<i128> {
            fn f<T2>(&self) {}
            fn g<T3>() {}
            fn tself<T4>(&self) {}
            fn tstatic<T5>() {}
        }

        pub struct StructTraitSetup(String);
        pub struct StructTraitStaticSetup;

        impl StructTraitSetup {
            pub fn f<T2>(&self, _: i32) {}
            pub fn tself<T4>(&self, _: i32) {}
        }

        impl StructTraitStaticSetup {
            pub fn g<T4>(&self, _: i32) {}
            pub fn tstatic<T5>(&self, _: i32) {}
        }

        impl StructSetup<i128> {
            pub fn as_Trait(&self) -> StructTraitSetup {
                StructTraitSetup(todo!())
            }
        }

        impl StructStaticSetup<i128> {
            pub fn as_Trait(&self) -> StructTraitStaticSetup {
                StructTraitStaticSetup
            }
        }

        impl<'__rs> Trait<i64> for StructMock<'__rs, i128> {
            fn f<T2>(&self) {}
            fn g<T3>() {}
            fn tself<T4>(&self) {}
            fn tstatic<T5>() {}
        }
    }

    use struct_impl_line48_col4::*;
    mod struct_impl_line48_col4 {
        use super::*;
        use std::marker::PhantomData;

        impl<G1, S1> Gen<G1> for Struct<S1> {
            fn f<T2>(&self) {}
            fn g<T3>() {}
            fn gself<T4>(&self) {}
            fn gstatic<T5>() {}
        }

        pub struct StructGenSetup<G1>(String, PhantomData<G1>);
        pub struct StructGenStaticSetup<G1>(PhantomData<G1>);

        impl<G1> StructGenSetup<G1> {
            pub fn f<T2>(&self, _: i32) {}
            pub fn gself<T4>(&self, _: i32) {}
        }

        impl<G1> StructGenStaticSetup<G1> {
            pub fn g<T4>(&self, _: i32) {}
            pub fn gstatic<T5>(&self, _: i32) {}
        }

        impl<S1> StructSetup<S1> {
            pub fn as_Gen<G1>(&self) -> StructGenSetup<G1> {
                StructGenSetup(todo!(), PhantomData)
            }
        }

        impl<S1> StructStaticSetup<S1> {
            pub fn as_Gen<G1>(&self) -> StructGenStaticSetup<G1> {
                StructGenStaticSetup(PhantomData)
            }
        }

        impl<'__rs, G1, S1> Gen<G1> for StructMock<'__rs, S1> {
            fn f<T2>(&self) {}
            fn g<T3>() {}
            fn gself<T4>(&self) {}
            fn gstatic<T5>() {}
        }
    }

    fn usage() {
        {
            let s = Struct(-1i64);
            // s.sself::<i32>();
            let mock = s.mock();
            mock.setup.f::<i32>(1);
            mock.setup.f::<[u8; 1]>(1);
            // mock.setup.sself::<i32>(1);
            mock.f::<i32>();
            // mock.sself();

            Struct::<i32>::setup.g::<i64>(2);
            Struct::<[u8; 1]>::setup.g::<[u8; 2]>(2);
            // Struct::<i32>::setup.sstatic::<i32>(2);
            Struct::<i32>::g::<i32>();
            Struct::<[u8; 1]>::g::<[u8; 2]>();
            // Struct::<i32>::sstatic();
        }
        {
            let s = Struct(-1i8);
            s.sself::<i32>();
            let mock = s.mock();
            mock.setup.f::<i32>(1);
            mock.setup.f::<[u8; 1]>(1);
            mock.setup.sself::<i32>(1);
            mock.f::<i32>();
            mock.sself::<i32>();

            Struct::<i32>::setup.g::<i64>(2);
            Struct::<[u8; 1]>::setup.g::<[u8; 2]>(2);
            Struct::<i8>::setup.sstatic::<i32>(2);
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
            mock.setup.as_Trait::<[u8; 1]>().tself::<[u8; 2]>(1);
            Struct::<i16>::setup
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
            mock.setup.as_Trait().tself::<[u8; 2]>(1);
            Struct::<i128>::setup.as_Trait().tstatic::<[u8; 2]>(2);
        }
        {
            let s = Struct(-1i32);
            <Struct<i32> as Gen<[u8; 1]>>::f::<[u8; 2]>(&s);
            <Struct<i32> as Gen<[u8; 1]>>::g::<[u8; 2]>();
            <Struct<i32> as Gen<[u8; 1]>>::gself::<[u8; 2]>(&s);
            <Struct<i32> as Gen<[u8; 1]>>::gstatic::<[u8; 2]>();
            let mock = s.mock();
            <Struct<i32> as Gen<i64>>::gself::<[u8; 1]>(&mock);
            mock.setup.as_Gen::<[u8; 2]>().gself::<[u8; 3]>(1);
            Struct::<i32>::setup
                .as_Gen::<[u8; 2]>()
                .gstatic::<[u8; 3]>(3);
        }
    }
}
