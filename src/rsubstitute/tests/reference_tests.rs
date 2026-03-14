use rsubstitute::macros::mock;
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
trait Trait<'a, 'b: 'a> {
    fn work<'s, 'c, 'd: 'a>(
        &'s self,
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
    ) -> &'a &'a &'b &'b &'c &'c &'d &'d i32;
}

// #[mock]
// fn work<'a, 'b: 'a, 'c, 'd: 'a>(
//     a: &'a i32,
//     b: &'b i32,
//     c: &'c i32,
//     d: &'d i32,
//     axb: &'a &&'b i32,
//     cxd: &'c &&'d i32,
//     abxbax: &'a &'b &&'b &'a &i32,
//     cdxdcx: &'c &'d &&'d &'c &i32,
//     abcd: &'a &'b &'c &'d i32,
//     xaxbxcxdx: &&'a &&'b &&'c &&'d &i32,
//     data: Data<'a, 'b, &&i32, &&'a &&'b &[&'c &&'b &Data<'c, 'a, &&&'c &i32, Vec<&'d &'b &()>>]>,
// ) {
// }

// mocked_base! {
//     struct Struct<'a, 'b: 'a> {
//         _phantom_a: PhantomData<&'a ()>,
//         _phantom_b: PhantomData<&'b ()>,
//     }
//
//     impl<'a, 'b: 'a> Struct<'a, 'b> {
//         pub fn new() -> Self {
//             Self {
//                 _phantom_a: PhantomData,
//                 _phantom_b: PhantomData,
//             }
//         }
//
//         fn work<'c, 'd: 'a>(
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
//         ) -> &&'a&&'a&&'b&&'b&&'c&&'c&&'d&&'d&i32 {
//             unreachable!()
//         }
//     }
//
//     impl<'a, 'b: 'a> Trait<'a, 'b> for Struct<'a, 'b> {
//         fn work<'c, 'd: 'a>(
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
//         )  -> &&'a&&'a&&'b&&'b&&'c&&'c&&'d&&'d&i32{
//             Self::work(self, a, b, c, d, axb, cxd, abxbax, cdxdcx, abcd, xaxbxcxdx, data)
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use rsubstitute_core::Times;

    #[test]
    fn trait_work_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let return_value = &&&&&&&&55;
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
