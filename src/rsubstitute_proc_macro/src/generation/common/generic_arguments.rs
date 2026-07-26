use crate::common::models::*;
use crate::generation::fn_info::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) struct Result {
    pub mock_generic_argument: GenericArgument,
    pub has_return_value_argument: GenericArgument,
    pub supports_base_calling_argument: GenericArgument,
    pub passes_mock_to_callback_argument: GenericArgument,
}

pub(crate) fn new(ctx: &Context, span: Span, mock_struct_path: Path, fn_info: &FnInfo) -> Result {
    let result = Result {
        mock_generic_argument: GenericArgument::Type(Type::Path(TypePath {
            attrs: Vec::new(),
            qself: None,
            path: mock_struct_path,
        })),
        has_return_value_argument: generic_argument::bool(
            span,
            match fn_info.return_type {
                ReturnType::Default => false,
                ReturnType::Type(_, _) => true,
            },
        ),
        supports_base_calling_argument: generic_argument::bool(
            span,
            ctx.support_base_calling && fn_info.maybe_base_impl.is_some(),
        ),
        passes_mock_to_callback_argument: generic_argument::bool(span, false),
    };
    return result;
}
