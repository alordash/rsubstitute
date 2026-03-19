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

// #[mock]
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

// #[mock]
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
) -> &'x &'a &'x &'a &'x &'b &'x &'b &'x &'c &'x &'c &'x &'d &'x &'d &'x i32 {
    unreachable!()
}

// mocked_base! {
//     #[allow(unused)]
//     struct Struct<'a, 'b: 'a, T1: Clone> {
//         _phantom_a: PhantomData<&'a ()>,
//         _phantom_b: PhantomData<&'b ()>,
//         _phantom_t1: PhantomData<T1>,
//     }
//
//     #[allow(unused)]
//     impl<'a, 'b: 'a, T1: Clone> Struct<'a, 'b, T1> {
//         pub fn new() -> Self {
//             Self {
//                 _phantom_a: PhantomData,
//                 _phantom_b: PhantomData,
//                 _phantom_t1: PhantomData,
//             }
//         }
//
//         #[allow(unused)]
//         fn work<'c, 'd: 'a, T2: Clone>(
//             &self,
//             a: &'a i32,
//             b: &'b i32,
//             c: &'c i32,
//             d: &'d i32,
//             axb: &'a &&'b i32,
//             cxd: &'c &&'d i32,
//             abxbax: &'a &'b &&'b &'a &i32,
//             cdxdcx: &'c &'d &&'d &'c &i32,
//             abcd: &'a &'b &'c &'d i32,
//             xaxbxcxdx: &&'a &&'b &&'c &&'d &i32,
//             data: Data<
//                 'a,
//                 'b,
//                 &&i32,
//                 &&'a &&'b &[&'c &&'b &Data<'c, 'a, &&&'c &i32, Vec<&'d &'b &()>>],
//             >,
//             t1: T1,
//             t1_ref: &T1,
//             xaxbxcxdx_t1_ref: &&'a &&'b &&'c &&'d &T1,
//             t2: T2,
//             t2_ref: &T2,
//             xaxbxcxdx_t2_ref: &&'a &&'b &&'c &&'d &T2,
//         ) -> &&'a &&'a &&'b &&'b &&'c &&'c &&'d &&'d &i32 {
//             unreachable!()
//         }
//     }
//
//     #[allow(unused)]
//     impl<'a, 'b: 'a, T1: Clone> Trait<'a, 'b, T1> for Struct<'a, 'b, T1> {
//         fn work<'c, 'd: 'a, T2: Clone>(
//             &self,
//             a: &'a i32,
//             b: &'b i32,
//             c: &'c i32,
//             d: &'d i32,
//             axb: &'a &&'b i32,
//             cxd: &'c &&'d i32,
//             abxbax: &'a &'b &&'b &'a &i32,
//             cdxdcx: &'c &'d &&'d &'c &i32,
//             abcd: &'a &'b &'c &'d i32,
//             xaxbxcxdx: &&'a &&'b &&'c &&'d &i32,
//             data: Data<
//                 'a,
//                 'b,
//                 &&i32,
//                 &&'a &&'b &[&'c &&'b &Data<'c, 'a, &&&'c &i32, Vec<&'d &'b &()>>],
//             >,
//             t1: T1,
//             t1_ref: &T1,
//             xaxbxcxdx_t1_ref: &&'a &&'b &&'c &&'d &T1,
//             t2: T2,
//             t2_ref: &T2,
//             xaxbxcxdx_t2_ref: &&'a &&'b &&'c &&'d &T2,
//         ) -> &&'a &&'a &&'b &&'b &&'c &&'c &&'d &&'d &i32 {
//             Self::work(
//                 self,
//                 a,
//                 b,
//                 c,
//                 d,
//                 axb,
//                 cxd,
//                 abxbax,
//                 cdxdcx,
//                 abcd,
//                 xaxbxcxdx,
//                 data,
//                 t1,
//                 t1_ref,
//                 xaxbxcxdx_t1_ref,
//                 t2,
//                 t2_ref,
//                 xaxbxcxdx_t2_ref,
//             )
//         }
//     }
// }

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    // #[test]
    // fn trait_work_Ok() {
    //     // Arrange
    //     let mock = TraitMock::new();
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
    //                                                 )
    //                                                 .returns(return_value);
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
    //                                             );
    //
    //                                             // Assert
    //                                             assert_eq!(return_value, actual_return_value);
    //
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
    //
    // #[test]
    // fn fn_work_Ok() {
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
    //                                             work::setup(
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
    //                                             )
    //                                             .returns(return_value);
    //
    //                                             // Act
    //                                             let actual_return_value = work(
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
    //                                             );
    //
    //                                             // Assert
    //                                             assert_eq!(return_value, actual_return_value);
    //
    //                                             work::received(
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
    //                                                 data,
    //                                                 t1,
    //                                                 t1_ref,
    //                                                 xaxbxcxdx_t1_ref,
    //                                                 t2,
    //                                                 t2_ref,
    //                                                 xaxbxcxdx_t2_ref,
    //                                                 Times::Once,
    //                                             )
    //                                             .no_other_calls()
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
    //
    // #[test]
    // fn struct_work_Ok() {
    //     // Arrange
    //     let mock = Struct::new();
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
    //                                             );
    //
    //                                             // Assert
    //                                             assert_eq!(return_value, actual_return_value);
    //
    //                                             mock.received.as_Trait.work(
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

    #[test]
    fn compile() {}

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
                                                let xapx = &&(&(&2) as *const _);
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
                                                // mock.setup
                                                //     .as_Trait
                                                //     .work(
                                                //         a,
                                                //         b,
                                                //         c,
                                                //         d,
                                                //         axb,
                                                //         cxd,
                                                //         abxbax,
                                                //         cdxdcx,
                                                //         abcd,
                                                //         xaxbxcxdx,
                                                //         data.clone(),
                                                //         t1,
                                                //         t1_ref,
                                                //         xaxbxcxdx_t1_ref,
                                                //         t2,
                                                //         t2_ref,
                                                //         xaxbxcxdx_t2_ref,
                                                //         xapx,
                                                //     )
                                                //     .call_base();

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

#[cfg(not(test))]
#[allow(unused)]
struct Struct<'a, 'b: 'a, T1: Clone> {
    _phantom_a: PhantomData<&'a ()>,
    _phantom_b: PhantomData<&'b ()>,
    _phantom_t1: PhantomData<T1>,
}
#[cfg(not(test))]
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
#[cfg(not(test))]
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
    impl<'__rs, 'a, 'b: 'a, 'c, 'd: 'a, T1: Clone, T2: Clone> IArgsInfosProvider
        for Trait_work_Call<'__rs, 'a, 'b, 'c, 'd, T1, T2>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![
                ArgInfo::new("a", &self.a, (&ArgPrinter(&"amogus")).debug_string()),
                ArgInfo::new("b", &self.b, (&ArgPrinter(&self.b)).debug_string()),
                ArgInfo::new("c", &self.c, (&ArgPrinter(&self.c)).debug_string()),
                ArgInfo::new("d", &self.d, (&ArgPrinter(&self.d)).debug_string()),
                ArgInfo::new("axb", &self.axb, (&ArgPrinter(&self.axb)).debug_string()),
                ArgInfo::new("cxd", &self.cxd, (&ArgPrinter(&self.cxd)).debug_string()),
                ArgInfo::new(
                    "abxbax",
                    &self.abxbax,
                    (&ArgPrinter(&self.abxbax)).debug_string(),
                ),
                ArgInfo::new(
                    "cdxdcx",
                    &self.cdxdcx,
                    (&ArgPrinter(&self.cdxdcx)).debug_string(),
                ),
                ArgInfo::new("abcd", &self.abcd, (&ArgPrinter(&self.abcd)).debug_string()),
                ArgInfo::new(
                    "xaxbxcxdx",
                    &self.xaxbxcxdx,
                    (&ArgPrinter(&self.xaxbxcxdx)).debug_string(),
                ),
                ArgInfo::new("data", &self.data, (&ArgPrinter(&self.data)).debug_string()),
                ArgInfo::new("t1", &self.t1, (&ArgPrinter(&self.t1)).debug_string()),
                ArgInfo::new(
                    "t1_ref",
                    &self.t1_ref,
                    (&ArgPrinter(&self.t1_ref)).debug_string(),
                ),
                ArgInfo::new(
                    "xaxbxcxdx_t1_ref",
                    &self.xaxbxcxdx_t1_ref,
                    (&ArgPrinter(&self.xaxbxcxdx_t1_ref)).debug_string(),
                ),
                ArgInfo::new("t2", &self.t2, (&ArgPrinter(&self.t2)).debug_string()),
                ArgInfo::new(
                    "t2_ref",
                    &self.t2_ref,
                    (&ArgPrinter(&self.t2_ref)).debug_string(),
                ),
                ArgInfo::new(
                    "xaxbxcxdx_t2_ref",
                    &self.xaxbxcxdx_t2_ref,
                    (&ArgPrinter(&self.xaxbxcxdx_t2_ref)).debug_string(),
                ),
            ]
        }
    }
    #[doc(hidden)]
    #[derive(IArgsTupleProvider, CloneForRSubstitute)]
    #[repr(C)]
    pub struct Trait_work_Call<'__rs, 'a, 'b: 'a, 'c, 'd: 'a, T1: Clone, T2: Clone> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom___rs: PhantomData<&'__rs ()>,
        _phantom_a: PhantomData<&'a ()>,
        _phantom_b: PhantomData<&'b ()>,
        _phantom_T1: PhantomData<T1>,
        _phantom_c: PhantomData<&'c ()>,
        _phantom_d: PhantomData<&'d ()>,
        _phantom_T2: PhantomData<T2>,
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
    impl<'__rs, 'a, 'b: 'a, 'c, 'd: 'a, T1: Clone, T2: Clone> IGenericsInfoProvider
        for Trait_work_Call<'__rs, 'a, 'b, 'c, 'd, T1, T2>
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
    #[derive(Debug, IArgsFormatter)]
    pub struct Trait_work_ArgsChecker<'__rs, 'a, 'b: 'a, 'c, 'd: 'a, T1: Clone, T2: Clone> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom___rs: PhantomData<&'__rs ()>,
        _phantom_a: PhantomData<&'a ()>,
        _phantom_b: PhantomData<&'b ()>,
        _phantom_T1: PhantomData<T1>,
        _phantom_c: PhantomData<&'c ()>,
        _phantom_d: PhantomData<&'d ()>,
        _phantom_T2: PhantomData<T2>,
        a: Arg<'__rs, *const i32>,
        b: Arg<'__rs, *const i32>,
        c: Arg<'__rs, *const i32>,
        d: Arg<'__rs, *const i32>,
        axb: Arg<'__rs, *const *const *const i32>,
        cxd: Arg<'__rs, *const *const *const i32>,
        abxbax: Arg<'__rs, *const *const *const *const *const *const i32>,
        cdxdcx: Arg<'__rs, *const *const *const *const *const *const i32>,
        abcd: Arg<'__rs, *const *const *const *const i32>,
        xaxbxcxdx: Arg<'__rs, *const *const *const *const *const *const *const *const *const i32>,
        data: Arg<
            '__rs,
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
        t1: Arg<'__rs, T1>,
        t1_ref: Arg<'__rs, *const T1>,
        xaxbxcxdx_t1_ref:
            Arg<'__rs, *const *const *const *const *const *const *const *const *const T1>,
        t2: Arg<'__rs, T2>,
        t2_ref: Arg<'__rs, *const T2>,
        xaxbxcxdx_t2_ref:
            Arg<'__rs, *const *const *const *const *const *const *const *const *const T2>,
        xapx: Arg<'__rs, *const *const *const *const i32>,
    }
    impl<'__rs, 'a, 'b: 'a, 'c, 'd: 'a, T1: Clone, T2: Clone> IArgsChecker
        for Trait_work_ArgsChecker<'__rs, 'a, 'b, 'c, 'd, T1, T2>
    {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &Trait_work_Call<'__rs, 'a, 'b, 'c, 'd, T1, T2> = dyn_call.downcast_ref();
            vec![
                self.a
                    .check_ref("a", &call.a, (&ArgPrinter(&&call.a)).debug_string()),
                self.b
                    .check_ref("b", &call.b, (&ArgPrinter(&&call.b)).debug_string()),
                self.c
                    .check_ref("c", &call.c, (&ArgPrinter(&&call.c)).debug_string()),
                self.d
                    .check_ref("d", &call.d, (&ArgPrinter(&&call.d)).debug_string()),
                self.axb
                    .check_ref("axb", &call.axb, (&ArgPrinter(&&call.axb)).debug_string()),
                self.cxd
                    .check_ref("cxd", &call.cxd, (&ArgPrinter(&&call.cxd)).debug_string()),
                self.abxbax.check_ref(
                    "abxbax",
                    &call.abxbax,
                    (&ArgPrinter(&&call.abxbax)).debug_string(),
                ),
                self.cdxdcx.check_ref(
                    "cdxdcx",
                    &call.cdxdcx,
                    (&ArgPrinter(&&call.cdxdcx)).debug_string(),
                ),
                self.abcd.check_ref(
                    "abcd",
                    &call.abcd,
                    (&ArgPrinter(&&call.abcd)).debug_string(),
                ),
                self.xaxbxcxdx.check_ref(
                    "xaxbxcxdx",
                    &call.xaxbxcxdx,
                    (&ArgPrinter(&&call.xaxbxcxdx)).debug_string(),
                ),
                self.data.check(
                    "data",
                    &call.data,
                    (&ArgPrinter(&&call.data)).debug_string(),
                ),
                self.t1
                    .check("t1", &call.t1, (&ArgPrinter(&&call.t1)).debug_string()),
                self.t1_ref.check_ref(
                    "t1_ref",
                    &call.t1_ref,
                    (&ArgPrinter(&&call.t1_ref)).debug_string(),
                ),
                self.xaxbxcxdx_t1_ref.check_ref(
                    "xaxbxcxdx_t1_ref",
                    &call.xaxbxcxdx_t1_ref,
                    (&ArgPrinter(&&call.xaxbxcxdx_t1_ref)).debug_string(),
                ),
                self.t2
                    .check("t2", &call.t2, (&ArgPrinter(&&call.t2)).debug_string()),
                self.t2_ref.check_ref(
                    "t2_ref",
                    &call.t2_ref,
                    (&ArgPrinter(&&call.t2_ref)).debug_string(),
                ),
                self.xaxbxcxdx_t2_ref.check_ref(
                    "xaxbxcxdx_t2_ref",
                    &call.xaxbxcxdx_t2_ref,
                    (&ArgPrinter(&&call.xaxbxcxdx_t2_ref)).debug_string(),
                ),
            ]
        }
    }
    impl<'__rs, 'a, 'b: 'a, 'c, 'd: 'a, T1: Clone, T2: Clone> IGenericsInfoProvider
        for Trait_work_ArgsChecker<'__rs, 'a, 'b, 'c, 'd, T1, T2>
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
    #[derive(CloneForRSubstitute)]
    pub struct TraitSetup<'__rs, 'a, 'b: 'a, T1: Clone> {
        data: Arc<StructData<'__rs, 'a, 'b, T1>>,
    }

    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct TraitReceived<'__rs, 'a, 'b: 'a, T1: Clone> {
        data: Arc<StructData<'__rs, 'a, 'b, T1>>,
    }

    impl<'__rs, 'a, 'b: 'a, T1: Clone> TraitSetup<'__rs, 'a, 'b, T1> {
        pub fn work<'__rsa, 'c, 'd : 'a, T2: Clone>(
            &self, a: impl Into<Arg<'__rsa, &'__rsa i32>>, b: impl Into<Arg<'__rsa, &'__rsa i32>>, c: impl Into<Arg<'__rsa, &'__rsa i32>>, d: impl Into<Arg<'__rsa, &'__rsa i32>>, axb: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa i32>>, cxd: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa i32>>, abxbax: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa i32>>, cdxdcx: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa i32>>, abcd: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa &'__rsa i32>>, xaxbxcxdx: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa i32>>, data: impl Into<Arg<'__rsa, Data<
                '__rsa,
                '__rsa,
                &'__rsa &'__rsa i32,
                &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa [&'__rsa &'__rsa &'__rsa &'__rsa Data<'__rsa, '__rsa, &'__rsa &'__rsa &'__rsa &'__rsa i32, Vec<&'__rsa &'__rsa &'__rsa ()>>],
            >>>, t1: impl Into<Arg<'__rsa, T1>>, t1_ref: impl Into<Arg<'__rsa, &'__rsa T1>>, xaxbxcxdx_t1_ref: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa T1>>, t2: impl Into<Arg<'__rsa, T2>>, t2_ref: impl Into<Arg<'__rsa, &'__rsa T2>>, xaxbxcxdx_t2_ref: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa T2>>, xapx: impl Into<Arg<'__rs, &'__rsa &'__rsa *const &'__rsa i32>>) -> FnTuner<'_, Struct<'__rs, 'a, 'b, T1>, Self, (&'__rs &'a i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'b i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'c i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'d i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'a &'__rs &'b i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'c &'__rs &'d i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'a &'b &'__rs &'b &'a &'__rs i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'c &'d &'__rs &'d &'c &'__rs i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'a &'b &'c &'d i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'__rs &'a &'__rs &'b &'__rs &'c &'__rs &'d &'__rs i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs Data<
                                                                                                                                                                                                                                                                                                                                                                                                                                                                          'a,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                          'b,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                          &'__rs &'__rs i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                          &'__rs &'a &'__rs &'b &'__rs [&'c &'__rs &'b &'__rs Data<'c, 'a, &'__rs &'__rs &'c &'__rs i32, Vec<&'d &'b &'__rs ()>>],
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      >,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs T1,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'__rs T1,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'__rs &'a &'__rs &'b &'__rs &'c &'__rs &'d &'__rs T1,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs T2,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'__rs T2,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'__rs &'a &'__rs &'b &'__rs &'c &'__rs &'d &'__rs T2),
            &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa i32, true, true> where
            T2: '__rsa,
            T1: '__rsa
        {
            let Trait_work_args_checker: Trait_work_ArgsChecker<'_, 'a, 'b, 'c, 'd, T1, T2> =
                Trait_work_ArgsChecker {
                    _phantom_lifetime: PhantomData,
                    _phantom___rs: PhantomData,
                    _phantom_a: PhantomData,
                    _phantom_b: PhantomData,
                    _phantom_T1: PhantomData,
                    _phantom_c: PhantomData,
                    _phantom_d: PhantomData,
                    _phantom_T2: PhantomData,
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
            let fn_tuner: FnTuner<'_, Struct<'__rs, 'a, 'b, T1>, Self, (&'__rs &'a i32,
                                                                        &'__rs &'b i32,
                                                                        &'__rs &'c i32,
                                                                        &'__rs &'d i32,
                                                                        &'__rs &'a &'__rs &'b i32,
                                                                        &'__rs &'c &'__rs &'d i32,
                                                                        &'__rs &'a &'b &'__rs &'b &'a &'__rs i32,
                                                                        &'__rs &'c &'d &'__rs &'d &'c &'__rs i32,
                                                                        &'__rs &'a &'b &'c &'d i32,
                                                                        &'__rs &'__rs &'a &'__rs &'b &'__rs &'c &'__rs &'d &'__rs i32,
                                                                        &'__rs Data<
                                                                            'a,
                                                                            'b,
                                                                            &'__rs &'__rs i32,
                                                                            &'__rs &'a &'__rs &'b &'__rs [&'c &'__rs &'b &'__rs Data<'c, 'a, &'__rs &'__rs &'c &'__rs i32, Vec<&'d &'b &'__rs ()>>],
                                                                        >,
                                                                        &'__rs T1,
                                                                        &'__rs &'__rs T1,
                                                                        &'__rs &'__rs &'a &'__rs &'b &'__rs &'c &'__rs &'d &'__rs T1,
                                                                        &'__rs T2,
                                                                        &'__rs &'__rs T2,
                                                                        &'__rs &'__rs &'a &'__rs &'b &'__rs &'c &'__rs &'d &'__rs T2),
                &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa i32, true, true> = self.data.Trait_work.add_config(Trait_work_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
    }
    impl<'__rs, 'a, 'b: 'a, T1: Clone> TraitReceived<'__rs, 'a, 'b, T1> {
        pub fn work<'__rsa, 'c, 'd: 'a, T2: Clone>(
            &self,
            a: impl Into<Arg<'__rsa, &'__rsa i32>>,
            b: impl Into<Arg<'__rsa, &'__rsa i32>>,
            c: impl Into<Arg<'__rsa, &'__rsa i32>>,
            d: impl Into<Arg<'__rsa, &'__rsa i32>>,
            axb: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa i32>>,
            cxd: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa i32>>,
            abxbax: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa i32>>,
            cdxdcx: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa i32>>,
            abcd: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa &'__rsa i32>>,
            xaxbxcxdx: impl Into<
                Arg<
                    '__rsa,
                    &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa i32,
                >,
            >,
            data: impl Into<Arg<'__rsa, Data<
                '__rsa,
                '__rsa,
                &'__rsa &'__rsa i32,
                &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa [&'__rsa &'__rsa &'__rsa &'__rsa Data<'__rsa, '__rsa, &'__rsa &'__rsa &'__rsa &'__rsa i32, Vec<&'__rsa &'__rsa &'__rsa ()>>],
            >>>,
            t1: impl Into<Arg<'__rsa, T1>>,
            t1_ref: impl Into<Arg<'__rsa, &'__rsa T1>>,
            xaxbxcxdx_t1_ref: impl Into<
                Arg<
                    '__rsa,
                    &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa T1,
                >,
            >,
            t2: impl Into<Arg<'__rsa, T2>>,
            t2_ref: impl Into<Arg<'__rsa, &'__rsa T2>>,
            xaxbxcxdx_t2_ref: impl Into<
                Arg<
                    '__rsa,
                    &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa T2,
                >,
            >,
            xapx: impl Into<Arg<'__rs, &'__rsa &'__rsa *const &'__rsa i32>>,
            times: Times,
        ) -> FnVerifier<
            Self,
            (
                &'__rs &'a i32,
                &'__rs &'b i32,
                &'__rs &'c i32,
                &'__rs &'d i32,
                &'__rs &'a &'__rs &'b i32,
                &'__rs &'c &'__rs &'d i32,
                &'__rs &'a &'b &'__rs &'b &'a &'__rs i32,
                &'__rs &'c &'d &'__rs &'d &'c &'__rs i32,
                &'__rs &'a &'b &'c &'d i32,
                &'__rs &'__rs &'a &'__rs &'b &'__rs &'c &'__rs &'d &'__rs i32,
                &'__rs Data<
                    'a,
                    'b,
                    &'__rs &'__rs i32,
                    &'__rs &'a &'__rs &'b &'__rs [&'c &'__rs &'b &'__rs Data<
                        'c,
                        'a,
                        &'__rs &'__rs &'c &'__rs i32,
                        Vec<&'d &'b &'__rs ()>,
                    >],
                >,
                &'__rs T1,
                &'__rs &'__rs T1,
                &'__rs &'__rs &'a &'__rs &'b &'__rs &'c &'__rs &'d &'__rs T1,
                &'__rs T2,
                &'__rs &'__rs T2,
                &'__rs &'__rs &'a &'__rs &'b &'__rs &'c &'__rs &'d &'__rs T2,
            ),
        >
        where
            T2: '__rsa,
            T1: '__rsa,
        {
            let Trait_work_args_checker: Trait_work_ArgsChecker<'_, 'a, 'b, 'c, 'd, T1, T2> =
                Trait_work_ArgsChecker {
                    _phantom_lifetime: PhantomData,
                    _phantom___rs: PhantomData,
                    _phantom_a: PhantomData,
                    _phantom_b: PhantomData,
                    _phantom_T1: PhantomData,
                    _phantom_c: PhantomData,
                    _phantom_d: PhantomData,
                    _phantom_T2: PhantomData,
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
    }
    impl<'__rs, 'a, 'b: 'a, 'c, 'd: 'a, T1: Clone, T2: Clone> IArgsInfosProvider
        for work_Call<'__rs, 'a, 'b, 'c, 'd, T1, T2>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![
                ArgInfo::new(
                    "a",
                    &self.a,
                    (&ArgPrinter::<&i32>(&transmute_lifetime!(self.a))).debug_string(),
                ),
                ArgInfo::new("b", &self.b, (&ArgPrinter(&self.b)).debug_string()),
                ArgInfo::new("c", &self.c, (&ArgPrinter(&self.c)).debug_string()),
                ArgInfo::new("d", &self.d, (&ArgPrinter(&self.d)).debug_string()),
                ArgInfo::new("axb", &self.axb, (&ArgPrinter(&self.axb)).debug_string()),
                ArgInfo::new("cxd", &self.cxd, (&ArgPrinter(&self.cxd)).debug_string()),
                ArgInfo::new(
                    "abxbax",
                    &self.abxbax,
                    (&ArgPrinter(&self.abxbax)).debug_string(),
                ),
                ArgInfo::new(
                    "cdxdcx",
                    &self.cdxdcx,
                    (&ArgPrinter(&self.cdxdcx)).debug_string(),
                ),
                ArgInfo::new("abcd", &self.abcd, (&ArgPrinter(&self.abcd)).debug_string()),
                ArgInfo::new(
                    "xaxbxcxdx",
                    &self.xaxbxcxdx,
                    (&ArgPrinter(&self.xaxbxcxdx)).debug_string(),
                ),
                ArgInfo::new("data", &self.data, (&ArgPrinter(&self.data)).debug_string()),
                ArgInfo::new("t1", &self.t1, (&ArgPrinter(&self.t1)).debug_string()),
                ArgInfo::new(
                    "t1_ref",
                    &self.t1_ref,
                    (&ArgPrinter(&self.t1_ref)).debug_string(),
                ),
                ArgInfo::new(
                    "xaxbxcxdx_t1_ref",
                    &self.xaxbxcxdx_t1_ref,
                    (&ArgPrinter(&self.xaxbxcxdx_t1_ref)).debug_string(),
                ),
                ArgInfo::new("t2", &self.t2, (&ArgPrinter(&self.t2)).debug_string()),
                ArgInfo::new(
                    "t2_ref",
                    &self.t2_ref,
                    (&ArgPrinter(&self.t2_ref)).debug_string(),
                ),
                ArgInfo::new(
                    "xaxbxcxdx_t2_ref",
                    &self.xaxbxcxdx_t2_ref,
                    (&ArgPrinter(&self.xaxbxcxdx_t2_ref)).debug_string(),
                ),
            ]
        }
    }
    #[doc(hidden)]
    #[derive(IArgsTupleProvider, CloneForRSubstitute)]
    #[repr(C)]
    pub struct work_Call<'__rs, 'a, 'b: 'a, 'c, 'd: 'a, T1: Clone, T2: Clone> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom___rs: PhantomData<&'__rs ()>,
        _phantom_a: PhantomData<&'a ()>,
        _phantom_b: PhantomData<&'b ()>,
        _phantom_T1: PhantomData<T1>,
        _phantom_c: PhantomData<&'c ()>,
        _phantom_d: PhantomData<&'d ()>,
        _phantom_T2: PhantomData<T2>,
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
    }
    impl<'__rs, 'a, 'b: 'a, 'c, 'd: 'a, T1: Clone, T2: Clone> IGenericsInfoProvider
        for work_Call<'__rs, 'a, 'b, 'c, 'd, T1, T2>
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
    #[derive(Debug, IArgsFormatter)]
    pub struct work_ArgsChecker<'__rs, 'a, 'b: 'a, 'c, 'd: 'a, T1: Clone, T2: Clone> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom___rs: PhantomData<&'__rs ()>,
        _phantom_a: PhantomData<&'a ()>,
        _phantom_b: PhantomData<&'b ()>,
        _phantom_T1: PhantomData<T1>,
        _phantom_c: PhantomData<&'c ()>,
        _phantom_d: PhantomData<&'d ()>,
        _phantom_T2: PhantomData<T2>,
        a: Arg<'__rs, *const i32>,
        b: Arg<'__rs, *const i32>,
        c: Arg<'__rs, *const i32>,
        d: Arg<'__rs, *const i32>,
        axb: Arg<'__rs, *const *const *const i32>,
        cxd: Arg<'__rs, *const *const *const i32>,
        abxbax: Arg<'__rs, *const *const *const *const *const *const i32>,
        cdxdcx: Arg<'__rs, *const *const *const *const *const *const i32>,
        abcd: Arg<'__rs, *const *const *const *const i32>,
        xaxbxcxdx: Arg<'__rs, *const *const *const *const *const *const *const *const *const i32>,
        data: Arg<
            '__rs,
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
        t1: Arg<'__rs, T1>,
        t1_ref: Arg<'__rs, *const T1>,
        xaxbxcxdx_t1_ref:
            Arg<'__rs, *const *const *const *const *const *const *const *const *const T1>,
        t2: Arg<'__rs, T2>,
        t2_ref: Arg<'__rs, *const T2>,
        xaxbxcxdx_t2_ref:
            Arg<'__rs, *const *const *const *const *const *const *const *const *const T2>,
        xapx: Arg<'__rs, *const *const *const *const i32>,
    }
    impl<'__rs, 'a, 'b: 'a, 'c, 'd: 'a, T1: Clone, T2: Clone> IArgsChecker
        for work_ArgsChecker<'__rs, 'a, 'b, 'c, 'd, T1, T2>
    {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &work_Call<'__rs, 'a, 'b, 'c, 'd, T1, T2> = dyn_call.downcast_ref();
            vec![
                self.a
                    .check_ref("a", &call.a, (&ArgPrinter(&&call.a)).debug_string()),
                self.b
                    .check_ref("b", &call.b, (&ArgPrinter(&&call.b)).debug_string()),
                self.c
                    .check_ref("c", &call.c, (&ArgPrinter(&&call.c)).debug_string()),
                self.d
                    .check_ref("d", &call.d, (&ArgPrinter(&&call.d)).debug_string()),
                self.axb
                    .check_ref("axb", &call.axb, (&ArgPrinter(&&call.axb)).debug_string()),
                self.cxd
                    .check_ref("cxd", &call.cxd, (&ArgPrinter(&&call.cxd)).debug_string()),
                self.abxbax.check_ref(
                    "abxbax",
                    &call.abxbax,
                    (&ArgPrinter(&&call.abxbax)).debug_string(),
                ),
                self.cdxdcx.check_ref(
                    "cdxdcx",
                    &call.cdxdcx,
                    (&ArgPrinter(&&call.cdxdcx)).debug_string(),
                ),
                self.abcd.check_ref(
                    "abcd",
                    &call.abcd,
                    (&ArgPrinter(&&call.abcd)).debug_string(),
                ),
                self.xaxbxcxdx.check_ref(
                    "xaxbxcxdx",
                    &call.xaxbxcxdx,
                    (&ArgPrinter(&&call.xaxbxcxdx)).debug_string(),
                ),
                self.data.check(
                    "data",
                    &call.data,
                    (&ArgPrinter(&&call.data)).debug_string(),
                ),
                self.t1
                    .check("t1", &call.t1, (&ArgPrinter(&&call.t1)).debug_string()),
                self.t1_ref.check_ref(
                    "t1_ref",
                    &call.t1_ref,
                    (&ArgPrinter(&&call.t1_ref)).debug_string(),
                ),
                self.xaxbxcxdx_t1_ref.check_ref(
                    "xaxbxcxdx_t1_ref",
                    &call.xaxbxcxdx_t1_ref,
                    (&ArgPrinter(&&call.xaxbxcxdx_t1_ref)).debug_string(),
                ),
                self.t2
                    .check("t2", &call.t2, (&ArgPrinter(&&call.t2)).debug_string()),
                self.t2_ref.check_ref(
                    "t2_ref",
                    &call.t2_ref,
                    (&ArgPrinter(&&call.t2_ref)).debug_string(),
                ),
                self.xaxbxcxdx_t2_ref.check_ref(
                    "xaxbxcxdx_t2_ref",
                    &call.xaxbxcxdx_t2_ref,
                    (&ArgPrinter(&&call.xaxbxcxdx_t2_ref)).debug_string(),
                ),
            ]
        }
    }
    impl<'__rs, 'a, 'b: 'a, 'c, 'd: 'a, T1: Clone, T2: Clone> IGenericsInfoProvider
        for work_ArgsChecker<'__rs, 'a, 'b, 'c, 'd, T1, T2>
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
    pub struct StructData<'__rs, 'a, 'b: 'a, T1: Clone> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom___rs: PhantomData<&'__rs ()>,
        _phantom_a: PhantomData<&'a ()>,
        _phantom_b: PhantomData<&'b ()>,
        _phantom_T1: PhantomData<T1>,
        pub work: FnData<'static, Struct<'__rs, 'a, 'b, T1>, true, true>,
        pub Trait_work: FnData<'static, Struct<'__rs, 'a, 'b, T1>, true, true>,
    }

    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct StructSetup<'__rs, 'a, 'b: 'a, T1: Clone> {
        data: Arc<StructData<'__rs, 'a, 'b, T1>>,
        pub as_Trait: TraitSetup<'__rs, 'a, 'b, T1>,
    }

    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct StructReceived<'__rs, 'a, 'b: 'a, T1: Clone> {
        data: Arc<StructData<'__rs, 'a, 'b, T1>>,
        pub as_Trait: TraitReceived<'__rs, 'a, 'b, T1>,
    }

    #[allow(unused)]
    #[doc(hidden)]
    pub struct Struct_InnerData<'a, 'b: 'a, T1: Clone> {
        _phantom_a: PhantomData<&'a ()>,
        _phantom_b: PhantomData<&'b ()>,
        _phantom_t1: PhantomData<T1>,
    }

    impl<'a, 'b: 'a, T1: Clone> Struct_InnerData<'a, 'b, T1> {
        pub fn new() -> Self {
            Self {
                _phantom_a: PhantomData,
                _phantom_b: PhantomData,
                _phantom_t1: PhantomData,
            }
        }
    }
    #[allow(unused)]
    pub struct Struct<'__rs, 'a, 'b: 'a, T1: Clone> {
        pub setup: StructSetup<'__rs, 'a, 'b, T1>,
        pub received: StructReceived<'__rs, 'a, 'b, T1>,
        pub data: Arc<StructData<'__rs, 'a, 'b, T1>>,
        inner_data: Struct_InnerData<'a, 'b, T1>,
    }

    impl<'__rs, 'a, 'b: 'a, T1: Clone> Deref for Struct<'__rs, 'a, 'b, T1> {
        type Target = Struct_InnerData<'a, 'b, T1>;
        fn deref(&self) -> &Self::Target {
            &self.inner_data
        }
    }
    impl<'__rs, 'a, 'b: 'a, T1: Clone> Trait<'a, 'b, T1> for Struct<'__rs, 'a, 'b, T1> {
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
            let call: Trait_work_Call<'_, '_, '_, '_, '_, T1, T2> = Trait_work_Call {
                _phantom_lifetime: PhantomData,
                _phantom___rs: PhantomData,
                _phantom_a: PhantomData,
                _phantom_b: PhantomData,
                _phantom_T1: PhantomData,
                _phantom_c: PhantomData,
                _phantom_d: PhantomData,
                _phantom_T2: PhantomData,
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
            return self
                .data
                .Trait_work
                .handle_base_returning(self, call, Self::base_Trait_work);
        }
    }
    #[allow(unused)]
    impl<'__rs, 'a, 'b: 'a, T1: Clone> Struct<'__rs, 'a, 'b, T1> {
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
            let call: work_Call<'_, '_, '_, '_, '_, T1, T2> = work_Call {
                _phantom_lifetime: PhantomData,
                _phantom___rs: PhantomData,
                _phantom_a: PhantomData,
                _phantom_b: PhantomData,
                _phantom_T1: PhantomData,
                _phantom_c: PhantomData,
                _phantom_d: PhantomData,
                _phantom_T2: PhantomData,
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
            };
            return self
                .data
                .work
                .handle_base_returning(self, call, Self::base_work);
        }
    }
    impl<'__rs, 'a, 'b: 'a, T1: Clone> Struct<'__rs, 'a, 'b, T1> {
        pub fn new() -> Self {
            let data = Arc::new(StructData {
                _phantom_lifetime: PhantomData,
                _phantom___rs: PhantomData,
                _phantom_a: PhantomData,
                _phantom_b: PhantomData,
                _phantom_T1: PhantomData,
                work: FnData::new("work"),
                Trait_work: FnData::new("Trait::work"),
            });
            let inner_data = Struct_InnerData::new();
            return Struct {
                setup: StructSetup {
                    data: data.clone(),
                    as_Trait: TraitSetup { data: data.clone() },
                },
                received: StructReceived {
                    data: data.clone(),
                    as_Trait: TraitReceived { data: data.clone() },
                },
                data,
                inner_data,
            };
        }
        fn base_work<'q, 'c, 'd: 'a, T2: Clone>(
            &self,
            call: work_Call<'__rs, 'a, 'b, 'c, 'd, T1, T2>,
        ) -> &&'a &&'a &&'b &&'b &&'c &&'c &&'d &&'d &i32
        where
            T1: 'q,
            T2: 'q, {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)]
            let work_Call::<'_, '_, '_, '_, '_, T1, T2> {
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
                ..
            } = call;
            unreachable!()
        }
        fn base_Trait_work<'q, 'c, 'd: 'a, T2: Clone>(
            &'q self,
            call: Trait_work_Call<'__rs, 'a, 'b, 'c, 'd, T1, T2>,
        ) -> &&'a &&'a &&'b &&'b &&'c &&'c &&'d &&'d &i32
        where
            T1: 'q,
            T2: 'q,
        {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)]
            let Trait_work_Call::<'_, '_, '_, '_, '_, T1, T2> {
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
            Self::work::<T2>(
                self,
                transmute_lifetime!(a),
                transmute_lifetime!(b),
                transmute_lifetime!(c),
                transmute_lifetime!(d),
                transmute_lifetime!(axb),
                transmute_lifetime!(cxd),
                transmute_lifetime!(abxbax),
                transmute_lifetime!(cdxdcx),
                transmute_lifetime!(abcd),
                transmute_lifetime!(xaxbxcxdx),
                transmute_lifetime!(data),
                transmute_lifetime!(t1),
                transmute_lifetime!(t1_ref),
                transmute_lifetime!(xaxbxcxdx_t1_ref),
                transmute_lifetime!(t2),
                transmute_lifetime!(t2_ref),
                transmute_lifetime!(xaxbxcxdx_t2_ref),
                transmute_lifetime!(xapx),
            )
        }
    }
    impl<'__rs, 'a, 'b: 'a, T1: Clone> StructSetup<'__rs, 'a, 'b, T1> {
        pub fn work<'__rsa, 'c, 'd : 'a, T2: Clone>(
            &self, a: impl Into<Arg<'__rsa, &'__rsa i32>>, b: impl Into<Arg<'__rsa, &'__rsa i32>>, c: impl Into<Arg<'__rsa, &'__rsa i32>>, d: impl Into<Arg<'__rsa, &'__rsa i32>>, axb: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa i32>>, cxd: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa i32>>, abxbax: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa i32>>, cdxdcx: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa i32>>, abcd: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa &'__rsa i32>>, xaxbxcxdx: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa i32>>, data: impl Into<Arg<'__rsa, Data<
                '__rsa,
                '__rsa,
                &'__rsa &'__rsa i32,
                &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa [&'__rsa &'__rsa &'__rsa &'__rsa Data<'__rsa, '__rsa, &'__rsa &'__rsa &'__rsa &'__rsa i32, Vec<&'__rsa &'__rsa &'__rsa ()>>],
            >>>, t1: impl Into<Arg<'__rsa, T1>>, t1_ref: impl Into<Arg<'__rsa, &'__rsa T1>>, xaxbxcxdx_t1_ref: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa T1>>, t2: impl Into<Arg<'__rsa, T2>>, t2_ref: impl Into<Arg<'__rsa, &'__rsa T2>>, xaxbxcxdx_t2_ref: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa T2>>, xapx: impl Into<Arg<'__rs, &'__rsa &'__rsa *const &'__rsa i32>>) -> FnTuner<'_, Struct<'__rs, 'a, 'b, T1>, Self, (&'__rs &'a i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'b i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'c i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'d i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'a &'__rs &'b i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'c &'__rs &'d i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'a &'b &'__rs &'b &'a &'__rs i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'c &'d &'__rs &'d &'c &'__rs i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'a &'b &'c &'d i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'__rs &'a &'__rs &'b &'__rs &'c &'__rs &'d &'__rs i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs Data<
                                                                                                                                                                                                                                                                                                                                                                                                                                                                          'a,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                          'b,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                          &'__rs &'__rs i32,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                          &'__rs &'a &'__rs &'b &'__rs [&'c &'__rs &'b &'__rs Data<'c, 'a, &'__rs &'__rs &'c &'__rs i32, Vec<&'d &'b &'__rs ()>>],
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      >,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs T1,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'__rs T1,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'__rs &'a &'__rs &'b &'__rs &'c &'__rs &'d &'__rs T1,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs T2,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'__rs T2,
                                                                                                                                                                                                                                                                                                                                                                                                                                                                      &'__rs &'__rs &'a &'__rs &'b &'__rs &'c &'__rs &'d &'__rs T2),
            &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa i32, true, true> where
            T1: '__rsa,
            T2: '__rsa
        {
            let work_args_checker: work_ArgsChecker<'_, 'a, 'b, 'c, 'd, T1, T2> =
                work_ArgsChecker {
                    _phantom_lifetime: PhantomData,
                    _phantom___rs: PhantomData,
                    _phantom_a: PhantomData,
                    _phantom_b: PhantomData,
                    _phantom_T1: PhantomData,
                    _phantom_c: PhantomData,
                    _phantom_d: PhantomData,
                    _phantom_T2: PhantomData,
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
            let fn_tuner: FnTuner<'_, Struct<'__rs, 'a, 'b, T1>, Self, (&'__rs &'a i32,
                                                                        &'__rs &'b i32,
                                                                        &'__rs &'c i32,
                                                                        &'__rs &'d i32,
                                                                        &'__rs &'a &'__rs &'b i32,
                                                                        &'__rs &'c &'__rs &'d i32,
                                                                        &'__rs &'a &'b &'__rs &'b &'a &'__rs i32,
                                                                        &'__rs &'c &'d &'__rs &'d &'c &'__rs i32,
                                                                        &'__rs &'a &'b &'c &'d i32,
                                                                        &'__rs &'__rs &'a &'__rs &'b &'__rs &'c &'__rs &'d &'__rs i32,
                                                                        &'__rs Data<
                                                                            'a,
                                                                            'b,
                                                                            &'__rs &'__rs i32,
                                                                            &'__rs &'a &'__rs &'b &'__rs [&'c &'__rs &'b &'__rs Data<'c, 'a, &'__rs &'__rs &'c &'__rs i32, Vec<&'d &'b &'__rs ()>>],
                                                                        >,
                                                                        &'__rs T1,
                                                                        &'__rs &'__rs T1,
                                                                        &'__rs &'__rs &'a &'__rs &'b &'__rs &'c &'__rs &'d &'__rs T1,
                                                                        &'__rs T2,
                                                                        &'__rs &'__rs T2,
                                                                        &'__rs &'__rs &'a &'__rs &'b &'__rs &'c &'__rs &'d &'__rs T2),
                &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa i32, true, true> = self.data.work.add_config(work_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
    }
    impl<'__rs, 'a, 'b: 'a, T1: Clone> StructReceived<'__rs, 'a, 'b, T1> {
        pub fn work<'__rsa, 'c, 'd: 'a, T2: Clone>(
            &self,
            a: impl Into<Arg<'__rsa, &'__rsa i32>>,
            b: impl Into<Arg<'__rsa, &'__rsa i32>>,
            c: impl Into<Arg<'__rsa, &'__rsa i32>>,
            d: impl Into<Arg<'__rsa, &'__rsa i32>>,
            axb: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa i32>>,
            cxd: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa i32>>,
            abxbax: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa i32>>,
            cdxdcx: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa i32>>,
            abcd: impl Into<Arg<'__rsa, &'__rsa &'__rsa &'__rsa &'__rsa i32>>,
            xaxbxcxdx: impl Into<
                Arg<
                    '__rsa,
                    &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa i32,
                >,
            >,
            data: impl Into<Arg<'__rsa, Data<
                '__rsa,
                '__rsa,
                &'__rsa &'__rsa i32,
                &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa [&'__rsa &'__rsa &'__rsa &'__rsa Data<'__rsa, '__rsa, &'__rsa &'__rsa &'__rsa &'__rsa i32, Vec<&'__rsa &'__rsa &'__rsa ()>>],
            >>>,
            t1: impl Into<Arg<'__rsa, T1>>,
            t1_ref: impl Into<Arg<'__rsa, &'__rsa T1>>,
            xaxbxcxdx_t1_ref: impl Into<
                Arg<
                    '__rsa,
                    &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa T1,
                >,
            >,
            t2: impl Into<Arg<'__rsa, T2>>,
            t2_ref: impl Into<Arg<'__rsa, &'__rsa T2>>,
            xaxbxcxdx_t2_ref: impl Into<
                Arg<
                    '__rsa,
                    &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa &'__rsa T2,
                >,
            >,
            xapx: impl Into<Arg<'__rs, &'__rsa &'__rsa *const &'__rsa i32>>,
            times: Times,
        ) -> FnVerifier<
            Self,
            (
                &'__rs &'a i32,
                &'__rs &'b i32,
                &'__rs &'c i32,
                &'__rs &'d i32,
                &'__rs &'a &'__rs &'b i32,
                &'__rs &'c &'__rs &'d i32,
                &'__rs &'a &'b &'__rs &'b &'a &'__rs i32,
                &'__rs &'c &'d &'__rs &'d &'c &'__rs i32,
                &'__rs &'a &'b &'c &'d i32,
                &'__rs &'__rs &'a &'__rs &'b &'__rs &'c &'__rs &'d &'__rs i32,
                &'__rs Data<
                    'a,
                    'b,
                    &'__rs &'__rs i32,
                    &'__rs &'a &'__rs &'b &'__rs [&'c &'__rs &'b &'__rs Data<
                        'c,
                        'a,
                        &'__rs &'__rs &'c &'__rs i32,
                        Vec<&'d &'b &'__rs ()>,
                    >],
                >,
                &'__rs T1,
                &'__rs &'__rs T1,
                &'__rs &'__rs &'a &'__rs &'b &'__rs &'c &'__rs &'d &'__rs T1,
                &'__rs T2,
                &'__rs &'__rs T2,
                &'__rs &'__rs &'a &'__rs &'b &'__rs &'c &'__rs &'d &'__rs T2,
            ),
        >
        where
            T2: '__rsa,
            T1: '__rsa,
        {
            let work_args_checker: work_ArgsChecker<'_, 'a, 'b, 'c, 'd, T1, T2> =
                work_ArgsChecker {
                    _phantom_lifetime: PhantomData,
                    _phantom___rs: PhantomData,
                    _phantom_a: PhantomData,
                    _phantom_b: PhantomData,
                    _phantom_T1: PhantomData,
                    _phantom_c: PhantomData,
                    _phantom_d: PhantomData,
                    _phantom_T2: PhantomData,
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
            self.data.work.verify_received(work_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
}
