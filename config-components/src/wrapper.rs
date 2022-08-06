pub use self::{config::Config, wrapper::Wrapper};

use crate::Component;

mod config;
#[allow(clippy::module_inception)]
mod wrapper;

pub trait ConfigWrapper {
    type Error;
    fn install(&mut self) -> Result<(), Self::Error>;
    fn remove(&mut self) -> Result<(), Self::Error>;

    fn component<T>(self, new: impl Into<T>) -> Wrapper<T, Self, Self::Error>
    where
        Self: Sized,
        T: Component,
        Self::Error: From<T::Error>,
    {
        Wrapper::new(self, new.into())
    }

}
