use crate::mock_attribute;
use quote::ToTokens;
use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::*;

pub fn rewrite(file: &mut File, valid_structs: &mut HashSet<String>) {
    let mut rewriter = Rewriter { valid_structs };
    rewriter.visit_file_mut(file)
}

struct Rewriter<'a> {
    pub valid_structs: &'a mut HashSet<String>,
}

impl<'a> VisitMut for Rewriter<'a> {
    fn visit_item_fn_mut(&mut self, i: &mut ItemFn) {
        if i.attrs.iter().any(|x| match &x.meta {
            Meta::Path(p) if p.segments.len() == 1 => p.segments[0].ident == "test",
            _ => false,
        }) {
            return;
        }
        i.attrs.insert(0, mock_attribute::new_base(i.span()));
    }

    fn visit_item_impl_mut(&mut self, i: &mut ItemImpl) {
        if !is_ok_impl(i, &self.valid_structs) {
            return;
        }

        i.attrs.insert(0, mock_attribute::new_base(i.span()))
    }

    fn visit_item_struct_mut(&mut self, i: &mut ItemStruct) {
        if let Fields::Unnamed(_) = i.fields {
            return;
        }

        i.attrs.insert(0, mock_attribute::new(i.span()));
        let struct_name = i.ident.to_token_stream().to_token_stream().to_string();
        self.valid_structs.insert(struct_name);
    }

    fn visit_item_trait_mut(&mut self, i: &mut ItemTrait) {
        i.attrs.insert(0, mock_attribute::new_base(i.span()))
    }
}

fn is_ok_impl(item_impl: &ItemImpl, valid_structs: &HashSet<String>) -> bool {
    match item_impl.self_ty.as_ref() {
        Type::Path(p) => {
            let maybe_struct_name = p
                .path
                .segments
                .last()
                .map(|x| x.ident.to_token_stream().to_string());
            if !maybe_struct_name.is_some_and(|struct_name| valid_structs.contains(&struct_name)) {
                return false;
            }
        }
        _ => return false,
    }

    for item in item_impl.items.iter() {
        match item {
            ImplItem::Macro(_) | ImplItem::Verbatim(_) => return false,
            _ => {}
        }
    }

    return true;
}
