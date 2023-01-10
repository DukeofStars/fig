use std::path::{Path, PathBuf};

use miette::{bail, Diagnostic, Result};
use thiserror::Error;

use self::Error::*;
use crate::{repository::Repository, Error::*};

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("The file is not part of any known namespace")]
    #[diagnostic(code(fig::namespace::no_namespace))]
    HasNoNamespace,
}

pub fn strip_namespace(path: impl AsRef<Path>, file: impl AsRef<Path>) -> Option<PathBuf> {
    let path = path.as_ref().to_str()?;
    let file = file.as_ref().to_str()?.replace("\\\\?\\", "");
    let path = PathBuf::from(file.strip_prefix(path)?.trim_start_matches("\\"));

    Some(path)
}

pub fn determine_namespace(
    repository: &Repository,
    path: impl Into<PathBuf>,
) -> Result<(String, PathBuf)> {
    let mut path = path.into();
    while let Some(parent) = path.clone().parent() {
        path = parent
            .to_str()
            .ok_or(PathConversionFail)?
            .trim_start_matches("\\\\?\\")
            .into();
        for (name, path_to_check) in repository.namespaces()? {
            if path_to_check == parent {
                return Ok((name, path_to_check));
            }
        }
    }
    bail!(HasNoNamespace)
}
