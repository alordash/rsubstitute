use rsubstitute::*;

#[mock]
trait SuperTrait {
    fn work(&self) -> i32;
}

// #[mock]
// trait Trait: SuperTrait {
//     fn overwork(&self) -> i32 {
//         self.work() + self.work()
//     }
// }

trait Trait: SuperTrait {
    fn overwork(&self) -> i32 {
        self.work() + self.work()
    }
}
#[allow(clippy::all)]
#[allow(unused)]
#[allow(unreachable_pub)]
#[allow(nonstandard_style)]
mod __rsubstitute_generated_TraitMock {
    use super::__rsubstitute_generated_SuperTraitMock::*;
    use super::*;
    #[doc(hidden)]
    pub struct overwork_Call<'__rsa> {
        pub __rs_generics: ::core::marker::PhantomData::<(&'__rsa (),)>
    }
    impl<'__rsa> ::rsubstitute::for_generated::IGenericsInfoProvider for overwork_Call::<'__rsa> {}
    impl<'__rsa> ::rsubstitute::for_generated::ICall for overwork_Call::<'__rsa> {}
    #[doc(hidden)]
    struct overwork_ArgsChecker<'__rsa> {
        pub __rs_generics: ::core::marker::PhantomData::<(&'__rsa (),)>
    }
    impl<'__rsa> ::rsubstitute::for_generated::IGenericsInfoProvider for overwork_ArgsChecker::<'__rsa> {}
    impl<'__rsa> ::rsubstitute::for_generated::IArgsChecker for overwork_ArgsChecker::<'__rsa> {}
    impl<'__rsa> Trait::<> for SuperTraitMock::<'__rsa> {
        fn overwork(&self) -> i32 {
            let call = overwork_Call::<'_> { __rs_generics: ::core::marker::PhantomData };
            let fn_data: &::rsubstitute::for_generated::FnData<'_, SuperTraitMock::<'_>, true, false, true> = ::rsubstitute::for_generated::ISharedMockData::get_shared_fn_data(&self.__rs_data, "Trait", "overwork", ::rsubstitute::for_generated::IGenericsInfoProvider::get_generics_hash_key(&call));
            fn_data.handle(self, call)
        }
    }
    impl<'__rsa> SuperTraitSetup::<'__rsa> {
        pub fn overwork(&self) -> ::rsubstitute::for_generated::FnConfigurator::<'_, SuperTraitMock::<'__rsa>, Self, (), i32, &SuperTraitMock::<'__rsa>, true, false, true> {
            let args_checker = overwork_ArgsChecker::<'__rsa> { __rs_generics: ::core::marker::PhantomData };
            let fn_data: &::rsubstitute::for_generated::FnData<'_, SuperTraitMock::<'__rsa>, true, false, true> = ::rsubstitute::for_generated::ISharedMockData::get_shared_fn_data(&self.__rs_data, "Trait", "overwork", ::rsubstitute::for_generated::IGenericsInfoProvider::get_generics_hash_key(&args_checker));
            let fn_configurator: ::rsubstitute::for_generated::FnConfigurator::<'_, SuperTraitMock::<'__rsa>, Self, (), i32, &SuperTraitMock::<'__rsa>, true, false, true> = fn_data.add_config(args_checker, self);
            ::rsubstitute::transmute_lifetime!(fn_configurator )
        }
    }
    impl<'__rsa> SuperTraitReceived::<'__rsa> {
        pub fn overwork(&self, times: ::rsubstitute::for_generated::Times) -> ::rsubstitute::for_generated::ArgRefsBinder::<Self, ()> {
            let args_checker = overwork_ArgsChecker::<'__rsa> { __rs_generics: ::core::marker::PhantomData };
            let fn_data: &::rsubstitute::for_generated::FnData<'_, SuperTraitMock::<'_>, true, false, true> = ::rsubstitute::for_generated::ISharedMockData::get_shared_fn_data(&self.__rs_data, "Trait", "overwork", ::rsubstitute::for_generated::IGenericsInfoProvider::get_generics_hash_key(&args_checker));
            fn_data.verify_received(args_checker, times);
            rsubstitute::for_generated::ArgRefsBinder::new(self.clone())
        }
    }
}

trait Foo {
    fn work(&self);
}

trait Bar: Foo {
    fn work(&self) {
        self.work();
    }
}

mod tests {
    #[test]
    fn compile() {}
}