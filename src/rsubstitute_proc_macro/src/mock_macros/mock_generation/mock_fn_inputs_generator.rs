// use crate::mock_macros::models::FnArgInfo;
// use syn::*;
// 
// pub trait IMockFnInputsGenerator {
//     fn generate(&self, original_inputs: &[FnArgInfo]) -> Vec<FnArg>;
// }
// 
// pub(crate) struct MockFnInputsGenerator;
// 
// impl IMockFnInputsGenerator for MockFnInputsGenerator {
//     fn generate(&self, original_inputs: &[FnArgInfo]) -> Vec<FnArg> {
//         let inputs = original_inputs
//             .iter()
//             .map(|fn_arg| match fn_arg {
//                 FnArgInfo::Receiver(receiver) => FnArg::Receiver(receiver.clone()),
//                 FnArgInfo::Typed(typed) => typed.clone_as_fn_arg(),
//             })
//             .collect();
//         return inputs;
//     }
// }
