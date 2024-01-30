pub trait Event: Clone + Send + Sync + 'static {
    fn name(&self) -> String;
    fn payload(&self) -> Vec<u8>;
}