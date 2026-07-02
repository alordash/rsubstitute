use crate::common::rsubstitute_lifetime;
use crate::preparation::r#fn::models::*;
use crate::preparation::r#fn::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn new((number, mut pat_type): (usize, PatType)) -> Argument {
    let ident = prepare_ident(number, &pat_type);
    pat_type = common::replace_arg_pat_with_ident(pat_type, ident.clone());

    let ptr_style_type = r#type::replace_references_with_pointers(pat_type.ty.clone());

    let ref_style_type = r#type::replace_anonymous_lifetimes_in_references(
        pat_type.ty.clone(),
        &rsubstitute_lifetime::new(pat_type.span()),
    );

    let generic_arg_style_type =
        r#type::replace_anonymous_references_with_pointers(pat_type.ty.clone());

    // TODO - perhaps need to pass here ptr_style_type
    let control_fn_arg =
        generate_control_fn_arg(ident.span(), pat_type.pat.clone(), ref_style_type.clone());

    let result = Argument {
        source_pat_type: pat_type,
        ident,
        ptr_style_type,
        ref_style_type,
        generic_arg_style_type,
        control_fn_arg,
    };
    return result;
}

fn prepare_ident(number: usize, pat_type: &PatType) -> Ident {
    let result = match pat_type.pat.as_ref() {
        Pat::Ident(pat_ident) => pat_ident.ident.clone(),
        not_ident => Ident::new(&format!("__pat_arg{number}"), not_ident.span()),
    };

    return result;
}

fn generate_control_fn_arg(span: Span, pat: Box<Pat>, ptr_style_type: Box<Type>) -> FnArg {
    let result = PatType {
        attrs: Vec::new(),
        pat,
        colon_token: Token![:](span),
        ty: Box::new(Type::ImplTrait(TypeImplTrait {
            impl_token: Token![impl](span),
            bounds: punctuated([TypeParamBound::Trait(TraitBound {
                paren_token: None,
                modifier: TraitBoundModifier::None,
                lifetimes: None,
                path: path::new_generics(
                    span,
                    ["Into"],
                    GenericArgument::Type(Type::Path(TypePath {
                        qself: None,
                        path: path::new_generics(
                            span,
                            ["Arg"],
                            GenericArgument::Type(*ptr_style_type),
                        ),
                    })),
                ),
            })]),
        })),
    };

    return FnArg::Typed(result);
}
