use rsubstitute::macros::mocked;

struct Foo {
    pub number: i32,
    pub string: String,
}

mocked! {
    struct Structd {
        pub value: i32
    }

    impl Structd {
        pub fn new(Foo {number, ..}: Foo) -> Self {
            Self { value: number }
        }
    }
}
fn ad() {
    let s = StructdMock::new(Foo {
        number: 32,
        string: "asd".to_owned(),
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn compile() {}
}
