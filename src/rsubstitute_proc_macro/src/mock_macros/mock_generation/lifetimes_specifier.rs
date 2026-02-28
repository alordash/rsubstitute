use crate::constants;
use crate::syntax::*;
use quote::ToTokens;
use std::sync::Arc;
use syn::punctuated::Punctuated;
use syn::*;

pub trait ILifetimesSpecifier {
    fn add_default_arg_lifetime(&self, item_trait: ItemTrait) -> ItemTrait;
}

pub(crate) struct LifetimesSpecifier {
    pub reference_type_crawler: Arc<dyn IReferenceTypeCrawler>,
}

impl ILifetimesSpecifier for LifetimesSpecifier {
    fn add_default_arg_lifetime(&self, mut item_trait: ItemTrait) -> ItemTrait {
        self.process_generics_lifetimes(&mut item_trait.generics);
        item_trait.generics.params.insert(
            0,
            GenericParam::Lifetime(LifetimeParam {
                attrs: Vec::new(),
                lifetime: constants::DEFAULT_ARG_FIELD_LIFETIME.clone(),
                colon_token: None,
                bounds: Punctuated::new(),
            }),
        );
        for mut trait_item in item_trait.items.iter_mut() {
            self.process_trait_item(&mut trait_item);
        }
        return item_trait;
    }
}

impl LifetimesSpecifier {
    fn process_generics_lifetimes(&self, generics: &mut Generics) {
        for lifetime_param in generics.lifetimes_mut() {
            if lifetime_param.colon_token.is_none() {
                lifetime_param.colon_token = Some(Default::default());
            }
            lifetime_param
                .bounds
                .insert(0, constants::DEFAULT_ARG_FIELD_LIFETIME.clone());
        }
    }

    fn process_trait_item(&self, trait_item: &mut TraitItem) {
        match trait_item {
            TraitItem::Const(_) => todo!("SUPPORT"),
            TraitItem::Fn(trait_item_fn) => {
                self.process_fn_signature(&mut trait_item_fn.sig);
            }
            TraitItem::Type(_) => todo!("SUPPORT"),
            TraitItem::Macro(_) => todo!("WTF"),
            _ => panic!(
                "Trait item is not supported: {}",
                trait_item.to_token_stream().to_string()
            ),
        }
    }

    fn process_fn_signature(&self, sig: &mut Signature) {
        self.process_generics_lifetimes(&mut sig.generics);
        let typed_fn_args = sig.inputs.iter_mut().filter_map(|fn_arg| match fn_arg {
            FnArg::Typed(typed_fn_arg) => Some(typed_fn_arg),
            _ => None,
        });
        for typed_fn_arg in typed_fn_args {
            let optional_lifetimes = self
                .reference_type_crawler
                .get_all_optional_lifetimes(&mut typed_fn_arg.ty);
            let empty_lifetimes = optional_lifetimes
                .into_iter()
                .filter(|optional_lifetime| optional_lifetime.is_none());
            for empty_lifetime in empty_lifetimes {
                *empty_lifetime = Some(constants::DEFAULT_ARG_FIELD_LIFETIME.clone());
            }
        }
    }
}
