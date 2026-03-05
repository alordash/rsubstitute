// use rsubstitute::macros::mock;
// 
// #[derive(Clone, Debug, PartialOrd, PartialEq)]
// struct Foo {
//     pub number: Vec<i32>,
// }
// #[cfg(not(test))]
// trait Trait<'a, 'b> {
//     fn accept_ref(&self, r: &'a &'b &'a &i32) -> &'b &'a &'b &'a i32;
// 
//     fn return_mut_ref(&self) -> &'a mut i32;
// 
//     fn return_mut_ref_with_base(&self) -> &'a mut i32 {
//         todo!()
//     }
// 
//     fn fooo(&mut self, Foo { mut number }: Foo) {
//         println!("number: {number:?}")
//     }
// }
// #[cfg(test)]
// trait Trait<'a, 'b> {
//     fn accept_ref(&self, r: &'a &'b &'a &i32) -> &'b &'a &'b &'a i32;
// 
//     fn return_mut_ref(&self) -> &'a mut i32;
// 
//     fn return_mut_ref_with_base(&self) -> &'a mut i32 {
//         todo!()
//     }
// 
//     fn fooo(&mut self, Foo { mut number }: Foo) {
//         println!("number: {number:?}")
//     }
// }
// #[cfg(test)]
// pub use __rsubstitute_generated_Trait::*;
// #[cfg(test)]
// #[allow(unused_parens)]
// #[allow(non_snake_case)]
// #[allow(non_camel_case_types)]
// #[allow(mismatched_lifetime_syntaxes)]
// mod __rsubstitute_generated_Trait {
//     use super::*;
//     use rsubstitute::for_generated::*;
//     #[doc(hidden)]
//     #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider)]
//     pub struct accept_ref_Call<'rs, 'a, 'b> {
//         _phantom_lifetime: PhantomData<&'rs ()>,
//         _phantom_a: PhantomData<&'a ()>,
//         _phantom_b: PhantomData<&'b ()>,
//         r: &'a &'b &'a &'rs i32,
//     }
//     #[doc(hidden)]
//     #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
//     pub struct accept_ref_ArgsChecker<'rs, 'a, 'b> {
//         _phantom_lifetime: PhantomData<&'rs ()>,
//         _phantom_a: PhantomData<&'a ()>,
//         _phantom_b: PhantomData<&'b ()>,
//         r: Arg<'rs, &'a &'b &'a &'rs i32>,
//     }
//     impl<'rs, 'a, 'b> IArgsChecker for accept_ref_ArgsChecker<'rs, 'a, 'b> {
//         fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
//             let call: &accept_ref_Call<'rs, 'a, 'b> = dyn_call.downcast_ref();
//             vec![self.r.check_ref("r", &call.r)]
//         }
//     }
//     #[doc(hidden)]
//     #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider)]
//     pub struct return_mut_ref_Call<'rs, 'a, 'b> {
//         _phantom_lifetime: PhantomData<&'rs ()>,
//         _phantom_a: PhantomData<&'a ()>,
//         _phantom_b: PhantomData<&'b ()>,
//     }
//     #[doc(hidden)]
//     #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
//     pub struct return_mut_ref_ArgsChecker<'rs, 'a, 'b> {
//         _phantom_lifetime: PhantomData<&'rs ()>,
//         _phantom_a: PhantomData<&'a ()>,
//         _phantom_b: PhantomData<&'b ()>,
//     }
//     impl<'rs, 'a, 'b> IArgsChecker for return_mut_ref_ArgsChecker<'rs, 'a, 'b> {
//         fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
//             let call: &return_mut_ref_Call<'rs, 'a, 'b> = dyn_call.downcast_ref();
//             vec![]
//         }
//     }
//     #[doc(hidden)]
//     #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider)]
//     pub struct return_mut_ref_with_base_Call<'rs, 'a, 'b> {
//         _phantom_lifetime: PhantomData<&'rs ()>,
//         _phantom_a: PhantomData<&'a ()>,
//         _phantom_b: PhantomData<&'b ()>,
//     }
//     #[doc(hidden)]
//     #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
//     pub struct return_mut_ref_with_base_ArgsChecker<'rs, 'a, 'b> {
//         _phantom_lifetime: PhantomData<&'rs ()>,
//         _phantom_a: PhantomData<&'a ()>,
//         _phantom_b: PhantomData<&'b ()>,
//     }
//     impl<'rs, 'a, 'b> IArgsChecker for return_mut_ref_with_base_ArgsChecker<'rs, 'a, 'b> {
//         fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
//             let call: &return_mut_ref_with_base_Call<'rs, 'a, 'b> = dyn_call.downcast_ref();
//             vec![]
//         }
//     }
//     #[doc(hidden)]
//     #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider)]
//     pub struct fooo_Call<'rs, 'a, 'b> {
//         _phantom_lifetime: PhantomData<&'rs ()>,
//         _phantom_a: PhantomData<&'a ()>,
//         _phantom_b: PhantomData<&'b ()>,
//         arg_1: Foo,
//     }
//     #[doc(hidden)]
//     #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
//     pub struct fooo_ArgsChecker<'rs, 'a, 'b> {
//         _phantom_lifetime: PhantomData<&'rs ()>,
//         _phantom_a: PhantomData<&'a ()>,
//         _phantom_b: PhantomData<&'b ()>,
//         arg_1: Arg<'rs, Foo>,
//     }
//     impl<'rs, 'a, 'b> IArgsChecker for fooo_ArgsChecker<'rs, 'a, 'b> {
//         fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
//             let call: &fooo_Call<'rs, 'a, 'b> = dyn_call.downcast_ref();
//             vec![self.arg_1.check("arg_1", &call.arg_1)]
//         }
//     }
//     #[doc(hidden)]
//     #[derive(IMockData)]
//     pub struct TraitMockData<'rs, 'a, 'b> {
//         _phantom_a: PhantomData<&'a ()>,
//         _phantom_b: PhantomData<&'b ()>,
//         accept_ref_data: FnData<'rs, TraitMock<'rs, 'a, 'b>, false>,
//         return_mut_ref_data: FnData<'rs, TraitMock<'rs, 'a, 'b>, false>,
//         return_mut_ref_with_base_data: FnData<'rs, TraitMock<'rs, 'a, 'b>, false>,
//         fooo_data: FnData<'rs, TraitMock<'rs, 'a, 'b>, false>,
//     }
//     #[doc(hidden)]
//     #[derive(Clone)]
//     pub struct TraitMockSetup<'rs, 'a, 'b> {
//         data: Arc<TraitMockData<'rs, 'a, 'b>>,
//     }
//     #[doc(hidden)]
//     #[derive(Clone)]
//     pub struct TraitMockReceived<'rs, 'a, 'b> {
//         data: Arc<TraitMockData<'rs, 'a, 'b>>,
//     }
//     #[derive(Clone)]
//     pub struct TraitMock<'rs, 'a, 'b> {
//         pub setup: TraitMockSetup<'rs, 'a, 'b>,
//         pub received: TraitMockReceived<'rs, 'a, 'b>,
//         data: Arc<TraitMockData<'rs, 'a, 'b>>,
//     }
//     impl<'rs, 'a, 'b> Trait<'a, 'b> for TraitMock<'rs, 'a, 'b> {
//         fn accept_ref(&self, r: &'a &'b &'a &i32) -> &'b &'a &'b &'a i32 {
//             let call: accept_ref_Call<'_, 'a, 'b> = unsafe {
//                 accept_ref_Call {
//                     _phantom_lifetime: PhantomData,
//                     _phantom_a: PhantomData,
//                     _phantom_b: PhantomData,
//                     r: core::mem::transmute(r),
//                 }
//             };
//             return self.data.accept_ref_data.handle_returning(call);
//         }
//         fn return_mut_ref(&self) -> &'a mut i32 {
//             let call: return_mut_ref_Call<'rs, 'a, 'b> = unsafe {
//                 return_mut_ref_Call {
//                     _phantom_lifetime: PhantomData,
//                     _phantom_a: PhantomData,
//                     _phantom_b: PhantomData,
//                 }
//             };
//             return self.data.return_mut_ref_data.handle_returning(call);
//         }
//         fn return_mut_ref_with_base(&self) -> &'a mut i32 {
//             let call: return_mut_ref_with_base_Call<'rs, 'a, 'b> = unsafe {
//                 return_mut_ref_with_base_Call {
//                     _phantom_lifetime: PhantomData,
//                     _phantom_a: PhantomData,
//                     _phantom_b: PhantomData,
//                 }
//             };
//             return self
//                 .data
//                 .return_mut_ref_with_base_data
//                 .handle_returning(call);
//         }
// 
//         fn fooo(&mut self, arg_1: Foo) {
//             let call: fooo_Call<'rs, 'a, 'b> = unsafe {
//                 fooo_Call {
//                     _phantom_lifetime: PhantomData,
//                     _phantom_a: PhantomData,
//                     _phantom_b: PhantomData,
//                     arg_1: core::mem::transmute(arg_1),
//                 }
//             };
//             self.data.fooo_data.handle(call);
//         }
//     }
//     impl<'rs, 'a, 'b> TraitMock<'rs, 'a, 'b> {
//         pub fn new() -> Self {
//             let data = Arc::new(TraitMockData {
//                 _phantom_a: PhantomData,
//                 _phantom_b: PhantomData,
//                 accept_ref_data: FnData::new("accept_ref"),
//                 return_mut_ref_data: FnData::new("return_mut_ref"),
//                 return_mut_ref_with_base_data: FnData::new("return_mut_ref_with_base"),
//                 fooo_data: FnData::new("fooo"),
//             });
//             return TraitMock {
//                 setup: TraitMockSetup { data: data.clone() },
//                 received: TraitMockReceived { data: data.clone() },
//                 data,
//             };
//         }
//         pub fn setup(&self) -> &'rs TraitMockSetup<'rs, 'a, 'b> {
//             unsafe { core::mem::transmute(&self.setup) }
//         }
//         pub fn received(&self) -> &'rs TraitMockReceived<'rs, 'a, 'b> {
//             unsafe { core::mem::transmute(&self.received) }
//         }
//     }
//     impl<'rs, 'a, 'b> TraitMockSetup<'rs, 'a, 'b> {
//         pub fn accept_ref(
//             &self,
//             r: impl Into<Arg<'rs, &'a &'b &'a &'rs i32>>,
//         ) -> FnTuner<'rs, Self, (&&'a &'b &'a &i32,), &'b &'a &'b &'a i32, false> {
//             let accept_ref_args_checker: accept_ref_ArgsChecker<'rs, 'a, 'b> =
//                 accept_ref_ArgsChecker {
//                     _phantom_lifetime: PhantomData,
//                     _phantom_a: PhantomData,
//                     _phantom_b: PhantomData,
//                     r: r.into(),
//                 };
//             let fn_tuner: FnTuner<'_, Self, (&&'a &'b &'a &i32,), &'b &'a &'b &'a i32, false> =
//                 self.data
//                     .accept_ref_data
//                     .add_config(accept_ref_args_checker, self);
//             return unsafe { core::mem::transmute(fn_tuner) };
//         }
//         pub fn return_mut_ref(&self) -> FnTuner<'rs, Self, (), &'a mut i32, false> {
//             let return_mut_ref_args_checker: return_mut_ref_ArgsChecker<'rs, 'a, 'b> =
//                 return_mut_ref_ArgsChecker {
//                     _phantom_lifetime: PhantomData,
//                     _phantom_a: PhantomData,
//                     _phantom_b: PhantomData,
//                 };
//             let fn_tuner: FnTuner<'_, Self, (), &'a mut i32, false> = self
//                 .data
//                 .return_mut_ref_data
//                 .add_config(return_mut_ref_args_checker, self);
//             return unsafe { core::mem::transmute(fn_tuner) };
//         }
//         pub fn return_mut_ref_with_base(&self) -> FnTuner<'rs, Self, (), &'a mut i32, false> {
//             let return_mut_ref_with_base_args_checker: return_mut_ref_with_base_ArgsChecker<
//                 'rs,
//                 'a,
//                 'b,
//             > = return_mut_ref_with_base_ArgsChecker {
//                 _phantom_lifetime: PhantomData,
//                 _phantom_a: PhantomData,
//                 _phantom_b: PhantomData,
//             };
//             let fn_tuner: FnTuner<'_, Self, (), &'a mut i32, false> = self
//                 .data
//                 .return_mut_ref_with_base_data
//                 .add_config(return_mut_ref_with_base_args_checker, self);
//             return unsafe { core::mem::transmute(fn_tuner) };
//         }
//         pub fn fooo(
//             &self,
//             arg_1: impl Into<Arg<'rs, Foo>>,
//         ) -> FnTuner<'rs, Self, (&Foo,), (), false> {
//             let fooo_args_checker: fooo_ArgsChecker<'rs, 'a, 'b> = fooo_ArgsChecker {
//                 _phantom_lifetime: PhantomData,
//                 _phantom_a: PhantomData,
//                 _phantom_b: PhantomData,
//                 arg_1: arg_1.into(),
//             };
//             let fn_tuner: FnTuner<'_, Self, (&Foo,), (), false> =
//                 self.data.fooo_data.add_config(fooo_args_checker, self);
//             return unsafe { core::mem::transmute(fn_tuner) };
//         }
//     }
//     impl<'rs, 'a, 'b> TraitMockReceived<'rs, 'a, 'b> {
//         pub fn accept_ref(
//             &self,
//             r: impl Into<Arg<'rs, &'a &'b &'a &'rs i32>>,
//             times: Times,
//         ) -> FnVerifier<'rs, Self, (&&'a &'b &'a &i32,)> {
//             let accept_ref_args_checker: accept_ref_ArgsChecker<'rs, 'a, 'b> =
//                 accept_ref_ArgsChecker {
//                     _phantom_lifetime: PhantomData,
//                     _phantom_a: PhantomData,
//                     _phantom_b: PhantomData,
//                     r: r.into(),
//                 };
//             self.data
//                 .accept_ref_data
//                 .verify_received(accept_ref_args_checker, times);
//             return FnVerifier::new(self);
//         }
//         pub fn return_mut_ref(&self, times: Times) -> FnVerifier<'rs, Self, ()> {
//             let return_mut_ref_args_checker: return_mut_ref_ArgsChecker<'rs, 'a, 'b> =
//                 return_mut_ref_ArgsChecker {
//                     _phantom_lifetime: PhantomData,
//                     _phantom_a: PhantomData,
//                     _phantom_b: PhantomData,
//                 };
//             self.data
//                 .return_mut_ref_data
//                 .verify_received(return_mut_ref_args_checker, times);
//             return FnVerifier::new(self);
//         }
//         pub fn return_mut_ref_with_base(&self, times: Times) -> FnVerifier<'rs, Self, ()> {
//             let return_mut_ref_with_base_args_checker: return_mut_ref_with_base_ArgsChecker<
//                 'rs,
//                 'a,
//                 'b,
//             > = return_mut_ref_with_base_ArgsChecker {
//                 _phantom_lifetime: PhantomData,
//                 _phantom_a: PhantomData,
//                 _phantom_b: PhantomData,
//             };
//             self.data
//                 .return_mut_ref_with_base_data
//                 .verify_received(return_mut_ref_with_base_args_checker, times);
//             return FnVerifier::new(self);
//         }
//         pub fn fooo(
//             &self,
//             arg_1: impl Into<Arg<'rs, Foo>>,
//             times: Times,
//         ) -> FnVerifier<'rs, Self, (&Foo,)> {
//             let fooo_args_checker: fooo_ArgsChecker<'rs, 'a, 'b> = fooo_ArgsChecker {
//                 _phantom_lifetime: PhantomData,
//                 _phantom_a: PhantomData,
//                 _phantom_b: PhantomData,
//                 arg_1: arg_1.into(),
//             };
//             self.data
//                 .fooo_data
//                 .verify_received(fooo_args_checker, times);
//             return FnVerifier::new(self);
//         }
//         pub fn no_other_calls(&self) {
//             self.data.verify_received_nothing_else();
//         }
//     }
// }
// 
// #[cfg(test)]
// mod tests {
//     #![allow(non_snake_case)]
//     use super::*;
//     use not_enough_asserts::panics::*;
//     use rsubstitute::*;
// 
//     #[test]
//     fn compile() {}
// 
//     #[test]
//     fn flex() {
//         let trait_mock = TraitMock::new();
// 
//         let v1 = 11;
//         let r1 = &&&&v1;
//         let v2 = 23;
//         let r2 = &&&&v2;
// 
//         trait_mock.setup().accept_ref(r2).returns(r1);
// 
//         let r = trait_mock.accept_ref(r2);
//         assert_eq!(r1, r);
//         
//         trait_mock
//             .received()
//             .accept_ref(r2, Times::Once)
//             .no_other_calls();
//     }
// }
