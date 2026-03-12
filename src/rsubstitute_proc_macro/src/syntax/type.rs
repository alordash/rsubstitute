use crate::constants;
use crate::syntax::*;
use proc_macro2::Ident;
use syn::*;

pub(crate) fn create(ident: Ident) -> Type {
    let path = path::create(ident);
    let result = Type::Path(TypePath { qself: None, path });
    return result;
}

pub(crate) fn create_with_generics(ident: Ident, generics: Generics) -> Type {
    let path = path::create_with_generics(ident, generics);
    let result = Type::Path(TypePath { qself: None, path });
    return result;
}

pub(crate) fn create_from_struct(item_struct: &ItemStruct) -> Type {
    let result = create_with_generics(item_struct.ident.clone(), item_struct.generics.clone());
    return result;
}

pub(crate) fn wrap_in(ty: Type, wrapper: Ident) -> Type {
    let result = Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: [PathSegment {
                ident: wrapper,
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Default::default(),
                    args: [GenericArgument::Type(ty)].into_iter().collect(),
                    gt_token: Default::default(),
                }),
            }]
            .into_iter()
            .collect(),
        },
    });
    return result;
}

pub(crate) fn wrap_in_arc(ty: Type) -> Type {
    let result = wrap_in(ty, constants::ARC_IDENT.clone());
    return result;
}

pub(crate) fn reference(ty: Type, lifetime: Option<Lifetime>) -> Type {
    let result = Type::Reference(TypeReference {
        and_token: Default::default(),
        lifetime,
        mutability: None,
        elem: Box::new(ty.clone()),
    });
    return result;
}

pub(crate) fn mut_reference(ty: Type, lifetime: Option<Lifetime>) -> Type {
    let result = Type::Reference(TypeReference {
        and_token: Default::default(),
        lifetime,
        mutability: Some(Default::default()),
        elem: Box::new(ty.clone()),
    });
    return result;
}

pub(crate) fn phantom_data_lifetime(lifetime: Lifetime) -> Type {
    let result = Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: [PathSegment {
                ident: constants::PHANTOM_DATA_IDENT.clone(),
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Default::default(),
                    args: [GenericArgument::Type(reference(
                        constants::VOID_TYPE.clone(),
                        Some(lifetime),
                    ))]
                    .into_iter()
                    .collect(),
                    gt_token: Default::default(),
                }),
            }]
            .into_iter()
            .collect(),
        },
    });
    return result;
}

pub(crate) fn phantom_data(ty_ident: Ident) -> Type {
    let result = Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: [PathSegment {
                ident: constants::PHANTOM_DATA_IDENT.clone(),
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Default::default(),
                    args: [GenericArgument::Type(create(ty_ident))]
                        .into_iter()
                        .collect(),
                    gt_token: Default::default(),
                }),
            }]
            .into_iter()
            .collect(),
        },
    });
    return result;
}
