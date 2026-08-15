use crate::constants;
use crate::syntax::*;
use syn::*;

pub(crate) fn normalize_lifetimes_in_generic_arguments(mut path: Path) -> Path {
    if let Some(last_segment) = path.segments.last_mut()
        && let PathArguments::AngleBracketed(angle_bracketed_path_arguments) =
            &mut last_segment.arguments
    {
        angle_bracketed_path_arguments.args =
            core::mem::take(&mut angle_bracketed_path_arguments.args)
                .into_iter()
                .filter_map(|x| match x {
                    GenericArgument::Lifetime(l) => {
                        if l.ident == constants::DEFAULT_ARG_LIFETIME {
                            Some(anonymous_lifetime_generic_argument(l.span()))
                        } else {
                            None
                        }
                    }
                    arg => Some(arg),
                })
                .collect();
    }
    return path;
}
