use rsubstitute_proc_macro::mocked_base;
use std::marker::PhantomData;

trait Same<'a, T> {
    fn work(&self, t: &'a T);
}

trait Different<'a, T> {
    fn work(&self, t: &'a T);
}

// mocked_base! {
//     struct Struct<'a, T>(PhantomData<&'a T>);
//
//     impl<'b, S> Struct<'b, S> {
//         pub fn new() -> Self {
//             Self(PhantomData)
//         }
//
//         pub fn work(&self, s: &'b S) {}
//     }
//
//     // impl<'a, T> Same<'a, T> for Struct<'a, T> {
//     //     fn work(&self, t: &'a T) {
//     //         unreachable!()
//     //     }
//     // }
//
//     // impl<'a, 'b, T1, T2> Different<'a, T1> for Struct<'b, T2> {
//     //     fn work(&self, t: &'a T1) {
//     //         unreachable!()
//     //     }
//     // }
// }

mod tests {
    #[test]
    fn compile() {}
}

#[cfg(not(test))]
struct Struct<'a, T>(PhantomData<&'a T>);
#[cfg(not(test))]
impl<'b, S> Struct<'b, S> {
    pub fn new() -> Self {
        Self(PhantomData)
    }

    pub fn work(&self, s: &'b S) {}
}
// impl<'a, 'b, T1, T2> Different<'a, T1> for Struct<'b, T2> {
//     fn work(&self, t: &'a T1) {
//         unreachable!()
//     }
// }
#[cfg(test)]
pub use __rsubstitute_generated_Struct::*;
#[cfg(test)]
#[allow(unused_parens)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(mismatched_lifetime_syntaxes)]
mod __rsubstitute_generated_Struct {
    use super::*;
    use rsubstitute::for_generated::*;
    #[doc(hidden)]
    pub struct work_Call<'a, T> {
        _phantom_GenericParam_a: PhantomData<&'a ()>,
        _phantom_GenericParam_T: PhantomData<T>,
        _phantom_s: PhantomData<&'b S>,
        s: *const S,
    }
    impl<'a, T> IArgsInfosProvider for work_Call<'a, T> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![ArgInfo::new(
                "s",
                &self.s,
                (&ArgPrinter::<&S>(transmute_lifetime!(&self.s))).debug_string(),
            )]
        }
    }
    impl<'a, T> IArgsTupleProvider for work_Call<'a, T> {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((&self.s,))) as *mut _ as *mut ()
        }
    }
    impl<'a, T> IGenericsInfoProvider for work_Call<'a, T> {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    impl<'a, T> Clone for work_Call<'a, T> {
        fn clone(&self) -> Self {
            Self {
                _phantom_GenericParam_a: (&self._phantom_GenericParam_a).clone(),
                _phantom_GenericParam_T: (&self._phantom_GenericParam_T).clone(),
                _phantom_s: (&self._phantom_s).clone(),
                s: (&self.s).clone(),
            }
        }
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct work_ArgsChecker<'a, T> {
        _phantom_GenericParam_a: PhantomData<&'a ()>,
        _phantom_GenericParam_T: PhantomData<T>,
        _phantom_s: PhantomData<&'b S>,
        s: Arg<*const S>,
    }
    impl<'a, T> IArgsChecker for work_ArgsChecker<'a, T> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &work_Call<'a, T> = dyn_call.downcast_ref();
            vec![transmute_lifetime!(&self.s, &Arg::<&'b S>).check_ref(
                "s",
                transmute_lifetime!(&call.s),
                (&ArgPrinter::<&S>(transmute_lifetime!(&call.s))).debug_string(),
            )]
        }
    }
    impl<'a, T> IArgsFormatter for work_ArgsChecker<'a, T> {
        fn fmt_args(&self) -> String {
            format!(
                "{}",
                (&ArgPrinter(&transmute_lifetime!(&self.s, &Arg::<&'b S>))).debug_string()
            )
        }
    }
    impl<'a, T> IGenericsInfoProvider for work_ArgsChecker<'a, T> {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct StructData<'a, T> {
        _phantom_GenericParam_a: PhantomData<&'a ()>,
        _phantom_GenericParam_T: PhantomData<T>,
        pub work: FnData<'static, Struct<'a, T>, true, true>,
    }
    #[doc(hidden)]
    pub struct StructSetup<'a, T> {
        data: Arc<StructData<'a, T>>,
    }
    impl<'a, T> Clone for StructSetup<'a, T> {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    #[doc(hidden)]
    pub struct StructReceived<'a, T> {
        data: Arc<StructData<'a, T>>,
    }
    impl<'a, T> Clone for StructReceived<'a, T> {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    #[doc(hidden)]
    pub struct Struct_InnerData<'a, T>(PhantomData<&'a T>);

    impl<'a, T> Struct_InnerData<'a, T> {
        pub fn new() -> Self {
            Self(PhantomData)
        }
    }
    pub struct Struct<'a, T> {
        pub setup: StructSetup<'a, T>,
        pub received: StructReceived<'a, T>,
        pub data: Arc<StructData<'a, T>>,
        inner_data: Struct_InnerData<'a, T>,
    }
    impl<'a, T> AsRef<Struct<'a, T>> for Struct<'a, T> {
        fn as_ref(&self) -> &Struct<'a, T> {
            self
        }
    }
    impl<'a, T> Deref for Struct<'a, T> {
        type Target = Struct_InnerData<'a, T>;
        fn deref(&self) -> &Self::Target {
            &self.inner_data
        }
    }
    impl<'a, T> Struct<'a, T> {
        pub fn work(&self, s: &'b S) {
            let call: work_Call<'_, T> = work_Call {
                _phantom_GenericParam_a: PhantomData,
                _phantom_GenericParam_T: PhantomData,
                _phantom_s: PhantomData,
                s: transmute_lifetime!(s),
            };
            self.data
                .clone()
                .work
                .handle_base(self, call, Self::base_work);
        }
    }
    impl<'a, T> Struct<'a, T> {
        pub fn new() -> Self {
            let data = Arc::new(StructData {
                _phantom_GenericParam_a: PhantomData,
                _phantom_GenericParam_T: PhantomData,
                work: FnData::new("work"),
            });
            let inner_data = Struct_InnerData::new();
            return Struct {
                setup: StructSetup { data: data.clone() },
                received: StructReceived { data: data.clone() },
                data,
                inner_data,
            };
        }
        fn base_work(&self, call: work_Call<'a, T>) {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)]
            let work_Call::<'_, T> { s: s, .. } = call;
            let s: &'b S = transmute_lifetime!(s);
        }
    }
    impl<'a, T> StructSetup<'a, T> {
        pub fn work<'__rsa>(
            &self,
            s: impl Into<Arg<&'b S>>,
        ) -> FnTuner<'_, Struct<'a, T>, Self, (&'__rsa &'b S,), (), &Self, true, true>
        where
            '__rsa: 'a,
            'a: '__rsa,
        {
            let work_args_checker: work_ArgsChecker<'a, T> = work_ArgsChecker {
                _phantom_GenericParam_a: PhantomData,
                _phantom_GenericParam_T: PhantomData,
                _phantom_s: PhantomData,
                s: transmute_lifetime!(s.into()),
            };
            let fn_tuner: FnTuner<
                '_,
                Struct<'a, T>,
                Self,
                (&'__rsa &'b S,),
                (),
                &Self,
                true,
                true,
            > = self.data.work.add_config(work_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
    }
    impl<'a, T> StructReceived<'a, T> {
        pub fn work<'__rsa>(
            &self,
            s: impl Into<Arg<&'b S>>,
            times: Times,
        ) -> FnVerifier<Self, (&'__rsa &'b S,)>
        where
            '__rsa: 'a,
            'a: '__rsa,
        {
            let work_args_checker: work_ArgsChecker<'a, T> = work_ArgsChecker {
                _phantom_GenericParam_a: PhantomData,
                _phantom_GenericParam_T: PhantomData,
                _phantom_s: PhantomData,
                s: transmute_lifetime!(s.into()),
            };
            self.data.work.verify_received(work_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
}
