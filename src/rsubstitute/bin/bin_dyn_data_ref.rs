// use rsubstitute::macros::*;
// 
// // #[mock]
// // trait Trait {
// //     fn work(&self, v: i32) -> i32;
// // }
// 
// trait Trait {
//     fn work<'a>(&self, r: &'a i32) -> &'a i32;
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
//     use std::any::Any;
// 
//     #[derive(Clone)]
//     pub struct work_Call<'a> {
//         v: &'a i32,
//     }
//     impl<'a > IArgInfosProvider for work_Call<'a> {
//         fn get_arg_infos(&self) -> Vec<ArgInfo> {
//             vec![ArgInfo::new("v", self.v.clone())]
//         }
//     }
// 
//     #[derive(Debug, IArgsFormatter)]
//     pub struct work_ArgsChecker<'rs, 'a> {
//         _phantom_lifetime: PhantomData<&'rs ()>,
//         v: Arg<&'a i32>,
//     }
//     impl<'rs, 'a: 'rs> IArgsChecker<'rs> for work_ArgsChecker<'rs, 'a> {
//         fn check(&self, dyn_call: &Call) -> Vec<ArgCheckResult> {
//             let call: &work_Call<'rs, 'a> = dyn_call.downcast_ref();
//             vec![self.v.check("v", &call.v)]
//         }
//     }
// 
//     #[derive(IMockData)]
//     pub struct TraitMockData<'rs> {
//         _phantom_lifetime: PhantomData<&'rs ()>,
//         work_data: FnData<'static, TraitMock<'rs>>,
//     }
// 
//     #[derive(Clone)]
//     pub struct TraitMockSetup<'rs> {
//         data: Arc<TraitMockData<'rs>>,
//     }
// 
//     #[derive(Clone)]
//     pub struct TraitMockReceived<'rs> {
//         data: Arc<TraitMockData<'rs>>,
//     }
//     #[derive(Clone)]
//     pub struct TraitMock<'rs> {
//         setup: TraitMockSetup<'rs>,
//         received: TraitMockReceived<'rs>,
//         data: Arc<TraitMockData<'rs>>,
//     }
//     impl<'rs> Trait for TraitMock<'rs> {
//         fn work(&self, v: i32) -> i32 {
//             let call = unsafe {
//                 work_Call {
//                     _phantom_lifetime: PhantomData,
//                     v: std::mem::transmute(v),
//                 }
//             };
//             return self.data.work_data.handle_returning(Call::new(call));
//         }
//     }
//     impl<'rs> TraitMock<'rs> {
//         pub fn new() -> Self {
//             let data = Arc::new(TraitMockData {
//                 _phantom_lifetime: PhantomData,
//                 work_data: FnData::new("work", &SERVICES),
//             });
//             return TraitMock {
//                 setup: TraitMockSetup { data: data.clone() },
//                 received: TraitMockReceived { data: data.clone() },
//                 data,
//             };
//         }
//         pub fn setup<'__rsubstitute_config>(&self) -> TraitMockSetup<'__rsubstitute_config> {
//             unsafe { std::mem::transmute(self.setup.clone()) }
//         }
//         pub fn received<'__rsubstitute_config>(&self) -> TraitMockReceived<'__rsubstitute_config> {
//             unsafe { std::mem::transmute(self.received.clone()) }
//         }
//     }
//     impl<'rs> TraitMockSetup<'rs> {
//         pub fn work(
//             &'rs self,
//             v: impl Into<Arg<i32>>,
//         ) -> SharedFnConfig<'rs, TraitMock<'rs>, i32, Self> {
//             let work_args_checker = work_ArgsChecker {
//                 _phantom_lifetime: PhantomData,
//                 v: v.into(),
//             };
//             let fn_config: Arc<RefCell<FnConfig<'rs, TraitMock<'rs>>>> =
//                 unsafe { std::mem::transmute(self.data.work_data.add_config(work_args_checker)) };
//             let shared_fn_config = SharedFnConfig::new(fn_config, self);
//             return shared_fn_config;
//         }
//     }
//     impl<'rs> TraitMockReceived<'rs> {
//         pub fn work(self, v: impl Into<Arg<i32>>, times: Times) -> Self {
//             let work_args_checker = work_ArgsChecker {
//                 _phantom_lifetime: PhantomData,
//                 v: v.into(),
//             };
//             self.data
//                 .work_data
//                 .verify_received(work_args_checker, times);
//             return self;
//         }
//         pub fn no_other_calls(&'rs self) {
//             self.data.verify_received_nothing_else();
//         }
//     }
// }
// 
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use rsubstitute_core::Times;
//     use rsubstitute_core::args_matching::Arg;
// 
//     #[test]
//     fn my_test() {
//         // Arrange
//         let mock = TraitMock::new();
//         let v1 = 10;
//         let v2 = 20;
//         let v3 = -31;
// 
//         let r1 = 111;
//         let r2 = 222;
//         let r3 = 333;
// 
//         mock.setup()
//             .work(v1)
//             .returns(r1)
//             .work(v2)
//             .returns(r2)
//             .work(Arg::Is(|x| *x < 0))
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
//         mock.received()
//             .work(v1, Times::Once)
//             .work(v2, Times::Once)
//             .work(v3, Times::Once)
//             .work(Arg::Any, Times::Exactly(3))
//             .no_other_calls();
//     }
// }
// 
fn main() {}
