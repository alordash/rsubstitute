use crate::constants;
use crate::syntax::*;
use proc_macro2::Ident;
use syn::*;

pub(crate) fn create(ident: Ident) -> Type {
    create_from_path(path::create(ident))
}

pub(crate) fn create_from_path(path: Path) -> Type {
    let result = Type::Path(TypePath { qself: None, path });
    return result;
}

pub(crate) fn create_with_generics_path(ident: Ident, generics: Generics) -> TypePath {
    let path = path::create_with_generics(ident, generics);
    let result = TypePath { qself: None, path };
    return result;
}

pub(crate) fn create_with_generics(ident: Ident, generics: Generics) -> Type {
    Type::Path(create_with_generics_path(ident, generics))
}

pub(crate) fn create_from_struct_path(item_struct: &ItemStruct) -> TypePath {
    create_with_generics_path(item_struct.ident.clone(), item_struct.generics.clone())
}

pub(crate) fn create_from_struct(item_struct: &ItemStruct) -> Type {
    Type::Path(create_from_struct_path(item_struct))
}

pub(crate) fn create_with_path_arguments(ident: Ident, path_arguments: PathArguments) -> Type {
    let path = path::create_with_path_arguments(ident, path_arguments);
    let result = TypePath { qself: None, path };
    return Type::Path(result);
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

pub(crate) fn wrap_in_phantom_data(ty: Type) -> Type {
    let result = wrap_in(ty, constants::PHANTOM_DATA_IDENT.clone());
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
