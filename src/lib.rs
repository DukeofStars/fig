use std::path::PathBuf;
use directories::ProjectDirs;

use log::{as_display, error, trace};
use namespace::Namespace;
use repository::Repository;
use thiserror::Error;

mod log_utils;
pub mod namespace;
pub mod repository;
pub mod template;
pub mod info;

#[cfg(feature = "commands")]
pub mod commands;

pub fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("", "", "fig")
        .expect("Failed to find home directory, maybe your operating system is unsupported?")
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("'{}' is not in any known namespace", .0.display())]
    HasNoNamespace(PathBuf),
    #[error("Failed to convert path to string")]
    PathConversionFail,
    #[error(transparent)]
    RepoError(#[from] repository::Error),
}

pub fn determine_namespace(
    repository: &Repository,
    path: impl Into<PathBuf>,
) -> Result<Namespace, Error> {
    let original_path: PathBuf = path.into();
    let mut path = original_path.as_path();

    trace!(
        repository = as_display!(repository.dir.display());
        "Determining namespace of '{path}'",
        path=path.display()
    );

    while let Some(parent) = path.parent() {
        path = parent;
        for ns in repository.namespaces()? {
            if ns.target == parent {
                return Ok(ns);
            }
        }
    }

    error!("'{path}' has no namespace", path = original_path.display());
    Err(Error::HasNoNamespace(original_path))
}
