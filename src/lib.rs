use std::path::PathBuf;

use directories::ProjectDirs;
use log::{as_display, error, trace};
use miette::Diagnostic;
use namespace::Namespace;
use repository::Repository;
use thiserror::Error;

mod log_utils;
pub mod namespace;
pub mod repository;
pub mod template;

#[derive(Debug, Diagnostic, Error)]
pub enum Error {
    #[error("'{}' is not in any known namespace", .0.display())]
    HasNoNamespace(PathBuf),
    #[error("Failed to convert path to string")]
    PathConversionFail,
    #[error(transparent)]
    RepoError(#[from] repository::Error),
}

/// Panics on fail, use try_project_dirs if you want to catch errors.
pub fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("", "", "fig")
        .expect("Failed to find home directory, maybe your operating system is unsupported?")
}
pub fn try_project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("", "", "fig")
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
    Err(Error::HasNoNamespace(original_path.into()))
}
