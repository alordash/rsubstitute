mod source {
    pub fn f<T>(_: T) -> T {
        todo!()
    }
}

mod result {
    use f::*;
    mod f {
        use rsubstitute_core::args::*;
        use rsubstitute_core::fn_parameters::*;
        use rsubstitute_core::infrastructure::*;
        use std::marker::PhantomData;

        pub struct fMock<T> {
            generics: PhantomData<(T,)>,
        }
        pub struct fStaticSetup<T> {
            generics: PhantomData<(T,)>,
        }

        impl<T> fStaticSetup<T> {
            pub fn setup<'__rsa>(
                &self,
                _: T,
                _: i32,
            ) -> FnConfigurator<'__rsa, fMock<T>, fStaticSetup<T>, (&'__rsa T,), T, (), true, false>
            {
                let data: &FnData<fMock<T>, true, false> = get_static_fn_data("f");
                todo!()
            }
        }

        pub fn setup<'__rsa, T>(
            _: T,
            _: i32,
        ) -> FnConfigurator<'__rsa, fMock<T>, fStaticSetup<T>, (&'__rsa T,), T, (), true, false>
        {
            let data: &FnData<fMock<T>, true, false> = get_static_fn_data("f");
            todo!()
        }

        struct fCall<T> {
            generics: PhantomData<(T,)>,
        }
        impl<T> IGenericsInfoProvider for fCall<T> {
            fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
                todo!()
            }

            fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {
                todo!()
            }
        }
        impl<T> ICall for fCall<T> {
            fn get_arg_infos(&self) -> Vec<ArgInfo> {
                todo!()
            }

            fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
                todo!()
            }
        }
        impl<T> Clone for fCall<T> {
            fn clone(&self) -> Self {
                todo!()
            }
        }

        pub fn f<T>(input: T) -> T {
            let data: &FnData<fMock<T>, true, false> = get_static_fn_data("f");
            data.handle_base(
                &fMock {
                    generics: PhantomData,
                },
                fCall {
                    generics: PhantomData,
                },
                __rs_base_f::<T>,
            );
            todo!()
        }

        fn __rs_base_f<T>(_: &fMock<T>, _: fCall<T>) {}
    }

    fn usage() {
        f::<i32>(1234);
        f::setup::<i32>(234, 21)
            .returns(546456)
            .setup(325235, 333)
            .returns(111);
        f::setup::<&str>("quo vadis", 43)
            .returns("veridis quo")
            .setup("amogus", 11)
            .returns_with(|(s,)| s);
    }
}
