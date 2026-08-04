use proc_macro2::Span;
use quote::ToTokens;
use syn::*;

pub fn new(span: Span) -> Attribute {
    Attribute {
        pound_token: Default::default(),
        style: AttrStyle::Outer,
        bracket_token: Default::default(),
        meta: Meta::Path(Path {
            leading_colon: None,
            segments: [
                PathSegment {
                    ident: Ident::new("rsubstitute", span),
                    arguments: PathArguments::None,
                },
                PathSegment {
                    ident: Ident::new("mock", span),
                    arguments: PathArguments::None,
                },
            ]
            .into_iter()
            .collect(),
        }),
    }
}

pub fn new_base(span: Span) -> Attribute {
    Attribute {
        pound_token: Default::default(),
        style: AttrStyle::Outer,
        bracket_token: Default::default(),
        meta: Meta::List(MetaList {
            path: Path {
                leading_colon: None,
                segments: [
                    PathSegment {
                        ident: Ident::new("rsubstitute", span),
                        arguments: PathArguments::None,
                    },
                    PathSegment {
                        ident: Ident::new("mock", span),
                        arguments: PathArguments::None,
                    },
                ]
                .into_iter()
                .collect(),
            },
            delimiter: MacroDelimiter::Paren(Default::default()),
            tokens: Ident::new("base", span).to_token_stream(),
        }),
    }
}
