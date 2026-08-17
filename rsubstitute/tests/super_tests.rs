#[derive(Clone)]
struct Dependency;

mod inner {
    use rsubstitute_proc_macro::mock;

    #[mock(base)]
    #[allow(unused)]
    fn work(_: super::Dependency) -> super::Dependency {
        use super::Dependency;
        let result = Dependency;
        return result;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn compile() {}
}
