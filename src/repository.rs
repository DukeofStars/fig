use std::{fs, path::PathBuf};

use log::debug;
use miette::Diagnostic;
use thiserror::Error;

use crate::namespace::Namespace;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("Repository already initialised")]
    #[diagnostic(code(fig::repository::already_initialised))]
    AlreadyInitialised,
    #[error("Repository not yet initialised")]
    #[diagnostic(code(fig::repository::not_initialised))]
    #[help("Try `fig init` to intialise")]
    NotInitialised,
    #[error(transparent)]
    #[diagnostic(code(fig::io_error))]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    GitError(#[from] git2::Error),
    #[error(transparent)]
    TemplateError(#[from] crate::template::Error),
    #[error("{}", .0)]
    OpenError(#[source] Box<Self>, #[related] Vec<Self>),
}

pub struct Repository {
    git_repository: git2::Repository,
    pub dir: PathBuf,
}

impl Repository {
    /// Returns list of namespaces and the paths they point to.
    ///
    /// **Note:** path does not point to the directory the namespace is in, but instead where it should be deployed
    pub fn namespaces(&self) -> Result<Vec<Namespace>, Error> {
        let mut out = vec![];
        for entry in Repository::dir().read_dir()? {
            let entry = entry?;
            if entry.file_type()?.is_dir() && entry.path().join("namespace.fig").exists() {
                let path = fs::read_to_string(entry.path().join("namespace.fig"))?;
                let namespace = Namespace {
                    target: PathBuf::from(path).canonicalize()?,
                    location: entry.path().canonicalize()?,
                };
                out.push(namespace);
            }
        }
        Ok(out)
    }

    pub fn dir() -> PathBuf {
        crate::project_dirs().data_dir().to_path_buf()
    }

    /// Open already existing repository
    pub fn open() -> Result<Self, Error> {
        let mut warnings = vec![];
        let dir = Repository::dir();

        debug!("Opening repository at '{dir}'", dir = dir.display());

        if !dir.exists() {
            warnings.push(Error::NotInitialised);
        }
        let repository = git2::Repository::open(&dir);
        if let Ok(repository) = repository {
            return Ok(Self {
                git_repository: repository,
                dir,
            });
        } else if let Err(e) = repository {
            return Err(Error::OpenError(Box::new(Error::from(e)), warnings));
        }
        unreachable!()
    }

    pub fn push(&self) -> Result<(), Error> {
        self.git_repository
            .find_remote("origin")?
            .push(&["master"], None)?;
        Ok(())
    }
}
