use rsubstitute_proc_macro::mock;

#[mock]
trait Trait {
    fn work(&self, r: &i32);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsubstitute::*;

    #[test]
    fn work_ReferenceDies_ReceviesRewroteReference() {
        // Arrange
        let mock = TraitMock::new();

        // Act
        {
            let v = 10;
            let r = &v;
            mock.work(r);
        }
        let stack = [32, 4, 51];
        println!("{stack:?}");

        // Assert
        mock.received
            .work(
                Arg::is(|actual_r| {
                    println!("actual_r = {actual_r}");
                    return true;
                }),
                Times::Once,
            )
            .no_other_calls();
    }
}
