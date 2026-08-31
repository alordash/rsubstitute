# WIP

# rsubstitute

Library for mocking functions, traits and structures in Rust.

## Overview

This library exposes `mock` attribute for making code mockable and an API for mocks configuration. No changes for source
code are needed (apart from adding `#[cfg_attr(test, mock)]` attribute).

## Usage

Import `rsubstitute::*` and apply `mock` attribute on your function, trait, struct, or `impl` block.

Here's an example of how to mock regular function:

```rust
use rsubstitute::*;

#[mock]
fn work(v: i32) -> i32 {
    1
}

# fn main() {
    // Arrange
    work::setup(10).returns(20);
    
    // Act
    let result = work(10);
    
    // Assert
    assert_eq!(result, 20);
    work::received(10, 1.time());
# }
```

Trait:

```rust
use rsubstitute::*;

#[mock]
trait Trait {
    fn work(&self, v: i32) -> i32;
}

fn use_trait(t: &dyn Trait, v: i32) -> i32 {
    t.work(v)
}

# fn main() {
    // Arrange
    let mut mock = TraitMock::new();
    mock.setup().work(10).returns(20);

    // Act
    let result = use_trait(&mock, 10);

    // Assert
    assert_eq!(result, 20);
    mock.received().work(10, 1.time());
# }
```