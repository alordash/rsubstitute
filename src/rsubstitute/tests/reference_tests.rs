use rsubstitute::prelude::*;
use std::marker::PhantomData;

#[derive(Default, Debug, PartialEq, Clone)]
struct Data<'a, 'b, T1, T2> {
    _phantoms: (
        PhantomData<&'a ()>,
        PhantomData<&'b ()>,
        PhantomData<T1>,
        PhantomData<T2>,
    ),
}

#[allow(unused)]
trait Trait<'a, 'b: 'a, T1> {
    fn work<'c, 'd: 'a, T2: Clone>(
        &self,
        a: &'a i32,
        b: &'b i32,
        c: &'c i32,
        d: &'d i32,
        axb: &'a &&'b i32,
        cxd: &'c &&'d i32,
        abxbax: &'a &'b &&'b &'a &i32,
        cdxdcx: &'c &'d &&'d &'c &i32,
        abcd: &'a &'b &'c &'d i32,
        xaxbxcxdx: &&'a &&'b &&'c &&'d &i32,
        data: Data<
            'a,
            'b,
            &&i32,
            &&'a &&'b &[&'c &&'b &Data<'c, 'a, &&&'c &i32, Vec<&'d &'b &()>>],
        >,
        t1: T1,
        t1_ref: &T1,
        xaxbxcxdx_t1_ref: &&'a &&'b &&'c &&'d &T1,
        t2: T2,
        t2_ref: &T2,
        xaxbxcxdx_t2_ref: &&'a &&'b &&'c &&'d &T2,
        xapx: &&'a *const &i32,
    ) -> &&'a &&'a &&'b &&'b &&'c &&'c &&'d &&'d &i32;
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
    pub struct Trait_work_Call<'a, 'b: 'a, 'c, 'd: 'a, T1, T2: Clone> {
        _phantom_GenericParam_a: PhantomData<&'a ()>,
        _phantom_GenericParam_b: PhantomData<&'b ()>,
        _phantom_GenericParam_T1: PhantomData<T1>,
        _phantom_a: PhantomData<&'a i32>,
        _phantom_b: PhantomData<&'b i32>,
        _phantom_c: PhantomData<&'c i32>,
        _phantom_d: PhantomData<&'d i32>,
        _phantom_axb: PhantomData<&'a *const &'b i32>,
        _phantom_cxd: PhantomData<&'c *const &'d i32>,
        _phantom_abxbax: PhantomData<&'a &'b *const &'b &'a *const i32>,
        _phantom_cdxdcx: PhantomData<&'c &'d *const &'d &'c *const i32>,
        _phantom_abcd: PhantomData<&'a &'b &'c &'d i32>,
        _phantom_xaxbxcxdx: PhantomData<*const &'a *const &'b *const &'c *const &'d *const i32>,
        _phantom_data: PhantomData<
            Data<
                'a,
                'b,
                *const *const i32,
                *const &'a *const &'b *const [&'c *const &'b *const Data<
                    'c,
                    'a,
                    *const *const &'c *const i32,
                    Vec<&'d &'b *const ()>,
                >],
            >,
        >,
        _phantom_t1: PhantomData<T1>,
        _phantom_t1_ref: PhantomData<*const T1>,
        _phantom_xaxbxcxdx_t1_ref:
            PhantomData<*const &'a *const &'b *const &'c *const &'d *const T1>,
        _phantom_t2: PhantomData<T2>,
        _phantom_t2_ref: PhantomData<*const T2>,
        _phantom_xaxbxcxdx_t2_ref:
            PhantomData<*const &'a *const &'b *const &'c *const &'d *const T2>,
        _phantom_xapx: PhantomData<*const &'a *const *const i32>,
        _phantom_GenericParam_c: PhantomData<&'c ()>,
        _phantom_GenericParam_d: PhantomData<&'d ()>,
        _phantom_GenericParam_T2: PhantomData<T2>,
        a: *const i32,
        b: *const i32,
        c: *const i32,
        d: *const i32,
        axb: *const *const *const i32,
        cxd: *const *const *const i32,
        abxbax: *const *const *const *const *const *const i32,
        cdxdcx: *const *const *const *const *const *const i32,
        abcd: *const *const *const *const i32,
        xaxbxcxdx: *const *const *const *const *const *const *const *const *const i32,
        data: Data<
            'a,
            'b,
            *const *const i32,
            *const *const *const *const *const [*const *const *const *const Data<
                'c,
                'a,
                *const *const *const *const i32,
                Vec<*const *const *const ()>,
            >],
        >,
        t1: T1,
        t1_ref: *const T1,
        xaxbxcxdx_t1_ref: *const *const *const *const *const *const *const *const *const T1,
        t2: T2,
        t2_ref: *const T2,
        xaxbxcxdx_t2_ref: *const *const *const *const *const *const *const *const *const T2,
        xapx: *const *const *const *const i32,
    }
    impl<'a, 'b: 'a, 'c, 'd: 'a, T1, T2: Clone> IArgsInfosProvider
        for Trait_work_Call<'a, 'b, 'c, 'd, T1, T2>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![
                ArgInfo::new(
                    "a",
                    &self.a,
                    (&ArgPrinter::<&i32>(transmute_lifetime!(&self.a))).debug_string(),
                ),
                ArgInfo::new(
                    "b",
                    &self.b,
                    (&ArgPrinter::<&i32>(transmute_lifetime!(&self.b))).debug_string(),
                ),
                ArgInfo::new(
                    "c",
                    &self.c,
                    (&ArgPrinter::<&i32>(transmute_lifetime!(&self.c))).debug_string(),
                ),
                ArgInfo::new(
                    "d",
                    &self.d,
                    (&ArgPrinter::<&i32>(transmute_lifetime!(&self.d))).debug_string(),
                ),
                ArgInfo::new(
                    "axb",
                    &self.axb,
                    (&ArgPrinter::<&&&i32>(transmute_lifetime!(&self.axb))).debug_string(),
                ),
                ArgInfo::new(
                    "cxd",
                    &self.cxd,
                    (&ArgPrinter::<&&&i32>(transmute_lifetime!(&self.cxd))).debug_string(),
                ),
                ArgInfo::new(
                    "abxbax",
                    &self.abxbax,
                    (&ArgPrinter::<&&&&&&i32>(transmute_lifetime!(&self.abxbax))).debug_string(),
                ),
                ArgInfo::new(
                    "cdxdcx",
                    &self.cdxdcx,
                    (&ArgPrinter::<&&&&&&i32>(transmute_lifetime!(&self.cdxdcx))).debug_string(),
                ),
                ArgInfo::new(
                    "abcd",
                    &self.abcd,
                    (&ArgPrinter::<&&&&i32>(transmute_lifetime!(&self.abcd))).debug_string(),
                ),
                ArgInfo::new(
                    "xaxbxcxdx",
                    &self.xaxbxcxdx,
                    (&ArgPrinter::<&&&&&&&&&i32>(transmute_lifetime!(&self.xaxbxcxdx)))
                        .debug_string(),
                ),
                ArgInfo::new(
                    "data",
                    &self.data,
                    (&ArgPrinter::<
                        Data<'a, 'b, &&i32, &&&&&[&&&&Data<'c, 'a, &&&&i32, Vec<&&&()>>]>,
                    >(transmute_lifetime!(&self.data)))
                        .debug_string(),
                ),
                ArgInfo::new("t1", &self.t1, (&ArgPrinter(&self.t1)).debug_string()),
                ArgInfo::new(
                    "t1_ref",
                    &self.t1_ref,
                    (&ArgPrinter::<&T1>(transmute_lifetime!(&self.t1_ref))).debug_string(),
                ),
                ArgInfo::new(
                    "xaxbxcxdx_t1_ref",
                    &self.xaxbxcxdx_t1_ref,
                    (&ArgPrinter::<&&&&&&&&&T1>(transmute_lifetime!(&self.xaxbxcxdx_t1_ref)))
                        .debug_string(),
                ),
                ArgInfo::new("t2", &self.t2, (&ArgPrinter(&self.t2)).debug_string()),
                ArgInfo::new(
                    "t2_ref",
                    &self.t2_ref,
                    (&ArgPrinter::<&T2>(transmute_lifetime!(&self.t2_ref))).debug_string(),
                ),
                ArgInfo::new(
                    "xaxbxcxdx_t2_ref",
                    &self.xaxbxcxdx_t2_ref,
                    (&ArgPrinter::<&&&&&&&&&T2>(transmute_lifetime!(&self.xaxbxcxdx_t2_ref)))
                        .debug_string(),
                ),
                ArgInfo::new(
                    "xapx",
                    &self.xapx,
                    (&ArgPrinter::<&&*const &i32>(transmute_lifetime!(&self.xapx))).debug_string(),
                ),
            ]
        }
    }
    impl<'a, 'b: 'a, 'c, 'd: 'a, T1, T2: Clone> IArgsTupleProvider
        for Trait_work_Call<'a, 'b, 'c, 'd, T1, T2>
    {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((
                &self.a,
                &self.b,
                &self.c,
                &self.d,
                &self.axb,
                &self.cxd,
                &self.abxbax,
                &self.cdxdcx,
                &self.abcd,
                &self.xaxbxcxdx,
                &self.data,
                &self.t1,
                &self.t1_ref,
                &self.xaxbxcxdx_t1_ref,
                &self.t2,
                &self.t2_ref,
                &self.xaxbxcxdx_t2_ref,
                &self.xapx,
            ))) as *mut _ as *mut ()
        }
    }
    impl<'a, 'b: 'a, 'c, 'd: 'a, T1, T2: Clone> IGenericsInfoProvider
        for Trait_work_Call<'a, 'b, 'c, 'd, T1, T2>
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![generic_type_info("T2", core::any::type_name::<T2>())]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {
            [tid::<T2>()].hash(hasher)
        }
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct Trait_work_ArgsChecker<'a, 'b: 'a, 'c, 'd: 'a, T1, T2: Clone> {
        _phantom_GenericParam_a: PhantomData<&'a ()>,
        _phantom_GenericParam_b: PhantomData<&'b ()>,
        _phantom_GenericParam_T1: PhantomData<T1>,
        _phantom_a: PhantomData<&'a i32>,
        _phantom_b: PhantomData<&'b i32>,
        _phantom_c: PhantomData<&'c i32>,
        _phantom_d: PhantomData<&'d i32>,
        _phantom_axb: PhantomData<&'a *const &'b i32>,
        _phantom_cxd: PhantomData<&'c *const &'d i32>,
        _phantom_abxbax: PhantomData<&'a &'b *const &'b &'a *const i32>,
        _phantom_cdxdcx: PhantomData<&'c &'d *const &'d &'c *const i32>,
        _phantom_abcd: PhantomData<&'a &'b &'c &'d i32>,
        _phantom_xaxbxcxdx: PhantomData<*const &'a *const &'b *const &'c *const &'d *const i32>,
        _phantom_data: PhantomData<
            Data<
                'a,
                'b,
                *const *const i32,
                *const &'a *const &'b *const [&'c *const &'b *const Data<
                    'c,
                    'a,
                    *const *const &'c *const i32,
                    Vec<&'d &'b *const ()>,
                >],
            >,
        >,
        _phantom_t1: PhantomData<T1>,
        _phantom_t1_ref: PhantomData<*const T1>,
        _phantom_xaxbxcxdx_t1_ref:
            PhantomData<*const &'a *const &'b *const &'c *const &'d *const T1>,
        _phantom_t2: PhantomData<T2>,
        _phantom_t2_ref: PhantomData<*const T2>,
        _phantom_xaxbxcxdx_t2_ref:
            PhantomData<*const &'a *const &'b *const &'c *const &'d *const T2>,
        _phantom_xapx: PhantomData<*const &'a *const *const i32>,
        _phantom_GenericParam_c: PhantomData<&'c ()>,
        _phantom_GenericParam_d: PhantomData<&'d ()>,
        _phantom_GenericParam_T2: PhantomData<T2>,
        a: Arg<&'a i32>,
        b: Arg<&'b i32>,
        c: Arg<&'c i32>,
        d: Arg<&'d i32>,
        axb: Arg<&'a &&'b i32>,
        cxd: Arg<&'c &&'d i32>,
        abxbax: Arg<&'a &'b &&'b &'a &i32>,
        cdxdcx: Arg<&'c &'d &&'d &'c &i32>,
        abcd: Arg<&'a &'b &'c &'d i32>,
        xaxbxcxdx: Arg<&&'a &&'b &&'c &&'d &i32>,
        data: Arg<
            Data<'a, 'b, &&i32, &&'a &&'b &[&'c &&'b &Data<'c, 'a, &&&'c &i32, Vec<&'d &'b &()>>]>,
        >,
        t1: Arg<T1>,
        t1_ref: Arg<&T1>,
        xaxbxcxdx_t1_ref: Arg<&&'a &&'b &&'c &&'d &T1>,
        t2: Arg<T2>,
        t2_ref: Arg<&T2>,
        xaxbxcxdx_t2_ref: Arg<&&'a &&'b &&'c &&'d &T2>,
        xapx: Arg<&&'a *const &i32>,
    }
    impl<'a, 'b: 'a, 'c, 'd: 'a, T1, T2: Clone> IArgsChecker
        for Trait_work_ArgsChecker<'a, 'b, 'c, 'd, T1, T2>
    {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &Trait_work_Call<'a, 'b, 'c, 'd, T1, T2> = dyn_call.downcast_ref();
            vec![
                self.a.check_ref(
                    "a",
                    transmute_lifetime!(&call.a),
                    (&ArgPrinter::<&i32>(transmute_lifetime!(&call.a))).debug_string(),
                ),
                self.b.check_ref(
                    "b",
                    transmute_lifetime!(&call.b),
                    (&ArgPrinter::<&i32>(transmute_lifetime!(&call.b))).debug_string(),
                ),
                self.c.check_ref(
                    "c",
                    transmute_lifetime!(&call.c),
                    (&ArgPrinter::<&i32>(transmute_lifetime!(&call.c))).debug_string(),
                ),
                self.d.check_ref(
                    "d",
                    transmute_lifetime!(&call.d),
                    (&ArgPrinter::<&i32>(transmute_lifetime!(&call.d))).debug_string(),
                ),
                self.axb.check_ref(
                    "axb",
                    transmute_lifetime!(&call.axb),
                    (&ArgPrinter::<&&&i32>(transmute_lifetime!(&call.axb))).debug_string(),
                ),
                self.cxd.check_ref(
                    "cxd",
                    transmute_lifetime!(&call.cxd),
                    (&ArgPrinter::<&&&i32>(transmute_lifetime!(&call.cxd))).debug_string(),
                ),
                self.abxbax.check_ref(
                    "abxbax",
                    transmute_lifetime!(&call.abxbax),
                    (&ArgPrinter::<&&&&&&i32>(transmute_lifetime!(&call.abxbax))).debug_string(),
                ),
                self.cdxdcx.check_ref(
                    "cdxdcx",
                    transmute_lifetime!(&call.cdxdcx),
                    (&ArgPrinter::<&&&&&&i32>(transmute_lifetime!(&call.cdxdcx))).debug_string(),
                ),
                self.abcd.check_ref(
                    "abcd",
                    transmute_lifetime!(&call.abcd),
                    (&ArgPrinter::<&&&&i32>(transmute_lifetime!(&call.abcd))).debug_string(),
                ),
                self.xaxbxcxdx.check_ref(
                    "xaxbxcxdx",
                    transmute_lifetime!(&call.xaxbxcxdx),
                    (&ArgPrinter::<&&&&&&&&&i32>(transmute_lifetime!(&call.xaxbxcxdx)))
                        .debug_string(),
                ),
                self.data.check(
                    "data",
                    transmute_lifetime!(&call.data),
                    (&ArgPrinter::<
                        Data<'a, 'b, &&i32, &&&&&[&&&&Data<'c, 'a, &&&&i32, Vec<&&&()>>]>,
                    >(transmute_lifetime!(&call.data)))
                        .debug_string(),
                ),
                self.t1.check(
                    "t1",
                    transmute_lifetime!(&call.t1),
                    (&ArgPrinter(&call.t1)).debug_string(),
                ),
                self.t1_ref.check_ref(
                    "t1_ref",
                    transmute_lifetime!(&call.t1_ref),
                    (&ArgPrinter::<&T1>(transmute_lifetime!(&call.t1_ref))).debug_string(),
                ),
                self.xaxbxcxdx_t1_ref.check_ref(
                    "xaxbxcxdx_t1_ref",
                    transmute_lifetime!(&call.xaxbxcxdx_t1_ref),
                    (&ArgPrinter::<&&&&&&&&&T1>(transmute_lifetime!(&call.xaxbxcxdx_t1_ref)))
                        .debug_string(),
                ),
                self.t2.check(
                    "t2",
                    transmute_lifetime!(&call.t2),
                    (&ArgPrinter(&call.t2)).debug_string(),
                ),
                self.t2_ref.check_ref(
                    "t2_ref",
                    transmute_lifetime!(&call.t2_ref),
                    (&ArgPrinter::<&T2>(transmute_lifetime!(&call.t2_ref))).debug_string(),
                ),
                self.xaxbxcxdx_t2_ref.check_ref(
                    "xaxbxcxdx_t2_ref",
                    transmute_lifetime!(&call.xaxbxcxdx_t2_ref),
                    (&ArgPrinter::<&&&&&&&&&T2>(transmute_lifetime!(&call.xaxbxcxdx_t2_ref)))
                        .debug_string(),
                ),
                self.xapx.check_ref(
                    "xapx",
                    transmute_lifetime!(&call.xapx),
                    (&ArgPrinter::<&&*const &i32>(transmute_lifetime!(&call.xapx))).debug_string(),
                ),
            ]
        }
    }
    impl<'a, 'b: 'a, 'c, 'd: 'a, T1, T2: Clone> IArgsFormatter
        for Trait_work_ArgsChecker<'a, 'b, 'c, 'd, T1, T2>
    {
        fn fmt_args(&self) -> String {
            format!(
                "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
                (&ArgPrinter(&self.a)).debug_string(),
                (&ArgPrinter(&self.b)).debug_string(),
                (&ArgPrinter(&self.c)).debug_string(),
                (&ArgPrinter(&self.d)).debug_string(),
                (&ArgPrinter(&self.axb)).debug_string(),
                (&ArgPrinter(&self.cxd)).debug_string(),
                (&ArgPrinter(&self.abxbax)).debug_string(),
                (&ArgPrinter(&self.cdxdcx)).debug_string(),
                (&ArgPrinter(&self.abcd)).debug_string(),
                (&ArgPrinter(&self.xaxbxcxdx)).debug_string(),
                (&ArgPrinter(&self.data)).debug_string(),
                (&ArgPrinter(&self.t1)).debug_string(),
                (&ArgPrinter(&self.t1_ref)).debug_string(),
                (&ArgPrinter(&self.xaxbxcxdx_t1_ref)).debug_string(),
                (&ArgPrinter(&self.t2)).debug_string(),
                (&ArgPrinter(&self.t2_ref)).debug_string(),
                (&ArgPrinter(&self.xaxbxcxdx_t2_ref)).debug_string(),
                (&ArgPrinter(&self.xapx)).debug_string()
            )
        }
    }
    impl<'a, 'b: 'a, 'c, 'd: 'a, T1, T2: Clone> IGenericsInfoProvider
        for Trait_work_ArgsChecker<'a, 'b, 'c, 'd, T1, T2>
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![generic_type_info("T2", core::any::type_name::<T2>())]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {
            [tid::<T2>()].hash(hasher)
        }
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct TraitMockData<'a, 'b: 'a, T1> {
        _phantom_GenericParam_a: PhantomData<&'a ()>,
        _phantom_GenericParam_b: PhantomData<&'b ()>,
        _phantom_GenericParam_T1: PhantomData<T1>,
        pub Trait_work: FnData<'static, TraitMock<'a, 'b, T1>, false, false>,
    }
    #[doc(hidden)]
    pub struct TraitMockSetup<'a, 'b: 'a, T1> {
        data: Arc<TraitMockData<'a, 'b, T1>>,
    }
    impl<'a, 'b: 'a, T1> Clone for TraitMockSetup<'a, 'b, T1> {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    #[doc(hidden)]
    pub struct TraitMockReceived<'a, 'b: 'a, T1> {
        data: Arc<TraitMockData<'a, 'b, T1>>,
    }
    impl<'a, 'b: 'a, T1> Clone for TraitMockReceived<'a, 'b, T1> {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    pub struct TraitMock<'a, 'b: 'a, T1> {
        pub setup: TraitMockSetup<'a, 'b, T1>,
        pub received: TraitMockReceived<'a, 'b, T1>,
        pub data: Arc<TraitMockData<'a, 'b, T1>>,
    }
    impl<'a, 'b: 'a, T1> AsRef<TraitMock<'a, 'b, T1>> for TraitMock<'a, 'b, T1> {
        fn as_ref(&self) -> &TraitMock<'a, 'b, T1> {
            self
        }
    }
    impl<'a, 'b: 'a, T1> Clone for TraitMock<'a, 'b, T1> {
        fn clone(&self) -> Self {
            Self {
                setup: (&self.setup).clone(),
                received: (&self.received).clone(),
                data: (&self.data).clone(),
            }
        }
    }
    impl<'a, 'b: 'a, T1> Trait<'a, 'b, T1> for TraitMock<'a, 'b, T1> {
        fn work<'c, 'd: 'a, T2: Clone>(
            &self,
            a: &'a i32,
            b: &'b i32,
            c: &'c i32,
            d: &'d i32,
            axb: &'a &&'b i32,
            cxd: &'c &&'d i32,
            abxbax: &'a &'b &&'b &'a &i32,
            cdxdcx: &'c &'d &&'d &'c &i32,
            abcd: &'a &'b &'c &'d i32,
            xaxbxcxdx: &&'a &&'b &&'c &&'d &i32,
            data: Data<
                'a,
                'b,
                &&i32,
                &&'a &&'b &[&'c &&'b &Data<'c, 'a, &&&'c &i32, Vec<&'d &'b &()>>],
            >,
            t1: T1,
            t1_ref: &T1,
            xaxbxcxdx_t1_ref: &&'a &&'b &&'c &&'d &T1,
            t2: T2,
            t2_ref: &T2,
            xaxbxcxdx_t2_ref: &&'a &&'b &&'c &&'d &T2,
            xapx: &&'a *const &i32,
        ) -> &&'a &&'a &&'b &&'b &&'c &&'c &&'d &&'d &i32 {
            let call: Trait_work_Call<'_, '_, '_, '_, T1, T2> = Trait_work_Call {
                _phantom_GenericParam_a: PhantomData,
                _phantom_GenericParam_b: PhantomData,
                _phantom_GenericParam_T1: PhantomData,
                _phantom_a: PhantomData,
                _phantom_b: PhantomData,
                _phantom_c: PhantomData,
                _phantom_d: PhantomData,
                _phantom_axb: PhantomData,
                _phantom_cxd: PhantomData,
                _phantom_abxbax: PhantomData,
                _phantom_cdxdcx: PhantomData,
                _phantom_abcd: PhantomData,
                _phantom_xaxbxcxdx: PhantomData,
                _phantom_data: PhantomData,
                _phantom_t1: PhantomData,
                _phantom_t1_ref: PhantomData,
                _phantom_xaxbxcxdx_t1_ref: PhantomData,
                _phantom_t2: PhantomData,
                _phantom_t2_ref: PhantomData,
                _phantom_xaxbxcxdx_t2_ref: PhantomData,
                _phantom_xapx: PhantomData,
                _phantom_GenericParam_c: PhantomData,
                _phantom_GenericParam_d: PhantomData,
                _phantom_GenericParam_T2: PhantomData,
                a: transmute_lifetime!(a),
                b: transmute_lifetime!(b),
                c: transmute_lifetime!(c),
                d: transmute_lifetime!(d),
                axb: transmute_lifetime!(axb),
                cxd: transmute_lifetime!(cxd),
                abxbax: transmute_lifetime!(abxbax),
                cdxdcx: transmute_lifetime!(cdxdcx),
                abcd: transmute_lifetime!(abcd),
                xaxbxcxdx: transmute_lifetime!(xaxbxcxdx),
                data: transmute_lifetime!(data),
                t1: transmute_lifetime!(t1),
                t1_ref: transmute_lifetime!(t1_ref),
                xaxbxcxdx_t1_ref: transmute_lifetime!(xaxbxcxdx_t1_ref),
                t2: transmute_lifetime!(t2),
                t2_ref: transmute_lifetime!(t2_ref),
                xaxbxcxdx_t2_ref: transmute_lifetime!(xaxbxcxdx_t2_ref),
                xapx: transmute_lifetime!(xapx),
            };
            return self.data.clone().Trait_work.handle_returning(self, call);
        }
    }
    impl<'a, 'b: 'a, T1> TraitMock<'a, 'b, T1> {
        pub fn new() -> Self {
            let data = Arc::new(TraitMockData {
                _phantom_GenericParam_a: PhantomData,
                _phantom_GenericParam_b: PhantomData,
                _phantom_GenericParam_T1: PhantomData,
                Trait_work: FnData::new("Trait::work"),
            });
            return TraitMock {
                setup: TraitMockSetup { data: data.clone() },
                received: TraitMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'a, 'b: 'a, T1> TraitMockSetup<'a, 'b, T1> {
        pub fn work<'__rsa, 'c, 'd : 'a, T2: Clone>(
            &self, a: impl Into<Arg<&'a i32>>, b: impl Into<Arg<&'b i32>>, c: impl Into<Arg<&'c i32>>, d: impl Into<Arg<&'d i32>>, axb: impl Into<Arg<&'a &&'b i32>>, cxd: impl Into<Arg<&'c &&'d i32>>, abxbax: impl Into<Arg<&'a &'b &&'b &'a &i32>>, cdxdcx: impl Into<Arg<&'c &'d &&'d &'c &i32>>, abcd: impl Into<Arg<&'a &'b &'c &'d i32>>, xaxbxcxdx: impl Into<Arg<&&'a &&'b &&'c &&'d &i32>>, data: impl Into<Arg<Data<
                'a,
                'b,
                &&i32,
                &&'a &&'b &[&'c &&'b &Data<'c, 'a, &&&'c &i32, Vec<&'d &'b &()>>],
            >>>, t1: impl Into<Arg<T1>>, t1_ref: impl Into<Arg<&T1>>, xaxbxcxdx_t1_ref: impl Into<Arg<&&'a &&'b &&'c &&'d &T1>>, t2: impl Into<Arg<T2>>, t2_ref: impl Into<Arg<&T2>>, xaxbxcxdx_t2_ref: impl Into<Arg<&&'a &&'b &&'c &&'d &T2>>, xapx: impl Into<Arg<&&'a *const &i32>>) -> FnTuner<'_, TraitMock<'a, 'b, T1>, Self, (&&'a i32,
                                                                                                                                                                                                                                                                                                                                      &&'b i32,
                                                                                                                                                                                                                                                                                                                                      &&'c i32,
                                                                                                                                                                                                                                                                                                                                      &&'d i32,
                                                                                                                                                                                                                                                                                                                                      &&'a &&'b i32,
                                                                                                                                                                                                                                                                                                                                      &&'c &&'d i32,
                                                                                                                                                                                                                                                                                                                                      &&'a &'b &&'b &'a &i32,
                                                                                                                                                                                                                                                                                                                                      &&'c &'d &&'d &'c &i32,
                                                                                                                                                                                                                                                                                                                                      &&'a &'b &'c &'d i32,
                                                                                                                                                                                                                                                                                                                                      &&&'a &&'b &&'c &&'d &i32,
                                                                                                                                                                                                                                                                                                                                      &Data<
                                                                                                                                                                                                                                                                                                                                          'a,
                                                                                                                                                                                                                                                                                                                                          'b,
                                                                                                                                                                                                                                                                                                                                          &&i32,
                                                                                                                                                                                                                                                                                                                                          &&'a &&'b &[&'c &&'b &Data<'c, 'a, &&&'c &i32, Vec<&'d &'b &()>>],
                                                                                                                                                                                                                                                                                                                                      >,
                                                                                                                                                                                                                                                                                                                                      &T1,
                                                                                                                                                                                                                                                                                                                                      &&T1,
                                                                                                                                                                                                                                                                                                                                      &&&'a &&'b &&'c &&'d &T1,
                                                                                                                                                                                                                                                                                                                                      &T2,
                                                                                                                                                                                                                                                                                                                                      &&T2,
                                                                                                                                                                                                                                                                                                                                      &&&'a &&'b &&'c &&'d &T2,
                                                                                                                                                                                                                                                                                                                                      &&&'a *const &i32),
            &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa i32, &Self,
        false, false>{
            let Trait_work_args_checker: Trait_work_ArgsChecker<'a, 'b, 'c, 'd, T1, T2> =
                Trait_work_ArgsChecker {
                    _phantom_GenericParam_a: PhantomData,
                    _phantom_GenericParam_b: PhantomData,
                    _phantom_GenericParam_T1: PhantomData,
                    _phantom_a: PhantomData,
                    _phantom_b: PhantomData,
                    _phantom_c: PhantomData,
                    _phantom_d: PhantomData,
                    _phantom_axb: PhantomData,
                    _phantom_cxd: PhantomData,
                    _phantom_abxbax: PhantomData,
                    _phantom_cdxdcx: PhantomData,
                    _phantom_abcd: PhantomData,
                    _phantom_xaxbxcxdx: PhantomData,
                    _phantom_data: PhantomData,
                    _phantom_t1: PhantomData,
                    _phantom_t1_ref: PhantomData,
                    _phantom_xaxbxcxdx_t1_ref: PhantomData,
                    _phantom_t2: PhantomData,
                    _phantom_t2_ref: PhantomData,
                    _phantom_xaxbxcxdx_t2_ref: PhantomData,
                    _phantom_xapx: PhantomData,
                    _phantom_GenericParam_c: PhantomData,
                    _phantom_GenericParam_d: PhantomData,
                    _phantom_GenericParam_T2: PhantomData,
                    a: transmute_lifetime!(a.into()),
                    b: transmute_lifetime!(b.into()),
                    c: transmute_lifetime!(c.into()),
                    d: transmute_lifetime!(d.into()),
                    axb: transmute_lifetime!(axb.into()),
                    cxd: transmute_lifetime!(cxd.into()),
                    abxbax: transmute_lifetime!(abxbax.into()),
                    cdxdcx: transmute_lifetime!(cdxdcx.into()),
                    abcd: transmute_lifetime!(abcd.into()),
                    xaxbxcxdx: transmute_lifetime!(xaxbxcxdx.into()),
                    data: transmute_lifetime!(data.into()),
                    t1: transmute_lifetime!(t1.into()),
                    t1_ref: transmute_lifetime!(t1_ref.into()),
                    xaxbxcxdx_t1_ref: transmute_lifetime!(xaxbxcxdx_t1_ref.into()),
                    t2: transmute_lifetime!(t2.into()),
                    t2_ref: transmute_lifetime!(t2_ref.into()),
                    xaxbxcxdx_t2_ref: transmute_lifetime!(xaxbxcxdx_t2_ref.into()),
                    xapx: transmute_lifetime!(xapx.into()),
                };
            let fn_tuner: FnTuner<'_, TraitMock<'a, 'b, T1>, Self, (&&'a i32,
                                                                    &&'b i32,
                                                                    &&'c i32,
                                                                    &&'d i32,
                                                                    &&'a &&'b i32,
                                                                    &&'c &&'d i32,
                                                                    &&'a &'b &&'b &'a &i32,
                                                                    &&'c &'d &&'d &'c &i32,
                                                                    &&'a &'b &'c &'d i32,
                                                                    &&&'a &&'b &&'c &&'d &i32,
                                                                    &Data<
                                                                        'a,
                                                                        'b,
                                                                        &&i32,
                                                                        &&'a &&'b &[&'c &&'b &Data<'c, 'a, &&&'c &i32, Vec<&'d &'b &()>>],
                                                                    >,
                                                                    &T1,
                                                                    &&T1,
                                                                    &&&'a &&'b &&'c &&'d &T1,
                                                                    &T2,
                                                                    &&T2,
                                                                    &&&'a &&'b &&'c &&'d &T2,
                                                                    &&&'a *const &i32),
                &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa i32, &Self,
                false, false> = self.data.Trait_work.add_config(Trait_work_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
    }
    impl<'a, 'b: 'a, T1> TraitMockReceived<'a, 'b, T1> {
        pub fn work<'__rsa, 'c, 'd: 'a, T2: Clone>(
            &self,
            a: impl Into<Arg<&'a i32>>,
            b: impl Into<Arg<&'b i32>>,
            c: impl Into<Arg<&'c i32>>,
            d: impl Into<Arg<&'d i32>>,
            axb: impl Into<Arg<&'a &&'b i32>>,
            cxd: impl Into<Arg<&'c &&'d i32>>,
            abxbax: impl Into<Arg<&'a &'b &&'b &'a &i32>>,
            cdxdcx: impl Into<Arg<&'c &'d &&'d &'c &i32>>,
            abcd: impl Into<Arg<&'a &'b &'c &'d i32>>,
            xaxbxcxdx: impl Into<Arg<&&'a &&'b &&'c &&'d &i32>>,
            data: impl Into<
                Arg<
                    Data<
                        'a,
                        'b,
                        &&i32,
                        &&'a &&'b &[&'c &&'b &Data<'c, 'a, &&&'c &i32, Vec<&'d &'b &()>>],
                    >,
                >,
            >,
            t1: impl Into<Arg<T1>>,
            t1_ref: impl Into<Arg<&T1>>,
            xaxbxcxdx_t1_ref: impl Into<Arg<&&'a &&'b &&'c &&'d &T1>>,
            t2: impl Into<Arg<T2>>,
            t2_ref: impl Into<Arg<&T2>>,
            xaxbxcxdx_t2_ref: impl Into<Arg<&&'a &&'b &&'c &&'d &T2>>,
            xapx: impl Into<Arg<&&'a *const &i32>>,
            times: Times,
        ) -> FnVerifier<
            Self,
            (
                &&'a i32,
                &&'b i32,
                &&'c i32,
                &&'d i32,
                &&'a &&'b i32,
                &&'c &&'d i32,
                &&'a &'b &&'b &'a &i32,
                &&'c &'d &&'d &'c &i32,
                &&'a &'b &'c &'d i32,
                &&&'a &&'b &&'c &&'d &i32,
                &Data<
                    'a,
                    'b,
                    &&i32,
                    &&'a &&'b &[&'c &&'b &Data<'c, 'a, &&&'c &i32, Vec<&'d &'b &()>>],
                >,
                &T1,
                &&T1,
                &&&'a &&'b &&'c &&'d &T1,
                &T2,
                &&T2,
                &&&'a &&'b &&'c &&'d &T2,
                &&&'a *const &i32,
            ),
        > {
            let Trait_work_args_checker: Trait_work_ArgsChecker<'a, 'b, 'c, 'd, T1, T2> =
                Trait_work_ArgsChecker {
                    _phantom_GenericParam_a: PhantomData,
                    _phantom_GenericParam_b: PhantomData,
                    _phantom_GenericParam_T1: PhantomData,
                    _phantom_a: PhantomData,
                    _phantom_b: PhantomData,
                    _phantom_c: PhantomData,
                    _phantom_d: PhantomData,
                    _phantom_axb: PhantomData,
                    _phantom_cxd: PhantomData,
                    _phantom_abxbax: PhantomData,
                    _phantom_cdxdcx: PhantomData,
                    _phantom_abcd: PhantomData,
                    _phantom_xaxbxcxdx: PhantomData,
                    _phantom_data: PhantomData,
                    _phantom_t1: PhantomData,
                    _phantom_t1_ref: PhantomData,
                    _phantom_xaxbxcxdx_t1_ref: PhantomData,
                    _phantom_t2: PhantomData,
                    _phantom_t2_ref: PhantomData,
                    _phantom_xaxbxcxdx_t2_ref: PhantomData,
                    _phantom_xapx: PhantomData,
                    _phantom_GenericParam_c: PhantomData,
                    _phantom_GenericParam_d: PhantomData,
                    _phantom_GenericParam_T2: PhantomData,
                    a: transmute_lifetime!(a.into()),
                    b: transmute_lifetime!(b.into()),
                    c: transmute_lifetime!(c.into()),
                    d: transmute_lifetime!(d.into()),
                    axb: transmute_lifetime!(axb.into()),
                    cxd: transmute_lifetime!(cxd.into()),
                    abxbax: transmute_lifetime!(abxbax.into()),
                    cdxdcx: transmute_lifetime!(cdxdcx.into()),
                    abcd: transmute_lifetime!(abcd.into()),
                    xaxbxcxdx: transmute_lifetime!(xaxbxcxdx.into()),
                    data: transmute_lifetime!(data.into()),
                    t1: transmute_lifetime!(t1.into()),
                    t1_ref: transmute_lifetime!(t1_ref.into()),
                    xaxbxcxdx_t1_ref: transmute_lifetime!(xaxbxcxdx_t1_ref.into()),
                    t2: transmute_lifetime!(t2.into()),
                    t2_ref: transmute_lifetime!(t2_ref.into()),
                    xaxbxcxdx_t2_ref: transmute_lifetime!(xaxbxcxdx_t2_ref.into()),
                    xapx: transmute_lifetime!(xapx.into()),
                };
            self.data
                .Trait_work
                .verify_received(Trait_work_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
}

// #[mock]
// #[allow(unused)]
// trait Trait<'a, 'b: 'a, T1> {
//     fn work<'c, 'd: 'a, T2: Clone>(
//         &self,
//         a: &'a i32,
//         b: &'b i32,
//         c: &'c i32,
//         d: &'d i32,
//         axb: &'a &&'b i32,
//         cxd: &'c &&'d i32,
//         abxbax: &'a &'b &&'b &'a &i32,
//         cdxdcx: &'c &'d &&'d &'c &i32,
//         abcd: &'a &'b &'c &'d i32,
//         xaxbxcxdx: &&'a &&'b &&'c &&'d &i32,
//         data: Data<
//             'a,
//             'b,
//             &&i32,
//             &&'a &&'b &[&'c &&'b &Data<'c, 'a, &&&'c &i32, Vec<&'d &'b &()>>],
//         >,
//         t1: T1,
//         t1_ref: &T1,
//         xaxbxcxdx_t1_ref: &&'a &&'b &&'c &&'d &T1,
//         t2: T2,
//         t2_ref: &T2,
//         xaxbxcxdx_t2_ref: &&'a &&'b &&'c &&'d &T2,
//         xapx: &&'a *const &i32,
//     ) -> &&'a &&'a &&'b &&'b &&'c &&'c &&'d &&'d &i32;
// }

#[mock]
#[allow(unused)]
fn work<'x, 'a, 'b: 'a, 'c, 'd: 'a, T1, T2>(
    a: &'a i32,
    b: &'b i32,
    c: &'c i32,
    d: &'d i32,
    axb: &'a &&'b i32,
    cxd: &'c &&'d i32,
    abxbax: &'a &'b &&'b &'a &i32,
    cdxdcx: &'c &'d &&'d &'c &i32,
    abcd: &'a &'b &'c &'d i32,
    xaxbxcxdx: &&'a &&'b &&'c &&'d &i32,
    data: Data<'a, 'b, &&i32, &&'a &&'b &[&'c &&'b &Data<'c, 'a, &&&'c &i32, Vec<&'d &'b &()>>]>,
    t1: T1,
    t1_ref: &T1,
    xaxbxcxdx_t1_ref: &&'a &&'b &&'c &&'d &T1,
    t2: T2,
    t2_ref: &T2,
    xaxbxcxdx_t2_ref: &&'a &&'b &&'c &&'d &T2,
    xapx: &&'a *const &i32,
) -> &'x &'a &'x &'a &'x &'b &'x &'b &'x &'c &'x &'c &'x &'d &'x &'d &'x i32 {
    unreachable!()
}

mocked_base! {
    #[allow(unused)]
    struct Struct<'a, 'b: 'a, T1: Clone> {
        _phantom_a: PhantomData<&'a ()>,
        _phantom_b: PhantomData<&'b ()>,
        _phantom_t1: PhantomData<T1>,
    }

    #[allow(unused)]
    impl<'a, 'b: 'a, T1: Clone> Struct<'a, 'b, T1> {
        pub fn new() -> Self {
            Self {
                _phantom_a: PhantomData,
                _phantom_b: PhantomData,
                _phantom_t1: PhantomData,
            }
        }

        #[allow(unused)]
        fn work<'c, 'd: 'a, T2: Clone>(
            &self,
            a: &'a i32,
            b: &'b i32,
            c: &'c i32,
            d: &'d i32,
            axb: &'a &&'b i32,
            cxd: &'c &&'d i32,
            abxbax: &'a &'b &&'b &'a &i32,
            cdxdcx: &'c &'d &&'d &'c &i32,
            abcd: &'a &'b &'c &'d i32,
            xaxbxcxdx: &&'a &&'b &&'c &&'d &i32,
            data: Data<
                'a,
                'b,
                &&i32,
                &&'a &&'b &[&'c &&'b &Data<'c, 'a, &&&'c &i32, Vec<&'d &'b &()>>],
            >,
            t1: T1,
            t1_ref: &T1,
            xaxbxcxdx_t1_ref: &&'a &&'b &&'c &&'d &T1,
            t2: T2,
            t2_ref: &T2,
            xaxbxcxdx_t2_ref: &&'a &&'b &&'c &&'d &T2,
            xapx: &&'a *const &i32,
        ) -> &&'a &&'a &&'b &&'b &&'c &&'c &&'d &&'d &i32 {
            unreachable!()
        }
    }

    #[allow(unused)]
    impl<'a, 'b: 'a, T1: Clone> Trait<'a, 'b, T1> for Struct<'a, 'b, T1> {
        fn work<'c, 'd: 'a, T2: Clone>(
            &self,
            a: &'a i32,
            b: &'b i32,
            c: &'c i32,
            d: &'d i32,
            axb: &'a &&'b i32,
            cxd: &'c &&'d i32,
            abxbax: &'a &'b &&'b &'a &i32,
            cdxdcx: &'c &'d &&'d &'c &i32,
            abcd: &'a &'b &'c &'d i32,
            xaxbxcxdx: &&'a &&'b &&'c &&'d &i32,
            data: Data<
                'a,
                'b,
                &&i32,
                &&'a &&'b &[&'c &&'b &Data<'c, 'a, &&&'c &i32, Vec<&'d &'b &()>>],
            >,
            t1: T1,
            t1_ref: &T1,
            xaxbxcxdx_t1_ref: &&'a &&'b &&'c &&'d &T1,
            t2: T2,
            t2_ref: &T2,
            xaxbxcxdx_t2_ref: &&'a &&'b &&'c &&'d &T2,
            xapx: &&'a *const &i32,
        ) -> &&'a &&'a &&'b &&'b &&'c &&'c &&'d &&'d &i32 {
            Self::work(
                self,
                a,
                b,
                c,
                d,
                axb,
                cxd,
                abxbax,
                cdxdcx,
                abcd,
                xaxbxcxdx,
                data,
                t1,
                t1_ref,
                xaxbxcxdx_t1_ref,
                t2,
                t2_ref,
                xaxbxcxdx_t2_ref,
                xapx,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn trait_work_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let return_value = &&&&&&&&&&&&&&&&&55;
        let a = &1;
        {
            let b = &2;
            {
                let c = &3;
                {
                    let d = &4;
                    {
                        let axb = &&&5;
                        {
                            let cxd = &&&6;
                            {
                                let abxbax = &&&&&&7;
                                {
                                    let cdxdcx = &&&&&&8;
                                    {
                                        let abcd = &&&&9;
                                        {
                                            let xaxbxcxdx = &&&&&&&&&10;
                                            {
                                                let data = Data::<'_, '_, _, _> {
                                                    _phantoms: Default::default(),
                                                };
                                                let t1 = [7, 77];
                                                let t1_ref = &[8, 88];
                                                let xaxbxcxdx_t1_ref = &&&&&&&&&[9, 99];
                                                let t2 = true;
                                                let t2_ref = &true;
                                                let xaxbxcxdx_t2_ref = &&&&&&&&&true;
                                                let xapx = &&(&(&188) as *const _);
                                                mock.setup
                                                    .work(
                                                        a,
                                                        b,
                                                        c,
                                                        d,
                                                        axb,
                                                        cxd,
                                                        abxbax,
                                                        cdxdcx,
                                                        abcd,
                                                        xaxbxcxdx,
                                                        data.clone(),
                                                        t1,
                                                        t1_ref,
                                                        xaxbxcxdx_t1_ref,
                                                        t2,
                                                        t2_ref,
                                                        xaxbxcxdx_t2_ref,
                                                        xapx,
                                                    )
                                                    .returns(return_value);

                                                // Act
                                                let actual_return_value = mock.work(
                                                    a,
                                                    b,
                                                    c,
                                                    d,
                                                    axb,
                                                    cxd,
                                                    abxbax,
                                                    cdxdcx,
                                                    abcd,
                                                    xaxbxcxdx,
                                                    data.clone(),
                                                    t1,
                                                    t1_ref,
                                                    xaxbxcxdx_t1_ref,
                                                    t2,
                                                    t2_ref,
                                                    xaxbxcxdx_t2_ref,
                                                    xapx,
                                                );

                                                // Assert
                                                assert_eq!(return_value, actual_return_value);

                                                mock.received
                                                    .work(
                                                        a,
                                                        b,
                                                        c,
                                                        d,
                                                        axb,
                                                        cxd,
                                                        abxbax,
                                                        cdxdcx,
                                                        abcd,
                                                        xaxbxcxdx,
                                                        data,
                                                        t1,
                                                        t1_ref,
                                                        xaxbxcxdx_t1_ref,
                                                        t2,
                                                        t2_ref,
                                                        xaxbxcxdx_t2_ref,
                                                        xapx,
                                                        Times::Once,
                                                    )
                                                    .no_other_calls()
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn fn_work_Ok() {
        let return_value = &&&&&&&&&&&&&&&&&55;
        let a = &1;
        {
            let b = &2;
            {
                let c = &3;
                {
                    let d = &4;
                    {
                        let axb = &&&5;
                        {
                            let cxd = &&&6;
                            {
                                let abxbax = &&&&&&7;
                                {
                                    let cdxdcx = &&&&&&8;
                                    {
                                        let abcd = &&&&9;
                                        {
                                            let xaxbxcxdx = &&&&&&&&&10;
                                            {
                                                let data = Data::<'_, '_, _, _> {
                                                    _phantoms: Default::default(),
                                                };
                                                let t1 = [7, 77];
                                                let t1_ref = &[8, 88];
                                                let xaxbxcxdx_t1_ref = &&&&&&&&&[9, 99];
                                                let t2 = true;
                                                let t2_ref = &true;
                                                let xaxbxcxdx_t2_ref = &&&&&&&&&true;
                                                let xapx = &&(&(&188) as *const _);
                                                work::setup(
                                                    a,
                                                    b,
                                                    c,
                                                    d,
                                                    axb,
                                                    cxd,
                                                    abxbax,
                                                    cdxdcx,
                                                    abcd,
                                                    xaxbxcxdx,
                                                    data.clone(),
                                                    t1,
                                                    t1_ref,
                                                    xaxbxcxdx_t1_ref,
                                                    t2,
                                                    t2_ref,
                                                    xaxbxcxdx_t2_ref,
                                                    xapx,
                                                )
                                                .returns(return_value);

                                                // Act
                                                let actual_return_value = work(
                                                    a,
                                                    b,
                                                    c,
                                                    d,
                                                    axb,
                                                    cxd,
                                                    abxbax,
                                                    cdxdcx,
                                                    abcd,
                                                    xaxbxcxdx,
                                                    data.clone(),
                                                    t1,
                                                    t1_ref,
                                                    xaxbxcxdx_t1_ref,
                                                    t2,
                                                    t2_ref,
                                                    xaxbxcxdx_t2_ref,
                                                    xapx,
                                                );

                                                // Assert
                                                assert_eq!(return_value, actual_return_value);

                                                work::received(
                                                    a,
                                                    b,
                                                    c,
                                                    d,
                                                    axb,
                                                    cxd,
                                                    abxbax,
                                                    cdxdcx,
                                                    abcd,
                                                    xaxbxcxdx,
                                                    data,
                                                    t1,
                                                    t1_ref,
                                                    xaxbxcxdx_t1_ref,
                                                    t2,
                                                    t2_ref,
                                                    xaxbxcxdx_t2_ref,
                                                    xapx,
                                                    Times::Once,
                                                )
                                                .no_other_calls()
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn struct_work_Ok() {
        // Arrange
        let mock = Struct::new();
        let return_value = &&&&&&&&&&&&&&&&&55;
        let a = &1;
        {
            let b = &2;
            {
                let c = &3;
                {
                    let d = &4;
                    {
                        let axb = &&&5;
                        {
                            let cxd = &&&6;
                            {
                                let abxbax = &&&&&&7;
                                {
                                    let cdxdcx = &&&&&&8;
                                    {
                                        let abcd = &&&&9;
                                        {
                                            let xaxbxcxdx = &&&&&&&&&10;
                                            {
                                                let data = Data::<'_, '_, _, _> {
                                                    _phantoms: Default::default(),
                                                };
                                                let t1 = [7, 77];
                                                let t1_ref = &[8, 88];
                                                let xaxbxcxdx_t1_ref = &&&&&&&&&[9, 99];
                                                let t2 = true;
                                                let t2_ref = &true;
                                                let xaxbxcxdx_t2_ref = &&&&&&&&&true;
                                                let xapx = &&(&(&188) as *const _);
                                                mock.setup
                                                    .work(
                                                        a,
                                                        b,
                                                        c,
                                                        d,
                                                        axb,
                                                        cxd,
                                                        abxbax,
                                                        cdxdcx,
                                                        abcd,
                                                        xaxbxcxdx,
                                                        data.clone(),
                                                        t1,
                                                        t1_ref,
                                                        xaxbxcxdx_t1_ref,
                                                        t2,
                                                        t2_ref,
                                                        xaxbxcxdx_t2_ref,
                                                        xapx,
                                                    )
                                                    .returns(return_value);
                                                mock.setup
                                                    .as_Trait
                                                    .work(
                                                        a,
                                                        b,
                                                        c,
                                                        d,
                                                        axb,
                                                        cxd,
                                                        abxbax,
                                                        cdxdcx,
                                                        abcd,
                                                        xaxbxcxdx,
                                                        data.clone(),
                                                        t1,
                                                        t1_ref,
                                                        xaxbxcxdx_t1_ref,
                                                        t2,
                                                        t2_ref,
                                                        xaxbxcxdx_t2_ref,
                                                        xapx,
                                                    )
                                                    .call_base();

                                                // Act
                                                let actual_return_value = mock.work(
                                                    a,
                                                    b,
                                                    c,
                                                    d,
                                                    axb,
                                                    cxd,
                                                    abxbax,
                                                    cdxdcx,
                                                    abcd,
                                                    xaxbxcxdx,
                                                    data.clone(),
                                                    t1,
                                                    t1_ref,
                                                    xaxbxcxdx_t1_ref,
                                                    t2,
                                                    t2_ref,
                                                    xaxbxcxdx_t2_ref,
                                                    xapx,
                                                );

                                                // Assert
                                                assert_eq!(return_value, actual_return_value);

                                                mock.received.as_Trait.work(
                                                    a,
                                                    b,
                                                    c,
                                                    d,
                                                    axb,
                                                    cxd,
                                                    abxbax,
                                                    cdxdcx,
                                                    abcd,
                                                    xaxbxcxdx,
                                                    data.clone(),
                                                    t1,
                                                    t1_ref,
                                                    xaxbxcxdx_t1_ref,
                                                    t2,
                                                    t2_ref,
                                                    xaxbxcxdx_t2_ref,
                                                    xapx,
                                                    Times::Once,
                                                );
                                                mock.received
                                                    .work(
                                                        a,
                                                        b,
                                                        c,
                                                        d,
                                                        axb,
                                                        cxd,
                                                        abxbax,
                                                        cdxdcx,
                                                        abcd,
                                                        xaxbxcxdx,
                                                        data,
                                                        t1,
                                                        t1_ref,
                                                        xaxbxcxdx_t1_ref,
                                                        t2,
                                                        t2_ref,
                                                        xaxbxcxdx_t2_ref,
                                                        xapx,
                                                        Times::Once,
                                                    )
                                                    .no_other_calls()
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
