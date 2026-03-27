use crate::constants;
use crate::mock_generation::models::*;
use quote::ToTokens;
use std::cell::LazyCell;
use syn::parse::*;
use syn::*;

pub(crate) fn parse(input: ParseStream) -> Result<StructMockSyntax> {
    let r#struct = input.call(ItemStruct::parse)?;
    let mut maybe_new_fn = None;
    let mut trait_impls = Vec::new();
    let mut struct_impls = Vec::new();
    while !input.is_empty() {
        let item_impl = input.call(ItemImpl::parse)?;
        let Type::Path(type_path) = item_impl.self_ty.as_ref() else {
            panic!("{STRUCT_MOCK_INVALID_IDENT_ERROR_MESSAGE}");
        };
        assert_eq!(
            type_path.path.segments.len(),
            1,
            "{STRUCT_MOCK_INVALID_IMPL_TARGET_PATH_ERROR_MSG}"
        );
        let type_path_segment = type_path.path.segments.first().unwrap();
        let type_ident = &type_path_segment.ident;
        if *type_ident != r#struct.ident {
            panic!("{STRUCT_MOCK_INVALID_IDENT_ERROR_MESSAGE}");
        }
        if item_impl.trait_.is_some() {
            let trait_path = item_impl
                .trait_
                .as_ref()
                .expect("trait_impls should contain only trait implementations.")
                .1
                .clone();
            let trait_impl = TraitImpl {
                trait_path,
                item_impl,
            };
            trait_impls.push(trait_impl);
        } else {
            validate_item_impl(&item_impl);
            if maybe_new_fn.is_none() {
                maybe_new_fn = try_extract_new_fn(&item_impl);
            }
            struct_impls.push(item_impl);
        }
    }
    let Some(new_fn) = maybe_new_fn else {
        panic!("{}", NO_NEW_FN_ERROR_MESSAGE);
    };
    let struct_mock_syntax = StructMockSyntax {
        r#struct,
        new_fn,
        trait_impls,
        struct_impls,
    };
    return Ok(struct_mock_syntax);
}

const STRUCT_MOCK_INVALID_IMPL_TARGET_PATH_ERROR_MSG: &'static str = "(`impl` target's path length) Struct type ident in `impl` block can not be long path, it should be just a single ident.";
const STRUCT_MOCK_INVALID_IDENT_ERROR_MESSAGE: &'static str =
    "Struct mock should contain only `impl` blocks for it's own type.";
const STRUCT_MOCK_INVALID_FN_SIG_ERROR_MESSAGE: &'static str = "Struct mock `impl` functions should all be associated.";
const NO_NEW_FN_ERROR_MESSAGE: &'static str = "In order to be mockable structure must have function `pub(crate) fn new(args) -> Self`, where `args` is arbitrary collection of user-defined arguments.";
const NEW_FN_MUST_BE_PUBLIC_ERROR_MESSAGE: &'static str = "Function `new` must be public.";
const NEW_FN_MUST_HAVE_RETURN_TYPE_ERROR_MESSAGE_PART: &'static str =
    "Function `new` must have return type that is equal to `Self`";

fn validate_item_impl(item_impl: &ItemImpl) {
    let impl_item_fns = item_impl.items.iter().filter_map(|x| match x {
        ImplItem::Fn(impl_item_fn) => Some(impl_item_fn),
        _ => None,
    });

    for impl_item_fn in impl_item_fns {
        if is_fn_impl_item_fn_is_new_fn(impl_item_fn) {
            validate_new_fn(impl_item_fn);
            continue;
        }
        if let Some(first_arg) = impl_item_fn.sig.inputs.first()
            && let FnArg::Receiver(_) = first_arg
        {
            continue;
        }
        panic!("{}", STRUCT_MOCK_INVALID_FN_SIG_ERROR_MESSAGE);
    }
}

fn try_extract_new_fn(item_impl: &ItemImpl) -> Option<ImplItemFn> {
    let maybe_new_fn = item_impl
        .items
        .iter()
        .filter_map(|item| match item {
            ImplItem::Fn(impl_item_fn) if is_fn_impl_item_fn_is_new_fn(impl_item_fn) => {
                Some(impl_item_fn.clone())
            }
            _ => None,
        })
        .next();
    return maybe_new_fn;
}

fn is_fn_impl_item_fn_is_new_fn(impl_item_fn: &ImplItemFn) -> bool {
    if impl_item_fn.sig.ident == constants::NEW_IDENT.clone() {
        return true;
    }
    return false;
}

fn validate_new_fn(impl_item_fn: &ImplItemFn) {
    let mut errors = Vec::new();

    match impl_item_fn.vis {
        Visibility::Public(_) => (),
        _ => errors.push(NEW_FN_MUST_BE_PUBLIC_ERROR_MESSAGE.to_owned()),
    };

    let ReturnType::Type(_, return_type) = &impl_item_fn.sig.output else {
        errors.push(format!(
            "{}. Actually does not have return type.",
            NEW_FN_MUST_HAVE_RETURN_TYPE_ERROR_MESSAGE_PART
        ));
        panic_with_new_fn_errors(errors);
    };

    let Type::Path(type_path) = &**return_type else {
        errors.push(format_new_fn_error_invalid_return_type(return_type));
        panic_with_new_fn_errors(errors);
    };

    let Some(type_ident) = type_path.path.get_ident() else {
        errors.push(format_new_fn_error_invalid_return_type(return_type));
        panic_with_new_fn_errors(errors);
    };

    if type_ident != &constants::SELF_TYPE_IDENT.clone() {
        errors.push(format_new_fn_error_invalid_return_type(return_type));
    }

    if !errors.is_empty() {
        panic_with_new_fn_errors(errors);
    }
}

fn format_new_fn_error_invalid_return_type(return_type: &Box<Type>) -> String {
    format!(
        "{}. Actual type: {}",
        NEW_FN_MUST_HAVE_RETURN_TYPE_ERROR_MESSAGE_PART.to_owned(),
        return_type.to_token_stream().to_string()
    )
}

fn panic_with_new_fn_errors(errors: Vec<String>) -> ! {
    let errors_count = errors.len();
    let numbered_errors: Vec<_> = errors
        .into_iter()
        .enumerate()
        .map(|(i, e)| {
            let number = i + 1;
            format!("{number}. {e}")
        })
        .collect();
    let errors_lines = numbered_errors.join("\n");
    panic!(
        "Function `new` is invalid. List of errors ({errors_count}):
{errors_lines}"
    )
}
