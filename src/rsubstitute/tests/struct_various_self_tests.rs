use rsubstitute::macros::*;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

// TODO - right now self type is always `&self` in base methods
mocked_base! {
    #[derive(Clone)]
    struct Struct;

    impl Struct {
        pub fn new() -> Self { Self }

        pub fn mutate(&mut self) {}

        pub fn consume(self) -> i32 { 10 }


        pub fn sbox(self: Box<Self>) -> i32 { 212 }
        pub fn src(self: Rc<Self>) -> i32 { 212 }
        pub fn sarc(self: Arc<Self>) -> i32 { 212 }

        pub fn spbox(self: Pin<Box<Self>>) -> i32 { 212 }
        pub fn sprc(self: Pin<Rc<Self>>) -> i32 { 212 }
        pub fn sparc(self: Pin<Arc<Self>>) -> i32 { 212 }
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use rsubstitute::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn flex() {
        let mut mock = Struct::new();

        mock.setup.sbox().call_base();
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
struct Struct;
#[cfg(not(test))]
impl Struct {
    pub fn new() -> Self { Self }

    pub fn mutate(&mut self) {}

    pub fn consume(self) -> i32 { 10 }


    pub fn sbox(self: Box<Self>) -> i32 { 212 }
    pub fn src(self: Rc<Self>) -> i32 { 212 }
    pub fn sarc(self: Arc<Self>) -> i32 { 212 }

    pub fn spbox(self: Pin<Box<Self>>) -> i32 { 212 }
    pub fn sprc(self: Pin<Rc<Self>>) -> i32 { 212 }
    pub fn sparc(self: Pin<Arc<Self>>) -> i32 { 212 }
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
    #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsInfoProvider, CloneForRSubstitute)]
    pub struct mutate_Call<'__rs> {
        _phantom_lifetime: PhantomData<&'__rs ()>
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsInfoProvider)]
    pub struct mutate_ArgsChecker<'__rs> {
        _phantom_lifetime: PhantomData<&'__rs ()>
    }
    impl<'__rs> IArgsChecker for mutate_ArgsChecker<'__rs> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &mutate_Call<'__rs> = dyn_call.downcast_ref();
            vec![]
        }
    }
    #[doc(hidden)]
    #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsInfoProvider, CloneForRSubstitute)]
    pub struct consume_Call<'__rs> {
        _phantom_lifetime: PhantomData<&'__rs ()>
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsInfoProvider)]
    pub struct consume_ArgsChecker<'__rs> {
        _phantom_lifetime: PhantomData<&'__rs ()>
    }
    impl<'__rs> IArgsChecker for consume_ArgsChecker<'__rs> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &consume_Call<'__rs> = dyn_call.downcast_ref();
            vec![]
        }
    }
    #[doc(hidden)]
    #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsInfoProvider, CloneForRSubstitute)]
    pub struct sbox_Call<'__rs> {
        _phantom_lifetime: PhantomData<&'__rs ()>
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsInfoProvider)]
    pub struct sbox_ArgsChecker<'__rs> {
        _phantom_lifetime: PhantomData<&'__rs ()>
    }
    impl<'__rs> IArgsChecker for sbox_ArgsChecker<'__rs> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &sbox_Call<'__rs> = dyn_call.downcast_ref();
            vec![]
        }
    }
    #[doc(hidden)]
    #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsInfoProvider, CloneForRSubstitute)]
    pub struct src_Call<'__rs> {
        _phantom_lifetime: PhantomData<&'__rs ()>
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsInfoProvider)]
    pub struct src_ArgsChecker<'__rs> {
        _phantom_lifetime: PhantomData<&'__rs ()>
    }
    impl<'__rs> IArgsChecker for src_ArgsChecker<'__rs> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &src_Call<'__rs> = dyn_call.downcast_ref();
            vec![]
        }
    }
    #[doc(hidden)]
    #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsInfoProvider, CloneForRSubstitute)]
    pub struct sarc_Call<'__rs> {
        _phantom_lifetime: PhantomData<&'__rs ()>
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsInfoProvider)]
    pub struct sarc_ArgsChecker<'__rs> {
        _phantom_lifetime: PhantomData<&'__rs ()>
    }
    impl<'__rs> IArgsChecker for sarc_ArgsChecker<'__rs> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &sarc_Call<'__rs> = dyn_call.downcast_ref();
            vec![]
        }
    }
    #[doc(hidden)]
    #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsInfoProvider, CloneForRSubstitute)]
    pub struct spbox_Call<'__rs> {
        _phantom_lifetime: PhantomData<&'__rs ()>
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsInfoProvider)]
    pub struct spbox_ArgsChecker<'__rs> {
        _phantom_lifetime: PhantomData<&'__rs ()>
    }
    impl<'__rs> IArgsChecker for spbox_ArgsChecker<'__rs> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &spbox_Call<'__rs> = dyn_call.downcast_ref();
            vec![]
        }
    }
    #[doc(hidden)]
    #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsInfoProvider, CloneForRSubstitute)]
    pub struct sprc_Call<'__rs> {
        _phantom_lifetime: PhantomData<&'__rs ()>
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsInfoProvider)]
    pub struct sprc_ArgsChecker<'__rs> {
        _phantom_lifetime: PhantomData<&'__rs ()>
    }
    impl<'__rs> IArgsChecker for sprc_ArgsChecker<'__rs> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &sprc_Call<'__rs> = dyn_call.downcast_ref();
            vec![]
        }
    }
    #[doc(hidden)]
    #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsInfoProvider, CloneForRSubstitute)]
    pub struct sparc_Call<'__rs> {
        _phantom_lifetime: PhantomData<&'__rs ()>
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsInfoProvider)]
    pub struct sparc_ArgsChecker<'__rs> {
        _phantom_lifetime: PhantomData<&'__rs ()>
    }
    impl<'__rs> IArgsChecker for sparc_ArgsChecker<'__rs> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &sparc_Call<'__rs> = dyn_call.downcast_ref();
            vec![]
        }
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct StructData<'__rs> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        pub mutate: FnData<'static, Struct<'__rs>, true, true>,
        pub consume: FnData<'static, Struct<'__rs>, true, true>,
        pub sbox: FnData<'static, Struct<'__rs>, true, true>,
        pub src: FnData<'static, Struct<'__rs>, true, true>,
        pub sarc: FnData<'static, Struct<'__rs>, true, true>,
        pub spbox: FnData<'static, Struct<'__rs>, true, true>,
        pub sprc: FnData<'static, Struct<'__rs>, true, true>,
        pub sparc: FnData<'static, Struct<'__rs>, true, true>
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct StructSetup<'__rs> {
        data: Arc<StructData<'__rs>>
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct StructReceived<'__rs> {
        data: Arc<StructData<'__rs>>
    }
    #[derive(Clone)]
    #[doc(hidden)]
    pub struct Struct_InnerData;

    impl Struct_InnerData {
        pub fn new() -> Self { Self }
    }
    #[derive(Clone)]
    pub struct Struct<'__rs> {
        pub setup: StructSetup<'__rs>,
        pub received: StructReceived<'__rs>,
        pub data: Arc<StructData<'__rs>>,
        inner_data: Struct_InnerData
    }
    impl<'__rs> Deref for Struct<'__rs> {
        type Target = Struct_InnerData;

        fn deref(&self) -> &Self::Target { &self.inner_data }
    }
    impl<'__rs> Struct<'__rs> {
        pub fn mutate(&mut self) {
            let call: mutate_Call<'_> = mutate_Call { _phantom_lifetime: PhantomData };
            self.data.mutate.handle_base(&self, call, Self::base_mutate);
        }

        pub fn consume(self) -> i32 {
            let call: consume_Call<'_> = consume_Call { _phantom_lifetime: PhantomData };
            return self.data.consume.handle_base_returning(&self, call, Self::base_consume);
        }


        pub fn sbox(self: Box<Self>) -> i32 {
            let call: sbox_Call<'_> = sbox_Call { _phantom_lifetime: PhantomData };
            return self.data.sbox.handle_base_returning(&self, call, Self::base_sbox);
        }
        pub fn src(self: Rc<Self>) -> i32 {
            let call: src_Call<'_> = src_Call { _phantom_lifetime: PhantomData };
            return self.data.src.handle_base_returning(&self, call, Self::base_src);
        }
        pub fn sarc(self: Arc<Self>) -> i32 {
            let call: sarc_Call<'_> = sarc_Call { _phantom_lifetime: PhantomData };
            return self.data.sarc.handle_base_returning(&self, call, Self::base_sarc);
        }

        pub fn spbox(self: Pin<Box<Self>>) -> i32 {
            let call: spbox_Call<'_> = spbox_Call { _phantom_lifetime: PhantomData };
            return self.data.spbox.handle_base_returning(&self, call, Self::base_spbox);
        }
        pub fn sprc(self: Pin<Rc<Self>>) -> i32 {
            let call: sprc_Call<'_> = sprc_Call { _phantom_lifetime: PhantomData };
            return self.data.sprc.handle_base_returning(&self, call, Self::base_sprc);
        }
        pub fn sparc(self: Pin<Arc<Self>>) -> i32 {
            let call: sparc_Call<'_> = sparc_Call { _phantom_lifetime: PhantomData };
            return self.data.sparc.handle_base_returning(&self, call, Self::base_sparc);
        }
    }
    impl<'__rs> Struct<'__rs> {
        pub fn new() -> Self {
            let data = Arc::new(StructData { _phantom_lifetime: PhantomData, mutate: FnData::new("mutate"), consume: FnData::new("consume"), sbox: FnData::new("sbox"), src: FnData::new("src"), sarc: FnData::new("sarc"), spbox: FnData::new("spbox"), sprc: FnData::new("sprc"), sparc: FnData::new("sparc") });
            let inner_data = Struct_InnerData::new();
            return Struct { setup: StructSetup { data: data.clone() }, received: StructReceived { data: data.clone() }, data, inner_data };
        }
        fn base_mutate(&self, call: mutate_Call<'_>) {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)] let mutate_Call { .. } = call;
        }
        fn base_consume(&self, call: consume_Call<'_>) -> i32 {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)] let consume_Call { .. } = call;
            10
        }


        fn base_sbox(&self, call: sbox_Call<'_>) -> i32 {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)] let sbox_Call { .. } = call;
            212
        }
        fn base_src(&self, call: src_Call<'_>) -> i32 {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)] let src_Call { .. } = call;
            212
        }
        fn base_sarc(&self, call: sarc_Call<'_>) -> i32 {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)] let sarc_Call { .. } = call;
            212
        }

        fn base_spbox(&self, call: spbox_Call<'_>) -> i32 {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)] let spbox_Call { .. } = call;
            212
        }
        fn base_sprc(&self, call: sprc_Call<'_>) -> i32 {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)] let sprc_Call { .. } = call;
            212
        }
        fn base_sparc(&self, call: sparc_Call<'_>) -> i32 {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)] let sparc_Call { .. } = call;
            212
        }
    }
    impl<'__rs> StructSetup<'__rs> {
        pub fn mutate<'__rsa>(&self) -> FnTuner<'_, Struct<'__rs>, Self, (), (), true, true> {
            let mutate_args_checker: mutate_ArgsChecker<'_> = mutate_ArgsChecker { _phantom_lifetime: PhantomData };
            let fn_tuner: FnTuner<'_, Struct<'__rs>, Self, (), (), true, true> = self.data.mutate.add_config(mutate_args_checker, self);
            return transmute_lifetime!(fn_tuner );
        }
        pub fn consume<'__rsa>(&self) -> FnTuner<'_, Struct<'__rs>, Self, (), i32, true, true> {
            let consume_args_checker: consume_ArgsChecker<'_> = consume_ArgsChecker { _phantom_lifetime: PhantomData };
            let fn_tuner: FnTuner<'_, Struct<'__rs>, Self, (), i32, true, true> = self.data.consume.add_config(consume_args_checker, self);
            return transmute_lifetime!(fn_tuner );
        }
        pub fn sbox<'__rsa>(&self) -> FnTuner<'_, Struct<'__rs>, Self, (), i32, true, true> {
            let sbox_args_checker: sbox_ArgsChecker<'_> = sbox_ArgsChecker { _phantom_lifetime: PhantomData };
            let fn_tuner: FnTuner<'_, Struct<'__rs>, Self, (), i32, true, true> = self.data.sbox.add_config(sbox_args_checker, self);
            return transmute_lifetime!(fn_tuner );
        }
        pub fn src<'__rsa>(&self) -> FnTuner<'_, Struct<'__rs>, Self, (), i32, true, true> {
            let src_args_checker: src_ArgsChecker<'_> = src_ArgsChecker { _phantom_lifetime: PhantomData };
            let fn_tuner: FnTuner<'_, Struct<'__rs>, Self, (), i32, true, true> = self.data.src.add_config(src_args_checker, self);
            return transmute_lifetime!(fn_tuner );
        }
        pub fn sarc<'__rsa>(&self) -> FnTuner<'_, Struct<'__rs>, Self, (), i32, true, true> {
            let sarc_args_checker: sarc_ArgsChecker<'_> = sarc_ArgsChecker { _phantom_lifetime: PhantomData };
            let fn_tuner: FnTuner<'_, Struct<'__rs>, Self, (), i32, true, true> = self.data.sarc.add_config(sarc_args_checker, self);
            return transmute_lifetime!(fn_tuner );
        }
        pub fn spbox<'__rsa>(&self) -> FnTuner<'_, Struct<'__rs>, Self, (), i32, true, true> {
            let spbox_args_checker: spbox_ArgsChecker<'_> = spbox_ArgsChecker { _phantom_lifetime: PhantomData };
            let fn_tuner: FnTuner<'_, Struct<'__rs>, Self, (), i32, true, true> = self.data.spbox.add_config(spbox_args_checker, self);
            return transmute_lifetime!(fn_tuner );
        }
        pub fn sprc<'__rsa>(&self) -> FnTuner<'_, Struct<'__rs>, Self, (), i32, true, true> {
            let sprc_args_checker: sprc_ArgsChecker<'_> = sprc_ArgsChecker { _phantom_lifetime: PhantomData };
            let fn_tuner: FnTuner<'_, Struct<'__rs>, Self, (), i32, true, true> = self.data.sprc.add_config(sprc_args_checker, self);
            return transmute_lifetime!(fn_tuner );
        }
        pub fn sparc<'__rsa>(&self) -> FnTuner<'_, Struct<'__rs>, Self, (), i32, true, true> {
            let sparc_args_checker: sparc_ArgsChecker<'_> = sparc_ArgsChecker { _phantom_lifetime: PhantomData };
            let fn_tuner: FnTuner<'_, Struct<'__rs>, Self, (), i32, true, true> = self.data.sparc.add_config(sparc_args_checker, self);
            return transmute_lifetime!(fn_tuner );
        }
    }
    impl<'__rs> StructReceived<'__rs> {
        pub fn mutate<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let mutate_args_checker: mutate_ArgsChecker<'_> = mutate_ArgsChecker { _phantom_lifetime: PhantomData };
            self.data.mutate.verify_received(mutate_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn consume<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let consume_args_checker: consume_ArgsChecker<'_> = consume_ArgsChecker { _phantom_lifetime: PhantomData };
            self.data.consume.verify_received(consume_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn sbox<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let sbox_args_checker: sbox_ArgsChecker<'_> = sbox_ArgsChecker { _phantom_lifetime: PhantomData };
            self.data.sbox.verify_received(sbox_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn src<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let src_args_checker: src_ArgsChecker<'_> = src_ArgsChecker { _phantom_lifetime: PhantomData };
            self.data.src.verify_received(src_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn sarc<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let sarc_args_checker: sarc_ArgsChecker<'_> = sarc_ArgsChecker { _phantom_lifetime: PhantomData };
            self.data.sarc.verify_received(sarc_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn spbox<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let spbox_args_checker: spbox_ArgsChecker<'_> = spbox_ArgsChecker { _phantom_lifetime: PhantomData };
            self.data.spbox.verify_received(spbox_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn sprc<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let sprc_args_checker: sprc_ArgsChecker<'_> = sprc_ArgsChecker { _phantom_lifetime: PhantomData };
            self.data.sprc.verify_received(sprc_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn sparc<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let sparc_args_checker: sparc_ArgsChecker<'_> = sparc_ArgsChecker { _phantom_lifetime: PhantomData };
            self.data.sparc.verify_received(sparc_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn no_other_calls(&self) { self.data.verify_received_nothing_else(); }
    }
}