use syn::*;

pub struct MockType {
    ty: Type,
}

impl MockType {
    pub fn new(ty: Type) -> Self {
        Self {ty}
    }
    
    pub fn get_ident(&self) -> &Ident {
        let Type::Path( type_path) = self.ty else { panic!("MockType should be Type::Path.")};
        assert_eq!(type_path.path.segments.len(), 1, "MockType should consist only of one path segmend.");
        let segment = type_path.path.segments[0];
        let qq = type_path.path.segments.
        match self.ty {
            Type::Path(type_path) => {type_path.path.segments[0].arguments.},
            _ => panic!(),
        }
    }
}