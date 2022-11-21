use directories::ProjectDirs;
use miette::{Diagnostic, Result};
use thiserror::Error;

pub mod add;
pub mod namespace;
pub mod repository;
pub mod target;
pub mod template;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(std::io::Error))]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    #[diagnostic(code(toml::de::Error))]
    TomlDeError(#[from] toml::de::Error),
    #[error(transparent)]
    #[diagnostic(code(toml::ser::Error))]
    TomlSerError(#[from] toml::ser::Error),
    #[error(transparent)]
    #[diagnostic(code(git2::Error))]
    GitError(#[from] git2::Error),
    #[error("Retrieving project path failed")]
    #[diagnostic(code(fig::project_path_failed))]
    ProjectPathFailed,
    #[error("Path conversion failed")]
    #[diagnostic(code(fig::path_conversion_fail))]
    PathConversionFail,
}

fn project_dirs() -> Result<ProjectDirs> {
    Ok(ProjectDirs::from("", "", "fig").ok_or(Error::ProjectPathFailed)?)
}
