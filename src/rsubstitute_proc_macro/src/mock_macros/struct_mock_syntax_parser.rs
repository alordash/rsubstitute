use crate::constants;
use crate::mock_macros::models::{StructMockSyntax, TraitImpl};
use quote::ToTokens;
use syn::parse::*;
use syn::*;

pub trait IStructMockSyntaxParser {
    fn parse(&self, input: ParseStream) -> Result<StructMockSyntax>;
}

pub struct StructMockSyntaxParser;

impl IStructMockSyntaxParser for StructMockSyntaxParser {
    fn parse(&self, input: ParseStream) -> Result<StructMockSyntax> {
        let r#struct = input.call(ItemStruct::parse)?;
        let mut maybe_new_fn = None;
        let mut trait_impls = Vec::new();
        let mut struct_impls = Vec::new();
        let mut ignored_impls = Vec::new();
        while !input.is_empty() {
            let item_impl = input.call(ItemImpl::parse)?;
            let Type::Path(type_path) = item_impl.self_ty.as_ref() else {
                panic!("{}", Self::STRUCT_MOCK_INVALID_IDENT_ERROR_MESSAGE);
            };
            let type_ident = type_path.path.require_ident()?;
            if *type_ident != r#struct.ident {
                panic!("{}", Self::STRUCT_MOCK_INVALID_IDENT_ERROR_MESSAGE);
            }
            if self.should_ignore_impl(&item_impl) {
                ignored_impls.push(item_impl);
            } else if item_impl.trait_.is_some() {
                let trait_impl = TraitImpl { item_impl };
                trait_impls.push(trait_impl);
            } else {
                if maybe_new_fn.is_none() {
                    maybe_new_fn = self.try_extract_new_fn(&item_impl);
                }
                struct_impls.push(item_impl);
            }
        }
        let Some(new_fn) = maybe_new_fn else {
            panic!("{}", Self::NO_NEW_FN_ERROR_MESSAGE);
        };
        let struct_mock_syntax = StructMockSyntax {
            r#struct,
            new_fn,
            trait_impls,
            struct_impls,
            ignored_impls,
        };
        return Ok(struct_mock_syntax);
    }
}

impl StructMockSyntaxParser {
    const STRUCT_MOCK_INVALID_IDENT_ERROR_MESSAGE: &'static str =
        "Struct mock should contain only `impl` blocks for it's own type.";
    const NO_NEW_FN_ERROR_MESSAGE: &'static str = "In order to be mockable structure must have function `pub fn new(args) -> Self`, where `args` is arbitrary collection of user-defined arguments.";
    const NEW_FN_MUST_BE_PUBLIC_ERROR_MESSAGE: &'static str = "Function `new` must be public.";
    const NEW_FN_MUST_HAVE_RETURN_TYPE_ERROR_MESSAGE_PART: &'static str =
        "Function `new` must have return type that is equal to `Self`";

    fn should_ignore_impl(&self, item_impl: &ItemImpl) -> bool {
        return item_impl
            .attrs
            .iter()
            .find(|attr| {
                attr.path().get_ident().is_some_and(|attr_ident| {
                    *attr_ident == constants::IGNORE_IMPL_ATTRIBUTE_IDENT.clone()
                })
            })
            .is_some();
    }

    fn try_extract_new_fn(&self, item_impl: &ItemImpl) -> Option<ImplItemFn> {
        let maybe_new_fn = item_impl
            .items
            .iter()
            .filter_map(|item| match item {
                ImplItem::Fn(impl_item_fn) if self.is_fn_impl_item_fn_is_new_fn(impl_item_fn) => {
                    Some(impl_item_fn.clone())
                }
                _ => None,
            })
            .next();
        return maybe_new_fn;
    }

    fn is_fn_impl_item_fn_is_new_fn(&self, impl_item_fn: &ImplItemFn) -> bool {
        if impl_item_fn.sig.ident == constants::NEW_IDENT.clone() {
            self.validate_new_fn(impl_item_fn);
            return true;
        }
        return false;
    }

    fn validate_new_fn(&self, impl_item_fn: &ImplItemFn) {
        let mut errors = Vec::new();

        match impl_item_fn.vis {
            Visibility::Public(_) => (),
            _ => errors.push(Self::NEW_FN_MUST_BE_PUBLIC_ERROR_MESSAGE.to_owned()),
        };

        let ReturnType::Type(_, return_type) = &impl_item_fn.sig.output else {
            errors.push(format!(
                "{}. Actually does not have return type.",
                Self::NEW_FN_MUST_HAVE_RETURN_TYPE_ERROR_MESSAGE_PART
            ));
            self.panic_with_new_fn_errors(errors);
        };

        let Type::Path(type_path) = &**return_type else {
            errors.push(self.format_new_fn_error_invalid_return_type(return_type));
            self.panic_with_new_fn_errors(errors);
        };

        let Some(type_ident) = type_path.path.get_ident() else {
            errors.push(self.format_new_fn_error_invalid_return_type(return_type));
            self.panic_with_new_fn_errors(errors);
        };

        if type_ident != &constants::SELF_TYPE_IDENT.clone() {
            errors.push(self.format_new_fn_error_invalid_return_type(return_type));
        }
    }

    fn format_new_fn_error_invalid_return_type(&self, return_type: &Box<Type>) -> String {
        format!(
            "{}. Actual type: {}",
            Self::NEW_FN_MUST_HAVE_RETURN_TYPE_ERROR_MESSAGE_PART.to_owned(),
            return_type.to_token_stream().to_string()
        )
    }

    fn panic_with_new_fn_errors(&self, errors: Vec<String>) -> ! {
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
}
