use crate::generation::common::*;
use crate::generation::fn_info::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new(
    span: Span,
    fn_info: &FnInfo,
    generic_arguments: &generic_arguments::Result,
    lifetime: Lifetime,
    maybe_owner_type: Option<Type>,
) -> Path {
    let result = Path {
        leading_colon: None,
        segments: punctuated([PathSegment {
            ident: Ident::new("FnConfigurator", span),
            arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: Token![<](span),
                args: punctuated([
                    GenericArgument::Lifetime(lifetime),
                    generic_arguments.mock_generic_argument.clone(),
                    GenericArgument::Type(
                        maybe_owner_type.unwrap_or_else(|| Type::Path(self_type(span))),
                    ),
                    GenericArgument::Type(Type::Tuple(fn_info.arg_refs_tuple.clone())),
                    GenericArgument::Type(match &fn_info.return_type {
                        ReturnType::Default => void_type(span),
                        ReturnType::Type(_, return_type) => *return_type.clone(),
                    }),
                    generic_arguments.mock_generic_argument.clone(),
                    generic_arguments.has_return_value_argument.clone(),
                    generic_arguments.supports_base_calling_argument.clone(),
                    generic_arguments.passes_mock_to_callback_argument.clone(),
                ]),
                gt_token: Token![>](span),
            }),
        }]),
    };
    return result;
}
