use std::path::{Path, PathBuf};

use miette::{Diagnostic, Result, bail};
use thiserror::Error;

use self::Error::*;
use crate::repository::Repository;

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

pub fn determine_namespace(repository: &Repository, path: &PathBuf) -> Result<(String, PathBuf)> {
    let mut path = path.as_path();
    while let Some(parent) = path.parent() {
        path = parent;
        for (name, path_to_check) in repository.namespaces()? {
            if path_to_check == parent {
                return Ok((name, path_to_check));
            }
        }
    }
    bail!(HasNoNamespace)
}
