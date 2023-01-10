use directories::ProjectDirs;
use miette::{Diagnostic, Result};
use thiserror::Error;

pub mod add;
pub mod deploy;
pub mod namespace;
pub mod purge;
pub mod repository;
pub mod target;
pub mod template;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("Io error")]
    #[diagnostic(code(std::io::Error))]
    IoError(#[source] std::io::Error),
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

#[derive(Error, Diagnostic, Debug)]
#[error("uh oh")]
pub struct ManyError<E: Diagnostic> {
    #[related]
    errors: Vec<E>,
}

impl<E: Diagnostic> ManyError<E> {
    pub fn new() -> Self {
        Self { errors: vec![] }
    }

    pub fn add(&mut self, err: E) {
        self.errors.push(err);
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn has_err(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn to_result(self) -> Result<(), Self> {
        if self.has_err() {
            Err(self)
        } else {
            Ok(())
        }
    }
}

fn project_dirs() -> Result<ProjectDirs> {
    Ok(ProjectDirs::from("", "", "fig").ok_or(Error::ProjectPathFailed)?)
}
