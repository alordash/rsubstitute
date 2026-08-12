use rsubstitute_proc_macro::mock;

#[mock(base)]
trait Trait<TA, const TB: usize, TC, const TD: usize = 3> {}

#[mock]
struct S<SA, const SB: usize, SC, const SD: usize = 2> {
    first: [SA; SB],
    second: [SC; SD],
}

#[mock(base)]
impl<TA, const TB: usize, TC, SA, const SB: usize, SC> Trait<TA, TB, TC> for S<SA, SB, SC> {}

#[cfg(test)]
mod tests {
    #[test]
    fn compile() {}
}
