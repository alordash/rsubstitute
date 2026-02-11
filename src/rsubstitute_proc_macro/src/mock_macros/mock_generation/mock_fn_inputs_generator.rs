use crate::syntax::IArgIdentExtractor;
use std::sync::Arc;
use syn::punctuated::Punctuated;
use syn::*;

pub trait IMockFnInputsGenerator {
    fn generate(&self, original_inputs: &[FnArg]) -> Punctuated<FnArg, Token![,]>;
}

pub(crate) struct MockFnInputsGenerator {
    pub arg_ident_extractor: Arc<dyn IArgIdentExtractor>,
}

impl IMockFnInputsGenerator for MockFnInputsGenerator {
    fn generate(&self, original_inputs: &[FnArg]) -> Punctuated<FnArg, Token!(,)> {
        let inputs = original_inputs
            .iter()
            .enumerate()
            .map(|(arg_number, fn_arg)| match fn_arg {
                FnArg::Receiver(_) => fn_arg.clone(),
                FnArg::Typed(typed) => self.generate_typed_fn_arg(arg_number, typed),
            })
            .collect();
        return inputs;
    }
}

impl MockFnInputsGenerator {
    fn generate_typed_fn_arg(&self, arg_number: usize, typed: &PatType) -> FnArg {
        let arg_ident = self.arg_ident_extractor.extract(arg_number, typed);
        let fn_arg = FnArg::Typed(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Ident(PatIdent {
                attrs: Vec::new(),
                by_ref: None,
                mutability: None,
                ident: arg_ident,
                subpat: None,
            })),
            colon_token: Default::default(),
            ty: typed.ty.clone(),
        });
        return fn_arg;
    }
}
