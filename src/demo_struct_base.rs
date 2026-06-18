#[cfg(not(test))]
struct Struct;
#[cfg(not(test))]
impl FirstTrait for Struct {
    fn get(&self) -> i32 {
        DEFAULT_FIRST_TRAIT_GET_VALUE
    }
}
#[cfg(not(test))]
impl SecondTrait for Struct {
    fn get(&self) -> &str {
        DEFAULT_SECOND_TRAIT_GET_VALUE
    }
}
#[cfg(not(test))]
impl Struct {
    pub fn new() -> Self {
        Self
    }

    pub fn get(&self) -> i32 {
        DEFAULT_STRUCT_GET_VALUE
    }

    pub fn get_plus_one(&self) -> i32 {
        let value = self.get() + FirstTrait::get(self);
        return value;
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
    pub struct FirstTrait_get_Call {}
    impl IArgsInfosProvider for FirstTrait_get_Call {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    impl IArgsTupleProvider for FirstTrait_get_Call {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new(())) as *mut _ as *mut ()
        }
    }
    impl IGenericsInfoProvider for FirstTrait_get_Call {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    impl Clone for FirstTrait_get_Call {
        fn clone(&self) -> Self {
            Self {}
        }
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct FirstTrait_get_ArgsChecker {}
    impl IArgsChecker for FirstTrait_get_ArgsChecker {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &FirstTrait_get_Call = dyn_call.downcast_ref();
            vec![]
        }
    }
    impl IArgsFormatter for FirstTrait_get_ArgsChecker {
        fn fmt_args(&self) -> String {
            format!("",)
        }
    }
    impl IGenericsInfoProvider for FirstTrait_get_ArgsChecker {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    pub struct FirstTraitSetup {
        data: Arc<StructData>,
    }
    impl Clone for FirstTraitSetup {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    #[doc(hidden)]
    pub struct FirstTraitReceived {
        data: Arc<StructData>,
    }
    impl Clone for FirstTraitReceived {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    impl FirstTraitSetup {
        pub fn get<'__rsa>(
            &self,
        ) -> FnConfigurator<'_, Struct, Self, (), i32, &Struct, true, true> {
            let FirstTrait_get_args_checker: FirstTrait_get_ArgsChecker =
                FirstTrait_get_ArgsChecker {};
            let fn_configurator: FnConfigurator<'_, Struct, Self, (), i32, &Struct, true, true> =
                self.data
                    .FirstTrait_get
                    .add_config(FirstTrait_get_args_checker, self);
            return transmute_lifetime!(fn_configurator);
        }
    }
    impl FirstTraitReceived {
        pub fn get<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let FirstTrait_get_args_checker: FirstTrait_get_ArgsChecker =
                FirstTrait_get_ArgsChecker {};
            self.data
                .FirstTrait_get
                .verify_received(FirstTrait_get_args_checker, times);
            return FnVerifier::new(self.clone());
        }
    }
    #[doc(hidden)]
    pub struct SecondTrait_get_Call {}
    impl IArgsInfosProvider for SecondTrait_get_Call {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    impl IArgsTupleProvider for SecondTrait_get_Call {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new(())) as *mut _ as *mut ()
        }
    }
    impl IGenericsInfoProvider for SecondTrait_get_Call {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    impl Clone for SecondTrait_get_Call {
        fn clone(&self) -> Self {
            Self {}
        }
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct SecondTrait_get_ArgsChecker {}
    impl IArgsChecker for SecondTrait_get_ArgsChecker {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &SecondTrait_get_Call = dyn_call.downcast_ref();
            vec![]
        }
    }
    impl IArgsFormatter for SecondTrait_get_ArgsChecker {
        fn fmt_args(&self) -> String {
            format!("",)
        }
    }
    impl IGenericsInfoProvider for SecondTrait_get_ArgsChecker {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    pub struct SecondTraitSetup {
        data: Arc<StructData>,
    }
    impl Clone for SecondTraitSetup {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    #[doc(hidden)]
    pub struct SecondTraitReceived {
        data: Arc<StructData>,
    }
    impl Clone for SecondTraitReceived {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    impl SecondTraitSetup {
        pub fn get<'__rsa>(
            &self,
        ) -> FnConfigurator<'_, Struct, Self, (), &'__rsa str, &Struct, true, true> {
            let SecondTrait_get_args_checker: SecondTrait_get_ArgsChecker =
                SecondTrait_get_ArgsChecker {};
            let fn_configurator: FnConfigurator<
                '_,
                Struct,
                Self,
                (),
                &'__rsa str,
                &Struct,
                true,
                true,
            > = self
                .data
                .SecondTrait_get
                .add_config(SecondTrait_get_args_checker, self);
            return transmute_lifetime!(fn_configurator);
        }
    }
    impl SecondTraitReceived {
        pub fn get<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let SecondTrait_get_args_checker: SecondTrait_get_ArgsChecker =
                SecondTrait_get_ArgsChecker {};
            self.data
                .SecondTrait_get
                .verify_received(SecondTrait_get_args_checker, times);
            return FnVerifier::new(self.clone());
        }
    }
    #[doc(hidden)]
    pub struct get_Call {}
    impl IArgsInfosProvider for get_Call {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    impl IArgsTupleProvider for get_Call {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new(())) as *mut _ as *mut ()
        }
    }
    impl IGenericsInfoProvider for get_Call {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    impl Clone for get_Call {
        fn clone(&self) -> Self {
            Self {}
        }
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct get_ArgsChecker {}
    impl IArgsChecker for get_ArgsChecker {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &get_Call = dyn_call.downcast_ref();
            vec![]
        }
    }
    impl IArgsFormatter for get_ArgsChecker {
        fn fmt_args(&self) -> String {
            format!("",)
        }
    }
    impl IGenericsInfoProvider for get_ArgsChecker {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    pub struct get_plus_one_Call {}
    impl IArgsInfosProvider for get_plus_one_Call {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    impl IArgsTupleProvider for get_plus_one_Call {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new(())) as *mut _ as *mut ()
        }
    }
    impl IGenericsInfoProvider for get_plus_one_Call {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    impl Clone for get_plus_one_Call {
        fn clone(&self) -> Self {
            Self {}
        }
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct get_plus_one_ArgsChecker {}
    impl IArgsChecker for get_plus_one_ArgsChecker {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &get_plus_one_Call = dyn_call.downcast_ref();
            vec![]
        }
    }
    impl IArgsFormatter for get_plus_one_ArgsChecker {
        fn fmt_args(&self) -> String {
            format!("",)
        }
    }
    impl IGenericsInfoProvider for get_plus_one_ArgsChecker {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct StructData {
        pub get: FnData<'static, Struct, true, true>,
        pub get_plus_one: FnData<'static, Struct, true, true>,
        pub FirstTrait_get: FnData<'static, Struct, true, true>,
        pub SecondTrait_get: FnData<'static, Struct, true, true>,
    }
    #[doc(hidden)]
    pub struct StructSetup {
        data: Arc<StructData>,
        pub as_FirstTrait: FirstTraitSetup,
        pub as_SecondTrait: SecondTraitSetup,
    }
    impl Clone for StructSetup {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
                as_FirstTrait: (&self.as_FirstTrait).clone(),
                as_SecondTrait: (&self.as_SecondTrait).clone(),
            }
        }
    }
    #[doc(hidden)]
    pub struct StructReceived {
        data: Arc<StructData>,
        pub as_FirstTrait: FirstTraitReceived,
        pub as_SecondTrait: SecondTraitReceived,
    }
    impl Clone for StructReceived {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
                as_FirstTrait: (&self.as_FirstTrait).clone(),
                as_SecondTrait: (&self.as_SecondTrait).clone(),
            }
        }
    }
    #[doc(hidden)]
    pub struct Struct_InnerData;

    impl Struct_InnerData {
        pub fn new() -> Self {
            Self
        }
    }
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
    impl FirstTrait for Struct {
        fn get(&self) -> i32 {
            let call: FirstTrait_get_Call = FirstTrait_get_Call {};
            return self.data.clone().FirstTrait_get.handle_base_returning(
                self,
                call,
                Self::base_FirstTrait_get,
            );
        }
    }
    impl SecondTrait for Struct {
        fn get(&self) -> &str {
            let call: SecondTrait_get_Call = SecondTrait_get_Call {};
            return self.data.clone().SecondTrait_get.handle_base_returning(
                self,
                call,
                Self::base_SecondTrait_get,
            );
        }
    }
    impl Struct {
        pub fn get(&self) -> i32 {
            let call: get_Call = get_Call {};
            return self
                .data
                .clone()
                .get
                .handle_base_returning(self, call, Self::base_get);
        }

        pub fn get_plus_one(&self) -> i32 {
            let call: get_plus_one_Call = get_plus_one_Call {};
            return self.data.clone().get_plus_one.handle_base_returning(
                self,
                call,
                Self::base_get_plus_one,
            );
        }
    }
    impl Struct {
        pub fn new() -> Self {
            let data = Arc::new(StructData {
                get: FnData::new("get"),
                get_plus_one: FnData::new("get_plus_one"),
                FirstTrait_get: FnData::new("FirstTrait::get"),
                SecondTrait_get: FnData::new("SecondTrait::get"),
            });
            let inner_data = Struct_InnerData::new();
            return Struct {
                setup: StructSetup {
                    data: data.clone(),
                    as_FirstTrait: FirstTraitSetup { data: data.clone() },
                    as_SecondTrait: SecondTraitSetup { data: data.clone() },
                },
                received: StructReceived {
                    data: data.clone(),
                    as_FirstTrait: FirstTraitReceived { data: data.clone() },
                    as_SecondTrait: SecondTraitReceived { data: data.clone() },
                },
                data,
                inner_data,
            };
        }
        fn base_get(self: &Struct, call: get_Call) -> i32 {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)]
            let get_Call { .. } = call;
            DEFAULT_STRUCT_GET_VALUE
        }

        fn base_get_plus_one(self: &Struct, call: get_plus_one_Call) -> i32 {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)]
            let get_plus_one_Call { .. } = call;
            let value = self.get() + FirstTrait::get(self);
            return value;
        }
        fn base_FirstTrait_get(self: &Struct, call: FirstTrait_get_Call) -> i32 {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)]
            let FirstTrait_get_Call { .. } = call;
            DEFAULT_FIRST_TRAIT_GET_VALUE
        }
        fn base_SecondTrait_get(self: &Struct, call: SecondTrait_get_Call) -> &str {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)]
            let SecondTrait_get_Call { .. } = call;
            DEFAULT_SECOND_TRAIT_GET_VALUE
        }
    }
    impl StructSetup {
        pub fn get<'__rsa>(
            &self,
        ) -> FnConfigurator<'_, Struct, Self, (), i32, &Struct, true, true> {
            let get_args_checker: get_ArgsChecker = get_ArgsChecker {};
            let fn_configurator: FnConfigurator<'_, Struct, Self, (), i32, &Struct, true, true> =
                self.data.get.add_config(get_args_checker, self);
            return transmute_lifetime!(fn_configurator);
        }
        pub fn get_plus_one<'__rsa>(
            &self,
        ) -> FnConfigurator<'_, Struct, Self, (), i32, &Struct, true, true> {
            let get_plus_one_args_checker: get_plus_one_ArgsChecker = get_plus_one_ArgsChecker {};
            let fn_configurator: FnConfigurator<'_, Struct, Self, (), i32, &Struct, true, true> =
                self.data
                    .get_plus_one
                    .add_config(get_plus_one_args_checker, self);
            return transmute_lifetime!(fn_configurator);
        }
    }
    impl StructReceived {
        pub fn get<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let get_args_checker: get_ArgsChecker = get_ArgsChecker {};
            self.data.get.verify_received(get_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn get_plus_one<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let get_plus_one_args_checker: get_plus_one_ArgsChecker = get_plus_one_ArgsChecker {};
            self.data
                .get_plus_one
                .verify_received(get_plus_one_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
}
