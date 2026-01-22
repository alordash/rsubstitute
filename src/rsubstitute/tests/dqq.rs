// #[cfg(not(test))]
// fn fds() {}
// #[cfg(test)]
// use fds::fds;
// #[allow(mismatched_lifetime_syntaxes)]
// mod fds {
//     use super::*;
//     use rsubstitute::for_generated::*;
//     fn base_fds() {}
//     #[allow(non_camel_case_types)]
//     #[derive(Clone)]
//     pub struct fds_Call<'__rsubstitute_arg_field_lifetime> {
//         _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
//     }
//     impl<'__rsubstitute_arg_field_lifetime> IArgInfosProvider
//         for fds_Call<'__rsubstitute_arg_field_lifetime>
//     {
//         fn get_arg_infos(&self) -> Vec<ArgInfo> {
//             vec![]
//         }
//     }
//     #[allow(non_camel_case_types)]
//     #[derive(Debug, IArgsFormatter)]
//     pub struct fds_ArgsChecker<'__rsubstitute_arg_field_lifetime> {
//         _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
//     }
//     impl<'__rsubstitute_arg_field_lifetime>
//         IArgsChecker<fds_Call<'__rsubstitute_arg_field_lifetime>>
//         for fds_ArgsChecker<'__rsubstitute_arg_field_lifetime>
//     {
//         fn check(&self, call: fds_Call<'__rsubstitute_arg_field_lifetime>) -> Vec<ArgCheckResult> {
//             vec![]
//         }
//     }
//     #[allow(non_camel_case_types)]
//     pub struct fdsBaseCaller {}
//     impl<'__rsubstitute_arg_field_lifetime>
//         IBaseCaller<fds_Call<'__rsubstitute_arg_field_lifetime>, ()> for fdsBaseCaller
//     {
//         fn call_base(&self, call: fds_Call<'__rsubstitute_arg_field_lifetime>) {
//             return base_fds();
//         }
//     }
//     #[allow(non_camel_case_types)]
//     #[derive(IMockData)]
//     pub struct fdsMockData<'__rsubstitute_arg_field_lifetime> {
//         _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
//         base_caller: Arc<RefCell<fdsBaseCaller>>,
//         fds_data: FnData<
//             fds_Call<'__rsubstitute_arg_field_lifetime>,
//             fds_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
//             (),
//             fdsBaseCaller,
//         >,
//     }
//     #[allow(non_camel_case_types)]
//     pub struct fdsMockSetup<'__rsubstitute_arg_field_lifetime> {
//         data: Arc<fdsMockData<'__rsubstitute_arg_field_lifetime>>,
//     }
//     #[allow(non_camel_case_types)]
//     pub struct fdsMockReceived<'__rsubstitute_arg_field_lifetime> {
//         data: Arc<fdsMockData<'__rsubstitute_arg_field_lifetime>>,
//     }
//     #[allow(non_camel_case_types)]
//     pub struct fdsMock {
//         pub setup: fdsMockSetup<'static>,
//         pub received: fdsMockReceived<'static>,
//         data: Arc<fdsMockData<'static>>,
//     }
//     unsafe impl Send for fdsMock {}
//     unsafe impl Sync for fdsMock {}
//     impl<'__rsubstitute_arg_field_lifetime> Default for fdsMock {
//         fn default() -> Self {
//             let data = Arc::new(fdsMockData {
//                 _phantom_lifetime: PhantomData,
//                 base_caller: Arc::new(RefCell::new(fdsBaseCaller {})),
//                 fds_data: FnData::new("fds", &SERVICES),
//             });
//             return fdsMock {
//                 setup: fdsMockSetup { data: data.clone() },
//                 received: fdsMockReceived { data: data.clone() },
//                 data,
//             };
//         }
//     }
//     impl<'__rsubstitute_arg_field_lifetime> fdsMockSetup<'__rsubstitute_arg_field_lifetime> {
//         #[allow(dead_code)]
//         #[allow(elided_named_lifetimes)]
//         pub fn setup(
//             &'__rsubstitute_arg_field_lifetime self,
//         ) -> SharedFnConfig<
//             '__rsubstitute_arg_field_lifetime,
//             fds_Call<'__rsubstitute_arg_field_lifetime>,
//             fds_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
//             (),
//             Self,
//             fdsBaseCaller,
//         > {
//             let fds_args_checker = fds_ArgsChecker {
//                 _phantom_lifetime: PhantomData,
//             };
//             let fn_config = self.data.fds_data.add_config(fds_args_checker);
//             let shared_fn_config =
//                 SharedFnConfig::new(fn_config, self, Some(self.data.base_caller.clone()));
//             return shared_fn_config;
//         }
//     }
//     impl<'__rsubstitute_arg_field_lifetime> fdsMockReceived<'__rsubstitute_arg_field_lifetime> {
//         #[allow(dead_code)]
//         #[allow(elided_named_lifetimes)]
//         pub fn received(
//             &'__rsubstitute_arg_field_lifetime self,
//             times: Times,
//         ) -> &'__rsubstitute_arg_field_lifetime Self {
//             let fds_args_checker = fds_ArgsChecker {
//                 _phantom_lifetime: PhantomData,
//             };
//             self.data.fds_data.verify_received(fds_args_checker, times);
//             return self;
//         }
//         pub fn no_other_calls(&'__rsubstitute_arg_field_lifetime self) {
//             self.data.verify_received_nothing_else();
//         }
//     }
//     pub fn setup<'__rsubstitute_arg_field_lifetime>() -> SharedFnConfig<
//         'static,
//         fds_Call<'static>,
//         fds_ArgsChecker<'static>,
//         (),
//         fdsMockSetup<'static>,
//         fdsBaseCaller,
//     > {
//         let mock = get_global_mock::<fdsMock>();
//         mock.data.fds_data.reset();
//         return mock.setup.setup();
//     }
//     pub fn received(times: Times) -> &'static fdsMockReceived<'static> {
//         return get_global_mock::<fdsMock>().received.received(times);
//     }
//     pub fn fds<'__rsubstitute_arg_anonymous, '__rsubstitute_arg_field_lifetime>() {
//         let call = unsafe {
//             fds_Call {
//                 _phantom_lifetime: PhantomData,
//             }
//         };
//         get_global_mock::<fdsMock>().data.fds_data.handle_base(call);
//     }
// }
