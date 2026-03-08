use rsubstitute::macros::mock;
use std::ops::Deref;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

// #[mock]
// trait Trait {
//     fn mutate(&mut self);
//
//     fn consume(self) -> i32;
//
//     fn sbox(self: Box<Self>) -> i32;
//     fn src(self: Rc<Self>) -> i32;
//     fn sarc(self: Arc<Self>) -> i32;
//
//     fn spbox(self: Pin<Box<Self>>) -> i32;
//     fn sprc(self: Pin<Rc<Self>>) -> i32;
//     fn sparc(self: Pin<Arc<Self>>) -> i32;
// }

trait Q {
    fn q(self: Box<Self>);
    fn w(self: Rc<Self>);
    fn e(self: Arc<Self>);
    fn qp(self: Pin<Box<Self>>);
    fn wp(self: Pin<Rc<Self>>);
    fn ep(self: Pin<Arc<Self>>);
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use not_enough_asserts::panics::*;
    use rsubstitute::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn flex() {
        let mut mock = TraitMock::new();

        mock.setup.sbox().returns(12);
        mock.setup.src().returns(12);
        mock.setup.sarc().returns(12);

        mock.setup.spbox().returns(12);
        mock.setup.sprc().returns(12);
        mock.setup.sparc().returns(12);

        assert_eq!(12, Box::new(mock.clone()).sbox());
        assert_eq!(12, Box::new(mock.clone()).sbox());
        assert_eq!(12, Box::new(mock.clone()).sbox());
        mock.received.sbox(Times::Exactly(3));

        assert_eq!(12, Box::new(mock.clone()).sbox());
        assert_eq!(12, Rc::new(mock.clone()).src());
        assert_eq!(12, Arc::new(mock.clone()).sarc());
        mock.received.sbox(Times::Exactly(4));
        mock.received.src(Times::Once);
        mock.received.sarc(Times::Once);

        assert_eq!(12, Pin::new(Box::new(mock.clone())).spbox());
        assert_eq!(12, Pin::new(Rc::new(mock.clone())).sprc());
        assert_eq!(12, Pin::new(Arc::new(mock.clone())).sparc());
        mock.received.spbox(Times::Once);
        mock.received.sprc(Times::Once);
        mock.received.sparc(Times::Once);

        let flag = Rc::new(Cell::new(false));
        let flag_clone = flag.clone();

        mock.setup.mutate().does(move |_| flag_clone.set(true));
        mock.setup.consume().returns(22);

        mock.mutate();
        assert!(flag.get());
        let value = mock.clone().consume();
        assert_eq!(22, value);

        mock.received
            .mutate(Times::Once)
            .consume(Times::Once)
            .no_other_calls();
    }

    // TODO - remove this thing
    #[test]
    fn compile() {}
}

#[cfg(not(test))]
trait Trait {
    fn mutate(&mut self);

    fn consume(self) -> i32;

    fn sbox(self: Box<Self>) -> i32;
    fn src(self: Rc<Self>) -> i32;
    fn sarc(self: Arc<Self>) -> i32;

    fn spbox(self: Pin<Box<Self>>) -> i32;
    fn sprc(self: Pin<Rc<Self>>) -> i32;
    fn sparc(self: Pin<Arc<Self>>) -> i32;
}
#[cfg(test)]
trait Trait {
    fn mutate(&mut self);

    fn consume(self) -> i32;

    fn sbox(self: Box<Self>) -> i32;
    fn src(self: Rc<Self>) -> i32;
    fn sarc(self: Arc<Self>) -> i32;

    fn spbox(self: Pin<Box<Self>>) -> i32;
    fn sprc(self: Pin<Rc<Self>>) -> i32;
    fn sparc(self: Pin<Arc<Self>>) -> i32;
}
#[cfg(test)]
pub use __rsubstitute_generated_Trait::*;
#[cfg(test)]
#[allow(unused_parens)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(mismatched_lifetime_syntaxes)]
mod __rsubstitute_generated_Trait {
    use super::*;
    use rsubstitute::for_generated::*;
    #[doc(hidden)]
    #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider)]
    pub struct mutate_Call<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
    pub struct mutate_ArgsChecker<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    impl<'rs> IArgsChecker for mutate_ArgsChecker<'rs> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &mutate_Call<'rs> = dyn_call.downcast_ref();
            vec![]
        }
    }
    #[doc(hidden)]
    #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider)]
    pub struct consume_Call<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
    pub struct consume_ArgsChecker<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    impl<'rs> IArgsChecker for consume_ArgsChecker<'rs> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &consume_Call<'rs> = dyn_call.downcast_ref();
            vec![]
        }
    }
    #[doc(hidden)]
    #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider)]
    pub struct sbox_Call<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
    pub struct sbox_ArgsChecker<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    impl<'rs> IArgsChecker for sbox_ArgsChecker<'rs> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &sbox_Call<'rs> = dyn_call.downcast_ref();
            vec![]
        }
    }
    #[doc(hidden)]
    #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider)]
    pub struct src_Call<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
    pub struct src_ArgsChecker<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    impl<'rs> IArgsChecker for src_ArgsChecker<'rs> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &src_Call<'rs> = dyn_call.downcast_ref();
            vec![]
        }
    }
    #[doc(hidden)]
    #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider)]
    pub struct sarc_Call<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
    pub struct sarc_ArgsChecker<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    impl<'rs> IArgsChecker for sarc_ArgsChecker<'rs> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &sarc_Call<'rs> = dyn_call.downcast_ref();
            vec![]
        }
    }
    #[doc(hidden)]
    #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider)]
    pub struct spbox_Call<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
    pub struct spbox_ArgsChecker<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    impl<'rs> IArgsChecker for spbox_ArgsChecker<'rs> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &spbox_Call<'rs> = dyn_call.downcast_ref();
            vec![]
        }
    }
    #[doc(hidden)]
    #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider)]
    pub struct sprc_Call<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
    pub struct sprc_ArgsChecker<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    impl<'rs> IArgsChecker for sprc_ArgsChecker<'rs> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &sprc_Call<'rs> = dyn_call.downcast_ref();
            vec![]
        }
    }
    #[doc(hidden)]
    #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider)]
    pub struct sparc_Call<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
    pub struct sparc_ArgsChecker<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    impl<'rs> IArgsChecker for sparc_ArgsChecker<'rs> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &sparc_Call<'rs> = dyn_call.downcast_ref();
            vec![]
        }
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct TraitMockData<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
        mutate_data: FnData<'rs, TraitMock<'rs>, false>,
        consume_data: FnData<'rs, TraitMock<'rs>, false>,
        sbox_data: FnData<'rs, TraitMock<'rs>, false>,
        src_data: FnData<'rs, TraitMock<'rs>, false>,
        sarc_data: FnData<'rs, TraitMock<'rs>, false>,
        spbox_data: FnData<'rs, TraitMock<'rs>, false>,
        sprc_data: FnData<'rs, TraitMock<'rs>, false>,
        sparc_data: FnData<'rs, TraitMock<'rs>, false>,
    }
    #[doc(hidden)]
    #[derive(Clone)]
    pub struct TraitMockSetup<'rs> {
        data: Arc<TraitMockData<'rs>>,
    }
    #[doc(hidden)]
    #[derive(Clone)]
    pub struct TraitMockReceived<'rs> {
        data: Arc<TraitMockData<'rs>>,
    }
    #[derive(Clone)]
    pub struct TraitMock<'rs> {
        pub setup: TraitMockSetup<'rs>,
        pub received: TraitMockReceived<'rs>,
        data: Arc<TraitMockData<'rs>>,
    }
    impl<'rs> Trait for TraitMock<'rs> {
        fn mutate(&mut self) {
            let call: mutate_Call<'_> = unsafe {
                mutate_Call {
                    _phantom_lifetime: PhantomData,
                }
            };
            self.data.mutate_data.handle(call);
        }
        fn consume(self) -> i32 {
            let call: consume_Call<'_> = unsafe {
                consume_Call {
                    _phantom_lifetime: PhantomData,
                }
            };
            return self.data.consume_data.handle_returning(call);
        }
        fn sbox(self: Box<Self>) -> i32 {
            let call: sbox_Call<'_> = unsafe {
                sbox_Call {
                    _phantom_lifetime: PhantomData,
                }
            };
            return self.data.sbox_data.handle_returning(call);
        }
        fn src(self: Rc<Self>) -> i32 {
            let call: src_Call<'_> = unsafe {
                src_Call {
                    _phantom_lifetime: PhantomData,
                }
            };
            return self.data.src_data.handle_returning(call);
        }
        fn sarc(self: Arc<Self>) -> i32 {
            let call: sarc_Call<'_> = unsafe {
                sarc_Call {
                    _phantom_lifetime: PhantomData,
                }
            };
            return self.data.sarc_data.handle_returning(call);
        }
        fn spbox(self: Pin<Box<Self>>) -> i32 {
            let call: spbox_Call<'_> = unsafe {
                spbox_Call {
                    _phantom_lifetime: PhantomData,
                }
            };
            return self.data.spbox_data.handle_returning(call);
        }
        fn sprc(self: Pin<Rc<Self>>) -> i32 {
            let call: sprc_Call<'_> = unsafe {
                sprc_Call {
                    _phantom_lifetime: PhantomData,
                }
            };
            return self.data.sprc_data.handle_returning(call);
        }
        fn sparc(self: Pin<Arc<Self>>) -> i32 {
            let call: sparc_Call<'_> = unsafe {
                sparc_Call {
                    _phantom_lifetime: PhantomData,
                }
            };
            return self.data.sparc_data.handle_returning(call);
        }
    }
    impl<'rs> TraitMock<'rs> {
        pub fn new() -> Self {
            let data = Arc::new(TraitMockData {
                _phantom_lifetime: PhantomData,
                mutate_data: FnData::new("mutate"),
                consume_data: FnData::new("consume"),
                sbox_data: FnData::new("sbox"),
                src_data: FnData::new("src"),
                sarc_data: FnData::new("sarc"),
                spbox_data: FnData::new("spbox"),
                sprc_data: FnData::new("sprc"),
                sparc_data: FnData::new("sparc"),
            });
            return TraitMock {
                setup: TraitMockSetup { data: data.clone() },
                received: TraitMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'rs> TraitMockSetup<'rs> {
        pub fn mutate(&self) -> FnTuner<'_, Self, (), (), false> {
            let mutate_args_checker: mutate_ArgsChecker<'rs> = mutate_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            let fn_tuner: FnTuner<'_, Self, (), (), false> =
                self.data.mutate_data.add_config(mutate_args_checker, self);
            return unsafe { core::mem::transmute(fn_tuner) };
        }
        pub fn consume(&self) -> FnTuner<'_, Self, (), i32, false> {
            let consume_args_checker: consume_ArgsChecker<'rs> = consume_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            let fn_tuner: FnTuner<'_, Self, (), i32, false> = self
                .data
                .consume_data
                .add_config(consume_args_checker, self);
            return unsafe { core::mem::transmute(fn_tuner) };
        }
        pub fn sbox(&self) -> FnTuner<'_, Self, (), i32, false> {
            let sbox_args_checker: sbox_ArgsChecker<'rs> = sbox_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            let fn_tuner: FnTuner<'_, Self, (), i32, false> =
                self.data.sbox_data.add_config(sbox_args_checker, self);
            return unsafe { core::mem::transmute(fn_tuner) };
        }
        pub fn src(&self) -> FnTuner<'_, Self, (), i32, false> {
            let src_args_checker: src_ArgsChecker<'rs> = src_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            let fn_tuner: FnTuner<'_, Self, (), i32, false> =
                self.data.src_data.add_config(src_args_checker, self);
            return unsafe { core::mem::transmute(fn_tuner) };
        }
        pub fn sarc(&self) -> FnTuner<'_, Self, (), i32, false> {
            let sarc_args_checker: sarc_ArgsChecker<'rs> = sarc_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            let fn_tuner: FnTuner<'_, Self, (), i32, false> =
                self.data.sarc_data.add_config(sarc_args_checker, self);
            return unsafe { core::mem::transmute(fn_tuner) };
        }
        pub fn spbox(&self) -> FnTuner<'_, Self, (), i32, false> {
            let spbox_args_checker: spbox_ArgsChecker<'rs> = spbox_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            let fn_tuner: FnTuner<'_, Self, (), i32, false> =
                self.data.spbox_data.add_config(spbox_args_checker, self);
            return unsafe { core::mem::transmute(fn_tuner) };
        }
        pub fn sprc(&self) -> FnTuner<'_, Self, (), i32, false> {
            let sprc_args_checker: sprc_ArgsChecker<'rs> = sprc_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            let fn_tuner: FnTuner<'_, Self, (), i32, false> =
                self.data.sprc_data.add_config(sprc_args_checker, self);
            return unsafe { core::mem::transmute(fn_tuner) };
        }
        pub fn sparc(&self) -> FnTuner<'_, Self, (), i32, false> {
            let sparc_args_checker: sparc_ArgsChecker<'rs> = sparc_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            let fn_tuner: FnTuner<'_, Self, (), i32, false> =
                self.data.sparc_data.add_config(sparc_args_checker, self);
            return unsafe { core::mem::transmute(fn_tuner) };
        }
    }
    impl<'rs> TraitMockReceived<'rs> {
        pub fn mutate(&self, times: Times) -> FnVerifier<Self, ()> {
            let mutate_args_checker: mutate_ArgsChecker<'rs> = mutate_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            self.data
                .mutate_data
                .verify_received(mutate_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn consume(&self, times: Times) -> FnVerifier<Self, ()> {
            let consume_args_checker: consume_ArgsChecker<'rs> = consume_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            self.data
                .consume_data
                .verify_received(consume_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn sbox(&self, times: Times) -> FnVerifier<Self, ()> {
            let sbox_args_checker: sbox_ArgsChecker<'rs> = sbox_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            self.data
                .sbox_data
                .verify_received(sbox_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn src(&self, times: Times) -> FnVerifier<Self, ()> {
            let src_args_checker: src_ArgsChecker<'rs> = src_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            self.data.src_data.verify_received(src_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn sarc(&self, times: Times) -> FnVerifier<Self, ()> {
            let sarc_args_checker: sarc_ArgsChecker<'rs> = sarc_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            self.data
                .sarc_data
                .verify_received(sarc_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn spbox(&self, times: Times) -> FnVerifier<Self, ()> {
            let spbox_args_checker: spbox_ArgsChecker<'rs> = spbox_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            self.data
                .spbox_data
                .verify_received(spbox_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn sprc(&self, times: Times) -> FnVerifier<Self, ()> {
            let sprc_args_checker: sprc_ArgsChecker<'rs> = sprc_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            self.data
                .sprc_data
                .verify_received(sprc_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn sparc(&self, times: Times) -> FnVerifier<Self, ()> {
            let sparc_args_checker: sparc_ArgsChecker<'rs> = sparc_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            self.data
                .sparc_data
                .verify_received(sparc_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
}
