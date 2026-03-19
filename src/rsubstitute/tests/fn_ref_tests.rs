#![allow(unused)]
use rsubstitute::macros::mock;

// #[mock(base)]
// fn accept_ref(r: &i32) {}

// TODO - do not forget to specify in docs that calling `setup` on static fn clears all existing configurations (this is done because otherwise configs would interrupt each other in tests)
const BASE_RETURN_REF: &'static i32 = &1000;
#[mock(base)]
fn return_ref() -> &'static i32 {
    BASE_RETURN_REF
}

const BASE_ACCEPT_REF_RETURN_REF: &'static i32 = &2000;
#[mock(base)]
fn accept_ref_return_ref(r: &i32) -> &'static i32 {
    BASE_ACCEPT_REF_RETURN_REF
}

#[mock(base)]
fn accept_two_refs(r1: &i32, r2: &f32) {}

const ACCEPT_TWO_REFS_RETURN_REF: &'static str = "quo vadis";
#[mock(base)]
fn accept_two_refs_return_ref(r1: &i32, r2: &f32) -> &'static str {
    ACCEPT_TWO_REFS_RETURN_REF
}

#[mock(base)]
fn accept_mut_ref(r: &mut i32) {}

static mut BASE_RETURN_MUT_REF: i32 = 12;
#[mock(base)]
fn return_mut_ref() -> &'static mut i32 {
    unsafe { &mut *&raw mut BASE_RETURN_MUT_REF }
}

#[mock(base)]
fn accept_mut_ref_return_mut_ref(r: &mut i32) -> &'static i32 {
    BASE_ACCEPT_REF_RETURN_REF
}

#[mock(base)]
fn accept_two_mut_refs(r1: &mut i32, r2: &mut f32) {}

static mut ACCEPT_TWO_REFS_RETURN_MUT_REF: i32 = 382;
#[mock(base)]
fn accept_two_mut_refs_return_mut_ref(r1: &mut i32, r2: &mut f32) -> &'static mut i32 {
    unsafe { &mut *&raw mut ACCEPT_TWO_REFS_RETURN_MUT_REF }
}

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
            let r = &12;

            {
                let v2 = 24;
                let r2 = &v2;
                accept_ref::setup(r2)
                    .does(|(ref received_r2,)| println!("received_r2 = {received_r2}"));
                accept_ref(r2);
            }

            let some_data = [15, 22, 32, 42, 52, 62, 72, 82, 92, 102];
            println!("some_data = {some_data:?}");

            // Act
            accept_ref(r);

            // Assert
            accept_ref::received(r, Times::Once)
                .received(
                    Arg::is(|ref received_r2| {
                        println!("checking received, received_r2 = {received_r2}");
                        return true;
                    }),
                    Times::Exactly(2),
                )
                .no_other_calls();
        }

        #[test]
        fn accept_value_Panics() {
            // Arrange
            let r = &11;
            let r_ptr = core::ptr::from_ref(r);

            // Act
            accept_ref(r);

            // Assert
            assert_panics(
                || accept_ref::received(Arg::Any, Times::Never),
                format!(
                    "Expected to never receive a call matching:
	accept_ref((&i32): any)
Actually received 1 matching call:
	accept_ref({r})
Received no non-matching calls"
                ),
            );

            assert_panics(
                || accept_ref::received(Arg::Any, Times::Exactly(3)),
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
                || accept_ref::received(invalid_r, Times::Once),
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
            let r = Box::leak(Box::new(11));
            return_ref::setup().returns(r);

            // Act
            let actual_r = return_ref();

            // Assert
            assert_eq!(r, actual_r);
        }

        #[test]
        fn return_ref_CallBase_Ok() {
            // Arrange
            return_ref::setup().call_base();

            // Act
            let actual_r = return_ref();

            // Assert
            assert_eq!(BASE_RETURN_REF, actual_r);
        }
    }

    mod accept_ref_return_ref_tests {
        use super::*;

        #[test]
        fn accept_ref_return_ref_Ok() {
            // Arrange
            let accepted_r = &10;
            let returned_r = &20;
            accept_ref_return_ref::setup(accepted_r).returns(returned_r);

            // Act
            let actual_returned_r = accept_ref_return_ref(accepted_r);

            // Assert
            assert_eq!(returned_r, actual_returned_r);

            accept_ref_return_ref::received(accepted_r, Times::Once)
                .received(Arg::not_eq(accepted_r), Times::Never)
                .no_other_calls();
        }

        #[test]
        fn accept_ref_return_ref_CallBase_Ok() {
            // Arrange
            let accepted_r = &10;
            accept_ref_return_ref::setup(accepted_r).call_base();

            // Act
            let actual_returned_r = accept_ref_return_ref(accepted_r);

            // Assert
            assert_eq!(BASE_ACCEPT_REF_RETURN_REF, actual_returned_r);
            accept_ref_return_ref::received(accepted_r, Times::Once).no_other_calls();
        }
    }

    mod accept_two_refs_tests {
        use super::*;

        #[test]
        fn accept_two_refs_Ok() {
            // Arrange
            let r1 = &10;
            let r2 = &20.2;

            // Act
            accept_two_refs(r1, r2);

            // Assert
            accept_two_refs::received(r1, r2, Times::Once)
                .received(Arg::not_eq(r1), Arg::not_eq(r2), Times::Never)
                .no_other_calls();
        }
    }

    mod accept_two_refs_return_ref_tests {
        use super::*;

        #[test]
        fn accept_two_refs_return_ref_Ok() {
            // Arrange
            let r1 = &10;
            let r2 = &20.2;
            let returned_r = "veridis quo";
            accept_two_refs_return_ref::setup(r1, r2).returns(returned_r);

            // Act
            let actual_returned_r = accept_two_refs_return_ref(r1, r2);

            // Assert
            assert_eq!(returned_r, actual_returned_r);

            accept_two_refs_return_ref::received(r1, r2, Times::Once).no_other_calls();
        }

        #[test]
        fn accept_two_refs_return_ref_CallBase_Ok() {
            // Arrange
            let r1 = &10;
            let r2 = &20.2;
            accept_two_refs_return_ref::setup(r1, r2).call_base();

            // Act
            let actual_returned_r = accept_two_refs_return_ref(r1, r2);

            // Assert
            assert_eq!(ACCEPT_TWO_REFS_RETURN_REF, actual_returned_r);

            accept_two_refs_return_ref::received(r1, r2, Times::Once).no_other_calls();
        }
    }
}

#[cfg(not(test))]
fn accept_ref(r: &i32) {}
#[cfg(test)]
pub use accept_ref::*;
#[cfg(test)]
#[allow(unused_parens)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(mismatched_lifetime_syntaxes)]
mod accept_ref {
    use super::*;
    use rsubstitute::for_generated::*;
    #[doc(hidden)]
    #[derive(IArgsInfosProvider, IArgsTupleProvider, CloneForRSubstitute)]
    pub struct accept_ref_Call<'__rs> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom___rs: PhantomData<&'__rs ()>,
        r: *const i32,
    }
    impl<'__rs> IGenericsInfoProvider for accept_ref_Call<'__rs> {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter)]
    pub struct accept_ref_ArgsChecker<'__rs> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom___rs: PhantomData<&'__rs ()>,
        r: Arg<'__rs, &'__rs i32>,
    }
    impl<'__rs> IArgsChecker for accept_ref_ArgsChecker<'__rs> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &accept_ref_Call<'__rs> = dyn_call.downcast_ref();
            vec![self.r.check(
                "r",
                transmute_lifetime!(&call.r),
                (&ArgPrinter(&&call.r)).debug_string(),
            )]
        }
    }
    impl<'__rs> IGenericsInfoProvider for accept_ref_ArgsChecker<'__rs> {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct accept_refMockData<'__rs> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom___rs: PhantomData<&'__rs ()>,
        pub accept_ref: FnData<'static, accept_refMock<'__rs>, true, false>,
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct accept_refMockSetup<'__rs> {
        data: Arc<accept_refMockData<'__rs>>,
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct accept_refMockReceived<'__rs> {
        data: Arc<accept_refMockData<'__rs>>,
    }
    #[doc(hidden)]
    pub struct accept_refMock<'__rs> {
        pub setup: accept_refMockSetup<'__rs>,
        pub received: accept_refMockReceived<'__rs>,
        pub data: Arc<accept_refMockData<'__rs>>,
    }
    impl<'__rs> Default for accept_refMock<'__rs> {
        fn default() -> Self {
            let data = Arc::new(accept_refMockData {
                _phantom_lifetime: PhantomData,
                _phantom___rs: PhantomData,
                accept_ref: FnData::new("accept_ref"),
            });
            return accept_refMock {
                setup: accept_refMockSetup { data: data.clone() },
                received: accept_refMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'__rs> accept_refMockSetup<'__rs> {
        pub fn setup<'__rsa>(
            &self,
            r: impl Into<Arg<'__rsa, &'__rsa i32>>,
        ) -> FnTuner<'_, accept_refMock<'__rs>, Self, (&'__rs &'__rs i32,), (), true, false>
        {
            let accept_ref_args_checker: accept_ref_ArgsChecker<'_> = accept_ref_ArgsChecker {
                _phantom_lifetime: PhantomData,
                _phantom___rs: PhantomData,
                r: transmute_lifetime!(r.into()),
            };
            let fn_tuner: FnTuner<
                '_,
                accept_refMock<'__rs>,
                Self,
                (&'__rs &'__rs i32,),
                (),
                true,
                false,
            > = self
                .data
                .accept_ref
                .add_config(accept_ref_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
    }
    impl<'__rs> accept_refMockReceived<'__rs> {
        pub fn received<'__rsa>(
            &self,
            r: impl Into<Arg<'__rsa, &'__rsa i32>>,
            times: Times,
        ) -> FnVerifier<Self, (&'__rs &'__rs i32,)> {
            let accept_ref_args_checker: accept_ref_ArgsChecker<'_> = accept_ref_ArgsChecker {
                _phantom_lifetime: PhantomData,
                _phantom___rs: PhantomData,
                r: transmute_lifetime!(r.into()),
            };
            self.data
                .accept_ref
                .verify_received(accept_ref_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
    pub fn get_mock<'__rsa, '__rs>() -> &'__rsa accept_refMock<'__rs> {
        get_global_mock::<accept_refMock<'__rs>>()
    }
    pub fn setup<'__rsa, '__rs>(
        r: impl Into<Arg<'__rsa, &'__rsa i32>>,
    ) -> FnTuner<
        '__rs,
        accept_refMock<'__rs>,
        accept_refMockSetup<'__rs>,
        (&'__rs &'__rs i32,),
        (),
        true,
        false,
    > {
        let mock = get_mock();
        mock.data.accept_ref.reset();
        return mock.setup.setup(r);
    }
    pub fn received<'__rsa, '__rs>(
        r: impl Into<Arg<'__rsa, &'__rsa i32>>,
        times: Times,
    ) -> FnVerifier<accept_refMockReceived<'__rs>, (&'__rs &'__rs i32,)> {
        return get_mock().received.clone().received(r, times);
    }
    pub fn accept_ref<'__rs>(r: &i32) {
        let call: accept_ref_Call<'_> = accept_ref_Call {
            _phantom_lifetime: PhantomData,
            _phantom___rs: PhantomData,
            r: transmute_lifetime!(r),
        };
        let mock = get_mock::<'_, '__rs>();
        mock.data
            .accept_ref
            .handle_base(&mock, call, base_accept_ref);
    }
    fn base_accept_ref<'__rs>(_: &accept_refMock<'__rs>, call: accept_ref_Call<'__rs>) {
        #[allow(non_shorthand_field_patterns)]
        #[allow(unused_variables)]
        let accept_ref_Call::<'_> { r: r, .. } = call;
    }
}
