struct Dependency;

mod inner {
    use rsubstitute_proc_macro::mock;

    #[mock(base)]
    #[allow(unused)]
    fn work() {
        use super::Dependency;
        let _ = Dependency;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn compile() {}
}
