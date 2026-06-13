#[cfg(not(test))]
fn accept_two_mut_refs_return_mut_ref(r1: &mut i32, r2: &mut f32) -> &'static mut i32 {
    unsafe { &mut *&raw mut ACCEPT_TWO_REFS_RETURN_MUT_REF }
}
#[cfg(test)]
pub use accept_two_mut_refs_return_mut_ref::*;
#[cfg(test)]
#[allow(unused_parens)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(mismatched_lifetime_syntaxes)]
mod accept_two_mut_refs_return_mut_ref {
    use super::*;
    use rsubstitute::for_generated::*;
    #[doc(hidden)]
    pub struct accept_two_mut_refs_return_mut_ref_Call {
        _phantom_r1: PhantomData<*mut i32>,
        _phantom_r2: PhantomData<*mut f32>,
        r1: *mut i32,
        r2: *mut f32,
    }
    impl IArgsInfosProvider for accept_two_mut_refs_return_mut_ref_Call {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![
                ArgInfo::new("r1", &self.r1, (&ArgPrinter(&self.r1)).debug_string()),
                ArgInfo::new("r2", &self.r2, (&ArgPrinter(&self.r2)).debug_string()),
            ]
        }
    }
    impl IArgsTupleProvider for accept_two_mut_refs_return_mut_ref_Call {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((&self.r1, &self.r2))) as *mut _ as *mut ()
        }
    }
    impl IGenericsInfoProvider for accept_two_mut_refs_return_mut_ref_Call {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    impl Clone for accept_two_mut_refs_return_mut_ref_Call {
        fn clone(&self) -> Self {
            Self {
                _phantom_r1: (&self._phantom_r1).clone(),
                _phantom_r2: (&self._phantom_r2).clone(),
                r1: (&self.r1).clone(),
                r2: (&self.r2).clone(),
            }
        }
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct accept_two_mut_refs_return_mut_ref_ArgsChecker {
        _phantom_r1: PhantomData<*mut i32>,
        _phantom_r2: PhantomData<*mut f32>,
        r1: Arg<*mut i32>,
        r2: Arg<*mut f32>,
    }
    impl IArgsChecker for accept_two_mut_refs_return_mut_ref_ArgsChecker {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &accept_two_mut_refs_return_mut_ref_Call = dyn_call.downcast_ref();
            vec![
                self.r1.check_mut(
                    "r1",
                    transmute_lifetime!(&call.r1),
                    (&ArgPrinter(&call.r1)).debug_string(),
                ),
                self.r2.check_mut(
                    "r2",
                    transmute_lifetime!(&call.r2),
                    (&ArgPrinter(&call.r2)).debug_string(),
                ),
            ]
        }
    }
    impl IArgsFormatter for accept_two_mut_refs_return_mut_ref_ArgsChecker {
        fn fmt_args(&self) -> String {
            format!(
                "{}, {}",
                (&ArgPrinter(&self.r1)).debug_string(),
                (&ArgPrinter(&self.r2)).debug_string()
            )
        }
    }
    impl IGenericsInfoProvider for accept_two_mut_refs_return_mut_ref_ArgsChecker {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct accept_two_mut_refs_return_mut_refMockData {
        pub accept_two_mut_refs_return_mut_ref:
            FnData<'static, accept_two_mut_refs_return_mut_refMock, true, false>,
    }
    impl IMockData for accept_two_mut_refs_return_mut_refMockData {
        fn get_received_nothing_else_error_msgs(&self) -> Vec<Vec<String>> {
            return vec![self
                .accept_two_mut_refs_return_mut_ref
                .get_unexpected_calls_error_msgs()];
        }
    }
    #[doc(hidden)]
    pub struct accept_two_mut_refs_return_mut_refMockSetup {
        data: Arc<accept_two_mut_refs_return_mut_refMockData>,
    }
    impl Clone for accept_two_mut_refs_return_mut_refMockSetup {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    #[doc(hidden)]
    pub struct accept_two_mut_refs_return_mut_refMockReceived {
        data: Arc<accept_two_mut_refs_return_mut_refMockData>,
    }
    impl Clone for accept_two_mut_refs_return_mut_refMockReceived {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    #[doc(hidden)]
    pub struct accept_two_mut_refs_return_mut_refMock {
        pub setup: accept_two_mut_refs_return_mut_refMockSetup,
        pub received: accept_two_mut_refs_return_mut_refMockReceived,
        pub data: Arc<accept_two_mut_refs_return_mut_refMockData>,
    }
    impl AsRef<accept_two_mut_refs_return_mut_refMock> for accept_two_mut_refs_return_mut_refMock {
        fn as_ref(&self) -> &accept_two_mut_refs_return_mut_refMock {
            self
        }
    }
    impl Clone for accept_two_mut_refs_return_mut_refMock {
        fn clone(&self) -> Self {
            Self {
                setup: (&self.setup).clone(),
                received: (&self.received).clone(),
                data: (&self.data).clone(),
            }
        }
    }
    impl Default for accept_two_mut_refs_return_mut_refMock {
        fn default() -> Self {
            let data = Arc::new(accept_two_mut_refs_return_mut_refMockData {
                accept_two_mut_refs_return_mut_ref: FnData::new(
                    "accept_two_mut_refs_return_mut_ref",
                ),
            });
            return accept_two_mut_refs_return_mut_refMock {
                setup: accept_two_mut_refs_return_mut_refMockSetup { data: data.clone() },
                received: accept_two_mut_refs_return_mut_refMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl accept_two_mut_refs_return_mut_refMockSetup {
        pub fn accept_two_mut_refs_return_mut_ref<'__rsa>(
            &self,
            r1: impl Into<Arg<*mut i32>>,
            r2: impl Into<Arg<*mut f32>>,
        ) -> FnConfigurator<
            '_,
            accept_two_mut_refs_return_mut_refMock,
            Self,
            (&'__rsa &'__rsa mut i32, &'__rsa &'__rsa mut f32),
            &'__rsa mut i32,
            accept_two_mut_refs_return_mut_refMock,
            true,
            false,
        > {
            let accept_two_mut_refs_return_mut_ref_args_checker: accept_two_mut_refs_return_mut_ref_ArgsChecker = accept_two_mut_refs_return_mut_ref_ArgsChecker { _phantom_r1: PhantomData, _phantom_r2: PhantomData, r1: transmute_lifetime!(r1 . into ()), r2: transmute_lifetime!(r2 . into ()) };
            let fn_configurator: FnConfigurator<
                '_,
                accept_two_mut_refs_return_mut_refMock,
                Self,
                (&'__rsa &'__rsa mut i32, &'__rsa &'__rsa mut f32),
                &'__rsa mut i32,
                accept_two_mut_refs_return_mut_refMock,
                true,
                false,
            > = self
                .data
                .accept_two_mut_refs_return_mut_ref
                .add_config(accept_two_mut_refs_return_mut_ref_args_checker, self);
            return transmute_lifetime!(fn_configurator);
        }
    }
    impl accept_two_mut_refs_return_mut_refMockReceived {
        pub fn accept_two_mut_refs_return_mut_ref<'__rsa>(
            &self,
            r1: impl Into<Arg<*mut i32>>,
            r2: impl Into<Arg<*mut f32>>,
            times: Times,
        ) -> FnVerifier<Self, (&'__rsa &'__rsa mut i32, &'__rsa &'__rsa mut f32)> {
            let accept_two_mut_refs_return_mut_ref_args_checker: accept_two_mut_refs_return_mut_ref_ArgsChecker = accept_two_mut_refs_return_mut_ref_ArgsChecker { _phantom_r1: PhantomData, _phantom_r2: PhantomData, r1: transmute_lifetime!(r1 . into ()), r2: transmute_lifetime!(r2 . into ()) };
            self.data
                .accept_two_mut_refs_return_mut_ref
                .verify_received(accept_two_mut_refs_return_mut_ref_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
    pub fn get_mock<'__rsa>() -> &'__rsa accept_two_mut_refs_return_mut_refMock {
        get_static_fn_global_mock::<accept_two_mut_refs_return_mut_refMock>()
    }
    pub fn setup<'__rsa>(
        r1: impl Into<Arg<*mut i32>>,
        r2: impl Into<Arg<*mut f32>>,
    ) -> FnConfigurator<
        '__rsa,
        accept_two_mut_refs_return_mut_refMock,
        accept_two_mut_refs_return_mut_refMockSetup,
        (&'__rsa &'__rsa mut i32, &'__rsa &'__rsa mut f32),
        &'__rsa mut i32,
        accept_two_mut_refs_return_mut_refMock,
        true,
        false,
    > {
        let mock: &'__rsa accept_two_mut_refs_return_mut_refMock = get_mock();
        mock.data.accept_two_mut_refs_return_mut_ref.reset();
        return transmute_lifetime!(mock.setup.setup(r1, r2));
    }
    pub fn received<'__rsa>(
        r1: impl Into<Arg<*mut i32>>,
        r2: impl Into<Arg<*mut f32>>,
        times: Times,
    ) -> FnVerifier<
        accept_two_mut_refs_return_mut_refMockReceived,
        (&'__rsa &'__rsa mut i32, &'__rsa &'__rsa mut f32),
    > {
        return get_mock().received.clone().received(r1, r2, times);
    }
    pub fn accept_two_mut_refs_return_mut_ref(r1: &mut i32, r2: &mut f32) -> &'static mut i32 {
        let call: accept_two_mut_refs_return_mut_ref_Call =
            accept_two_mut_refs_return_mut_ref_Call {
                _phantom_r1: PhantomData,
                _phantom_r2: PhantomData,
                r1: transmute_lifetime!(r1),
                r2: transmute_lifetime!(r2),
            };
        let mock = get_mock::<'_>();
        return mock
            .data
            .clone()
            .accept_two_mut_refs_return_mut_ref
            .handle_base_returning(mock, call, base_accept_two_mut_refs_return_mut_ref);
    }
    fn base_accept_two_mut_refs_return_mut_ref(
        _: &accept_two_mut_refs_return_mut_refMock,
        call: accept_two_mut_refs_return_mut_ref_Call,
    ) -> &'static mut i32 {
        #[allow(non_shorthand_field_patterns)]
        #[allow(unused_variables)]
        let accept_two_mut_refs_return_mut_ref_Call { r1: r1, r2: r2, .. } = call;
        unsafe { &mut *&raw mut ACCEPT_TWO_REFS_RETURN_MUT_REF }
    }
}
