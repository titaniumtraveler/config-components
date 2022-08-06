use std::marker::PhantomData;

use crate::wrapper::ConfigWrapper;

// Replace `Error` with `!` as soon as it's stabilized.
pub struct Config<Error>(PhantomData<Error>);

impl<Error> Config<Error> {
    pub fn builder() -> Self {
        Self(PhantomData)
    }
}

impl<Error> ConfigWrapper for Config<Error> {
    type Error = Error;

    fn install(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn remove(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
