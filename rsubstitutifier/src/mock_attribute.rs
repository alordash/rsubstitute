use proc_macro2::Span;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::*;

pub fn new(span: Span) -> Attribute {
    return wrap_in_cfg_attr(
        span,
        Meta::Path(Path {
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
    );
}

pub fn new_base(span: Span) -> Attribute {
    wrap_in_cfg_attr(
        span,
        Meta::List(MetaList {
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
    )
}

fn wrap_in_cfg_attr(span: Span, meta: Meta) -> Attribute {
    let tokens: Punctuated<Meta, Token![,]> = [
        Meta::Path(Path {
            leading_colon: None,
            segments: [PathSegment {
                ident: Ident::new("test", span),
                arguments: PathArguments::None,
            }]
            .into_iter()
            .collect(),
        }),
        meta,
    ]
    .into_iter()
    .collect();
    let result = Attribute {
        pound_token: Default::default(),
        style: AttrStyle::Outer,
        bracket_token: Default::default(),
        meta: Meta::List(MetaList {
            path: Path {
                leading_colon: None,
                segments: [PathSegment {
                    ident: Ident::new("cfg_attr", span),
                    arguments: PathArguments::None,
                }]
                .into_iter()
                .collect(),
            },
            delimiter: MacroDelimiter::Paren(token::Paren(span)),
            tokens: tokens.to_token_stream(),
        }),
    };
    return result;
}
