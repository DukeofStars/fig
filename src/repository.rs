use std::{collections::BTreeMap, fs, path::PathBuf};

use log::debug;
use miette::{bail, Diagnostic, Result};
use thiserror::Error;

use self::Error::*;
use crate::{template, Error::*, ManyError};

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
    SuperError(#[from] super::Error),
}

pub struct Repository {
    git_repository: git2::Repository,
    pub dir: PathBuf,
}

#[derive(clap::Args)]
pub struct RepositoryInitOptions {
    #[clap(short, long)]
    force: bool,
}

impl Repository {
    /// Returns list of namespaces and the paths they point to.
    ///
    /// **Note:** path does not point to the directory the namespace is in, but instead where it should be deployed
    pub fn namespaces(&self) -> Result<BTreeMap<String, PathBuf>> {
        let mut out = BTreeMap::new();
        for entry in Repository::dir()?.read_dir().map_err(IoError)? {
            let entry = entry.map_err(IoError)?;
            if entry.file_type().map_err(IoError)?.is_dir()
                && entry.path().join("namespace.fig").exists()
            {
                let path =
                    fs::read_to_string(entry.path().join("namespace.fig")).map_err(IoError)?;
                out.insert(
                    String::from(
                        entry
                            .file_name()
                            .into_string()
                            .map_err(|_| PathConversionFail)?,
                    ),
                    path.into(),
                );
            }
        }
        Ok(out)
    }

    pub fn dir() -> Result<PathBuf> {
        Ok(crate::project_dirs()?.data_dir().to_path_buf())
    }

    /// Create a repository
    pub fn init(options: RepositoryInitOptions) -> Result<Self> {
        let dir = Repository::dir()?;

        debug!("Creating repository at '{dir}'", dir = dir.display());

        if dir.exists() && !options.force {
            bail!(AlreadyInitialised)
        }

        fs::create_dir_all(&dir).map_err(IoError)?;

        template::generate(&dir)?;

        let dot_gitignore = "";
        fs::write(dir.join(".gitignore"), dot_gitignore).map_err(IoError)?;

        // Initialise git
        let repository = git2::Repository::init(&dir).map_err(GitError)?;

        Ok(Self {
            git_repository: repository,
            dir,
        })
    }

    /// Open already existing repository
    pub fn open() -> Result<Self> {
        let mut many_error = ManyError::new();
        let dir = Repository::dir()?;

        debug!("Opening repository at '{dir}'", dir = dir.display());

        if !dir.exists() {
            many_error.add(NotInitialised);
        }
        let repository = git2::Repository::open(&dir).map_err(GitError);
        if let Ok(repository) = repository {
            return Ok(Self {
                git_repository: repository,
                dir,
            });
        } else if let Err(e) = repository {
            many_error.add(SuperError(e));
            many_error.to_result()?;
        }
        unreachable!()
    }

    pub fn push(&self) -> Result<()> {
        self.git_repository
            .find_remote("origin")
            .map_err(GitError)?
            .push(&["master"], None)
            .map_err(GitError)?;
        Ok(())
    }
}
