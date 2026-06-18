use std::marker::PhantomData;
use std::ops::Deref;

pub struct FnVerifier<TOwner, TArgRefsTuple> {
    owner: TOwner,
    _phantom_arg_refs_tuple: PhantomData<TArgRefsTuple>,
}

impl<TOwner, TArgsRefsTuple> FnVerifier<TOwner, TArgsRefsTuple> {
    pub fn new(owner: TOwner) -> FnVerifier<TOwner, TArgsRefsTuple> {
        FnVerifier {
            owner,
            _phantom_arg_refs_tuple: PhantomData,
        }
    }
}

impl<TOwner, TArgsRefsTuple> Deref for FnVerifier<TOwner, TArgsRefsTuple> {
    type Target = TOwner;

    fn deref(&self) -> &Self::Target {
        &self.owner
    }
}
