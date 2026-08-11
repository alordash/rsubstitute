use rsubstitute::*;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct Foo {
    pub number: Vec<i32>,
}

#[mock]
struct Struct;

#[mock(base)]
impl Struct {
    fn new() -> Self {
        Self
    }

    fn fooo(&mut self, Foo { mut number }: Foo, mut qq: &mut &mut &&&&mut i32) {
        println!("number: {number:?}")
    }
}

#[mock(base)]
impl From<i32> for Struct {
    fn from(_: i32) -> Self {
        Self::new()
    }
}

impl From<f32> for Struct {
    fn from(_: f32) -> Self {
        Self::new()
    }
}

#[mock]
struct Selfish;

#[mock(base)]
impl Selfish {
    fn work(&self, _: &Self) {}
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    #[test]
    fn flex() {}

    #[test]
    fn compile() {}
}
