pub trait Executable {
    type Error;

    fn execute(&self) -> Result<(), Self::Error>;
}