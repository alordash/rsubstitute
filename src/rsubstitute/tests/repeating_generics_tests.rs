use rsubstitute::prelude::*;
use std::marker::PhantomData;

trait Same<'a, T> {
    fn work(&self, t: &'a T);
}

trait Different<'a, T> {
    fn work(&self, t: &'a T);
}

mocked_base! {
    struct Struct<'a, T>(PhantomData<&'a T>);

    impl<'a, T> Struct<'a, T> {
        pub fn new() -> Self { Self(PhantomData) }
    }

    // TODO - write in doc that this is not supported, that generic params idents in impl blocks should be same as in item definition
    // impl<'b, S> Struct<'b, S> {
    //     pub fn new() -> Self {
    //         Self(PhantomData)
    //     }
    //
    //     pub fn work(&self, s: &'b S) {}
    // }

    impl<'a, T> Same<'a, T> for Struct<'a, T> {
        fn work(&self, t: &'a T) {
            unreachable!()
        }
    }

    // TODO - as_Different::<Td>
    impl<'a, 'd, T, Td> Different<'d, Td> for Struct<'a, T> {
        fn work(&self, t: &'d Td) {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn compile() {
        let mock = Struct::<i32, String>::new();
        
        Same::work(&mock, &3);
        
        mock.received.as_Same.work(&3, Times::Once);
    }
}
