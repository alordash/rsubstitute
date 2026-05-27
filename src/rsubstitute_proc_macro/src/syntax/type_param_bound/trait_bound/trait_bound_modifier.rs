use crate::syntax::constants::QUESTION;
use syn::*;

pub(crate) fn to_maybe_ident(trait_bound_modifier: &TraitBoundModifier) -> Option<Ident> {
    let result = match trait_bound_modifier {
        TraitBoundModifier::None => None,
        TraitBoundModifier::Maybe(question_token) => {
            Some(Ident::new(QUESTION, question_token.span))
        }
    };
    return result;
}
