use rsubstitute::prelude::*;

mocked_base! {
    #[derive(Clone)]
struct Struct;
impl Struct {
        pub fn new() -> Self {Self}
        pub fn same(self) {}
    pub fn flex(self: Box<Self>) {}
}}

#[cfg(test)]
mod test {
    use super::*;
    use not_enough_asserts::assert_type_eq;

    #[test]
    fn compile() {
        let mock = Struct::new();
        mock.setup.same().does(|a, _| assert_type_eq!(a, Struct));
        mock.setup.flex().does(|a, _| assert_type_eq!(a, Box<Struct>));
        mock.clone().same();
        Box::new(mock).flex();
    }
}

// #[cfg(not(test))]
// struct Struct;
// #[cfg(not(test))]
// impl Struct {
//     pub fn new() -> Self {
//         Self
//     }
//     pub fn same(self) {}
//     pub fn flex(self: Box<Self>) {}
// }
// #[cfg(test)]
// pub use __rsubstitute_generated_Struct::*;
// #[cfg(test)]
// #[allow(unused_parens)]
// #[allow(non_snake_case)]
// #[allow(non_camel_case_types)]
// #[allow(mismatched_lifetime_syntaxes)]
// mod __rsubstitute_generated_Struct {
//     use super::*;
//     use rsubstitute::for_generated::*;
//     #[doc(hidden)]
//     pub struct same_Call {}
//     impl IArgsInfosProvider for same_Call {
//         fn get_arg_infos(&self) -> Vec<ArgInfo> {
//             vec![]
//         }
//     }
//     impl IArgsTupleProvider for same_Call {
//         fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
//             Box::leak(Box::new(())) as *mut _ as *mut ()
//         }
//     }
//     impl IGenericsInfoProvider for same_Call {
//         fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
//             vec![]
//         }
//         fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
//         fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
//     }
//     impl Clone for same_Call {
//         fn clone(&self) -> Self {
//             Self {}
//         }
//     }
//     #[doc(hidden)]
//     #[derive(Debug)]
//     pub struct same_ArgsChecker {}
//     impl IArgsChecker for same_ArgsChecker {
//         #[allow(unused)]
//         fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
//             let call: &same_Call = dyn_call.downcast_ref();
//             vec![]
//         }
//     }
//     impl IArgsFormatter for same_ArgsChecker {
//         fn fmt_args(&self) -> String {
//             format!("",)
//         }
//     }
//     impl IGenericsInfoProvider for same_ArgsChecker {
//         fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
//             vec![]
//         }
//         fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
//         fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
//     }
//     #[doc(hidden)]
//     pub struct flex_Call {}
//     impl IArgsInfosProvider for flex_Call {
//         fn get_arg_infos(&self) -> Vec<ArgInfo> {
//             vec![]
//         }
//     }
//     impl IArgsTupleProvider for flex_Call {
//         fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
//             Box::leak(Box::new(())) as *mut _ as *mut ()
//         }
//     }
//     impl IGenericsInfoProvider for flex_Call {
//         fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
//             vec![]
//         }
//         fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
//         fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
//     }
//     impl Clone for flex_Call {
//         fn clone(&self) -> Self {
//             Self {}
//         }
//     }
//     #[doc(hidden)]
//     #[derive(Debug)]
//     pub struct flex_ArgsChecker {}
//     impl IArgsChecker for flex_ArgsChecker {
//         #[allow(unused)]
//         fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
//             let call: &flex_Call = dyn_call.downcast_ref();
//             vec![]
//         }
//     }
//     impl IArgsFormatter for flex_ArgsChecker {
//         fn fmt_args(&self) -> String {
//             format!("",)
//         }
//     }
//     impl IGenericsInfoProvider for flex_ArgsChecker {
//         fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
//             vec![]
//         }
//         fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
//         fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
//     }
//     #[doc(hidden)]
//     #[derive(IMockData)]
//     pub struct StructData {
//         pub same: FnData<'static, Struct, true, true>,
//         pub flex: FnData<'static, Struct, true, true>,
//     }
//     #[doc(hidden)]
//     pub struct StructSetup {
//         data: Arc<StructData>,
//     }
//     impl Clone for StructSetup {
//         fn clone(&self) -> Self {
//             Self {
//                 data: (&self.data).clone(),
//             }
//         }
//     }
//     #[doc(hidden)]
//     pub struct StructReceived {
//         data: Arc<StructData>,
//     }
//     impl Clone for StructReceived {
//         fn clone(&self) -> Self {
//             Self {
//                 data: (&self.data).clone(),
//             }
//         }
//     }
//     #[doc(hidden)]
//     pub struct Struct_InnerData;
//     impl Struct_InnerData {
//         pub fn new() -> Self {
//             Self
//         }
//     }
//     pub struct Struct {
//         pub setup: StructSetup,
//         pub received: StructReceived,
//         pub data: Arc<StructData>,
//         inner_data: Struct_InnerData,
//     }
//     impl AsRef<Struct> for Struct {
//         fn as_ref(&self) -> &Struct {
//             self
//         }
//     }
//     impl Deref for Struct {
//         type Target = Struct_InnerData;
//         fn deref(&self) -> &Self::Target {
//             &self.inner_data
//         }
//     }
//     impl Struct {
//         pub fn same(self) {
//             let call: same_Call = same_Call {};
//             self.data
//                 .clone()
//                 .same
//                 .handle_base(self, call, Self::base_same);
//         }
//         pub fn flex(self: Box<Self>) {
//             let call: flex_Call = flex_Call {};
//             self.data
//                 .clone()
//                 .flex
//                 .handle_base(self, call, Self::base_flex);
//         }
//     }
//     impl Struct {
//         pub fn new() -> Self {
//             let data = Arc::new(StructData {
//                 same: FnData::new("same"),
//                 flex: FnData::new("flex"),
//             });
//             let inner_data = Struct_InnerData::new();
//             return Struct {
//                 setup: StructSetup { data: data.clone() },
//                 received: StructReceived { data: data.clone() },
//                 data,
//                 inner_data,
//             };
//         }
//         fn base_same(self, call: same_Call) {
//             #[allow(non_shorthand_field_patterns)]
//             #[allow(unused_variables)]
//             let same_Call { .. } = call;
//         }
//         fn base_flex(self: Box<Struct>, call: flex_Call) {
//             #[allow(non_shorthand_field_patterns)]
//             #[allow(unused_variables)]
//             let flex_Call { .. } = call;
//         }
//     }
//     impl StructSetup {
//         pub fn same<'__rsa>(&self) -> FnTuner<'_, Struct, Self, (), (), Self, true, true> {
//             let same_args_checker: same_ArgsChecker = same_ArgsChecker {};
//             let fn_tuner: FnTuner<'_, Struct, Self, (), (), Self, true, true> =
//                 self.data.same.add_config(same_args_checker, self);
//             return transmute_lifetime!(fn_tuner);
//         }
//         pub fn flex<'__rsa>(&self) -> FnTuner<'_, Struct, Self, (), (), Box<Struct>, true, true> {
//             let flex_args_checker: flex_ArgsChecker = flex_ArgsChecker {};
//             let fn_tuner: FnTuner<'_, Struct, Self, (), (), Box<Struct>, true, true> =
//                 self.data.flex.add_config(flex_args_checker, self);
//             return transmute_lifetime!(fn_tuner);
//         }
//     }
//     impl StructReceived {
//         pub fn same<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
//             let same_args_checker: same_ArgsChecker = same_ArgsChecker {};
//             self.data.same.verify_received(same_args_checker, times);
//             return FnVerifier::new(self.clone());
//         }
//         pub fn flex<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
//             let flex_args_checker: flex_ArgsChecker = flex_ArgsChecker {};
//             self.data.flex.verify_received(flex_args_checker, times);
//             return FnVerifier::new(self.clone());
//         }
//         pub fn no_other_calls(&self) {
//             self.data.verify_received_nothing_else();
//         }
//     }
// }
