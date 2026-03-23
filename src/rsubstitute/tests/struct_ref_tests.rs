use rsubstitute::macros::*;

// mocked! {
//     struct Struct;
//
//     impl Struct {
//         pub fn new() -> Self { Self }
//
//         pub fn accept_ref(&self, r: &i32) { unreachable!() }
//
//         pub fn return_ref(&self) -> &'static i32 { unreachable!() }
//
//         pub fn accept_ref_return_ref(&self, r: &i32) -> &'static i32 { unreachable!() }
//
//         pub fn accept_two_refs(&self, r1: &i32, r2: &f32) { unreachable!() }
//
//         pub fn accept_two_refs_return_ref(&self, r1: &i32, r2: &f32) -> &'static str { unreachable!() }
//     }
// }

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::panics::*;
    use rsubstitute::*;

    mod accept_ref_tests {
        use super::*;

        #[test]
        fn accept_ref_Ok() {
            // Arrange
            let mock = Struct::new();
            let r = &1;

            // Act
            mock.accept_ref(r);

            // Assert
            mock.received.accept_ref(r, Times::Once).no_other_calls();
        }

        #[test]
        fn accept_rc_Panics() {
            // Arrange
            let mock = Struct::new();
            let r = &11;
            let r_ptr = core::ptr::from_ref(r);

            // Act
            mock.accept_ref(r);

            // Assert
            assert_panics(
                || mock.received.accept_ref(Arg::Any, Times::Never),
                format!(
                    "Expected to never receive a call matching:
	accept_ref((&i32): any)
Actually received 1 matching call:
	accept_ref({r})
Received no non-matching calls"
                ),
            );

            assert_panics(
                || mock.received.accept_ref(Arg::Any, Times::Exactly(3)),
                format!(
                    "Expected to receive a call 3 times matching:
	accept_ref((&i32): any)
Actually received 1 matching call:
	accept_ref({r})
Received no non-matching calls"
                ),
            );

            let invalid_r = &22;
            let invalid_r_ptr = core::ptr::from_ref(invalid_r);
            assert_panics(
                || mock.received.accept_ref(invalid_r, Times::Once),
                format!(
                    "Expected to receive a call exactly once matching:
	accept_ref((&i32): equal to {invalid_r})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_ref(*{r}*)
	1. r (&i32):
		Expected reference (ptr: {invalid_r_ptr:?}): {invalid_r}
		Actual reference   (ptr: {r_ptr:?}): {r}"
                ),
            )
        }
    }

    mod return_ref_tests {
        use super::*;

        #[test]
        fn return_ref_Ok() {
            // Arrange
            let mock = Struct::new();
            let r = Box::leak(Box::new(11));
            mock.setup.return_ref().returns(r);

            // Act
            let actual_r = mock.return_ref();

            // Assert
            assert_eq!(r, actual_r);
        }
    }

    mod accept_ref_return_ref_tests {
        use super::*;

        #[test]
        fn accept_ref_return_ref_Ok() {
            // Arrange
            let mock = Struct::new();
            let accepted_r = &10;
            let returned_r = &20;
            mock.setup
                .accept_ref_return_ref(accepted_r)
                .returns(returned_r);

            // Act
            let actual_returned_r = mock.accept_ref_return_ref(accepted_r);

            // Assert
            assert_eq!(returned_r, actual_returned_r);

            mock.received
                .accept_ref_return_ref(accepted_r, Times::Once)
                .accept_ref_return_ref(Arg::not_eq(accepted_r), Times::Never)
                .no_other_calls();
        }
    }

    mod accept_two_refs_tests {
        use super::*;

        #[test]
        fn accept_two_refs_Ok() {
            // Arrange
            let mock = Struct::new();
            let r1 = &10;
            let r2 = &20.2;

            // Act
            mock.accept_two_refs(r1, r2);

            // Assert
            mock.received
                .accept_two_refs(r1, r2, Times::Once)
                .accept_two_refs(Arg::not_eq(r1), Arg::not_eq(r2), Times::Never)
                .no_other_calls();
        }
    }

    mod accept_two_refs_return_ref_tests {
        use super::*;

        #[test]
        fn accept_two_refs_return_ref_Ok() {
            // Arrange
            let mock = Struct::new();
            let r1 = &10;
            let r2 = &20.2;
            let returned_r = "veridis quo";
            mock.setup
                .accept_two_refs_return_ref(r1, r2)
                .returns(returned_r);

            // Act
            let actual_returned_r = mock.accept_two_refs_return_ref(r1, r2);

            // Assert
            assert_eq!(returned_r, actual_returned_r);

            mock.received
                .accept_two_refs_return_ref(r1, r2, Times::Once)
                .no_other_calls();
        }
    }
}

#[cfg(not(test))]
struct Struct;
#[cfg(not(test))]
impl Struct {
    pub fn new() -> Self {
        Self
    }

    pub fn accept_ref(&self, r: &i32) {
        unreachable!()
    }

    pub fn return_ref(&self) -> &'static i32 {
        unreachable!()
    }

    pub fn accept_ref_return_ref(&self, r: &i32) -> &'static i32 {
        unreachable!()
    }

    pub fn accept_two_refs(&self, r1: &i32, r2: &f32) {
        unreachable!()
    }

    pub fn accept_two_refs_return_ref(&self, r1: &i32, r2: &f32) -> &'static str {
        unreachable!()
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
    pub struct accept_ref_Call {
        _phantom_r: PhantomData<*const i32>,
        r: *const i32,
    }
    impl IArgsInfosProvider for accept_ref_Call {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![ArgInfo::new(
                "r",
                &self.r,
                (&ArgPrinter::<&i32>(transmute_lifetime!(&self.r))).debug_string(),
            )]
        }
    }
    impl IArgsTupleProvider for accept_ref_Call {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((&self.r,))) as *mut _ as *mut ()
        }
    }
    impl IGenericsInfoProvider for accept_ref_Call {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct accept_ref_ArgsChecker {
        _phantom_r: PhantomData<*const i32>,
        r: Arg<*const i32>,
    }
    impl IArgsChecker for accept_ref_ArgsChecker {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &accept_ref_Call = dyn_call.downcast_ref();
            vec![self.r.check_ref(
                "r",
                transmute_lifetime!(&call.r),
                (&ArgPrinter::<&i32>(transmute_lifetime!(&call.r))).debug_string(),
            )]
        }
    }
    impl IArgsFormatter for accept_ref_ArgsChecker {
        fn fmt_args(&self) -> String {
            format!("{}", (&ArgPrinter(&self.r)).debug_string())
        }
    }
    impl IGenericsInfoProvider for accept_ref_ArgsChecker {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    pub struct return_ref_Call {}
    impl IArgsInfosProvider for return_ref_Call {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    impl IArgsTupleProvider for return_ref_Call {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new(())) as *mut _ as *mut ()
        }
    }
    impl IGenericsInfoProvider for return_ref_Call {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct return_ref_ArgsChecker {}
    impl IArgsChecker for return_ref_ArgsChecker {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &return_ref_Call = dyn_call.downcast_ref();
            vec![]
        }
    }
    impl IArgsFormatter for return_ref_ArgsChecker {
        fn fmt_args(&self) -> String {
            format!("",)
        }
    }
    impl IGenericsInfoProvider for return_ref_ArgsChecker {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    pub struct accept_ref_return_ref_Call {
        _phantom_r: PhantomData<*const i32>,
        r: *const i32,
    }
    impl IArgsInfosProvider for accept_ref_return_ref_Call {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![ArgInfo::new(
                "r",
                &self.r,
                (&ArgPrinter::<&i32>(transmute_lifetime!(&self.r))).debug_string(),
            )]
        }
    }
    impl IArgsTupleProvider for accept_ref_return_ref_Call {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((&self.r,))) as *mut _ as *mut ()
        }
    }
    impl IGenericsInfoProvider for accept_ref_return_ref_Call {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct accept_ref_return_ref_ArgsChecker {
        _phantom_r: PhantomData<*const i32>,
        r: Arg<*const i32>,
    }
    impl IArgsChecker for accept_ref_return_ref_ArgsChecker {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &accept_ref_return_ref_Call = dyn_call.downcast_ref();
            vec![self.r.check_ref(
                "r",
                transmute_lifetime!(&call.r),
                (&ArgPrinter::<&i32>(transmute_lifetime!(&call.r))).debug_string(),
            )]
        }
    }
    impl IArgsFormatter for accept_ref_return_ref_ArgsChecker {
        fn fmt_args(&self) -> String {
            format!("{}", (&ArgPrinter(&self.r)).debug_string())
        }
    }
    impl IGenericsInfoProvider for accept_ref_return_ref_ArgsChecker {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    pub struct accept_two_refs_Call {
        _phantom_r1: PhantomData<*const i32>,
        _phantom_r2: PhantomData<*const f32>,
        r1: *const i32,
        r2: *const f32,
    }
    impl IArgsInfosProvider for accept_two_refs_Call {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![
                ArgInfo::new(
                    "r1",
                    &self.r1,
                    (&ArgPrinter::<&i32>(transmute_lifetime!(&self.r1))).debug_string(),
                ),
                ArgInfo::new(
                    "r2",
                    &self.r2,
                    (&ArgPrinter::<&f32>(transmute_lifetime!(&self.r2))).debug_string(),
                ),
            ]
        }
    }
    impl IArgsTupleProvider for accept_two_refs_Call {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((&self.r1, &self.r2))) as *mut _ as *mut ()
        }
    }
    impl IGenericsInfoProvider for accept_two_refs_Call {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct accept_two_refs_ArgsChecker {
        _phantom_r1: PhantomData<*const i32>,
        _phantom_r2: PhantomData<*const f32>,
        r1: Arg<*const i32>,
        r2: Arg<*const f32>,
    }
    impl IArgsChecker for accept_two_refs_ArgsChecker {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &accept_two_refs_Call = dyn_call.downcast_ref();
            vec![
                self.r1.check_ref(
                    "r1",
                    transmute_lifetime!(&call.r1),
                    (&ArgPrinter::<&i32>(transmute_lifetime!(&call.r1))).debug_string(),
                ),
                self.r2.check_ref(
                    "r2",
                    transmute_lifetime!(&call.r2),
                    (&ArgPrinter::<&f32>(transmute_lifetime!(&call.r2))).debug_string(),
                ),
            ]
        }
    }
    impl IArgsFormatter for accept_two_refs_ArgsChecker {
        fn fmt_args(&self) -> String {
            format!(
                "{}, {}",
                (&ArgPrinter(&self.r1)).debug_string(),
                (&ArgPrinter(&self.r2)).debug_string()
            )
        }
    }
    impl IGenericsInfoProvider for accept_two_refs_ArgsChecker {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    pub struct accept_two_refs_return_ref_Call {
        _phantom_r1: PhantomData<*const i32>,
        _phantom_r2: PhantomData<*const f32>,
        r1: *const i32,
        r2: *const f32,
    }
    impl IArgsInfosProvider for accept_two_refs_return_ref_Call {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![
                ArgInfo::new(
                    "r1",
                    &self.r1,
                    (&ArgPrinter::<&i32>(transmute_lifetime!(&self.r1))).debug_string(),
                ),
                ArgInfo::new(
                    "r2",
                    &self.r2,
                    (&ArgPrinter::<&f32>(transmute_lifetime!(&self.r2))).debug_string(),
                ),
            ]
        }
    }
    impl IArgsTupleProvider for accept_two_refs_return_ref_Call {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((&self.r1, &self.r2))) as *mut _ as *mut ()
        }
    }
    impl IGenericsInfoProvider for accept_two_refs_return_ref_Call {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct accept_two_refs_return_ref_ArgsChecker {
        _phantom_r1: PhantomData<*const i32>,
        _phantom_r2: PhantomData<*const f32>,
        r1: Arg<*const i32>,
        r2: Arg<*const f32>,
    }
    impl IArgsChecker for accept_two_refs_return_ref_ArgsChecker {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &accept_two_refs_return_ref_Call = dyn_call.downcast_ref();
            vec![
                self.r1.check_ref(
                    "r1",
                    transmute_lifetime!(&call.r1),
                    (&ArgPrinter::<&i32>(transmute_lifetime!(&call.r1))).debug_string(),
                ),
                self.r2.check_ref(
                    "r2",
                    transmute_lifetime!(&call.r2),
                    (&ArgPrinter::<&f32>(transmute_lifetime!(&call.r2))).debug_string(),
                ),
            ]
        }
    }
    impl IArgsFormatter for accept_two_refs_return_ref_ArgsChecker {
        fn fmt_args(&self) -> String {
            format!(
                "{}, {}",
                (&ArgPrinter(&self.r1)).debug_string(),
                (&ArgPrinter(&self.r2)).debug_string()
            )
        }
    }
    impl IGenericsInfoProvider for accept_two_refs_return_ref_ArgsChecker {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct StructData {
        pub accept_ref: FnData<'static, Struct, false, true>,
        pub return_ref: FnData<'static, Struct, false, true>,
        pub accept_ref_return_ref: FnData<'static, Struct, false, true>,
        pub accept_two_refs: FnData<'static, Struct, false, true>,
        pub accept_two_refs_return_ref: FnData<'static, Struct, false, true>,
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
    impl Struct {
        pub fn accept_ref(&self, r: &i32) {
            let call: accept_ref_Call = accept_ref_Call {
                _phantom_r: PhantomData,
                r: transmute_lifetime!(r),
            };
            self.data.clone().accept_ref.handle(self, call);
        }

        pub fn return_ref(&self) -> &'static i32 {
            let call: return_ref_Call = return_ref_Call {};
            return self.data.clone().return_ref.handle_returning(self, call);
        }

        pub fn accept_ref_return_ref(&self, r: &i32) -> &'static i32 {
            let call: accept_ref_return_ref_Call = accept_ref_return_ref_Call {
                _phantom_r: PhantomData,
                r: transmute_lifetime!(r),
            };
            return self
                .data
                .clone()
                .accept_ref_return_ref
                .handle_returning(self, call);
        }

        pub fn accept_two_refs(&self, r1: &i32, r2: &f32) {
            let call: accept_two_refs_Call = accept_two_refs_Call {
                _phantom_r1: PhantomData,
                _phantom_r2: PhantomData,
                r1: transmute_lifetime!(r1),
                r2: transmute_lifetime!(r2),
            };
            self.data.clone().accept_two_refs.handle(self, call);
        }

        pub fn accept_two_refs_return_ref(&self, r1: &i32, r2: &f32) -> &'static str {
            let call: accept_two_refs_return_ref_Call = accept_two_refs_return_ref_Call {
                _phantom_r1: PhantomData,
                _phantom_r2: PhantomData,
                r1: transmute_lifetime!(r1),
                r2: transmute_lifetime!(r2),
            };
            return self
                .data
                .clone()
                .accept_two_refs_return_ref
                .handle_returning(self, call);
        }
    }
    impl Struct {
        pub fn new() -> Self {
            let data = Arc::new(StructData {
                accept_ref: FnData::new("accept_ref"),
                return_ref: FnData::new("return_ref"),
                accept_ref_return_ref: FnData::new("accept_ref_return_ref"),
                accept_two_refs: FnData::new("accept_two_refs"),
                accept_two_refs_return_ref: FnData::new("accept_two_refs_return_ref"),
            });
            let inner_data = Struct_InnerData::new();
            return Struct {
                setup: StructSetup { data: data.clone() },
                received: StructReceived { data: data.clone() },
                data,
                inner_data,
            };
        }
    }
    impl StructSetup {
        pub fn accept_ref<'__rsa>(
            &self,
            r: impl Into<Arg<&'__rsa i32>>,
        ) -> FnTuner<'_, Struct, Self, (&&i32,), (), &Self, false, true> {
            let accept_ref_args_checker: accept_ref_ArgsChecker = accept_ref_ArgsChecker {
                _phantom_r: PhantomData,
                r: transmute_lifetime!(r.into()),
            };
            let fn_tuner: FnTuner<'_, Struct, Self, (&&i32,), (), &Self, false, true> = self
                .data
                .accept_ref
                .add_config(accept_ref_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
        pub fn return_ref<'__rsa>(
            &self,
        ) -> FnTuner<'_, Struct, Self, (), &'__rsa i32, &Self, false, true> {
            let return_ref_args_checker: return_ref_ArgsChecker = return_ref_ArgsChecker {};
            let fn_tuner: FnTuner<'_, Struct, Self, (), &'__rsa i32, &Self, false, true> = self
                .data
                .return_ref
                .add_config(return_ref_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
        pub fn accept_ref_return_ref<'__rsa>(
            &self,
            r: impl Into<Arg<&'__rsa i32>>,
        ) -> FnTuner<'_, Struct, Self, (&&i32,), &'__rsa i32, &Self, false, true> {
            let accept_ref_return_ref_args_checker: accept_ref_return_ref_ArgsChecker =
                accept_ref_return_ref_ArgsChecker {
                    _phantom_r: PhantomData,
                    r: transmute_lifetime!(r.into()),
                };
            let fn_tuner: FnTuner<'_, Struct, Self, (&&i32,), &'__rsa i32, &Self, false, true> =
                self.data
                    .accept_ref_return_ref
                    .add_config(accept_ref_return_ref_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
        pub fn accept_two_refs<'__rsa>(
            &self,
            r1: impl Into<Arg<&'__rsa i32>>,
            r2: impl Into<Arg<&'__rsa f32>>,
        ) -> FnTuner<'_, Struct, Self, (&&i32, &&f32), (), &Self, false, true> {
            let accept_two_refs_args_checker: accept_two_refs_ArgsChecker =
                accept_two_refs_ArgsChecker {
                    _phantom_r1: PhantomData,
                    _phantom_r2: PhantomData,
                    r1: transmute_lifetime!(r1.into()),
                    r2: transmute_lifetime!(r2.into()),
                };
            let fn_tuner: FnTuner<'_, Struct, Self, (&&i32, &&f32), (), &Self, false, true> = self
                .data
                .accept_two_refs
                .add_config(accept_two_refs_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
        pub fn accept_two_refs_return_ref<'__rsa>(
            &self,
            r1: impl Into<Arg<&'__rsa i32>>,
            r2: impl Into<Arg<&'__rsa f32>>,
        ) -> FnTuner<'_, Struct, Self, (&&i32, &&f32), &'__rsa str, &Self, false, true> {
            let accept_two_refs_return_ref_args_checker: accept_two_refs_return_ref_ArgsChecker =
                accept_two_refs_return_ref_ArgsChecker {
                    _phantom_r1: PhantomData,
                    _phantom_r2: PhantomData,
                    r1: transmute_lifetime!(r1.into()),
                    r2: transmute_lifetime!(r2.into()),
                };
            let fn_tuner: FnTuner<
                '_,
                Struct,
                Self,
                (&&i32, &&f32),
                &'__rsa str,
                &Self,
                false,
                true,
            > = self
                .data
                .accept_two_refs_return_ref
                .add_config(accept_two_refs_return_ref_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
    }
    impl StructReceived {
        pub fn accept_ref<'__rsa>(
            &self,
            r: impl Into<Arg<&'__rsa i32>>,
            times: Times,
        ) -> FnVerifier<Self, (&&i32,)> {
            let accept_ref_args_checker: accept_ref_ArgsChecker = accept_ref_ArgsChecker {
                _phantom_r: PhantomData,
                r: transmute_lifetime!(r.into()),
            };
            self.data
                .accept_ref
                .verify_received(accept_ref_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn return_ref<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let return_ref_args_checker: return_ref_ArgsChecker = return_ref_ArgsChecker {};
            self.data
                .return_ref
                .verify_received(return_ref_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn accept_ref_return_ref<'__rsa>(
            &self,
            r: impl Into<Arg<&'__rsa i32>>,
            times: Times,
        ) -> FnVerifier<Self, (&&i32,)> {
            let accept_ref_return_ref_args_checker: accept_ref_return_ref_ArgsChecker =
                accept_ref_return_ref_ArgsChecker {
                    _phantom_r: PhantomData,
                    r: transmute_lifetime!(r.into()),
                };
            self.data
                .accept_ref_return_ref
                .verify_received(accept_ref_return_ref_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn accept_two_refs<'__rsa>(
            &self,
            r1: impl Into<Arg<&'__rsa i32>>,
            r2: impl Into<Arg<&'__rsa f32>>,
            times: Times,
        ) -> FnVerifier<Self, (&&i32, &&f32)> {
            let accept_two_refs_args_checker: accept_two_refs_ArgsChecker =
                accept_two_refs_ArgsChecker {
                    _phantom_r1: PhantomData,
                    _phantom_r2: PhantomData,
                    r1: transmute_lifetime!(r1.into()),
                    r2: transmute_lifetime!(r2.into()),
                };
            self.data
                .accept_two_refs
                .verify_received(accept_two_refs_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn accept_two_refs_return_ref<'__rsa>(
            &self,
            r1: impl Into<Arg<&'__rsa i32>>,
            r2: impl Into<Arg<&'__rsa f32>>,
            times: Times,
        ) -> FnVerifier<Self, (&&i32, &&f32)> {
            let accept_two_refs_return_ref_args_checker: accept_two_refs_return_ref_ArgsChecker =
                accept_two_refs_return_ref_ArgsChecker {
                    _phantom_r1: PhantomData,
                    _phantom_r2: PhantomData,
                    r1: transmute_lifetime!(r1.into()),
                    r2: transmute_lifetime!(r2.into()),
                };
            self.data
                .accept_two_refs_return_ref
                .verify_received(accept_two_refs_return_ref_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
}
