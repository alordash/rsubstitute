// use rsubstitute::macros::*;
// 
// // #[mock]
// // trait Trait {
// //     fn work<'a>(&self, v: &'a i32) -> &'a i32;
// // }
// 
// trait Trait {
//     fn work<'a>(&self, v: &'a i32) -> &'a i32;
// }
// #[cfg(test)]
// pub use __rsubstitute_generated_Trait::*;
// #[cfg(test)]
// #[allow(dead_code)]
// #[allow(unused)]
// #[allow(non_snake_case)]
// #[allow(non_camel_case_types)]
// mod __rsubstitute_generated_Trait {
//     use super::*;
//     use rsubstitute::for_generated::*;
//     use std::hash::Hash;
// 
//     #[derive(Clone, IGenericsHashKeyProvider, IArgInfosProvider)]
//     pub struct work_Call<'a> {
//         v: &'a i32,
//     }
// 
//     #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
//     pub struct work_ArgsChecker<'a> {
//         v: Arg<&'a i32>,
//     }
//     impl<'rs, 'a: 'rs> IArgsChecker<'rs> for work_ArgsChecker<'a> {
//         fn check(&self, dyn_call: &'rs Call<'rs>) -> Vec<ArgCheckResult> {
//             let call: &work_Call = dyn_call.downcast_ref();
//             vec![self.v.check("v", &call.v)]
//         }
//     }
// 
//     #[derive(IMockData)]
//     pub struct TraitMockData {
//         work_data: FnData<'static, TraitMock>,
//     }
// 
//     #[derive(Clone)]
//     pub struct TraitMockSetup {
//         data: Arc<TraitMockData>,
//     }
// 
//     #[derive(Clone)]
//     pub struct TraitMockReceived {
//         data: Arc<TraitMockData>,
//     }
//     #[derive(Clone)]
//     pub struct TraitMock {
//         pub setup: TraitMockSetup,
//         pub received: TraitMockReceived,
//         data: Arc<TraitMockData>,
//     }
//     impl Trait for TraitMock {
//         fn work<'a>(&self, v: &'a i32) -> &'a i32 {
//             let call = unsafe {
//                 work_Call {
//                     v: std::mem::transmute(v),
//                 }
//             };
//             return self.data.work_data.handle_returning(Call::new(call));
//         }
//     }
//     impl TraitMock {
//         pub fn new() -> Self {
//             let data = Arc::new(TraitMockData {
//                 work_data: FnData::new("work", &SERVICES),
//             });
//             return TraitMock {
//                 setup: TraitMockSetup { data: data.clone() },
//                 received: TraitMockReceived { data: data.clone() },
//                 data,
//             };
//         }
//     }
//     impl TraitMockSetup {
//         pub fn work<'rs, 'a: 'rs>(
//             &'rs self,
//             v: impl Into<Arg<&'a i32>>,
//         ) -> SharedFnConfig<'rs, TraitMock, &'a i32, Self> {
//             let work_args_checker = work_ArgsChecker { v: v.into() };
//             let fn_config = self.data.work_data.as_local().add_config(work_args_checker);
//             let shared_fn_config = SharedFnConfig::new(fn_config, self);
//             return shared_fn_config;
//         }
//     }
//     impl TraitMockReceived {
//         pub fn work<'a>(self, v: impl Into<Arg<&'a i32>>, times: Times) -> Self {
//             let work_args_checker: work_ArgsChecker<'a> = work_ArgsChecker { v: v.into() };
//             self.data
//                 .work_data
//                 .as_local()
//                 .verify_received(work_args_checker, times);
//             return self;
//         }
//         pub fn no_other_calls(&self) {
//             self.data.verify_received_nothing_else();
//         }
//     }
// }
// 
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rsubstitute_core::Times;
//     use rsubstitute_core::args::Arg;
// 
//     #[test]
//     fn my_test() {
//         // Arrange
//         let mock = TraitMock::new();
//         let v1 = &10;
//         let v2 = &20;
//         let v3 = &-31;
// 
//         let r1 = &111;
//         let r2 = &222;
//         let r3 = &333;
// 
//         mock.setup
//             .work(v1)
//             .returns(r1)
//             .work(v2)
//             .returns(r2)
//             .work(Arg::Is(|x: &&i32| **x < 0))
//             .returns(r3);
// 
//         // Act
//         let actual_r1 = mock.work(v1);
//         let actual_r2 = mock.work(v2);
//         let actual_r3 = mock.work(v3);
// 
//         // Assert
//         assert_eq!(r1, actual_r1);
//         assert_eq!(r2, actual_r2);
//         assert_eq!(r3, actual_r3);
// 
//         mock.received
//             .work(v1, Times::Once)
//             .work(v2, Times::Once)
//             .work(v3, Times::Once)
//             .work(Arg::Any, Times::Exactly(3))
//             .no_other_calls();
//     }
// }

fn main() {}
