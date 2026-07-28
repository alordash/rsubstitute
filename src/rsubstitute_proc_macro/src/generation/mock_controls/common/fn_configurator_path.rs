use crate::generation::common::*;
use crate::generation::fn_info::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new(
    span: Span,
    mock_struct_path: Path,
    fn_info: &FnInfo,
    generic_arguments: &generic_arguments::Result,
    lifetime: Lifetime,
    maybe_owner_type: Option<Type>,
) -> Path {
    let mock_struct_type = Type::Path(TypePath {
        attrs: Vec::new(),
        qself: None,
        path: mock_struct_path,
    });
    let mock_arg = if let Some(first) = fn_info.source_signature.inputs.first()
        && let FnArg::Receiver(receiver) = first
        && let Some((and_token, mutability)) = match &receiver.kind {
            ReceiverKind::Reference(and_token, _, mutability) => {
                Some((and_token.clone(), mutability.clone()))
            }
            ReceiverKind::Typed(_, boxed_type) => {
                if let Type::Reference(reference) = boxed_type.as_ref() {
                    Some((reference.and_token.clone(), reference.mutability.clone()))
                } else {
                    None
                }
            }
            _ => None,
        } {
        GenericArgument::Type(Type::Reference(TypeReference {
            attrs: Vec::new(),
            and_token: and_token.clone(),
            lifetime: None,
            mutability: mutability.clone(),
            elem: Box::new(mock_struct_type),
        }))
    } else {
        GenericArgument::Type(mock_struct_type)
    };
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
                    mock_arg,
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
