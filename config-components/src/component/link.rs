use std::{
    error,
    fmt::Display,
    fs::{remove_file, symlink_metadata},
    io,
    os::unix::fs::symlink,
    path::{Path, PathBuf},
};

use crate::Component;

pub struct Link<'a> {
    src: &'a Path,
    dst: &'a Path,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum Error {
    CantCreateLink(io::Error, PathBuf),
    CantRemoveLink(io::Error, PathBuf),
    FileIsNotALink(PathBuf),
}

impl Component for Link<'_> {
    type Error = Error;

    fn install(&mut self) -> Result<(), Self::Error> {
        symlink(self.src, self.dst)
            .map_err(|err| Error::CantCreateLink(err, self.dst.to_path_buf()))?;
        Ok(())
    }

    fn remove(&mut self) -> Result<(), Self::Error> {
        // Would like to check if symlink points to src, but [Metadata] doesn't seems to expose that yet.
        if !symlink_metadata(self.dst)
            .map_err(|err| Error::CantRemoveLink(err, self.dst.to_path_buf()))?
            .is_symlink()
        {
            Err(Error::FileIsNotALink(self.dst.to_path_buf()))
        } else {
            remove_file(self.dst)
                .map_err(|err| Error::CantRemoveLink(err, self.dst.to_path_buf()))?;
            Ok(())
        }
    }
}

impl<'a> From<(&'a Path, &'a Path)> for Link<'a> {
    fn from((src, dst): (&'a Path, &'a Path)) -> Self {
        Self { src, dst }
    }
}

impl Display for Link<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{dst} -> {src}",
            dst = self.dst.display(),
            src = self.src.display()
        )
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::CantCreateLink(io_error, dst) => {
                write!(
                    f,
                    "couldn't create symbolic link at {dst}, because of {io_error}",
                    dst = dst.display()
                )
            }
            Error::CantRemoveLink(io_error, dst) => {
                write!(
                    f,
                    "couldn't remove symbolic link at {dst}, because of {io_error}",
                    dst = dst.display()
                )
            }
            Error::FileIsNotALink(dst) => {
                write!(
                    f,
                    "{dst} is not a symlink and will not be deleted",
                    dst = dst.display()
                )
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::CantCreateLink(err, _) => Some(err),
            Error::CantRemoveLink(err, _) => Some(err),
            Error::FileIsNotALink(_) => None,
        }
    }
}
