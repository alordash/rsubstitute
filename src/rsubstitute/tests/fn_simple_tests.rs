use std::cell::RefCell;
use std::sync::Arc;

// #[mock]
// fn f() {}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::panics::*;
    use rsubstitute::*;

    #[test]
    fn f_Ok() {
        // Arrange
        let callback_flag = Arc::new(RefCell::new(false));
        let callback_flag_clone = callback_flag.clone();
        f::setup()
            .returns(())
            .and_does(move |_| *callback_flag_clone.borrow_mut() = true);

        // Act
        let result = f();

        // Assert
        assert_eq!((), result);
        assert!(*callback_flag.borrow());
        f::received(Times::Once).no_other_calls();
    }

    #[test]
    fn f_NoConfig_Ok() {
        // Arrange
        f::setup();

        // Act
        let result = f();

        // Assert
        assert_eq!((), result);
        f::received(Times::Once).no_other_calls();
    }

    #[test]
    fn f_MultipleTime_Ok() {
        // Arrange
        f::setup();

        // Act
        let result1 = f();
        let result2 = f();
        let result3 = f();

        // Assert
        assert_eq!((), result1);
        assert_eq!((), result2);
        assert_eq!((), result3);

        f::received(Times::Exactly(3)).no_other_calls();
    }

    #[test]
    fn f_MultipleTimes_Panics() {
        // Arrange
        f::setup();

        // Act
        f();
        f();
        f();

        // Assert
        assert_panics(
            || f::received(Times::Once),
            r#"Expected to receive a call exactly once matching:
	f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
        );

        assert_panics(
            || f::received(Times::Exactly(1)),
            r#"Expected to receive a call exactly once matching:
	f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
        );

        assert_panics(
            || f::received(Times::Exactly(2)),
            r#"Expected to receive a call 2 times matching:
	f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
        );

        assert_panics(
            || f::received(Times::Exactly(4)),
            r#"Expected to receive a call 4 times matching:
	f()
Actually received 3 matching calls:
	f()
	f()
	f()
Received no non-matching calls"#,
        );
    }
}

#[cfg(not(test))]
fn f() {}
#[cfg(test)]
pub use f::*;
#[cfg(test)]
#[allow(unused_parens)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(mismatched_lifetime_syntaxes)]
mod f {
    use super::*;
    use rsubstitute::for_generated::*;
    #[doc(hidden)]
    #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider)]
    pub struct f_Call<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
    pub struct f_ArgsChecker<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    impl<'rs> IArgsChecker for f_ArgsChecker<'rs> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &f_Call<'rs> = dyn_call.downcast_ref();
            vec![]
        }
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct fMockData<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
        f_data: FnData<'rs, fMock<'rs>, false>,
    }
    #[doc(hidden)]
    #[derive(Clone)]
    pub struct fMockSetup<'rs> {
        data: Arc<fMockData<'rs>>,
    }
    #[doc(hidden)]
    #[derive(Clone)]
    pub struct fMockReceived<'rs> {
        data: Arc<fMockData<'rs>>,
    }
    #[doc(hidden)]
    pub struct fMock<'rs> {
        pub setup: fMockSetup<'rs>,
        pub received: fMockReceived<'rs>,
        data: Arc<fMockData<'rs>>,
    }
    unsafe impl<'rs> Send for fMock<'rs> {}
    unsafe impl<'rs> Sync for fMock<'rs> {}
    impl<'rs> Default for fMock<'rs> {
        fn default() -> Self {
            let data = Arc::new(fMockData {
                _phantom_lifetime: PhantomData,
                f_data: FnData::new("f"),
            });
            return fMock {
                setup: fMockSetup { data: data.clone() },
                received: fMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'rs> fMockSetup<'rs> {
        pub fn setup(&self) -> FnTuner<'_, Self, (), (), false> {
            let f_args_checker: f_ArgsChecker<'rs> = f_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            let fn_tuner: FnTuner<'_, Self, (), (), false> =
                self.data.f_data.add_config(f_args_checker, self);
            return unsafe { core::mem::transmute(fn_tuner) };
        }
    }
    impl<'rs> fMockReceived<'rs> {
        pub fn received(&self, times: Times) -> FnVerifier<'rs, Self, ()> {
            let f_args_checker: f_ArgsChecker<'rs> = f_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            self.data.f_data.verify_received(f_args_checker, times);
            return FnVerifier::new(self);
        }
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
    pub fn setup<'rs>() -> FnTuner<'rs, fMockSetup<'rs>, (), (), false> {
        let mock = get_global_mock::<fMock<'rs>>();
        mock.data.f_data.reset();
        return mock.setup.setup();
    }
    pub fn received<'rs>(times: Times) -> FnVerifier<'rs, fMockReceived<'rs>, ()> {
        return get_global_mock::<fMock<'rs>>()
            .received
            .clone()
            .received(times);
    }
    pub fn f<'__rsubstitute_arg_anonymous, 'rs>() {
        let call: f_Call<'_> = unsafe {
            f_Call {
                _phantom_lifetime: PhantomData,
            }
        };
        let mock = get_global_mock::<fMock<'rs>>();
        mock.data.f_data.handle(call);
    }
}
