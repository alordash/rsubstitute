use std::marker::PhantomData;
use std::ops::Deref;

pub struct FnVerifier<'rs, TOwner, TArgRefsTuple> {
    owner: &'rs TOwner,
    _phantom_arg_refs_tuple: PhantomData<TArgRefsTuple>,
}

impl<'rs, TOwner, TArgsRefsTuple> FnVerifier<'rs, TOwner, TArgsRefsTuple> {
    pub fn new<'a>(owner: &'a TOwner) -> FnVerifier<'rs, TOwner, TArgsRefsTuple> {
        FnVerifier {
            owner: unsafe { core::mem::transmute(owner) },
            _phantom_arg_refs_tuple: PhantomData,
        }
    }
}

impl<'rs, TOwner, TArgsRefsTuple> Deref for FnVerifier<'rs, TOwner, TArgsRefsTuple> {
    type Target = TOwner;

    fn deref(&self) -> &Self::Target {
        self.owner
    }
}
