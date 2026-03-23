use rsubstitute::macros::*;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

// TODO - right now self type is always `&self` in base methods
// TODO - add also methods that accept mut self and that do not return anything
// TODO - also test callbacks - they must receive reference to same `self` type that was used in fn
// definition and VERIFY that data passed to them is correct
// type Alias = Struct;
// mocked_base! {
//     #[derive(Clone)]
//     struct Struct {
//         pub number: i32
//     }
//
//     impl Struct {
//         pub fn new(number: i32) -> Self { Self { number } }
//
//         pub fn mutate(&mut self) {}
//
//         pub fn consume(self) -> i32 { 10 }
//
//         pub fn sbox(self: Box<Self>) -> i32 { 212 }
//         pub fn src(self: Rc<Self>) -> i32 { 212 }
//         pub fn sarc(self: Arc<Self>) -> i32 { 212 }
//
//         pub fn spbox(self: Pin<Box<Self>>) -> i32 { 212 }
//         pub fn sprc(self: Pin<Rc<Self>>) -> i32 { 212 }
//         pub fn sparc(self: Pin<Arc<Self>>) -> i32 { 212 }
//
//         // TODO - support
//         pub fn nested<'a>(self: &mut &'a Arc<Rc<Box<Box<Struct>>>>) -> i32 { 212 }
//     }
// }

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use rsubstitute::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn flex() {
        let number = 45;
        let mut mock = Struct::new(number);

        mock.setup
            .sbox()
            .call_base()
            .and_does(move |s, _| assert_eq!(number, s.number));
        mock.setup.src().call_base();
        mock.setup.sarc().call_base();

        mock.setup.spbox().call_base();
        mock.setup.sprc().call_base();
        mock.setup.sparc().call_base();

        assert_eq!(212, Box::new(mock.clone()).sbox());
        assert_eq!(212, Box::new(mock.clone()).sbox());
        assert_eq!(212, Box::new(mock.clone()).sbox());
        mock.received.sbox(Times::Exactly(3));

        assert_eq!(212, Box::new(mock.clone()).sbox());
        assert_eq!(212, Rc::new(mock.clone()).src());
        assert_eq!(212, Arc::new(mock.clone()).sarc());
        mock.received.sbox(Times::Exactly(4));
        mock.received.src(Times::Once);
        mock.received.sarc(Times::Once);

        assert_eq!(212, Pin::new(Box::new(mock.clone())).spbox());
        assert_eq!(212, Pin::new(Rc::new(mock.clone())).sprc());
        assert_eq!(212, Pin::new(Arc::new(mock.clone())).sparc());
        mock.received.spbox(Times::Once);
        mock.received.sprc(Times::Once);
        mock.received.sparc(Times::Once);

        let flag = Rc::new(Cell::new(false));
        let flag_clone = flag.clone();

        mock.setup.mutate().does(move |_, _| flag_clone.set(true));
        mock.setup.consume().call_base();

        mock.mutate();
        assert!(flag.get());
        let value = mock.clone().consume();
        assert_eq!(10, value);

        mock.received
            .mutate(Times::Once)
            .consume(Times::Once)
            .no_other_calls();
    }

    #[test]
    fn compile() {}
}

#[cfg(not(test))]
#[derive(Clone)]
struct Struct {
    pub number: i32,
}
#[cfg(not(test))]
impl Struct {
    pub fn new(number: i32) -> Self {
        Self { number }
    }

    pub fn mutate(&mut self) {}

    pub fn consume(self) -> i32 {
        10
    }

    pub fn sbox(self: Box<Self>) -> i32 {
        212
    }
    pub fn src(self: Rc<Self>) -> i32 {
        212
    }
    pub fn sarc(self: Arc<Self>) -> i32 {
        212
    }

    pub fn spbox(self: Pin<Box<Self>>) -> i32 {
        212
    }
    pub fn sprc(self: Pin<Rc<Self>>) -> i32 {
        212
    }
    pub fn sparc(self: Pin<Arc<Self>>) -> i32 {
        212
    }

    // TODO - support
    pub fn nested<'a>(self: &mut &'a Arc<Rc<Box<Box<Struct>>>>) -> i32 {
        212
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
    #[doc(hidden)]
    pub struct mutate_Call {}
    impl IArgsInfosProvider for mutate_Call {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    impl IArgsTupleProvider for mutate_Call {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new(())) as *mut _ as *mut ()
        }
    }
    impl IGenericsInfoProvider for mutate_Call {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    impl Clone for mutate_Call {
        fn clone(&self) -> Self {
            Self {}
        }
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct mutate_ArgsChecker {}
    impl IArgsChecker for mutate_ArgsChecker {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &mutate_Call = dyn_call.downcast_ref();
            vec![]
        }
    }
    impl IArgsFormatter for mutate_ArgsChecker {
        fn fmt_args(&self) -> String {
            format!("",)
        }
    }
    impl IGenericsInfoProvider for mutate_ArgsChecker {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    pub struct consume_Call {}
    impl IArgsInfosProvider for consume_Call {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    impl IArgsTupleProvider for consume_Call {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new(())) as *mut _ as *mut ()
        }
    }
    impl IGenericsInfoProvider for consume_Call {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    impl Clone for consume_Call {
        fn clone(&self) -> Self {
            Self {}
        }
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct consume_ArgsChecker {}
    impl IArgsChecker for consume_ArgsChecker {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &consume_Call = dyn_call.downcast_ref();
            vec![]
        }
    }
    impl IArgsFormatter for consume_ArgsChecker {
        fn fmt_args(&self) -> String {
            format!("",)
        }
    }
    impl IGenericsInfoProvider for consume_ArgsChecker {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    pub struct sbox_Call {}
    impl IArgsInfosProvider for sbox_Call {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    impl IArgsTupleProvider for sbox_Call {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new(())) as *mut _ as *mut ()
        }
    }
    impl IGenericsInfoProvider for sbox_Call {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    impl Clone for sbox_Call {
        fn clone(&self) -> Self {
            Self {}
        }
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct sbox_ArgsChecker {}
    impl IArgsChecker for sbox_ArgsChecker {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &sbox_Call = dyn_call.downcast_ref();
            vec![]
        }
    }
    impl IArgsFormatter for sbox_ArgsChecker {
        fn fmt_args(&self) -> String {
            format!("",)
        }
    }
    impl IGenericsInfoProvider for sbox_ArgsChecker {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    pub struct src_Call {}
    impl IArgsInfosProvider for src_Call {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    impl IArgsTupleProvider for src_Call {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new(())) as *mut _ as *mut ()
        }
    }
    impl IGenericsInfoProvider for src_Call {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    impl Clone for src_Call {
        fn clone(&self) -> Self {
            Self {}
        }
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct src_ArgsChecker {}
    impl IArgsChecker for src_ArgsChecker {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &src_Call = dyn_call.downcast_ref();
            vec![]
        }
    }
    impl IArgsFormatter for src_ArgsChecker {
        fn fmt_args(&self) -> String {
            format!("",)
        }
    }
    impl IGenericsInfoProvider for src_ArgsChecker {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    pub struct sarc_Call {}
    impl IArgsInfosProvider for sarc_Call {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    impl IArgsTupleProvider for sarc_Call {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new(())) as *mut _ as *mut ()
        }
    }
    impl IGenericsInfoProvider for sarc_Call {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    impl Clone for sarc_Call {
        fn clone(&self) -> Self {
            Self {}
        }
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct sarc_ArgsChecker {}
    impl IArgsChecker for sarc_ArgsChecker {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &sarc_Call = dyn_call.downcast_ref();
            vec![]
        }
    }
    impl IArgsFormatter for sarc_ArgsChecker {
        fn fmt_args(&self) -> String {
            format!("",)
        }
    }
    impl IGenericsInfoProvider for sarc_ArgsChecker {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    pub struct spbox_Call {}
    impl IArgsInfosProvider for spbox_Call {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    impl IArgsTupleProvider for spbox_Call {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new(())) as *mut _ as *mut ()
        }
    }
    impl IGenericsInfoProvider for spbox_Call {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    impl Clone for spbox_Call {
        fn clone(&self) -> Self {
            Self {}
        }
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct spbox_ArgsChecker {}
    impl IArgsChecker for spbox_ArgsChecker {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &spbox_Call = dyn_call.downcast_ref();
            vec![]
        }
    }
    impl IArgsFormatter for spbox_ArgsChecker {
        fn fmt_args(&self) -> String {
            format!("",)
        }
    }
    impl IGenericsInfoProvider for spbox_ArgsChecker {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    pub struct sprc_Call {}
    impl IArgsInfosProvider for sprc_Call {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    impl IArgsTupleProvider for sprc_Call {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new(())) as *mut _ as *mut ()
        }
    }
    impl IGenericsInfoProvider for sprc_Call {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    impl Clone for sprc_Call {
        fn clone(&self) -> Self {
            Self {}
        }
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct sprc_ArgsChecker {}
    impl IArgsChecker for sprc_ArgsChecker {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &sprc_Call = dyn_call.downcast_ref();
            vec![]
        }
    }
    impl IArgsFormatter for sprc_ArgsChecker {
        fn fmt_args(&self) -> String {
            format!("",)
        }
    }
    impl IGenericsInfoProvider for sprc_ArgsChecker {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    pub struct sparc_Call {}
    impl IArgsInfosProvider for sparc_Call {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    impl IArgsTupleProvider for sparc_Call {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new(())) as *mut _ as *mut ()
        }
    }
    impl IGenericsInfoProvider for sparc_Call {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    impl Clone for sparc_Call {
        fn clone(&self) -> Self {
            Self {}
        }
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct sparc_ArgsChecker {}
    impl IArgsChecker for sparc_ArgsChecker {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &sparc_Call = dyn_call.downcast_ref();
            vec![]
        }
    }
    impl IArgsFormatter for sparc_ArgsChecker {
        fn fmt_args(&self) -> String {
            format!("",)
        }
    }
    impl IGenericsInfoProvider for sparc_ArgsChecker {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    pub struct nested_Call<'a> {
        _phantom_GenericParam_a: PhantomData<&'a ()>,
    }
    impl<'a> IArgsInfosProvider for nested_Call<'a> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    impl<'a> IArgsTupleProvider for nested_Call<'a> {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new(())) as *mut _ as *mut ()
        }
    }
    impl<'a> IGenericsInfoProvider for nested_Call<'a> {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    impl<'a> Clone for nested_Call<'a> {
        fn clone(&self) -> Self {
            Self {
                _phantom_GenericParam_a: (&self._phantom_GenericParam_a).clone(),
            }
        }
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct nested_ArgsChecker<'a> {
        _phantom_GenericParam_a: PhantomData<&'a ()>,
    }
    impl<'a> IArgsChecker for nested_ArgsChecker<'a> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &nested_Call<'a> = dyn_call.downcast_ref();
            vec![]
        }
    }
    impl<'a> IArgsFormatter for nested_ArgsChecker<'a> {
        fn fmt_args(&self) -> String {
            format!("",)
        }
    }
    impl<'a> IGenericsInfoProvider for nested_ArgsChecker<'a> {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct StructData {
        pub mutate: FnData<'static, Struct, true, true>,
        pub consume: FnData<'static, Struct, true, true>,
        pub sbox: FnData<'static, Struct, true, true>,
        pub src: FnData<'static, Struct, true, true>,
        pub sarc: FnData<'static, Struct, true, true>,
        pub spbox: FnData<'static, Struct, true, true>,
        pub sprc: FnData<'static, Struct, true, true>,
        pub sparc: FnData<'static, Struct, true, true>,
        pub nested: FnData<'static, Struct, true, true>,
    }
    #[doc(hidden)]
    pub struct StructSetup {
        data: Arc<StructData>,
    }
    impl Clone for StructSetup {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    #[doc(hidden)]
    pub struct StructReceived {
        data: Arc<StructData>,
    }
    impl Clone for StructReceived {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    #[derive(Clone)]
    #[doc(hidden)]
    pub struct Struct_InnerData {
        pub number: i32,
    }

    impl Struct_InnerData {
        pub fn new(number: i32) -> Self {
            Self { number }
        }
    }

    #[derive(Clone)]
    pub struct Struct {
        pub setup: StructSetup,
        pub received: StructReceived,
        pub data: Arc<StructData>,
        inner_data: Struct_InnerData,
    }
    impl AsRef<Struct> for Struct {
        fn as_ref(&self) -> &Struct {
            self
        }
    }
    impl Deref for Struct {
        type Target = Struct_InnerData;
        fn deref(&self) -> &Self::Target {
            &self.inner_data
        }
    }
    impl Struct {
        pub fn mutate(&mut self) {
            let call: mutate_Call = mutate_Call {};
            self.data
                .clone()
                .mutate
                .handle_base(self, call, Self::base_mutate);
        }

        pub fn consume(self) -> i32 {
            let call: consume_Call = consume_Call {};
            return self
                .data
                .clone()
                .consume
                .handle_base_returning(self, call, Self::base_consume);
        }

        pub fn sbox(self: Box<Self>) -> i32 {
            let call: sbox_Call = sbox_Call {};
            return self
                .data
                .clone()
                .sbox
                .handle_base_returning(self, call, Self::base_sbox);
        }
        pub fn src(self: Rc<Self>) -> i32 {
            let call: src_Call = src_Call {};
            return self
                .data
                .clone()
                .src
                .handle_base_returning(self, call, Self::base_src);
        }
        pub fn sarc(self: Arc<Self>) -> i32 {
            let call: sarc_Call = sarc_Call {};
            return self
                .data
                .clone()
                .sarc
                .handle_base_returning(self, call, Self::base_sarc);
        }

        pub fn spbox(self: Pin<Box<Self>>) -> i32 {
            let call: spbox_Call = spbox_Call {};
            return self
                .data
                .clone()
                .spbox
                .handle_base_returning(self, call, Self::base_spbox);
        }
        pub fn sprc(self: Pin<Rc<Self>>) -> i32 {
            let call: sprc_Call = sprc_Call {};
            return self
                .data
                .clone()
                .sprc
                .handle_base_returning(self, call, Self::base_sprc);
        }
        pub fn sparc(self: Pin<Arc<Self>>) -> i32 {
            let call: sparc_Call = sparc_Call {};
            return self
                .data
                .clone()
                .sparc
                .handle_base_returning(self, call, Self::base_sparc);
        }

        // TODO - support
        pub fn nested<'a>(self: &mut &'a Arc<Rc<Box<Box<Struct>>>>) -> i32 {
            let call: nested_Call<'_> = nested_Call {
                _phantom_GenericParam_a: PhantomData,
            };
            return self
                .data
                .clone()
                .nested
                .handle_base_returning(self, call, Self::base_nested);
        }
    }
    impl Struct {
        pub fn new(number: i32) -> Self {
            let data = Arc::new(StructData {
                mutate: FnData::new("mutate"),
                consume: FnData::new("consume"),
                sbox: FnData::new("sbox"),
                src: FnData::new("src"),
                sarc: FnData::new("sarc"),
                spbox: FnData::new("spbox"),
                sprc: FnData::new("sprc"),
                sparc: FnData::new("sparc"),
                nested: FnData::new("nested"),
            });
            let inner_data = Struct_InnerData::new(number);
            return Struct {
                setup: StructSetup { data: data.clone() },
                received: StructReceived { data: data.clone() },
                data,
                inner_data,
            };
        }
        fn base_mutate(&mut self, call: mutate_Call) {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)]
            let mutate_Call { .. } = call;
        }
        fn base_consume(self, call: consume_Call) -> i32 {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)]
            let consume_Call { .. } = call;
            10
        }

        fn base_sbox(self: Box<Struct>, call: sbox_Call) -> i32 {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)]
            let sbox_Call { .. } = call;
            212
        }
        fn base_src(self: Rc<Struct>, call: src_Call) -> i32 {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)]
            let src_Call { .. } = call;
            212
        }
        fn base_sarc(self: Arc<Struct>, call: sarc_Call) -> i32 {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)]
            let sarc_Call { .. } = call;
            212
        }

        fn base_spbox(self: Pin<Box<Struct>>, call: spbox_Call) -> i32 {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)]
            let spbox_Call { .. } = call;
            212
        }
        fn base_sprc(self: Pin<Rc<Struct>>, call: sprc_Call) -> i32 {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)]
            let sprc_Call { .. } = call;
            212
        }
        fn base_sparc(self: Pin<Arc<Struct>>, call: sparc_Call) -> i32 {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)]
            let sparc_Call { .. } = call;
            212
        }

        // TODO - support
        fn base_nested<'a>(self: &mut &'a Arc<Rc<Box<Box<Struct>>>>, call: nested_Call<'a>) -> i32 {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)]
            let nested_Call::<'_> { .. } = call;
            212
        }
    }
    impl StructSetup {
        pub fn mutate<'__rsa>(&self) -> FnTuner<'_, Struct, Self, (), (), &mut Self, true, true> {
            let mutate_args_checker: mutate_ArgsChecker = mutate_ArgsChecker {};
            let fn_tuner: FnTuner<'_, Struct, Self, (), (), &mut Self, true, true> =
                self.data.mutate.add_config(mutate_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
        pub fn consume<'__rsa>(&self) -> FnTuner<'_, Struct, Self, (), i32, Self, true, true> {
            let consume_args_checker: consume_ArgsChecker = consume_ArgsChecker {};
            let fn_tuner: FnTuner<'_, Struct, Self, (), i32, Self, true, true> =
                self.data.consume.add_config(consume_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
        pub fn sbox<'__rsa>(&self) -> FnTuner<'_, Struct, Self, (), i32, Box<Struct>, true, true> {
            let sbox_args_checker: sbox_ArgsChecker = sbox_ArgsChecker {};
            let fn_tuner: FnTuner<'_, Struct, Self, (), i32, Box<Struct>, true, true> =
                self.data.sbox.add_config(sbox_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
        pub fn src<'__rsa>(&self) -> FnTuner<'_, Struct, Self, (), i32, Rc<Struct>, true, true> {
            let src_args_checker: src_ArgsChecker = src_ArgsChecker {};
            let fn_tuner: FnTuner<'_, Struct, Self, (), i32, Rc<Struct>, true, true> =
                self.data.src.add_config(src_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
        pub fn sarc<'__rsa>(&self) -> FnTuner<'_, Struct, Self, (), i32, Arc<Struct>, true, true> {
            let sarc_args_checker: sarc_ArgsChecker = sarc_ArgsChecker {};
            let fn_tuner: FnTuner<'_, Struct, Self, (), i32, Arc<Struct>, true, true> =
                self.data.sarc.add_config(sarc_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
        pub fn spbox<'__rsa>(
            &self,
        ) -> FnTuner<'_, Struct, Self, (), i32, Pin<Box<Struct>>, true, true> {
            let spbox_args_checker: spbox_ArgsChecker = spbox_ArgsChecker {};
            let fn_tuner: FnTuner<'_, Struct, Self, (), i32, Pin<Box<Struct>>, true, true> =
                self.data.spbox.add_config(spbox_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
        pub fn sprc<'__rsa>(
            &self,
        ) -> FnTuner<'_, Struct, Self, (), i32, Pin<Rc<Struct>>, true, true> {
            let sprc_args_checker: sprc_ArgsChecker = sprc_ArgsChecker {};
            let fn_tuner: FnTuner<'_, Struct, Self, (), i32, Pin<Rc<Struct>>, true, true> =
                self.data.sprc.add_config(sprc_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
        pub fn sparc<'__rsa>(
            &self,
        ) -> FnTuner<'_, Struct, Self, (), i32, Pin<Arc<Struct>>, true, true> {
            let sparc_args_checker: sparc_ArgsChecker = sparc_ArgsChecker {};
            let fn_tuner: FnTuner<'_, Struct, Self, (), i32, Pin<Arc<Struct>>, true, true> =
                self.data.sparc.add_config(sparc_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
        pub fn nested<'__rsa, 'a>(
            &self,
        ) -> FnTuner<'_, Struct, Self, (), i32, &mut &'a Arc<Rc<Box<Box<Struct>>>>, true, true>
        {
            let nested_args_checker: nested_ArgsChecker<'a> = nested_ArgsChecker {
                _phantom_GenericParam_a: PhantomData,
            };
            let fn_tuner: FnTuner<
                '_,
                Struct,
                Self,
                (),
                i32,
                &mut &'a Arc<Rc<Box<Box<Struct>>>>,
                true,
                true,
            > = self.data.nested.add_config(nested_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
    }
    impl StructReceived {
        pub fn mutate<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let mutate_args_checker: mutate_ArgsChecker = mutate_ArgsChecker {};
            self.data.mutate.verify_received(mutate_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn consume<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let consume_args_checker: consume_ArgsChecker = consume_ArgsChecker {};
            self.data
                .consume
                .verify_received(consume_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn sbox<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let sbox_args_checker: sbox_ArgsChecker = sbox_ArgsChecker {};
            self.data.sbox.verify_received(sbox_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn src<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let src_args_checker: src_ArgsChecker = src_ArgsChecker {};
            self.data.src.verify_received(src_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn sarc<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let sarc_args_checker: sarc_ArgsChecker = sarc_ArgsChecker {};
            self.data.sarc.verify_received(sarc_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn spbox<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let spbox_args_checker: spbox_ArgsChecker = spbox_ArgsChecker {};
            self.data.spbox.verify_received(spbox_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn sprc<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let sprc_args_checker: sprc_ArgsChecker = sprc_ArgsChecker {};
            self.data.sprc.verify_received(sprc_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn sparc<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let sparc_args_checker: sparc_ArgsChecker = sparc_ArgsChecker {};
            self.data.sparc.verify_received(sparc_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn nested<'__rsa, 'a>(&self, times: Times) -> FnVerifier<Self, ()> {
            let nested_args_checker: nested_ArgsChecker<'a> = nested_ArgsChecker {
                _phantom_GenericParam_a: PhantomData,
            };
            self.data.nested.verify_received(nested_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
}
