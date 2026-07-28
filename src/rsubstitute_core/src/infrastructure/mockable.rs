pub trait Mockable {
    type Mock;
    fn mock(self) -> Self::Mock;
    type StaticSetup;
    fn static_setup() -> Self::StaticSetup;
    type StaticReceived;
    fn static_received() -> Self::StaticReceived;
}
