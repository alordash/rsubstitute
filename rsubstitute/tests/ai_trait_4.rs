use rsubstitute::*;

//
// ============================================================================
// Payload
// ============================================================================
//

#[mock]
trait Monster<'a, T, const N: usize>
where
    T: Clone + Send + Sync + 'a,
{
    // ------------------------------------------------------------------------
    // Associated items
    // ------------------------------------------------------------------------

    type Output: Clone;

    type Iterator: Iterator<Item = T>;

    const SIZE: usize = N;

    const MAGIC: char = 'm';

    // ------------------------------------------------------------------------
    // Ordinary methods
    // ------------------------------------------------------------------------

    fn no_args(&self);

    fn one_arg(&self, value: i32);

    fn many_args(&self, a: i32, b: String, c: bool, d: Option<Vec<u8>>) -> i32;

    // ------------------------------------------------------------------------
    // Every receiver form
    // ------------------------------------------------------------------------

    fn by_value(self);

    fn by_ref(&self);

    fn by_mut_ref(&mut self);

    fn explicit_ref(self: &Self);

    fn explicit_mut_ref(self: &mut Self);

    fn boxed(self: Box<Self>);

    // ------------------------------------------------------------------------
    // Generic methods
    // ------------------------------------------------------------------------

    fn generic<T2>(&self, value: T2) -> T2;

    fn generic_two<T2, U>(&self, a: T2, b: U);

    fn generic_bounded<T2>(&self, value: T2)
    where
        T2: Clone + Send + Sync;

    // ------------------------------------------------------------------------
    // Generic method with lifetime
    // ------------------------------------------------------------------------

    fn generic_lifetime<'b, T2>(&'b self, value: &'b T2) -> &'b T2;

    // ------------------------------------------------------------------------
    // Const generic methods
    // ------------------------------------------------------------------------

    fn const_generic<const M: usize>(&self, value: [u8; M]);

    // ------------------------------------------------------------------------
    // Lifetimes
    // ------------------------------------------------------------------------

    fn lifetime(&'a self, value: &'a T);

    fn multiple_lifetimes<'b, 'c>(&'b self, a: &'b str, b: &'c str);

    // ------------------------------------------------------------------------
    // Where clauses
    // ------------------------------------------------------------------------

    fn where_clause<T2>(&self, value: T2)
    where
        T2: Clone + Send + Sync;

    fn where_projection<T2>(&self, value: T2)
    where
        T2: Iterator,
        T2::Item: Clone;

    // ------------------------------------------------------------------------
    // Associated type projections
    // ------------------------------------------------------------------------

    fn associated_type(&self, value: Self::Output) -> Self::Output;

    fn associated_type_nested(&self, value: Option<Vec<Self::Output>>)
    -> Option<Vec<Self::Output>>;

    // ------------------------------------------------------------------------
    // Deeply nested types
    // ------------------------------------------------------------------------

    fn type_monster(
        &self,
        value: Option<
            Result<
                Vec<
                    Box<
                        std::sync::Arc<
                            std::cell::RefCell<
                                std::collections::HashMap<
                                    String,
                                    Vec<Option<Result<u64, Box<dyn std::error::Error>>>>,
                                >,
                            >,
                        >,
                    >,
                >,
                String,
            >,
        >,
    );

    // ------------------------------------------------------------------------
    // References
    // ------------------------------------------------------------------------

    fn references(&self, a: &i32, b: &mut i32, c: &&i32, d: &mut &i32);

    // ------------------------------------------------------------------------
    // Raw pointers
    // ------------------------------------------------------------------------

    fn raw_pointers(&self, a: *const i32, b: *mut i32, c: *const *mut i32);

    // ------------------------------------------------------------------------
    // Function pointers
    // ------------------------------------------------------------------------

    fn function_pointer(&self, f: fn(i32) -> i32) -> i32;

    unsafe fn unsafe_function_pointer(&self, f: unsafe fn(*const u8) -> usize) -> usize;

    extern "C" fn c_function_pointer(&self, f: extern "C" fn(i32) -> i32) -> i32;

    // ------------------------------------------------------------------------
    // Closures / impl Trait
    // ------------------------------------------------------------------------

    fn closure(&self, f: impl Fn(i32) -> i32) -> i32;

    fn closure_mut(&self, f: impl FnMut(i32) -> i32) -> i32;

    fn closure_once(&self, f: impl FnOnce(i32) -> i32) -> i32;

    fn impl_iterator(&self, value: impl Iterator<Item = i32>);

    // ------------------------------------------------------------------------
    // dyn Trait
    // ------------------------------------------------------------------------

    fn dyn_display(&self, value: Box<dyn std::fmt::Display>);

    fn dyn_debug(&self, value: &dyn std::fmt::Debug);

    // ------------------------------------------------------------------------
    // Self in types
    // ------------------------------------------------------------------------

    fn self_argument(&self, value: Option<Box<Self>>);

    // ------------------------------------------------------------------------
    // Return types
    // ------------------------------------------------------------------------

    fn return_self(&self) -> Self;

    fn return_option_self(&self) -> Option<Self>
    where
        Self: Sized;

    // ------------------------------------------------------------------------
    // Unsafe
    // ------------------------------------------------------------------------

    unsafe fn unsafe_method(&self, value: *mut T);

    // ------------------------------------------------------------------------
    // ABI
    // ------------------------------------------------------------------------

    extern "C" fn extern_c(&self, value: i32) -> i32;

    unsafe extern "C" fn unsafe_extern_c(&self, value: *mut i32);

    // ------------------------------------------------------------------------
    // Async
    // ------------------------------------------------------------------------

    async fn async_no_args(&self);

    async fn async_method(&self, value: i32) -> i32;

    async fn async_generic<T2>(&self, value: T2) -> T2
    where
        T2: Clone;

    // ------------------------------------------------------------------------
    // Async + unsafe
    // ------------------------------------------------------------------------

    async unsafe fn async_unsafe(&self, value: *mut i32);

    // ------------------------------------------------------------------------
    // Associated functions / static methods
    // ------------------------------------------------------------------------

    fn static_no_args();

    fn static_with_args(value: i32, text: String) -> i32;

    fn static_generic<T2>(value: T2) -> T2;

    fn static_const<const M: usize>(value: [u8; M]);

    fn static_where<T2>(value: T2)
    where
        T2: Clone + Send;

    // ------------------------------------------------------------------------
    // Default implementations
    // ------------------------------------------------------------------------

    fn default_method(&self, value: i32) -> i32 {
        value + 1
    }

    fn default_generic<T2>(&self, value: T2) -> T2 {
        value
    }
}

//
// ============================================================================
// Helper functions
// ============================================================================
//

fn increment(value: i32) -> i32 {
    value + 1
}

unsafe fn pointer_length(value: *const u8) -> usize {
    if value.is_null() { 0 } else { 1 }
}

extern "C" fn c_increment(value: i32) -> i32 {
    value + 1
}

//
// ============================================================================
// Tests
// ============================================================================
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinary_methods() {
        // Arrange
        let mut mock = MonsterMock::<i32, 4>::new();

        mock.setup().no_args().returns(());

        mock.setup().one_arg(42).returns(());

        mock.setup()
            .many_args(42, "hello".to_owned(), true, Some(vec![1, 2, 3]))
            .returns(123);

        // Act
        mock.no_args();
        mock.one_arg(42);

        let result = mock.many_args(42, "hello".to_owned(), true, Some(vec![1, 2, 3]));

        // Assert
        assert_eq!(result, 123);

        mock.received().no_args(Times::Once);

        mock.received().one_arg(42, Times::Once);

        mock.received().many_args(
            42,
            "hello".to_owned(),
            true,
            Some(vec![1, 2, 3]),
            Times::Once,
        );
    }

    #[test]
    fn generic_methods() {
        // Arrange
        let mut mock = MonsterMock::<i32, 4>::new();

        mock.setup().generic::<i32>(42).returns(123);

        mock.setup()
            .generic_two::<i32, String>(42, "hello".to_owned())
            .returns(());

        mock.setup().generic_bounded::<i32>(42).returns(());

        // Act
        let result = mock.generic::<i32>(42);

        mock.generic_two::<i32, String>(42, "hello".to_owned());

        mock.generic_bounded::<i32>(42);

        // Assert
        assert_eq!(result, 123);

        mock.received().generic::<i32>(42, Times::Once);

        mock.received()
            .generic_two::<i32, String>(42, "hello".to_owned(), Times::Once);

        mock.received().generic_bounded::<i32>(42, Times::Once);
    }

    #[test]
    fn generic_lifetime() {
        // Arrange
        let mut mock = MonsterMock::<i32, 4>::new();

        let value = 42;

        mock.setup().generic_lifetime(&value).returns(&value);

        // Act
        let result = mock.generic_lifetime(&value);

        // Assert
        assert_eq!(result, &42);

        mock.received().generic_lifetime(&value, Times::Once);
    }

    #[test]
    fn const_generic_method() {
        // Arrange
        let mut mock = MonsterMock::<i32, 4>::new();

        mock.setup().const_generic::<4>([1, 2, 3, 4]).returns(());

        // Act
        mock.const_generic::<4>([1, 2, 3, 4]);

        // Assert
        mock.received()
            .const_generic::<4>([1, 2, 3, 4], Times::Once);
    }

    #[test]
    fn lifetimes() {
        // Arrange
        let mut mock = MonsterMock::<i32, 4>::new();

        let value = 42;

        mock.setup().lifetime(&value).returns(());

        mock.setup()
            .multiple_lifetimes("hello", "world")
            .returns(());

        // Act
        mock.lifetime(&value);

        mock.multiple_lifetimes("hello", "world");

        // Assert
        mock.received().lifetime(&value, Times::Once);

        mock.received()
            .multiple_lifetimes("hello", "world", Times::Once);
    }

    #[test]
    fn where_clauses() {
        // Arrange
        let mut mock = MonsterMock::<i32, 4>::new();

        mock.setup()
            .where_clause::<String>("hello".to_owned())
            .returns(());

        mock.setup()
            .where_projection::<std::vec::IntoIter<i32>>(vec![1, 2, 3].into_iter())
            .returns(());

        // Act
        mock.where_clause::<String>("hello".to_owned());

        mock.where_projection::<std::vec::IntoIter<i32>>(vec![1, 2, 3].into_iter());

        // Assert
        mock.received()
            .where_clause::<String>("hello".to_owned(), Times::Once);

        mock.received()
            .where_projection::<std::vec::IntoIter<i32>>(Arg::Any, Times::Once);
    }

    #[test]
    fn associated_types() {
        // Arrange
        let mut mock = MonsterMock::<i32, 4>::new();

        mock.setup().associated_type(42).returns(123);

        mock.setup()
            .associated_type_nested(Some(vec![42]))
            .returns(Some(vec![123]));

        // Act
        let result = mock.associated_type(42);

        let nested = mock.associated_type_nested(Some(vec![42]));

        // Assert
        assert_eq!(result, 123);
        assert_eq!(nested, Some(vec![123]));

        mock.received().associated_type(42, Times::Once);

        mock.received()
            .associated_type_nested(Some(vec![42]), Times::Once);
    }

    #[test]
    fn deeply_nested_types() {
        // Arrange
        let mut mock = MonsterMock::<i32, 4>::new();

        mock.setup().type_monster(Arg::Any).returns(());

        // Act
        mock.type_monster(None);

        // Assert
        mock.received().type_monster(Arg::Any, Times::Once);
    }

    #[test]
    fn references_and_raw_pointers() {
        // Arrange
        let mut mock = MonsterMock::<i32, 4>::new();

        mock.setup()
            .references(Arg::Any, Arg::Any, Arg::Any, Arg::Any)
            .returns(());

        mock.setup()
            .raw_pointers(Arg::Any, Arg::Any, Arg::Any)
            .returns(());

        let value = 42;
        let mut mutable = 123;
        let reference = &value;

        // Act
        mock.references(&value, &mut mutable, &reference, &mut &value);

        mock.raw_pointers(&value, &mut mutable, &mut mutable);

        // Assert
        mock.received()
            .references(Arg::Any, Arg::Any, Arg::Any, Arg::Any, Times::Once);

        mock.received()
            .raw_pointers(Arg::Any, Arg::Any, Arg::Any, Times::Once);
    }

    #[test]
    fn function_pointers() {
        // Arrange
        let mut mock = MonsterMock::<i32, 4>::new();

        mock.setup().function_pointer(increment).returns(123);

        mock.setup()
            .unsafe_function_pointer(pointer_length)
            .returns(456);

        mock.setup().c_function_pointer(c_increment).returns(789);

        // Act
        let normal = mock.function_pointer(increment);

        let unsafe_result = unsafe { mock.unsafe_function_pointer(pointer_length) };

        let c_result = mock.c_function_pointer(c_increment);

        // Assert
        assert_eq!(normal, 123);
        assert_eq!(unsafe_result, 456);
        assert_eq!(c_result, 789);

        mock.received().function_pointer(increment, Times::Once);

        mock.received()
            .unsafe_function_pointer(pointer_length, Times::Once);

        mock.received().c_function_pointer(c_increment, Times::Once);
    }

    #[test]
    fn closures() {
        // Arrange
        let mut mock = MonsterMock::<i32, 4>::new();

        mock.setup().closure(Arg::Any).returns(10);

        mock.setup().closure_mut(Arg::Any).returns(20);

        mock.setup().closure_once(Arg::Any).returns(30);

        // Act
        let result1 = mock.closure(|x| x + 1);

        let result2 = mock.closure_mut(|x| x + 2);

        let result3 = mock.closure_once(|x| x + 3);

        // Assert
        assert_eq!(result1, 10);
        assert_eq!(result2, 20);
        assert_eq!(result3, 30);

        mock.received().closure(Arg::Any, Times::Once);

        mock.received().closure_mut(Arg::Any, Times::Once);

        mock.received().closure_once(Arg::Any, Times::Once);
    }

    #[test]
    fn dyn_arguments() {
        // Arrange
        let mut mock = MonsterMock::<i32, 4>::new();

        mock.setup().dyn_display(Arg::Any).returns(());

        mock.setup().dyn_debug(Arg::Any).returns(());

        let display: Box<dyn std::fmt::Display> = Box::new(42);

        let debug: &dyn std::fmt::Debug = &"hello";

        // Act
        mock.dyn_display(display);
        mock.dyn_debug(debug);

        // Assert
        mock.received().dyn_display(Arg::Any, Times::Once);

        mock.received().dyn_debug(Arg::Any, Times::Once);
    }

    #[test]
    fn self_arguments() {
        // Arrange
        let mut mock = MonsterMock::<i32, 4>::new();

        mock.setup().self_argument(Arg::Any).returns(());

        // Act
        mock.self_argument(None);

        // Assert
        mock.received().self_argument(Arg::Any, Times::Once);
    }

    #[test]
    fn return_self() {
        // Arrange
        let mut mock = MonsterMock::<i32, 4>::new();

        mock.setup()
            .return_self()
            .returns(MonsterMock::<i32, 4>::new());

        mock.setup()
            .return_option_self()
            .returns(Some(MonsterMock::<i32, 4>::new()));

        // Act
        let result = mock.return_self();

        let option = mock.return_option_self();

        // Assert
        let _ = result;
        assert!(option.is_some());

        mock.received().return_self(Times::Once);

        mock.received().return_option_self(Times::Once);
    }

    #[test]
    fn unsafe_method() {
        // Arrange
        let mut mock = MonsterMock::<i32, 4>::new();

        mock.setup().unsafe_method(Arg::Any).returns(());

        let mut value = 42;

        // Act
        unsafe {
            mock.unsafe_method(&mut value);
        }

        // Assert
        mock.received().unsafe_method(Arg::Any, Times::Once);
    }

    #[test]
    fn extern_c() {
        // Arrange
        let mut mock = MonsterMock::<i32, 4>::new();

        mock.setup().extern_c(42).returns(123);

        mock.setup().unsafe_extern_c(Arg::Any).returns(());

        let mut value = 42;

        // Act
        let result = mock.extern_c(42);

        unsafe {
            mock.unsafe_extern_c(&mut value);
        }

        // Assert
        assert_eq!(result, 123);

        mock.received().extern_c(42, Times::Once);

        mock.received().unsafe_extern_c(Arg::Any, Times::Once);
    }

    #[tokio::test]
    async fn async_methods() {
        // Arrange
        let mut mock = MonsterMock::<i32, 4>::new();

        mock.setup().async_no_args().returns(());

        mock.setup().async_method(42).returns(123);

        mock.setup()
            .async_generic::<String>("hello".to_owned())
            .returns("world".to_owned());

        // Act
        mock.async_no_args().await;

        let result = mock.async_method(42).await;

        let generic = mock.async_generic::<String>("hello".to_owned()).await;

        // Assert
        assert_eq!(result, 123);
        assert_eq!(generic, "world");

        mock.received().async_no_args(Times::Once);

        mock.received().async_method(42, Times::Once);

        mock.received()
            .async_generic::<String>("hello".to_owned(), Times::Once);
    }

    #[tokio::test]
    async fn async_unsafe_method() {
        // Arrange
        let mut mock = MonsterMock::<i32, 4>::new();

        mock.setup().async_unsafe(Arg::Any).returns(());

        let mut value = 42;

        // Act
        unsafe {
            mock.async_unsafe(&mut value).await;
        }

        // Assert
        mock.received().async_unsafe(Arg::Any, Times::Once);
    }

    #[test]
    fn static_functions() {
        // Arrange
        let mut mock = MonsterMock::<i32, 4>::new();

        mock.setup().static_no_args().returns(());

        mock.setup()
            .static_with_args(42, "hello".to_owned())
            .returns(123);

        mock.setup().static_generic::<i32>(42).returns(456);

        mock.setup().static_const::<4>([1, 2, 3, 4]).returns(());

        mock.setup().static_where::<i32>(42).returns(());

        // Act
        MonsterMock::<i32, 4>::static_no_args();

        let result = MonsterMock::<i32, 4>::static_with_args(42, "hello".to_owned());

        let generic = MonsterMock::<i32, 4>::static_generic::<i32>(42);

        MonsterMock::<i32, 4>::static_const::<4>([1, 2, 3, 4]);

        MonsterMock::<i32, 4>::static_where::<i32>(42);

        // Assert
        assert_eq!(result, 123);
        assert_eq!(generic, 456);

        MonsterMock::<i32, 4>::static_received().static_no_args(Times::Once);

        MonsterMock::<i32, 4>::static_received().static_with_args(
            42,
            "hello".to_owned(),
            Times::Once,
        );

        MonsterMock::<i32, 4>::static_received().static_generic::<i32>(42, Times::Once);

        MonsterMock::<i32, 4>::static_received().static_const::<4>([1, 2, 3, 4], Times::Once);

        MonsterMock::<i32, 4>::static_received().static_where::<i32>(42, Times::Once);
    }

    #[test]
    fn default_methods() {
        // Arrange
        let mut mock = MonsterMock::<i32, 4>::new();

        mock.setup().default_method(42).returns(123);

        mock.setup()
            .default_generic::<String>("hello".to_owned())
            .returns("world".to_owned());

        // Act
        let result = mock.default_method(42);

        let generic = mock.default_generic::<String>("hello".to_owned());

        // Assert
        assert_eq!(result, 123);
        assert_eq!(generic, "world");

        mock.received().default_method(42, Times::Once);

        mock.received()
            .default_generic::<String>("hello".to_owned(), Times::Once);
    }

    #[test]
    fn every_receiver_except_consuming_one() {
        // Arrange
        let mut mock = MonsterMock::<i32, 4>::new();

        mock.setup().by_ref().returns(());

        mock.setup().by_mut_ref().returns(());

        mock.setup().explicit_ref().returns(());

        mock.setup().explicit_mut_ref().returns(());

        // Act
        mock.by_ref();
        mock.by_mut_ref();
        mock.explicit_ref();
        mock.explicit_mut_ref();

        // Assert
        mock.received().by_ref(Times::Once);

        mock.received().by_mut_ref(Times::Once);

        mock.received().explicit_ref(Times::Once);

        mock.received().explicit_mut_ref(Times::Once);
    }

    #[test]
    fn boxed_receiver() {
        // Arrange
        let mut mock = MonsterMock::<i32, 4>::new();

        mock.setup().boxed().returns(());

        // Act
        Box::new(mock).boxed();

        // Assert
        // The boxed mock is consumed by the Act.
    }

    #[test]
    fn const_and_associated_items_exist() {
        // Arrange
        let _mock = MonsterMock::<i32, 4>::new();

        // Act
        let size = <MonsterMock<i32, 4> as Monster<'static, i32, 4>>::SIZE;

        // Assert
        assert_eq!(size, 4);
    }
}

