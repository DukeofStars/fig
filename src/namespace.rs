use std::path::{Path, PathBuf};

use miette::{Diagnostic, Result};
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
    dbg!(&path);
    let file = file.as_ref().to_str()?.replace("\\\\?\\", "");
    dbg!(&file);

    Some(PathBuf::from(file.strip_prefix(path)?))
}

pub fn determine_namespace(repository: &Repository, path: &PathBuf) -> Result<(String, PathBuf)> {
    let mut out: Option<(String, PathBuf)> = None;
    let mut path = path.as_path();
    while let Some(parent) = path.parent() {
        path = parent;
        for (name, path_to_check) in repository.namespaces()? {
            if path_to_check == parent {
                out = Some((name, path_to_check));
            }
        }
    }
    Ok(out.ok_or(HasNoNamespace)?)
}
