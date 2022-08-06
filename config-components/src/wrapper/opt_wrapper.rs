use crate::{wrapper::ConfigWrapper, Component};

pub struct OptWrapper<Conf, Inner, Error>
where
    Conf: Component,
    Inner: ConfigWrapper<Error = Error>,
    Error: From<Conf::Error>,
{
    inner: Inner,
    conf: Option<Conf>,
}

impl<Conf, Inner, Error> OptWrapper<Conf, Inner, Error>
where
    Conf: Component,
    Inner: ConfigWrapper<Error = Error>,
    Error: From<Conf::Error>,
{
    pub fn new(inner: Inner, conf: Option<Conf>) -> Self {
        Self { inner, conf }
    }
}

impl<Conf, Inner, Error> ConfigWrapper for OptWrapper<Conf, Inner, Error>
where
    Conf: Component,
    Inner: ConfigWrapper<Error = Error>,
    Error: From<Conf::Error>,
{
    type Error = Error;

    fn install(&mut self) -> Result<(), Self::Error> {
        self.inner.install()?;
        if let Some(conf) = &mut self.conf {
            conf.install()?;
        }
        Ok(())
    }

    fn remove(&mut self) -> Result<(), Self::Error> {
        self.inner.install()?;
        if let Some(conf) = &mut self.conf {
            conf.install()?;
        }
        Ok(())
    }
}
