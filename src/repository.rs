use std::path::Path;
use std::{fs, path::PathBuf};

use log::debug;
use thiserror::Error;

use crate::namespace::Namespace;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Repository already initialised")]
    AlreadyInitialised,
    #[error("Repository not yet initialised")]
    NotInitialised,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Could not open git repository")]
    GitError(#[from] git2::Error),
    #[error(transparent)]
    TemplateError(#[from] crate::template::Error),
    #[error("{}", .0)]
    OpenError(#[source] Box<Self>, Vec<Self>),
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
        for entry in self.dir.read_dir()? {
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

    /// Open already existing repository
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let mut warnings = vec![];
        let path = path.as_ref();

        debug!("Opening repository at '{dir}'", dir = path.display());

        if !path.exists() {
            warnings.push(Error::NotInitialised);
        }
        let repository = git2::Repository::open(&path);
        if let Ok(repository) = repository {
            return Ok(Self {
                git_repository: repository,
                dir: path.to_path_buf(),
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
