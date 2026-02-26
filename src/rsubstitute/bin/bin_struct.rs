#![allow(unused)]
use rsubstitute_proc_macro::*;
use std::fmt::*;

mocked! {
    struct Struct {
        number: i32,
    }

    impl Struct {
        fn first_struct_impl(&self) {
            println!("first_struct_impl");
        }
    }

    impl MyTrait for Struct {
        fn work(&self, value: i32) -> String {
            return "working...".to_owned();
        }
    }

    impl Struct {
        pub fn new(number: i32) -> Self {
            Self { number }
        }

        pub fn get_number(&self) -> i32 {
            self.number
        }

        pub fn format(&self) -> String {
            let number = self.get_number();
            let work_result = self.work(number);
            let result = format!("Struct, number = {number}, work_result = {work_result}");
            return result;
        }
    }

    #[unmock]
    impl Debug for Struct {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            return write!(f, "Struct = {{ number = {} }}", self.number);
        }
    }
}

#[cfg(not(test))]
fn main() {
    // let r#struct = Struct::new(2);
    // dbg!(r#struct);

    println!("done")
}

trait MyTrait {
    fn work(&self, value: i32) -> String;
}

trait Gen<T> {}

#[cfg(test)]
mod tests {
    use crate::{MyTrait, StructMock};
    use rsubstitute_core::Times;

    #[test]
    fn struct_test() {
        // Arrange
        let mock_number = 10;
        let mock = StructMock::new(mock_number);

        let get_number_returned_value = 22;
        mock.setup()
            .get_number()
            .returns(get_number_returned_value)
            .format()
            .call_base();

        let my_trait_work_returned_value_for_format = "for format!".to_owned();
        let my_trait_work_accepted_value_for_call_base = 333;
        let my_trait_work_returned_value_for_mock = "Mocked value!".to_owned();
        let my_trait_work_accepted_value_for_mock = 4;
        mock.setup()
            .MyTrait
            .work(get_number_returned_value)
            .returns(my_trait_work_returned_value_for_format.clone())
            .work(my_trait_work_accepted_value_for_call_base)
            .call_base()
            .work(my_trait_work_accepted_value_for_mock)
            .returns(my_trait_work_returned_value_for_mock.clone());

        // Act
        let actual_get_number_returned_value = mock.get_number();
        let actual_format_value = mock.format();

        let actual_my_trait_work_call_base_value =
            mock.work(my_trait_work_accepted_value_for_call_base);
        let actual_my_trait_work_returned_value_for_mock =
            mock.work(my_trait_work_accepted_value_for_mock);

        // Assert
        assert_eq!(get_number_returned_value, actual_get_number_returned_value);
        let expected_format_value = format!(
            "Struct, number = {get_number_returned_value}, work_result = {my_trait_work_returned_value_for_format}"
        );
        assert_eq!(expected_format_value, actual_format_value);

        let expected_my_trait_work_call_base_value = "working...".to_owned();
        assert_eq!(
            expected_my_trait_work_call_base_value,
            actual_my_trait_work_call_base_value
        );
        assert_eq!(
            my_trait_work_returned_value_for_mock,
            actual_my_trait_work_returned_value_for_mock
        );

        mock.received()
            .get_number(Times::Exactly(2))
            .format(Times::Once);
        mock.received()
            .MyTrait
            .work(my_trait_work_accepted_value_for_call_base, Times::Once)
            .work(my_trait_work_accepted_value_for_mock, Times::Once)
            .work(get_number_returned_value, Times::Once);
        mock.received().no_other_calls();
    }
}

// pub use __rsubstitute_generated_Struct::*;
//
// mod __rsubstitute_generated_Struct {
//     #![allow(non_camel_case_types)]
//     #![allow(non_snake_case)]
//
//     use super::*;
//     use rsubstitute::for_generated::*;
//     use std::ops::Deref;
//
//     #[derive(Clone)]
//     pub struct MyTrait_work_Call<'a> {
//         _phantom_lifetime: PhantomData<&'a ()>,
//         pub value: i32,
//     }
//     impl<'a> IArgInfosProvider for MyTrait_work_Call<'a> {
//         fn get_arg_infos(&self) -> Vec<ArgInfo> {
//             vec![ArgInfo::new("value", self.value)]
//         }
//     }
//
//     #[derive(Debug, IArgsFormatter)]
//     pub struct MyTrait_work_ArgsChecker<'a> {
//         _phantom_lifetime: PhantomData<&'a ()>,
//         pub value: Arg<i32>,
//     }
//
//     pub struct MyTraitSetup<'a> {
//         data: Arc<StructMockData<'a>>,
//     }
//
//     pub struct MyTraitReceived<'a> {
//         data: Arc<StructMockData<'a>>,
//     }
//
//     impl<'a> IArgsChecker<MyTrait_work_Call<'a>> for MyTrait_work_ArgsChecker<'a> {
//         fn check(&self, call: MyTrait_work_Call<'a>) -> Vec<ArgCheckResult> {
//             vec![self.value.check("value", call.value)]
//         }
//     }
//
//     impl<'a> IBaseCaller<MyTrait_work_Call<'a>, String> for StructMock<'a> {
//         fn call_base(&self, call: MyTrait_work_Call<'a>) -> String {
//             let MyTrait_work_Call { value, .. } = call;
//             return "working...".to_owned();
//         }
//     }
//
//     impl<'a> MyTraitSetup<'a> {
//         #[allow(dead_code)]
//         #[allow(mismatched_lifetime_syntaxes)]
//         pub fn work(
//             &'a self,
//             value: impl Into<Arg<i32>>,
//         ) -> FnTuner<
//             'a,
//             StructMock<'a>,
//             MyTrait_work_Call<'a>,
//             MyTrait_work_ArgsChecker<'a>,
//             String,
//             Self,
//         > {
//             let MyTrait_work_ArgsChecker = MyTrait_work_ArgsChecker {
//                 _phantom_lifetime: PhantomData,
//                 value: value.into(),
//             };
//             let fn_config = self
//                 .data
//                 .MyTrait_work_data
//                 .add_config(MyTrait_work_ArgsChecker);
//             let fn_tuner = FnTuner::new(fn_config, self);
//             return fn_tuner;
//         }
//     }
//
//     impl<'a> MyTraitReceived<'a> {
//         #[allow(dead_code)]
//         #[allow(mismatched_lifetime_syntaxes)]
//         pub fn work(&'a self, value: impl Into<Arg<i32>>, times: Times) -> &'a Self {
//             let MyTrait_work_ArgsChecker = MyTrait_work_ArgsChecker {
//                 _phantom_lifetime: PhantomData,
//                 value: value.into(),
//             };
//             self.data
//                 .MyTrait_work_data
//                 .verify_received(MyTrait_work_ArgsChecker, times);
//             return self;
//         }
//
//         pub fn no_other_calls(&self) {
//             self.data.verify_received_nothing_else()
//         }
//     }
//
//     #[derive(Clone)]
//     pub struct get_number_Call<'a> {
//         _phantom_lifetime: PhantomData<&'a ()>,
//     }
//     impl<'a> IArgInfosProvider for get_number_Call<'a> {
//         fn get_arg_infos(&self) -> Vec<ArgInfo> {
//             vec![]
//         }
//     }
//
//     #[derive(Debug, IArgsFormatter)]
//     pub struct get_number_ArgsChecker<'a> {
//         _phantom_lifetime: PhantomData<&'a ()>,
//     }
//
//     impl<'a> IArgsChecker<get_number_Call<'a>> for get_number_ArgsChecker<'a> {
//         fn check(&self, call: get_number_Call<'a>) -> Vec<ArgCheckResult> {
//             vec![]
//         }
//     }
//
//     impl<'a> IBaseCaller<get_number_Call<'a>, i32> for StructMock<'a> {
//         fn call_base(&self, call: get_number_Call<'a>) -> i32 {
//             let get_number_Call { .. } = call;
//             return self.number;
//         }
//     }
//
//     #[derive(Clone)]
//     pub struct format_Call<'a> {
//         _phantom_lifetime: PhantomData<&'a ()>,
//     }
//     impl<'a> IArgInfosProvider for format_Call<'a> {
//         fn get_arg_infos(&self) -> Vec<ArgInfo> {
//             vec![]
//         }
//     }
//
//     #[derive(Debug, IArgsFormatter)]
//     pub struct format_ArgsChecker<'a> {
//         _phantom_lifetime: PhantomData<&'a ()>,
//     }
//
//     impl<'a> IArgsChecker<format_Call<'a>> for format_ArgsChecker<'a> {
//         fn check(&self, call: format_Call<'a>) -> Vec<ArgCheckResult> {
//             vec![]
//         }
//     }
//
//     impl<'a> IBaseCaller<format_Call<'a>, String> for StructMock<'a> {
//         fn call_base(&self, call: format_Call<'a>) -> String {
//             let format_Call { .. } = call;
//             let number = self.get_number();
//             let work_result = self.work(number);
//             let result = format!("Struct, number = {number}, work_result = {work_result}");
//             return result;
//         }
//     }
//
//     #[derive(IMockData)]
//     pub struct StructMockData<'a> {
//         _phantom_lifetime: PhantomData<&'a ()>,
//         MyTrait_work_data:
//             FnData<StructMock<'a>, MyTrait_work_Call<'a>, MyTrait_work_ArgsChecker<'a>, String>,
//         get_number_data:
//             FnData<StructMock<'a>, get_number_Call<'a>, get_number_ArgsChecker<'a>, i32>,
//         format_data: FnData<StructMock<'a>, format_Call<'a>, format_ArgsChecker<'a>, String>,
//     }
//
//     pub struct StructMockSetup<'a> {
//         pub MyTrait: MyTraitSetup<'a>,
//         data: Arc<StructMockData<'a>>,
//     }
//
//     pub struct StructMockReceived<'a> {
//         pub MyTrait: MyTraitReceived<'a>,
//         data: Arc<StructMockData<'a>>,
//     }
//
//     pub struct Struct_InnerData {
//         number: i32,
//     }
//
//     impl Struct_InnerData {
//         fn new(number: i32) -> Self {
//             Self { number }
//         }
//     }
//
//     #[allow(non_camel_case_types)]
//     pub struct StructMock<'a> {
//         pub setup: StructMockSetup<'a>,
//         pub received: StructMockReceived<'a>,
//         data: Arc<StructMockData<'a>>,
//         inner_data: Struct_InnerData,
//     }
//
//     impl<'a> Deref for StructMock<'a> {
//         type Target = Struct_InnerData;
//
//         fn deref(&self) -> &Self::Target {
//             &self.inner_data
//         }
//     }
//
//     impl<'a> MyTrait for StructMock<'a> {
//         fn work(&self, value: i32) -> String {
//             let call = unsafe {
//                 MyTrait_work_Call {
//                     _phantom_lifetime: PhantomData,
//                     value,
//                 }
//             };
//             return self
//                 .data
//                 .MyTrait_work_data
//                 .handle_base_returning(self, call);
//         }
//     }
//
//     impl<'a> StructMock<'a> {
//         pub fn get_number<'__rsubstitute_arg_anonymous>(&'__rsubstitute_arg_anonymous self) -> i32 {
//             let call = unsafe {
//                 get_number_Call {
//                     _phantom_lifetime: PhantomData,
//                 }
//             };
//             return self.data.get_number_data.handle_base_returning(self, call);
//         }
//
//         pub fn format<'__rsubstitute_arg_anonymous>(&'__rsubstitute_arg_anonymous self) -> String {
//             let call = unsafe {
//                 format_Call {
//                     _phantom_lifetime: PhantomData,
//                 }
//             };
//             return self.data.format_data.handle_base_returning(self, call);
//         }
//     }
//
//     impl<'a> StructMock<'a> {
//         #[allow(dead_code)]
//         pub fn new(number: i32) -> Self {
//             let data = Arc::new(StructMockData {
//                 _phantom_lifetime: PhantomData,
//                 MyTrait_work_data: FnData::new("MyTrait_work", &SERVICES),
//                 get_number_data: FnData::new("get_number", &SERVICES),
//                 format_data: FnData::new("format", &SERVICES),
//             });
//             let inner_data = Struct_InnerData::new(number);
//             return StructMock {
//                 setup: StructMockSetup {
//                     MyTrait: MyTraitSetup { data: data.clone() },
//                     data: data.clone(),
//                 },
//                 received: StructMockReceived {
//                     MyTrait: MyTraitReceived { data: data.clone() },
//                     data: data.clone(),
//                 },
//                 data,
//                 inner_data,
//             };
//         }
//     }
//
//     impl<'a> StructMockSetup<'a> {
//         #[allow(dead_code)]
//         #[allow(mismatched_lifetime_syntaxes)]
//         pub fn get_number(
//             &'a self,
//         ) -> FnTuner<
//             'a,
//             StructMock<'a>,
//             get_number_Call<'a>,
//             get_number_ArgsChecker<'a>,
//             i32,
//             Self,
//         > {
//             let get_number_args_checker = get_number_ArgsChecker {
//                 _phantom_lifetime: PhantomData,
//             };
//             let fn_config = self
//                 .data
//                 .get_number_data
//                 .add_config(get_number_args_checker);
//             let fn_tuner = FnTuner::new(fn_config, self);
//             return fn_tuner;
//         }
//         #[allow(dead_code)]
//         #[allow(mismatched_lifetime_syntaxes)]
//         pub fn format(
//             &'a self,
//         ) -> FnTuner<'a, StructMock<'a>, format_Call<'a>, format_ArgsChecker<'a>, String, Self>
//         {
//             let format_args_checker = format_ArgsChecker {
//                 _phantom_lifetime: PhantomData,
//             };
//             let fn_config = self.data.format_data.add_config(format_args_checker);
//             let fn_tuner = FnTuner::new(fn_config, self);
//             return fn_tuner;
//         }
//     }
//
//     impl<'a> StructMockReceived<'a> {
//         #[allow(dead_code)]
//         #[allow(mismatched_lifetime_syntaxes)]
//         pub fn get_number(&'a self, times: Times) -> &'a Self {
//             let get_number_args_checker = get_number_ArgsChecker {
//                 _phantom_lifetime: PhantomData,
//             };
//             self.data
//                 .get_number_data
//                 .verify_received(get_number_args_checker, times);
//             return self;
//         }
//         #[allow(dead_code)]
//         #[allow(mismatched_lifetime_syntaxes)]
//         pub fn format(&'a self, times: Times) -> &'a Self {
//             let format_args_checker = format_ArgsChecker {
//                 _phantom_lifetime: PhantomData,
//             };
//             self.data
//                 .format_data
//                 .verify_received(format_args_checker, times);
//             return self;
//         }
//         pub fn no_other_calls(&'a self) {
//             self.data.verify_received_nothing_else();
//         }
//     }
// }
