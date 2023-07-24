use log::{as_display, error, trace};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::repository::Repository;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    StripPrefixError(#[from] std::path::StripPrefixError),
    #[error("'{}' is not in any known namespace", .0.display())]
    HasNoNamespace(PathBuf),
    #[error(transparent)]
    Repository(#[from] crate::repository::Error),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Namespace {
    /// The output location, where files are deployed to.
    pub target: PathBuf,
    /// The physical location of the namespace, where files are stored.
    pub location: PathBuf,
}

impl Namespace {
    pub fn files(&self) -> Result<Vec<PathBuf>, Error> {
        let mut files = vec![];
        self.recurse_dir(&self.location, &mut files, 50)?;
        Ok(files)
    }

    fn recurse_dir(&self, dir: &Path, files: &mut Vec<PathBuf>, depth: u8) -> Result<(), Error> {
        if depth == 0 {
            panic!("Overflowed depth")
        }
        for entry in dir.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if path.file_name().unwrap() != "namespace.fig" {
                    let relative_path = path.strip_prefix(&self.location)?;
                    let display_path = self.target.join(relative_path);
                    files.push(display_path);
                }
            } else {
                self.recurse_dir(&path, files, depth - 1)?;
            }
        }

        Ok(())
    }
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
