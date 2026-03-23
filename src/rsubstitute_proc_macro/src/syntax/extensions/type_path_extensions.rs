use syn::*;

pub(crate) trait ITypePathExtensions {
    fn set_first_generic_lifetime_argument(self, lifetime: Lifetime) -> Self;
}

impl ITypePathExtensions for TypePath {
    fn set_first_generic_lifetime_argument(mut self, lifetime: Lifetime) -> Self {
        let PathArguments::AngleBracketed(ref mut type_path_arguments) =
            self.path.segments[0].arguments
        else {
            panic!("TypePath arguments must be AngleBracketed.")
        };
        type_path_arguments.args[0] = GenericArgument::Lifetime(lifetime);
        return self;
    }
}
