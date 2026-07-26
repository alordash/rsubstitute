use rsubstitute::*;
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

#[mock]
#[allow(unused)]
trait Trait<'a, 'b: 'a, T1: Clone> {
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
        // TODO - mock(base) to verify that anonymous lifetiems are correct
        unreachable!()
    }
}

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

#[mock]
#[allow(unused)]
struct Struct<'a, 'b: 'a, T1: Clone> {
    pub(super) _phantom_a: PhantomData<&'a ()>,
    pub(super) _phantom_b: PhantomData<&'b ()>,
    pub(super) _phantom_t1: PhantomData<T1>,
}

pub use __rsubstitute_generated_Struct_1_1::*;
#[allow(non_camel_case_types)]
pub mod __rsubstitute_generated_Struct_1_1 {
    use super::__rsubstitute_generated_StructMock::*;
    #[allow(unused_imports)]
    use super::*;
    use rsubstitute::for_generated::*;
    #[allow(unused)]
    impl<'a, 'b: 'a, T1: Clone> Struct<'a, 'b, T1> {
        pub fn new() -> Struct<'a, 'b, T1> {
            StructMock::<'a, 'b, T1>::new()
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
    pub struct work_Call<'c, 'd: 'a, 'a, 'b: 'a, T2: Clone, T1: Clone> {
        pub generics: ::core::marker::PhantomData<(
            &'c (),
            &'d (),
            T2,
            &'a (),
            &'b (),
            T1,
            &'a i32,
            &'b i32,
            &'c i32,
            &'d i32,
            &'a *const &'b i32,
            &'c *const &'d i32,
            &'a &'b *const &'b &'a *const i32,
            &'c &'d *const &'d &'c *const i32,
            &'a &'b &'c &'d i32,
            *const &'a *const &'b *const &'c *const &'d *const i32,
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
            T1,
            *const T1,
            *const &'a *const &'b *const &'c *const &'d *const T1,
            T2,
            *const T2,
            *const &'a *const &'b *const &'c *const &'d *const T2,
            *const &'a *const *const i32,
        )>,
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
    impl<'c, 'd: 'a, 'a, 'b: 'a, T2: Clone, T1: Clone> IGenericsInfoProvider
        for work_Call<'c, 'd, 'a, 'b, T2, T1>
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![generic_type_info("T2", core::any::type_name::<T2>())]
        }
        fn hash_generics_type_ids(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {
            [tid::<T2>()].hash(hasher);
        }
        fn hash_const_values(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
    }
    impl<'c, 'd: 'a, 'a, 'b: 'a, T2: Clone, T1: Clone> ICall for work_Call<'c, 'd, 'a, 'b, T2, T1> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![
                ArgInfo::new(
                    "a",
                    &self.a,
                    (&ArgPrinter(transmute_lifetime!(&self.a, &&'a i32))).debug_string(),
                ),
                ArgInfo::new(
                    "b",
                    &self.b,
                    (&ArgPrinter(transmute_lifetime!(&self.b, &&'b i32))).debug_string(),
                ),
                ArgInfo::new(
                    "c",
                    &self.c,
                    (&ArgPrinter(transmute_lifetime!(&self.c, &&'c i32))).debug_string(),
                ),
                ArgInfo::new(
                    "d",
                    &self.d,
                    (&ArgPrinter(transmute_lifetime!(&self.d, &&'d i32))).debug_string(),
                ),
                ArgInfo::new(
                    "axb",
                    &self.axb,
                    (&ArgPrinter(transmute_lifetime!(&self.axb, &&'a &&'b i32))).debug_string(),
                ),
                ArgInfo::new(
                    "cxd",
                    &self.cxd,
                    (&ArgPrinter(transmute_lifetime!(&self.cxd, &&'c &&'d i32))).debug_string(),
                ),
                ArgInfo::new(
                    "abxbax",
                    &self.abxbax,
                    (&ArgPrinter(transmute_lifetime!(&self.abxbax, &&'a &'b &&'b &'a &i32)))
                        .debug_string(),
                ),
                ArgInfo::new(
                    "cdxdcx",
                    &self.cdxdcx,
                    (&ArgPrinter(transmute_lifetime!(&self.cdxdcx, &&'c &'d &&'d &'c &i32)))
                        .debug_string(),
                ),
                ArgInfo::new(
                    "abcd",
                    &self.abcd,
                    (&ArgPrinter(transmute_lifetime!(&self.abcd, &&'a &'b &'c &'d i32)))
                        .debug_string(),
                ),
                ArgInfo::new(
                    "xaxbxcxdx",
                    &self.xaxbxcxdx,
                    (&ArgPrinter(transmute_lifetime!(
                        &self.xaxbxcxdx,
                        &&&'a &&'b &&'c &&'d &i32
                    )))
                        .debug_string(),
                ),
                ArgInfo::new(
                    "data",
                    &self.data,
                    (&ArgPrinter(transmute_lifetime!(
                        &self.data,
                        &Data<
                            'a,
                            'b,
                            &&i32,
                            &&'a &&'b &[&'c &&'b &Data<'c, 'a, &&&'c &i32, Vec<&'d &'b &()>>],
                        >
                    )))
                        .debug_string(),
                ),
                ArgInfo::new(
                    "t1",
                    &self.t1,
                    (&ArgPrinter(transmute_lifetime!(&self.t1, &T1))).debug_string(),
                ),
                ArgInfo::new(
                    "t1_ref",
                    &self.t1_ref,
                    (&ArgPrinter(transmute_lifetime!(&self.t1_ref, &&T1))).debug_string(),
                ),
                ArgInfo::new(
                    "xaxbxcxdx_t1_ref",
                    &self.xaxbxcxdx_t1_ref,
                    (&ArgPrinter(transmute_lifetime!(
                        &self.xaxbxcxdx_t1_ref,
                        &&&'a &&'b &&'c &&'d &T1
                    )))
                        .debug_string(),
                ),
                ArgInfo::new(
                    "t2",
                    &self.t2,
                    (&ArgPrinter(transmute_lifetime!(&self.t2, &T2))).debug_string(),
                ),
                ArgInfo::new(
                    "t2_ref",
                    &self.t2_ref,
                    (&ArgPrinter(transmute_lifetime!(&self.t2_ref, &&T2))).debug_string(),
                ),
                ArgInfo::new(
                    "xaxbxcxdx_t2_ref",
                    &self.xaxbxcxdx_t2_ref,
                    (&ArgPrinter(transmute_lifetime!(
                        &self.xaxbxcxdx_t2_ref,
                        &&&'a &&'b &&'c &&'d &T2
                    )))
                        .debug_string(),
                ),
                ArgInfo::new(
                    "xapx",
                    &self.xapx,
                    (&ArgPrinter(transmute_lifetime!(&self.xapx, &&&'a *const &i32)))
                        .debug_string(),
                ),
            ]
        }
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
    impl<'c, 'd: 'a, 'a, 'b: 'a, T2: Clone, T1: Clone> ::core::clone::Clone
        for work_Call<'c, 'd, 'a, 'b, T2, T1>
    {
        #[inline]
        fn clone(&self) -> work_Call<'c, 'd, 'a, 'b, T2, T1> {
            work_Call::<'c, 'd, 'a, 'b, T2, T1> {
                generics: ::core::clone::Clone::clone(&self.generics),
                a: ::core::clone::Clone::clone(&self.a),
                b: ::core::clone::Clone::clone(&self.b),
                c: ::core::clone::Clone::clone(&self.c),
                d: ::core::clone::Clone::clone(&self.d),
                axb: ::core::clone::Clone::clone(&self.axb),
                cxd: ::core::clone::Clone::clone(&self.cxd),
                abxbax: ::core::clone::Clone::clone(&self.abxbax),
                cdxdcx: ::core::clone::Clone::clone(&self.cdxdcx),
                abcd: ::core::clone::Clone::clone(&self.abcd),
                xaxbxcxdx: ::core::clone::Clone::clone(&self.xaxbxcxdx),
                data: ::core::clone::Clone::clone(&self.data),
                t1: ::core::clone::Clone::clone(&self.t1),
                t1_ref: ::core::clone::Clone::clone(&self.t1_ref),
                xaxbxcxdx_t1_ref: ::core::clone::Clone::clone(&self.xaxbxcxdx_t1_ref),
                t2: ::core::clone::Clone::clone(&self.t2),
                t2_ref: ::core::clone::Clone::clone(&self.t2_ref),
                xaxbxcxdx_t2_ref: ::core::clone::Clone::clone(&self.xaxbxcxdx_t2_ref),
                xapx: ::core::clone::Clone::clone(&self.xapx),
            }
        }
    }
    struct work_ArgsChecker<'c, 'd: 'a, 'a, 'b: 'a, T2: Clone, T1: Clone> {
        pub generics: ::core::marker::PhantomData<(
            &'c (),
            &'d (),
            T2,
            &'a (),
            &'b (),
            T1,
            &'a i32,
            &'b i32,
            &'c i32,
            &'d i32,
            &'a *const &'b i32,
            &'c *const &'d i32,
            &'a &'b *const &'b &'a *const i32,
            &'c &'d *const &'d &'c *const i32,
            &'a &'b &'c &'d i32,
            *const &'a *const &'b *const &'c *const &'d *const i32,
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
            T1,
            *const T1,
            *const &'a *const &'b *const &'c *const &'d *const T1,
            T2,
            *const T2,
            *const &'a *const &'b *const &'c *const &'d *const T2,
            *const &'a *const *const i32,
        )>,
        a: Arg<*const i32>,
        b: Arg<*const i32>,
        c: Arg<*const i32>,
        d: Arg<*const i32>,
        axb: Arg<*const *const *const i32>,
        cxd: Arg<*const *const *const i32>,
        abxbax: Arg<*const *const *const *const *const *const i32>,
        cdxdcx: Arg<*const *const *const *const *const *const i32>,
        abcd: Arg<*const *const *const *const i32>,
        xaxbxcxdx: Arg<*const *const *const *const *const *const *const *const *const i32>,
        data: Arg<
            Data<
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
        >,
        t1: Arg<T1>,
        t1_ref: Arg<*const T1>,
        xaxbxcxdx_t1_ref: Arg<*const *const *const *const *const *const *const *const *const T1>,
        t2: Arg<T2>,
        t2_ref: Arg<*const T2>,
        xaxbxcxdx_t2_ref: Arg<*const *const *const *const *const *const *const *const *const T2>,
        xapx: Arg<*const *const *const *const i32>,
    }
    impl<'c, 'd: 'a, 'a, 'b: 'a, T2: Clone, T1: Clone> IGenericsInfoProvider
        for work_ArgsChecker<'c, 'd, 'a, 'b, T2, T1>
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![generic_type_info("T2", core::any::type_name::<T2>())]
        }
        fn hash_generics_type_ids(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {
            [tid::<T2>()].hash(hasher);
        }
        fn hash_const_values(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
    }
    impl<'c, 'd: 'a, 'a, 'b: 'a, T2: Clone, T1: Clone> IArgsChecker
        for work_ArgsChecker<'c, 'd, 'a, 'b, T2, T1>
    {
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            #[allow(unused_variables)]
            let call: &work_Call<'c, 'd, 'a, 'b, T2, T1> = dyn_call.downcast_ref();
            vec![
                transmute_lifetime!(&self.a, &Arg<&'a i32>).check_ref(
                    "a",
                    transmute_lifetime!(&call.a),
                    (&ArgPrinter(transmute_lifetime!(&call.a, &&'a i32))).debug_string(),
                ),
                transmute_lifetime!(&self.b, &Arg<&'b i32>).check_ref(
                    "b",
                    transmute_lifetime!(&call.b),
                    (&ArgPrinter(transmute_lifetime!(&call.b, &&'b i32))).debug_string(),
                ),
                transmute_lifetime!(&self.c, &Arg<&'c i32>).check_ref(
                    "c",
                    transmute_lifetime!(&call.c),
                    (&ArgPrinter(transmute_lifetime!(&call.c, &&'c i32))).debug_string(),
                ),
                transmute_lifetime!(&self.d, &Arg<&'d i32>).check_ref(
                    "d",
                    transmute_lifetime!(&call.d),
                    (&ArgPrinter(transmute_lifetime!(&call.d, &&'d i32))).debug_string(),
                ),
                transmute_lifetime!(&self.axb, &Arg<&'a &&'b i32>).check_ref(
                    "axb",
                    transmute_lifetime!(&call.axb),
                    (&ArgPrinter(transmute_lifetime!(&call.axb, &&'a &&'b i32))).debug_string(),
                ),
                transmute_lifetime!(&self.cxd, &Arg<&'c &&'d i32>).check_ref(
                    "cxd",
                    transmute_lifetime!(&call.cxd),
                    (&ArgPrinter(transmute_lifetime!(&call.cxd, &&'c &&'d i32))).debug_string(),
                ),
                transmute_lifetime!(&self.abxbax, &Arg<&'a &'b &&'b &'a &i32>).check_ref(
                    "abxbax",
                    transmute_lifetime!(&call.abxbax),
                    (&ArgPrinter(transmute_lifetime!(&call.abxbax, &&'a &'b &&'b &'a &i32)))
                        .debug_string(),
                ),
                transmute_lifetime!(&self.cdxdcx, &Arg<&'c &'d &&'d &'c &i32>).check_ref(
                    "cdxdcx",
                    transmute_lifetime!(&call.cdxdcx),
                    (&ArgPrinter(transmute_lifetime!(&call.cdxdcx, &&'c &'d &&'d &'c &i32)))
                        .debug_string(),
                ),
                transmute_lifetime!(&self.abcd, &Arg<&'a &'b &'c &'d i32>).check_ref(
                    "abcd",
                    transmute_lifetime!(&call.abcd),
                    (&ArgPrinter(transmute_lifetime!(&call.abcd, &&'a &'b &'c &'d i32)))
                        .debug_string(),
                ),
                transmute_lifetime!(&self.xaxbxcxdx, &Arg<&&'a &&'b &&'c &&'d &i32>).check_ref(
                    "xaxbxcxdx",
                    transmute_lifetime!(&call.xaxbxcxdx),
                    (&ArgPrinter(transmute_lifetime!(
                        &call.xaxbxcxdx,
                        &&&'a &&'b &&'c &&'d &i32
                    )))
                        .debug_string(),
                ),
                transmute_lifetime!(
                    &self.data,
                    &Arg<
                        Data<
                            'a,
                            'b,
                            &&i32,
                            &&'a &&'b &[&'c &&'b &Data<'c, 'a, &&&'c &i32, Vec<&'d &'b &()>>],
                        >,
                    >
                )
                .check(
                    "data",
                    transmute_lifetime!(&call.data),
                    (&ArgPrinter(transmute_lifetime!(
                        &call.data,
                        &Data<
                            'a,
                            'b,
                            &&i32,
                            &&'a &&'b &[&'c &&'b &Data<'c, 'a, &&&'c &i32, Vec<&'d &'b &()>>],
                        >
                    )))
                        .debug_string(),
                ),
                transmute_lifetime!(&self.t1, &Arg<T1>).check(
                    "t1",
                    transmute_lifetime!(&call.t1),
                    (&ArgPrinter(transmute_lifetime!(&call.t1, &T1))).debug_string(),
                ),
                transmute_lifetime!(&self.t1_ref, &Arg<&T1>).check_ref(
                    "t1_ref",
                    transmute_lifetime!(&call.t1_ref),
                    (&ArgPrinter(transmute_lifetime!(&call.t1_ref, &&T1))).debug_string(),
                ),
                transmute_lifetime!(&self.xaxbxcxdx_t1_ref, &Arg<&&'a &&'b &&'c &&'d &T1>)
                    .check_ref(
                        "xaxbxcxdx_t1_ref",
                        transmute_lifetime!(&call.xaxbxcxdx_t1_ref),
                        (&ArgPrinter(transmute_lifetime!(
                            &call.xaxbxcxdx_t1_ref,
                            &&&'a &&'b &&'c &&'d &T1
                        )))
                            .debug_string(),
                    ),
                transmute_lifetime!(&self.t2, &Arg<T2>).check(
                    "t2",
                    transmute_lifetime!(&call.t2),
                    (&ArgPrinter(transmute_lifetime!(&call.t2, &T2))).debug_string(),
                ),
                transmute_lifetime!(&self.t2_ref, &Arg<&T2>).check_ref(
                    "t2_ref",
                    transmute_lifetime!(&call.t2_ref),
                    (&ArgPrinter(transmute_lifetime!(&call.t2_ref, &&T2))).debug_string(),
                ),
                transmute_lifetime!(&self.xaxbxcxdx_t2_ref, &Arg<&&'a &&'b &&'c &&'d &T2>)
                    .check_ref(
                        "xaxbxcxdx_t2_ref",
                        transmute_lifetime!(&call.xaxbxcxdx_t2_ref),
                        (&ArgPrinter(transmute_lifetime!(
                            &call.xaxbxcxdx_t2_ref,
                            &&&'a &&'b &&'c &&'d &T2
                        )))
                            .debug_string(),
                    ),
                transmute_lifetime!(&self.xapx, &Arg<&&'a *const &i32>).check_ref(
                    "xapx",
                    transmute_lifetime!(&call.xapx),
                    (&ArgPrinter(transmute_lifetime!(&call.xapx, &&&'a *const &i32)))
                        .debug_string(),
                ),
            ]
        }
        fn fmt_args(&self) -> String {
            format!(
                "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
                (&ArgPrinter(transmute_lifetime!(&&self.a, &&Arg<&'a i32>))).debug_string(),
                (&ArgPrinter(transmute_lifetime!(&&self.b, &&Arg<&'b i32>))).debug_string(),
                (&ArgPrinter(transmute_lifetime!(&&self.c, &&Arg<&'c i32>))).debug_string(),
                (&ArgPrinter(transmute_lifetime!(&&self.d, &&Arg<&'d i32>))).debug_string(),
                (&ArgPrinter(transmute_lifetime!(&&self.axb, &&Arg<&'a &&'b i32>))).debug_string(),
                (&ArgPrinter(transmute_lifetime!(&&self.cxd, &&Arg<&'c &&'d i32>))).debug_string(),
                (&ArgPrinter(transmute_lifetime!(
                    &&self.abxbax,
                    &&Arg<&'a &'b &&'b &'a &i32>
                )))
                    .debug_string(),
                (&ArgPrinter(transmute_lifetime!(
                    &&self.cdxdcx,
                    &&Arg<&'c &'d &&'d &'c &i32>
                )))
                    .debug_string(),
                (&ArgPrinter(transmute_lifetime!(&&self.abcd, &&Arg<&'a &'b &'c &'d i32>)))
                    .debug_string(),
                (&ArgPrinter(transmute_lifetime!(
                    &&self.xaxbxcxdx,
                    &&Arg<&&'a &&'b &&'c &&'d &i32>
                )))
                    .debug_string(),
                (&ArgPrinter(transmute_lifetime!(
                    &&self.data,
                    &&Arg<
                        Data<
                            'a,
                            'b,
                            &&i32,
                            &&'a &&'b &[&'c &&'b &Data<'c, 'a, &&&'c &i32, Vec<&'d &'b &()>>],
                        >,
                    >
                )))
                    .debug_string(),
                (&ArgPrinter(transmute_lifetime!(&&self.t1, &&Arg<T1>))).debug_string(),
                (&ArgPrinter(transmute_lifetime!(&&self.t1_ref, &&Arg<&T1>))).debug_string(),
                (&ArgPrinter(transmute_lifetime!(
                    &&self.xaxbxcxdx_t1_ref,
                    &&Arg<&&'a &&'b &&'c &&'d &T1>
                )))
                    .debug_string(),
                (&ArgPrinter(transmute_lifetime!(&&self.t2, &&Arg<T2>))).debug_string(),
                (&ArgPrinter(transmute_lifetime!(&&self.t2_ref, &&Arg<&T2>))).debug_string(),
                (&ArgPrinter(transmute_lifetime!(
                    &&self.xaxbxcxdx_t2_ref,
                    &&Arg<&&'a &&'b &&'c &&'d &T2>
                )))
                    .debug_string(),
                (&ArgPrinter(transmute_lifetime!(&&self.xapx, &&Arg<&&'a *const &i32>)))
                    .debug_string()
            )
        }
    }
    pub struct new_Call<'a, 'b: 'a, T1: Clone> {
        pub generics: ::core::marker::PhantomData<(&'a (), &'b (), T1)>,
    }
    impl<'a, 'b: 'a, T1: Clone> IGenericsInfoProvider for new_Call<'a, 'b, T1> {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
    }
    impl<'a, 'b: 'a, T1: Clone> ICall for new_Call<'a, 'b, T1> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new(())) as *mut _ as *mut ()
        }
    }
    impl<'a, 'b: 'a, T1: Clone> ::core::clone::Clone for new_Call<'a, 'b, T1> {
        #[inline]
        fn clone(&self) -> new_Call<'a, 'b, T1> {
            new_Call::<'a, 'b, T1> {
                generics: ::core::clone::Clone::clone(&self.generics),
            }
        }
    }
    struct new_ArgsChecker<'a, 'b: 'a, T1: Clone> {
        pub generics: ::core::marker::PhantomData<(&'a (), &'b (), T1)>,
    }
    impl<'a, 'b: 'a, T1: Clone> IGenericsInfoProvider for new_ArgsChecker<'a, 'b, T1> {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, #[allow(unused_variables)] hasher: &mut GenericsHasher) {}
    }
    impl<'a, 'b: 'a, T1: Clone> IArgsChecker for new_ArgsChecker<'a, 'b, T1> {
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            #[allow(unused_variables)]
            let call: &new_Call<'a, 'b, T1> = dyn_call.downcast_ref();
            vec![]
        }
        fn fmt_args(&self) -> String {
            format!("")
        }
    }
    impl<'a, 'b: 'a, T1: Clone> StructMock<'a, 'b, T1> {
        fn new() -> Struct<'a, 'b, T1> {
            let call = new_Call::<'a, 'b, T1> {
                generics: ::core::marker::PhantomData,
            };
            let fn_data: &FnData<StructMock<'a, 'b, T1>, true, true, false> =
                get_static_fn_data("new");
            fn_data.handle((), call, Self::__rs_base_new)
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
            let call = work_Call::<'c, 'd, 'a, 'b, T2, T1> {
                generics: ::core::marker::PhantomData,
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
            let fn_data: &FnData<StructMock<'a, 'b, T1>, true, true, false> = self
                .data
                .get_shared_fn_data("work", call.get_generics_hash_key());
            fn_data.handle(self, call, Self::__rs_base_work::<T2>)
        }
        fn __rs_base_work<'__rs_base, 'c, 'd: 'a, T2: Clone>(
            __rsa_self: &'__rs_base StructMock<'a, 'b, T1>,
            call: work_Call<'c, 'd, 'a, 'b, T2, T1>,
        ) -> &'__rs_base &'a &'__rs_base &'a &'__rs_base &'b &'__rs_base &'b &'__rs_base &'c &'__rs_base &'c &'__rs_base &'d &'__rs_base &'d &'__rs_base i32{
            let work_Call::<'c, 'd, 'a, 'b, T2, T1> {
                a: a,
                b: b,
                c: c,
                d: d,
                axb: axb,
                cxd: cxd,
                abxbax: abxbax,
                cdxdcx: cdxdcx,
                abcd: abcd,
                xaxbxcxdx: xaxbxcxdx,
                data: data,
                t1: t1,
                t1_ref: t1_ref,
                xaxbxcxdx_t1_ref: xaxbxcxdx_t1_ref,
                t2: t2,
                t2_ref: t2_ref,
                xaxbxcxdx_t2_ref: xaxbxcxdx_t2_ref,
                xapx: xapx,
                ..
            } = call;
            let (
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
            ): (
                &'a i32,
                &'b i32,
                &'c i32,
                &'d i32,
                &'a *const &'b i32,
                &'c *const &'d i32,
                &'a &'b *const &'b &'a *const i32,
                &'c &'d *const &'d &'c *const i32,
                &'a &'b &'c &'d i32,
                *const &'a *const &'b *const &'c *const &'d *const i32,
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
                T1,
                *const T1,
                *const &'a *const &'b *const &'c *const &'d *const T1,
                T2,
                *const T2,
                *const &'a *const &'b *const &'c *const &'d *const T2,
                *const &'a *const *const i32,
            ) = transmute_lifetime!((
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
                xapx
            ));
            { unreachable!() }
        }
        fn __rs_base_new(_: (), call: new_Call<'a, 'b, T1>) -> Struct<'a, 'b, T1> {
            let new_Call::<'a, 'b, T1> { .. } = call;
            let (): () = transmute_lifetime!(());
            {
                Struct::<'a, 'b, T1> {
                    _phantom_a: PhantomData,
                    _phantom_b: PhantomData,
                    _phantom_t1: PhantomData,
                }
            }
        }
    }
    impl<'a, 'b: 'a, T1: Clone> StructSetup<'a, 'b, T1> {
        pub fn work<'__rsa, 'c, 'd : 'a, T2: Clone>(&self, a: impl Into::<Arg::<&'a i32>>, b: impl Into::<Arg::<&'b i32>>, c: impl Into::<Arg::<&'c i32>>, d: impl Into::<Arg::<&'d i32>>, axb: impl Into::<Arg::<&'a &'__rsa &'b i32>>, cxd: impl Into::<Arg::<&'c &'__rsa &'d i32>>, abxbax: impl Into::<Arg::<&'a &'b &'__rsa &'b &'a &'__rsa i32>>, cdxdcx: impl Into::<Arg::<&'c &'d &'__rsa &'d &'c &'__rsa i32>>, abcd: impl Into::<Arg::<&'a &'b &'c &'d i32>>, xaxbxcxdx: impl Into::<Arg::<&'__rsa &'a &'__rsa &'b &'__rsa &'c &'__rsa &'d &'__rsa i32>>, data: impl Into::<Arg::<Data<
            'a,
            'b,
            &'__rsa &'__rsa i32,
            &'__rsa &'a &'__rsa &'b &'__rsa [&'c &'__rsa &'b &'__rsa Data<'c, 'a, &'__rsa &'__rsa &'c &'__rsa i32, Vec<&'d &'b &'__rsa ()>>],
        >>>, t1: impl Into::<Arg::<T1>>, t1_ref: impl Into::<Arg::<&'__rsa T1>>, xaxbxcxdx_t1_ref: impl Into::<Arg::<&'__rsa &'a &'__rsa &'b &'__rsa &'c &'__rsa &'d &'__rsa T1>>, t2: impl Into::<Arg::<T2>>, t2_ref: impl Into::<Arg::<&'__rsa T2>>, xaxbxcxdx_t2_ref: impl Into::<Arg::<&'__rsa &'a &'__rsa &'b &'__rsa &'c &'__rsa &'d &'__rsa T2>>, xapx: impl Into::<Arg::<&'__rsa &'a *const &'__rsa i32>>) -> FnConfigurator<'_, StructMock<'a, 'b, T1>, Self, (&'__rsa &'a i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        &'__rsa &'b i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        &'__rsa &'c i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        &'__rsa &'d i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        &'__rsa &'a &'__rsa &'b i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        &'__rsa &'c &'__rsa &'d i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        &'__rsa &'a &'b &'__rsa &'b &'a &'__rsa i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        &'__rsa &'c &'d &'__rsa &'d &'c &'__rsa i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        &'__rsa &'a &'b &'c &'d i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        &'__rsa &'__rsa &'a &'__rsa &'b &'__rsa &'c &'__rsa &'d &'__rsa i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        &'__rsa Data<
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            'a,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            'b,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            &'__rsa &'__rsa i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            &'__rsa &'a &'__rsa &'b &'__rsa [&'c &'__rsa &'b &'__rsa Data<'c, 'a, &'__rsa &'__rsa &'c &'__rsa i32, Vec<&'d &'b &'__rsa ()>>],
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        >,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        &'__rsa T1,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        &'__rsa &'__rsa T1,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        &'__rsa &'__rsa &'a &'__rsa &'b &'__rsa &'c &'__rsa &'d &'__rsa T1,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        &'__rsa T2,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        &'__rsa &'__rsa T2,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        &'__rsa &'__rsa &'a &'__rsa &'b &'__rsa &'c &'__rsa &'d &'__rsa T2,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        &'__rsa &'__rsa &'a *const &'__rsa i32),
        &'__rsa &'a &'__rsa &'a &'__rsa &'b &'__rsa &'b &'__rsa &'c &'__rsa &'c &'__rsa &'d &'__rsa &'d &'__rsa i32, StructMock<'a, 'b, T1>, true, true, false>{
            let args_checker = work_ArgsChecker::<'c, 'd, 'a, 'b, T2, T1> {
                generics: ::core::marker::PhantomData,
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
            let fn_data: &FnData<StructMock<'a, 'b, T1>, true, true, false> = self
                .data
                .get_shared_fn_data("work", args_checker.get_generics_hash_key());
            let fn_configurator: FnConfigurator<'_, StructMock<'a, 'b, T1>, Self, (&'__rsa &'a i32,
                                                                                   &'__rsa &'b i32,
                                                                                   &'__rsa &'c i32,
                                                                                   &'__rsa &'d i32,
                                                                                   &'__rsa &'a &'__rsa &'b i32,
                                                                                   &'__rsa &'c &'__rsa &'d i32,
                                                                                   &'__rsa &'a &'b &'__rsa &'b &'a &'__rsa i32,
                                                                                   &'__rsa &'c &'d &'__rsa &'d &'c &'__rsa i32,
                                                                                   &'__rsa &'a &'b &'c &'d i32,
                                                                                   &'__rsa &'__rsa &'a &'__rsa &'b &'__rsa &'c &'__rsa &'d &'__rsa i32,
                                                                                   &'__rsa Data<
                                                                                       'a,
                                                                                       'b,
                                                                                       &'__rsa &'__rsa i32,
                                                                                       &'__rsa &'a &'__rsa &'b &'__rsa [&'c &'__rsa &'b &'__rsa Data<'c, 'a, &'__rsa &'__rsa &'c &'__rsa i32, Vec<&'d &'b &'__rsa ()>>],
                                                                                   >,
                                                                                   &'__rsa T1,
                                                                                   &'__rsa &'__rsa T1,
                                                                                   &'__rsa &'__rsa &'a &'__rsa &'b &'__rsa &'c &'__rsa &'d &'__rsa T1,
                                                                                   &'__rsa T2,
                                                                                   &'__rsa &'__rsa T2,
                                                                                   &'__rsa &'__rsa &'a &'__rsa &'b &'__rsa &'c &'__rsa &'d &'__rsa T2,
                                                                                   &'__rsa &'__rsa &'a *const &'__rsa i32),
                &'__rsa &'a &'__rsa &'a &'__rsa &'b &'__rsa &'b &'__rsa &'c &'__rsa &'c &'__rsa &'d &'__rsa &'d &'__rsa i32, StructMock<'a, 'b, T1>, true, true, false> = fn_data.add_config(args_checker, self);
            transmute_lifetime!(fn_configurator)
        }
    }
    impl<'a, 'b: 'a, T1: Clone> StructReceived<'a, 'b, T1> {
        pub fn work<'__rsa, 'c, 'd: 'a, T2: Clone>(
            &self,
            a: impl Into<Arg<&'a i32>>,
            b: impl Into<Arg<&'b i32>>,
            c: impl Into<Arg<&'c i32>>,
            d: impl Into<Arg<&'d i32>>,
            axb: impl Into<Arg<&'a &'__rsa &'b i32>>,
            cxd: impl Into<Arg<&'c &'__rsa &'d i32>>,
            abxbax: impl Into<Arg<&'a &'b &'__rsa &'b &'a &'__rsa i32>>,
            cdxdcx: impl Into<Arg<&'c &'d &'__rsa &'d &'c &'__rsa i32>>,
            abcd: impl Into<Arg<&'a &'b &'c &'d i32>>,
            xaxbxcxdx: impl Into<Arg<&'__rsa &'a &'__rsa &'b &'__rsa &'c &'__rsa &'d &'__rsa i32>>,
            data: impl Into<
                Arg<
                    Data<
                        'a,
                        'b,
                        &'__rsa &'__rsa i32,
                        &'__rsa &'a &'__rsa &'b &'__rsa [&'c &'__rsa &'b &'__rsa Data<
                            'c,
                            'a,
                            &'__rsa &'__rsa &'c &'__rsa i32,
                            Vec<&'d &'b &'__rsa ()>,
                        >],
                    >,
                >,
            >,
            t1: impl Into<Arg<T1>>,
            t1_ref: impl Into<Arg<&'__rsa T1>>,
            xaxbxcxdx_t1_ref: impl Into<Arg<&'__rsa &'a &'__rsa &'b &'__rsa &'c &'__rsa &'d &'__rsa T1>>,
            t2: impl Into<Arg<T2>>,
            t2_ref: impl Into<Arg<&'__rsa T2>>,
            xaxbxcxdx_t2_ref: impl Into<Arg<&'__rsa &'a &'__rsa &'b &'__rsa &'c &'__rsa &'d &'__rsa T2>>,
            xapx: impl Into<Arg<&'__rsa &'a *const &'__rsa i32>>,
            times: Times,
        ) -> ::rsubstitute::for_generated::ArgRefsBinder<
            Self,
            (
                &'__rsa &'a i32,
                &'__rsa &'b i32,
                &'__rsa &'c i32,
                &'__rsa &'d i32,
                &'__rsa &'a &'__rsa &'b i32,
                &'__rsa &'c &'__rsa &'d i32,
                &'__rsa &'a &'b &'__rsa &'b &'a &'__rsa i32,
                &'__rsa &'c &'d &'__rsa &'d &'c &'__rsa i32,
                &'__rsa &'a &'b &'c &'d i32,
                &'__rsa &'__rsa &'a &'__rsa &'b &'__rsa &'c &'__rsa &'d &'__rsa i32,
                &'__rsa Data<
                    'a,
                    'b,
                    &'__rsa &'__rsa i32,
                    &'__rsa &'a &'__rsa &'b &'__rsa [&'c &'__rsa &'b &'__rsa Data<
                        'c,
                        'a,
                        &'__rsa &'__rsa &'c &'__rsa i32,
                        Vec<&'d &'b &'__rsa ()>,
                    >],
                >,
                &'__rsa T1,
                &'__rsa &'__rsa T1,
                &'__rsa &'__rsa &'a &'__rsa &'b &'__rsa &'c &'__rsa &'d &'__rsa T1,
                &'__rsa T2,
                &'__rsa &'__rsa T2,
                &'__rsa &'__rsa &'a &'__rsa &'b &'__rsa &'c &'__rsa &'d &'__rsa T2,
                &'__rsa &'__rsa &'a *const &'__rsa i32,
            ),
        >
        where
            'c: '__rsa,
            'd: '__rsa,
            'a: '__rsa,
            'b: '__rsa,
            '__rsa: 'c + 'd + 'a + 'b,
        {
            let args_checker = work_ArgsChecker::<'c, 'd, 'a, 'b, T2, T1> {
                generics: ::core::marker::PhantomData,
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
            let fn_data: &FnData<StructMock<'a, 'b, T1>, true, true, false> = self
                .data
                .get_shared_fn_data("work", args_checker.get_generics_hash_key());
            fn_data.verify_received(args_checker, times);
            rsubstitute::for_generated::ArgRefsBinder::new(self.clone())
        }
    }
    impl<'a, 'b: 'a, T1: Clone> StructStaticSetup<'a, 'b, T1> {
        pub fn new<'__rsa>(
            &self,
        ) -> FnConfigurator<
            '_,
            StructMock<'a, 'b, T1>,
            Self,
            (),
            Struct<'a, 'b, T1>,
            StructMock<'a, 'b, T1>,
            true,
            true,
            false,
        > {
            let args_checker = new_ArgsChecker::<'a, 'b, T1> {
                generics: ::core::marker::PhantomData,
            };
            let fn_data: &FnData<StructMock<'a, 'b, T1>, true, true, false> =
                get_static_fn_data("new");
            let fn_configurator: FnConfigurator<
                '_,
                StructMock<'a, 'b, T1>,
                Self,
                (),
                Struct<'a, 'b, T1>,
                StructMock<'a, 'b, T1>,
                true,
                true,
                false,
            > = fn_data.add_config(args_checker, self);
            transmute_lifetime!(fn_configurator)
        }
    }
    impl<'a, 'b: 'a, T1: Clone> StructStaticReceived<'a, 'b, T1> {
        pub fn new<'__rsa>(
            &self,
            times: Times,
        ) -> ::rsubstitute::for_generated::ArgRefsBinder<Self, ()>
        where
            'a: '__rsa,
            'b: '__rsa,
            '__rsa: 'a + 'b,
        {
            let args_checker = new_ArgsChecker::<'a, 'b, T1> {
                generics: ::core::marker::PhantomData,
            };
            let fn_data: &FnData<StructMock<'a, 'b, T1>, true, true, false> =
                get_static_fn_data("new");
            fn_data.verify_received(args_checker, times);
            rsubstitute::for_generated::ArgRefsBinder::new(self.clone())
        }
    }
}

// #[mock(base)]
// #[allow(unused)]
// impl<'a, 'b: 'a, T1: Clone> Trait<'a, 'b, T1> for Struct<'a, 'b, T1> {
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
//     ) -> &&'a &&'a &&'b &&'b &&'c &&'c &&'d &&'d &i32 {
//         Self::work(
//             self,
//             a,
//             b,
//             c,
//             d,
//             axb,
//             cxd,
//             abxbax,
//             cdxdcx,
//             abcd,
//             xaxbxcxdx,
//             data,
//             t1,
//             t1_ref,
//             xaxbxcxdx_t1_ref,
//             t2,
//             t2_ref,
//             xaxbxcxdx_t2_ref,
//             xapx,
//         )
//     }
// }

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn trait_work_Ok() {
        // Arrange
        let mut mock = TraitMock::new();
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
                                                mock.setup()
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

                                                mock.received()
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

    // #[test]
    // fn struct_work_Ok() {
    //     // Arrange
    //     let mut mock = Struct::new();
    //     let return_value = &&&&&&&&&&&&&&&&&55;
    //     let a = &1;
    //     {
    //         let b = &2;
    //         {
    //             let c = &3;
    //             {
    //                 let d = &4;
    //                 {
    //                     let axb = &&&5;
    //                     {
    //                         let cxd = &&&6;
    //                         {
    //                             let abxbax = &&&&&&7;
    //                             {
    //                                 let cdxdcx = &&&&&&8;
    //                                 {
    //                                     let abcd = &&&&9;
    //                                     {
    //                                         let xaxbxcxdx = &&&&&&&&&10;
    //                                         {
    //                                             let data = Data::<'_, '_, _, _> {
    //                                                 _phantoms: Default::default(),
    //                                             };
    //                                             let t1 = [7, 77];
    //                                             let t1_ref = &[8, 88];
    //                                             let xaxbxcxdx_t1_ref = &&&&&&&&&[9, 99];
    //                                             let t2 = true;
    //                                             let t2_ref = &true;
    //                                             let xaxbxcxdx_t2_ref = &&&&&&&&&true;
    //                                             let xapx = &&(&(&188) as *const _);
    //                                             mock.setup
    //                                                 .work(
    //                                                     a,
    //                                                     b,
    //                                                     c,
    //                                                     d,
    //                                                     axb,
    //                                                     cxd,
    //                                                     abxbax,
    //                                                     cdxdcx,
    //                                                     abcd,
    //                                                     xaxbxcxdx,
    //                                                     data.clone(),
    //                                                     t1,
    //                                                     t1_ref,
    //                                                     xaxbxcxdx_t1_ref,
    //                                                     t2,
    //                                                     t2_ref,
    //                                                     xaxbxcxdx_t2_ref,
    //                                                     xapx,
    //                                                 )
    //                                                 .returns(return_value);
    //                                             mock.setup
    //                                                 .as_Trait
    //                                                 .work(
    //                                                     a,
    //                                                     b,
    //                                                     c,
    //                                                     d,
    //                                                     axb,
    //                                                     cxd,
    //                                                     abxbax,
    //                                                     cdxdcx,
    //                                                     abcd,
    //                                                     xaxbxcxdx,
    //                                                     data.clone(),
    //                                                     t1,
    //                                                     t1_ref,
    //                                                     xaxbxcxdx_t1_ref,
    //                                                     t2,
    //                                                     t2_ref,
    //                                                     xaxbxcxdx_t2_ref,
    //                                                     xapx,
    //                                                 )
    //                                                 .call_base();
    //
    //                                             // Act
    //                                             let actual_return_value = mock.work(
    //                                                 a,
    //                                                 b,
    //                                                 c,
    //                                                 d,
    //                                                 axb,
    //                                                 cxd,
    //                                                 abxbax,
    //                                                 cdxdcx,
    //                                                 abcd,
    //                                                 xaxbxcxdx,
    //                                                 data.clone(),
    //                                                 t1,
    //                                                 t1_ref,
    //                                                 xaxbxcxdx_t1_ref,
    //                                                 t2,
    //                                                 t2_ref,
    //                                                 xaxbxcxdx_t2_ref,
    //                                                 xapx,
    //                                             );
    //
    //                                             // Assert
    //                                             assert_eq!(return_value, actual_return_value);
    //
    //                                             mock.received().as_Trait.work(
    //                                                 a,
    //                                                 b,
    //                                                 c,
    //                                                 d,
    //                                                 axb,
    //                                                 cxd,
    //                                                 abxbax,
    //                                                 cdxdcx,
    //                                                 abcd,
    //                                                 xaxbxcxdx,
    //                                                 data.clone(),
    //                                                 t1,
    //                                                 t1_ref,
    //                                                 xaxbxcxdx_t1_ref,
    //                                                 t2,
    //                                                 t2_ref,
    //                                                 xaxbxcxdx_t2_ref,
    //                                                 xapx,
    //                                                 Times::Once,
    //                                             );
    //                                             mock.received
    //                                                 .work(
    //                                                     a,
    //                                                     b,
    //                                                     c,
    //                                                     d,
    //                                                     axb,
    //                                                     cxd,
    //                                                     abxbax,
    //                                                     cdxdcx,
    //                                                     abcd,
    //                                                     xaxbxcxdx,
    //                                                     data,
    //                                                     t1,
    //                                                     t1_ref,
    //                                                     xaxbxcxdx_t1_ref,
    //                                                     t2,
    //                                                     t2_ref,
    //                                                     xaxbxcxdx_t2_ref,
    //                                                     xapx,
    //                                                     Times::Once,
    //                                                 )
    //                                                 .no_other_calls()
    //                                         }
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
}
