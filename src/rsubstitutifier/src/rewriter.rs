use crate::mock_attribute;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::*;

pub fn rewrite(file: &mut File) {
    Rewriter.visit_file_mut(file)
}

struct Rewriter;

impl VisitMut for Rewriter {
    fn visit_item_fn_mut(&mut self, i: &mut ItemFn) {
        i.attrs.insert(0, mock_attribute::new_base(i.span()));
    }

    fn visit_item_impl_mut(&mut self, i: &mut ItemImpl) {
        if !is_ok_impl(i) {
            return;
        }

        i.attrs.insert(0, mock_attribute::new_base(i.span()))
    }

    fn visit_item_struct_mut(&mut self, i: &mut ItemStruct) {
        if let Fields::Unnamed(_) = i.fields {
            return;
        }

        i.attrs.insert(0, mock_attribute::new(i.span()));
    }

    fn visit_item_trait_mut(&mut self, i: &mut ItemTrait) {
        i.attrs.insert(0, mock_attribute::new_base(i.span()))
    }
}

fn is_ok_impl(item_impl: &ItemImpl) -> bool {
    match item_impl.self_ty.as_ref() {
        Type::Path(_) => {}
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
