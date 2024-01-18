use std::fmt::Debug;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::instrument;

use crate::repository::Repository;

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
    pub targets: Vec<PathBuf>,
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
        assert!(depth != 0, "Overflowed depth");
        for entry in dir.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if path.file_name().unwrap() != "namespace.fig" {
                    for target in &self.targets {
                        let relative_path = path.strip_prefix(&self.location)?;
                        let display_path = target.join(relative_path);
                        files.push(display_path);
                    }
                }
            } else {
                self.recurse_dir(&path, files, depth - 1)?;
            }
        }

        Ok(())
    }
}

#[instrument(skip(repository), fields(repository = % repository.path().display()))]
pub fn determine_namespace(
    repository: &Repository,
    path: impl Into<PathBuf> + Debug,
) -> Result<Namespace, Error> {
    let original_path: PathBuf = path.into();
    let mut path = original_path.as_path();

    tracing::debug!("Determining namespace of '{}'", path.display());

    while let Some(parent) = path.parent() {
        path = parent;
        for ns in repository.namespaces()? {
            // Only allow the file to be added to the namespace if it is in all of the targets.
            if ns.targets.iter().all(|target| target == parent) {
                return Ok(ns);
            }
        }
    }

    tracing::error!("'{}' has no namespace", original_path.display());
    Err(Error::HasNoNamespace(original_path))
}
