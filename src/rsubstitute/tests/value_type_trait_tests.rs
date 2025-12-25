#![allow(non_snake_case)]
use rsubstitute_proc_macro::mock;

#[mock]
trait Trait {
    fn accept_value(&self, v: i32);

    fn return_value(&self) -> i32;

    fn accept_value_return_value(&self, v: i32) -> f32;

    fn accept_two_values(&self, v1: i32, v2: f32);

    fn accept_two_values_return_value(&self, v1: i32, v2: f32) -> String;
}

use __rsubstitute_generated_Trait::*;
use rsubstitute_core::Times;
use rsubstitute_core::args_matching::Arg;

#[test]
fn accept_value_Ok() {
    // Arrange
    let mock = TraitMock::new();
    let value = 10;

    // Act
    Trait::accept_value(&mock, value);

    // Assert
    mock.accept_value_received(Arg::Any, Times::Once)
        .accept_value_received(Arg::Eq(value), Times::Once)
        .accept_value_received(Arg::is(|actual_value| actual_value == 10), Times::Once);
}
