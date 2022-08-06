use crate::{wrapper::ConfigWrapper, Component};

pub struct Wrapper<Conf, Inner, Error>
where
    Conf: Component,
    Inner: ConfigWrapper<Error = Error>,
    Error: From<Conf::Error>,
{
    inner: Inner,
    conf: Conf,
}

impl<Conf, Inner, Error> Wrapper<Conf, Inner, Error>
where
    Conf: Component,
    Inner: ConfigWrapper<Error = Error>,
    Error: From<Conf::Error>,
{
    pub fn new(inner: Inner, conf: Conf) -> Self {
        Self { inner, conf }
    }
}

impl<Conf, Inner, Error> ConfigWrapper for Wrapper<Conf, Inner, Error>
where
    Conf: Component,
    Inner: ConfigWrapper<Error = Error>,
    Error: From<Conf::Error>,
{
    type Error = Error;

    fn install(&mut self) -> Result<(), Self::Error> {
        self.inner.install()?;
        self.conf.install()?;
        Ok(())
    }

    fn remove(&mut self) -> Result<(), Self::Error> {
        self.inner.remove()?;
        self.remove()?;
        Ok(())
    }
}
