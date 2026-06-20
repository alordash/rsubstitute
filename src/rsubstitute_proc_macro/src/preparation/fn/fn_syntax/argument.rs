use crate::generation::rsubstitute_lifetime;
use crate::preparation::r#fn::models::*;
use crate::syntax::{path, punctuated};
use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::visit_mut::*;
use syn::*;

pub(crate) fn new((number, pat_type): (usize, PatType)) -> Argument {
    let ident = prepare_ident(number, &pat_type);

    let mut ptr_style_type = pat_type.ty.clone();
    replace_refs_with_ptrs(&mut ptr_style_type);

    let mut ref_style_type = pat_type.ty.clone();
    replace_anonymous_lifetimes(&mut ref_style_type);

    let control_fn_arg =
        generate_control_fn_arg(ident.span(), pat_type.pat.clone(), *ref_style_type.clone());

    let result = Argument {
        source_pat_type: pat_type,
        ident,
        ptr_style_type,
        ref_style_type,
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

fn replace_refs_with_ptrs(ty: &mut Type) {
    ReferenceToPtrConverter.visit_type_mut(ty);
}

struct ReferenceToPtrConverter;

impl VisitMut for ReferenceToPtrConverter {
    fn visit_type_mut(&mut self, i: &mut Type) {
        let Type::Reference(i_ref) = i else {
            visit_mut::visit_type_mut(self, i);
            return;
        };
        let (const_token, mutability) = match i_ref.mutability {
            Some(x) => (None, Some(x)),
            None => (Some(Token![const](Span::call_site())), None),
        };

        // SAFETY: `empty_elem` is not used in any way except to mutate `Box<Type>` in place.
        let empty_elem = unsafe { Box::new_uninit().assume_init() };
        let elem = core::mem::replace(&mut i_ref.elem, empty_elem);
        let mut i_ptr = TypePtr {
            star_token: Token![*](i_ref.and_token.span),
            const_token,
            mutability,
            elem,
        };
        visit_mut::visit_type_ptr_mut(self, &mut i_ptr);
        *i = Type::Ptr(i_ptr);
    }
}

fn replace_anonymous_lifetimes(ty: &mut Type) {
    AnonymousLifetimeReplacer.visit_type_mut(ty)
}

struct AnonymousLifetimeReplacer;

impl VisitMut for AnonymousLifetimeReplacer {
    fn visit_type_reference_mut(&mut self, i: &mut TypeReference) {
        if i.lifetime.is_none() {
            i.lifetime = Some(rsubstitute_lifetime::new(i.span()))
        }
        visit_mut::visit_type_reference_mut(self, i);
    }
}

fn generate_control_fn_arg(span: Span, pat: Box<Pat>, ref_style_type: Type) -> FnArg {
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
                path: path::new_generics(span, ["Into"], GenericArgument::Type(ref_style_type)),
            })]),
        })),
    };

    return FnArg::Typed(result);
}
