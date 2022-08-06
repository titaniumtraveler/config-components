pub trait Component {
    type Error;
    fn install(&mut self) -> Result<(), Self::Error>;
    fn remove(&mut self) -> Result<(), Self::Error>;
}
