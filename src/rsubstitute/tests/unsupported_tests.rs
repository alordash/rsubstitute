#[allow(unused)]
use rsubstitute::macros::mock;

#[mock]
trait Trait<'a> {
    fn accept_ref(&self, r: &'a&i32);
    
    // TODO - make test. Can not return anonymous reference behind typed reference
    fn return_ref(&self) -> &'a &i32;
    
    // TODO - make test. Can not use Trait's generic lifetimes for &self reference.
    fn self_ref(&'a self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // Arrange
        let mock = TraitMock::new();
        let v = 3;
        let r = &&43;
        let result = &&12;

        // mock.setup.work().returns(result);

        // let actual_result = mock.work();
        
        let _ = mock.accept_ref(r);
    }
}
