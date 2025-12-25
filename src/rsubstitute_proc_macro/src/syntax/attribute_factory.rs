use proc_macro2::{Ident, TokenStream};
use std::str::FromStr;
use syn::{AttrStyle, Attribute, MacroDelimiter, Meta, MetaList, Path, PathArguments, PathSegment};

pub trait IAttributeFactory {
    fn create(&self, ident: Ident, arguments: &str) -> Attribute;
}

pub struct AttributeFactory;

impl IAttributeFactory for AttributeFactory {
    fn create(&self, ident: Ident, arguments: &str) -> Attribute {
        let tokens =
            TokenStream::from_str(arguments).expect("Should be able to parse attribute arg.");
        let result = Attribute {
            pound_token: Default::default(),
            style: AttrStyle::Outer,
            bracket_token: Default::default(),
            meta: Meta::List(MetaList {
                path: Path {
                    leading_colon: None,
                    segments: [PathSegment {
                        ident,
                        arguments: PathArguments::None,
                    }]
                    .into_iter()
                    .collect(),
                },
                delimiter: MacroDelimiter::Paren(Default::default()),
                tokens,
            }),
        };
        return result;
    }
}
