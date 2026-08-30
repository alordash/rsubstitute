use rsubstitute::*;

#[mock]
trait Trait<T, const N: usize> {
    const M: usize = N;
    const CHAR: char = 'm';
}

mod tests {
    #![allow(non_snake_case)]
    use super::*;
    
    #[test]
    fn MockCreation_RequiresOnlyFirstTwoGenericArgument() {
        // Arrange
        // Act
        // Assert
        let mock = TraitMock::<i32, 3>::new();
    }
}