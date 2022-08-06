use std::{error, fmt::Display};

use crate::Component;

pub struct Installed<'a>(&'a mut bool);

#[derive(Debug)]
pub enum Error {
    NotInstalled,
    AlreadyInstalled,
}

impl Component for Installed<'_> {
    type Error = Error;

    fn install(&mut self) -> Result<(), Self::Error> {
        if !*self.0 {
            *self.0 = true;
            Ok(())
        } else {
            Err(Error::AlreadyInstalled)
        }
    }

    fn remove(&mut self) -> Result<(), Self::Error> {
        if *self.0 {
            *self.0 = false;
            Ok(())
        } else {
            Err(Error::NotInstalled)
        }
    }
}

impl<'a> From<&'a mut bool> for Installed<'a> {
    fn from(b: &'a mut bool) -> Self {
        Self(b)
    }
}

impl Display for Installed<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "currently installed: {}", self.0)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotInstalled => write!(f, "Item is not installed"),
            Error::AlreadyInstalled => write!(f, "Item is already installed"),
        }
    }
}

impl error::Error for Error {}
